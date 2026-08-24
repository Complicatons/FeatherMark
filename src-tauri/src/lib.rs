use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag};
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::{
    path::BaseDirectory, AppHandle, DragDropEvent, Emitter, LogicalSize, Manager, RunEvent, Size,
    State, WebviewWindow, WindowEvent,
};
use tauri_plugin_dialog::DialogExt;
use url::Url;

mod platform;

const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;
const APP_NAME: &str = "FeatherMark";
const DEFAULT_THEME: &str = "dracula";
const THEME_IDS: &[&str] = &[
    "system",
    "light",
    "dark",
    "github-light",
    "github-dark",
    "nord",
    "solarized-light",
    "solarized-dark",
    "sepia",
    "dracula",
];

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DocumentPayload {
    id: u64,
    path: String,
    filename: String,
    source: String,
    saved_source: String,
    html: String,
    dirty: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CloseDocumentPayload {
    active: Option<DocumentPayload>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TabPayload {
    id: u64,
    path: String,
    filename: String,
    dirty: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct InitialDocumentsPayload {
    tabs: Vec<TabPayload>,
    active: Option<DocumentPayload>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ImagePayload {
    bytes: Vec<u8>,
    mime_type: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DirectoryEntry {
    name: String,
    path: String,
    is_directory: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DirectoryListing {
    root_name: String,
    root_path: String,
    entries: Vec<DirectoryEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Preferences {
    theme: String,
    text_size: u8,
    window_width: u32,
    window_height: u32,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            theme: DEFAULT_THEME.into(),
            text_size: 17,
            window_width: 1040,
            window_height: 750,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct DocumentState {
    id: u64,
    path: Option<PathBuf>,
    source: String,
    saved_source: String,
    dirty: bool,
}

#[derive(Debug)]
struct DocumentsState {
    tabs: Vec<DocumentState>,
    active_id: Option<u64>,
    next_id: u64,
}

impl Default for DocumentsState {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            active_id: None,
            next_id: 1,
        }
    }
}

impl DocumentsState {
    fn active(&self) -> Option<&DocumentState> {
        let id = self.active_id?;
        self.tabs.iter().find(|document| document.id == id)
    }

    fn document(&self, id: u64) -> Option<&DocumentState> {
        self.tabs.iter().find(|document| document.id == id)
    }

    fn document_mut(&mut self, id: u64) -> Option<&mut DocumentState> {
        self.tabs.iter_mut().find(|document| document.id == id)
    }

    fn add_or_activate(&mut self, path: PathBuf, source: String) -> u64 {
        let normalized = path.canonicalize().unwrap_or(path);
        if let Some(existing) = self
            .tabs
            .iter()
            .find(|document| document.path.as_ref() == Some(&normalized))
        {
            self.active_id = Some(existing.id);
            return existing.id;
        }
        let id = self.next_id;
        self.next_id += 1;
        let mut document = DocumentState {
            id,
            ..DocumentState::default()
        };
        document.replace(normalized, source);
        self.tabs.push(document);
        self.active_id = Some(id);
        id
    }

    fn any_dirty(&self) -> bool {
        self.tabs.iter().any(|document| document.dirty)
    }

    fn remove(&mut self, id: u64) -> bool {
        let Some(index) = self.tabs.iter().position(|document| document.id == id) else {
            return false;
        };
        self.tabs.remove(index);
        if self.active_id == Some(id) {
            self.active_id = self
                .tabs
                .get(index.min(self.tabs.len().saturating_sub(1)))
                .map(|document| document.id);
        }
        true
    }
}

impl DocumentState {
    fn replace(&mut self, path: PathBuf, source: String) {
        self.path = Some(path);
        self.saved_source = source.clone();
        self.source = source;
        self.dirty = false;
    }

    fn update(&mut self, source: String) -> bool {
        self.source = source;
        self.dirty = self.source != self.saved_source;
        self.dirty
    }

    fn mark_saved(&mut self) {
        self.saved_source = self.source.clone();
        self.dirty = false;
    }
}

struct AppState {
    documents: Mutex<DocumentsState>,
    preferences: Mutex<Preferences>,
    preferences_path: Option<PathBuf>,
}

fn is_portable_executable(path: &Path) -> bool {
    path.file_stem()
        .and_then(OsStr::to_str)
        .is_some_and(|name| name.to_ascii_lowercase().contains("portable"))
}

fn is_markdown_path(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown"))
}

fn path_for_ui(path: &Path) -> String {
    let value = path.to_string_lossy();
    #[cfg(windows)]
    {
        value.strip_prefix(r"\\?\").unwrap_or(&value).to_owned()
    }
    #[cfg(not(windows))]
    value.into_owned()
}

fn directory_listing_for(document_path: &Path) -> Result<DirectoryListing, String> {
    let root = document_path
        .parent()
        .ok_or("The document has no parent directory.")?;
    let mut entries = Vec::new();

    for item in fs::read_dir(root).map_err(|error| format!("Could not read the folder: {error}"))? {
        let item = item.map_err(|error| format!("Could not read a folder entry: {error}"))?;
        let path = item.path();
        let name = item.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let is_directory = item
            .file_type()
            .map_err(|error| format!("Could not inspect {name}: {error}"))?
            .is_dir();
        if is_directory || is_markdown_path(&path) {
            entries.push(DirectoryEntry {
                name,
                path: path_for_ui(&path),
                is_directory,
            });
        }
        if entries.len() >= 500 {
            break;
        }
    }

    entries.sort_by(|left, right| {
        right
            .is_directory
            .cmp(&left.is_directory)
            .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
    });
    Ok(DirectoryListing {
        root_name: root
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("Folder")
            .to_owned(),
        root_path: path_for_ui(root),
        entries,
    })
}

fn read_markdown_file(path: &Path) -> Result<String, String> {
    if !is_markdown_path(path) {
        return Err("Choose a .md or .markdown file.".into());
    }
    let metadata =
        fs::metadata(path).map_err(|error| format!("Could not open the file: {error}"))?;
    if !metadata.is_file() {
        return Err("The selected path is not a file.".into());
    }
    if metadata.len() > MAX_FILE_BYTES {
        return Err("This file is larger than FeatherMark's 16 MB safety limit.".into());
    }
    let bytes = fs::read(path).map_err(|error| format!("Could not read the file: {error}"))?;
    String::from_utf8(bytes).map_err(|_| "This file is not valid UTF-8.".into())
}

fn link_destination_is_safe(destination: &str) -> bool {
    let trimmed = destination.trim();
    if trimmed.starts_with('#') || !trimmed.contains(':') {
        return true;
    }
    Url::parse(trimmed)
        .map(|url| matches!(url.scheme(), "http" | "https" | "mailto"))
        .unwrap_or(false)
}

fn render_markdown(source: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_HEADING_ATTRIBUTES;

    let events = Parser::new_ext(source, options).map(|event| match event {
        // Raw Markdown HTML is displayed as text, never interpreted by the webview.
        Event::Html(raw) | Event::InlineHtml(raw) => Event::Text(raw),
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) if !link_destination_is_safe(&dest_url) => Event::Start(Tag::Link {
            link_type,
            dest_url: CowStr::Borrowed("#blocked-link"),
            title,
            id,
        }),
        other => other,
    });

    let mut output = String::with_capacity(source.len() + source.len() / 3);
    html::push_html(&mut output, events);
    output
}

fn title_for(document: Option<&DocumentState>) -> String {
    match document.and_then(|document| {
        document
            .path
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(OsStr::to_str)
            .map(|filename| (filename, document.dirty))
    }) {
        Some((filename, true)) => format!("*{filename} — {APP_NAME}"),
        Some((filename, false)) => format!("{filename} — {APP_NAME}"),
        None => APP_NAME.into(),
    }
}

fn payload_for(document: &DocumentState) -> DocumentPayload {
    let path = document.path.as_deref().unwrap_or_else(|| Path::new(""));
    DocumentPayload {
        id: document.id,
        path: path_for_ui(path),
        filename: path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("Markdown")
            .into(),
        html: render_markdown(&document.source),
        source: document.source.clone(),
        saved_source: document.saved_source.clone(),
        dirty: document.dirty,
    }
}

fn tab_payload_for(document: &DocumentState) -> TabPayload {
    let path = document.path.as_deref().unwrap_or_else(|| Path::new(""));
    TabPayload {
        id: document.id,
        path: path_for_ui(path),
        filename: path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("Markdown")
            .into(),
        dirty: document.dirty,
    }
}

fn open_path_inner(
    window: &WebviewWindow,
    state: &AppState,
    path: PathBuf,
) -> Result<DocumentPayload, String> {
    let source = read_markdown_file(&path)?;
    let mut documents = state
        .documents
        .lock()
        .map_err(|_| "Document state is unavailable.")?;
    let id = documents.add_or_activate(path, source);
    let document = documents
        .document(id)
        .ok_or("The opened document is unavailable.")?;
    let payload = payload_for(document);
    window
        .set_title(&title_for(Some(document)))
        .map_err(|error| error.to_string())?;
    Ok(payload)
}

fn save_preferences(state: &AppState) -> Result<(), String> {
    let Some(preferences_path) = state.preferences_path.as_ref() else {
        return Ok(());
    };
    let preferences = state
        .preferences
        .lock()
        .map_err(|_| "Preferences are unavailable.")?;
    let json = serde_json::to_string_pretty(&*preferences).map_err(|error| error.to_string())?;
    if let Some(parent) = preferences_path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(preferences_path, json).map_err(|error| error.to_string())
}

fn load_preferences(path: &Path) -> Preferences {
    fs::read_to_string(path)
        .ok()
        .and_then(|source| serde_json::from_str(&source).ok())
        .filter(|prefs: &Preferences| {
            theme_is_valid(&prefs.theme)
                && (13..=24).contains(&prefs.text_size)
                && prefs.window_width >= 640
                && prefs.window_height >= 480
        })
        .unwrap_or_default()
}

fn theme_is_valid(theme: &str) -> bool {
    THEME_IDS.contains(&theme)
}

fn initial_cli_paths() -> Vec<PathBuf> {
    std::env::args_os()
        .skip(1)
        .map(PathBuf::from)
        .filter(|path| is_markdown_path(path) && path.is_file())
        .collect()
}

#[tauri::command]
fn initial_documents(state: State<'_, AppState>) -> InitialDocumentsPayload {
    let Ok(documents) = state.documents.lock() else {
        return InitialDocumentsPayload {
            tabs: Vec::new(),
            active: None,
        };
    };
    InitialDocumentsPayload {
        tabs: documents.tabs.iter().map(tab_payload_for).collect(),
        active: documents.active().map(payload_for),
    }
}

#[tauri::command]
fn activate_document(
    window: WebviewWindow,
    state: State<'_, AppState>,
    id: u64,
) -> Result<DocumentPayload, String> {
    let mut documents = state
        .documents
        .lock()
        .map_err(|_| "Document state is unavailable.")?;
    if documents.document(id).is_none() {
        return Err("That tab is no longer open.".into());
    }
    documents.active_id = Some(id);
    let document = documents
        .document(id)
        .ok_or("That tab is no longer open.")?;
    window
        .set_title(&title_for(Some(document)))
        .map_err(|error| error.to_string())?;
    Ok(payload_for(document))
}

#[tauri::command]
fn list_document_directory(
    state: State<'_, AppState>,
    id: u64,
) -> Result<DirectoryListing, String> {
    let documents = state
        .documents
        .lock()
        .map_err(|_| "Document state is unavailable.")?;
    let path = documents
        .document(id)
        .and_then(|document| document.path.as_ref())
        .ok_or("That tab is no longer open.")?;
    directory_listing_for(path)
}

#[tauri::command]
fn close_document(
    window: WebviewWindow,
    state: State<'_, AppState>,
    id: u64,
    discard: bool,
) -> Result<CloseDocumentPayload, String> {
    let mut documents = state
        .documents
        .lock()
        .map_err(|_| "Document state is unavailable.")?;
    let document = documents
        .document(id)
        .ok_or("That tab is no longer open.")?;
    if document.dirty && !discard {
        return Err("Save or discard the unsaved changes before closing.".into());
    }
    documents.remove(id);
    let active = documents.active().map(payload_for);
    window
        .set_title(&title_for(documents.active()))
        .map_err(|error| error.to_string())?;
    Ok(CloseDocumentPayload { active })
}

#[tauri::command]
fn get_preferences(state: State<'_, AppState>) -> Preferences {
    state
        .preferences
        .lock()
        .map(|prefs| prefs.clone())
        .unwrap_or_default()
}

#[tauri::command]
fn update_preferences(
    state: State<'_, AppState>,
    theme: String,
    text_size: u8,
) -> Result<(), String> {
    if !theme_is_valid(&theme) || !(13..=24).contains(&text_size) {
        return Err("Invalid preferences.".into());
    }
    let mut preferences = state
        .preferences
        .lock()
        .map_err(|_| "Preferences are unavailable.")?;
    preferences.theme = theme;
    preferences.text_size = text_size;
    drop(preferences);
    save_preferences(&state)
}

#[tauri::command]
fn render_source(source: String) -> String {
    render_markdown(&source)
}

#[tauri::command]
fn choose_markdown(
    window: WebviewWindow,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Option<DocumentPayload>, String> {
    let selected = app
        .dialog()
        .file()
        .add_filter("Markdown", &["md", "markdown"])
        .blocking_pick_file();
    let Some(path) = selected.and_then(|file| file.as_path().map(Path::to_path_buf)) else {
        return Ok(None);
    };
    open_path_inner(&window, &state, path).map(Some)
}

#[tauri::command]
fn open_path(
    window: WebviewWindow,
    state: State<'_, AppState>,
    path: String,
) -> Result<DocumentPayload, String> {
    open_path_inner(&window, &state, PathBuf::from(path))
}

#[tauri::command]
fn update_dirty(
    window: WebviewWindow,
    state: State<'_, AppState>,
    id: u64,
    source: String,
) -> Result<bool, String> {
    let mut documents = state
        .documents
        .lock()
        .map_err(|_| "Document state is unavailable.")?;
    let is_active = documents.active_id == Some(id);
    let document = documents
        .document_mut(id)
        .ok_or("That tab is no longer open.")?;
    let dirty = document.update(source);
    if is_active {
        window
            .set_title(&title_for(Some(document)))
            .map_err(|error| error.to_string())?;
    }
    Ok(dirty)
}

fn write_document(
    window: &WebviewWindow,
    state: &AppState,
    id: u64,
    path: PathBuf,
    source: String,
) -> Result<DocumentPayload, String> {
    if !is_markdown_path(&path) {
        return Err("Save with a .md or .markdown extension.".into());
    }
    fs::write(&path, source.as_bytes())
        .map_err(|error| format!("Could not save the file: {error}"))?;
    let mut documents = state
        .documents
        .lock()
        .map_err(|_| "Document state is unavailable.")?;
    let is_active = documents.active_id == Some(id);
    let document = documents
        .document_mut(id)
        .ok_or("That tab is no longer open.")?;
    document.path = Some(path.canonicalize().unwrap_or(path));
    document.source = source;
    document.mark_saved();
    let payload = payload_for(document);
    if is_active {
        window
            .set_title(&title_for(Some(document)))
            .map_err(|error| error.to_string())?;
    }
    Ok(payload)
}

#[tauri::command]
fn save_document(
    window: WebviewWindow,
    state: State<'_, AppState>,
    id: u64,
    source: String,
) -> Result<DocumentPayload, String> {
    let path = state
        .documents
        .lock()
        .map_err(|_| "Document state is unavailable.")?
        .document(id)
        .and_then(|document| document.path.clone())
        .ok_or("That tab is no longer open.")?;
    write_document(&window, &state, id, path, source)
}

#[tauri::command]
fn save_document_as(
    window: WebviewWindow,
    app: AppHandle,
    state: State<'_, AppState>,
    id: u64,
    source: String,
) -> Result<Option<DocumentPayload>, String> {
    let current_name = state
        .documents
        .lock()
        .ok()
        .and_then(|documents| {
            documents
                .document(id)?
                .path
                .as_ref()
                .and_then(|path| path.file_name())
                .and_then(OsStr::to_str)
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "document.md".into());
    let selected = app
        .dialog()
        .file()
        .add_filter("Markdown", &["md", "markdown"])
        .set_file_name(&current_name)
        .blocking_save_file();
    let Some(path) = selected.and_then(|file| file.as_path().map(Path::to_path_buf)) else {
        return Ok(None);
    };
    write_document(&window, &state, id, path, source).map(Some)
}

#[tauri::command]
fn reload_document(
    window: WebviewWindow,
    state: State<'_, AppState>,
    id: u64,
) -> Result<DocumentPayload, String> {
    let path = state
        .documents
        .lock()
        .map_err(|_| "Document state is unavailable.")?
        .document(id)
        .and_then(|document| document.path.clone())
        .ok_or("That tab is no longer open.")?;
    let source = read_markdown_file(&path)?;
    let mut documents = state
        .documents
        .lock()
        .map_err(|_| "Document state is unavailable.")?;
    let is_active = documents.active_id == Some(id);
    let document = documents
        .document_mut(id)
        .ok_or("That tab is no longer open.")?;
    document.replace(path, source);
    let payload = payload_for(document);
    if is_active {
        window
            .set_title(&title_for(Some(document)))
            .map_err(|error| error.to_string())?;
    }
    Ok(payload)
}

fn resolve_relative_markdown(document_path: &Path, requested: &str) -> Result<PathBuf, String> {
    let decoded = percent_decode_simple(requested);
    let path_part = decoded.split('#').next().unwrap_or("").trim();
    let requested_path = Path::new(path_part);
    if path_part.is_empty()
        || requested_path.is_absolute()
        || path_part.contains("://")
        || path_part.starts_with("data:")
    {
        return Err("Only relative Markdown links can be opened here.".into());
    }
    let base = document_path
        .parent()
        .ok_or("The document has no parent directory.")?;
    let target = base.join(requested_path);
    if !is_markdown_path(&target) {
        return Err("That link is not a Markdown document.".into());
    }
    target
        .canonicalize()
        .map_err(|_| "The linked Markdown file was not found.".into())
}

#[tauri::command]
fn open_relative_markdown(
    window: WebviewWindow,
    state: State<'_, AppState>,
    id: u64,
    target: String,
    new_tab: bool,
) -> Result<DocumentPayload, String> {
    let current_path = state
        .documents
        .lock()
        .map_err(|_| "Document state is unavailable.")?
        .document(id)
        .and_then(|document| document.path.clone())
        .ok_or("That tab is no longer open.")?;
    let target_path = resolve_relative_markdown(&current_path, &target)?;
    if new_tab {
        return open_path_inner(&window, &state, target_path);
    }

    let source = read_markdown_file(&target_path)?;
    let mut documents = state
        .documents
        .lock()
        .map_err(|_| "Document state is unavailable.")?;
    let document = documents
        .document_mut(id)
        .ok_or("That tab is no longer open.")?;
    document.replace(target_path, source);
    let payload = payload_for(document);
    window
        .set_title(&title_for(Some(document)))
        .map_err(|error| error.to_string())?;
    Ok(payload)
}

fn resolve_local_image(document_path: &Path, requested: &str) -> Result<PathBuf, String> {
    let requested_path = Path::new(requested);
    if requested.trim().is_empty()
        || requested_path.is_absolute()
        || requested.contains("://")
        || requested.starts_with("data:")
    {
        return Err("Only relative local images are allowed.".into());
    }
    let base = document_path
        .parent()
        .ok_or("The document has no parent directory.")?;
    let canonical_base = base
        .canonicalize()
        .map_err(|_| "Could not resolve the document directory.")?;
    let canonical_target = base
        .join(requested_path)
        .canonicalize()
        .map_err(|_| "Image not found.")?;
    if !canonical_target.starts_with(&canonical_base) {
        return Err("Images outside the document directory are blocked.".into());
    }
    Ok(canonical_target)
}

fn image_mime(path: &Path) -> Option<&'static str> {
    match path
        .extension()
        .and_then(OsStr::to_str)?
        .to_ascii_lowercase()
        .as_str()
    {
        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "gif" => Some("image/gif"),
        "webp" => Some("image/webp"),
        "bmp" => Some("image/bmp"),
        _ => None,
    }
}

#[tauri::command]
fn read_local_image(
    state: State<'_, AppState>,
    id: u64,
    path: String,
) -> Result<ImagePayload, String> {
    let document_path = state
        .documents
        .lock()
        .map_err(|_| "Document state is unavailable.")?
        .document(id)
        .and_then(|document| document.path.clone())
        .ok_or("That tab is no longer open.")?;
    let decoded = percent_decode_simple(&path);
    let resolved = resolve_local_image(&document_path, &decoded)?;
    let mime_type = image_mime(&resolved).ok_or("Unsupported image type.")?;
    let metadata = fs::metadata(&resolved).map_err(|_| "Image not found.")?;
    if metadata.len() > MAX_FILE_BYTES {
        return Err("This image is larger than 16 MB.".into());
    }
    let bytes =
        fs::read(&resolved).map_err(|error| format!("Could not read the image: {error}"))?;
    Ok(ImagePayload {
        bytes,
        mime_type: mime_type.into(),
    })
}

fn percent_decode_simple(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let (Some(high), Some(low)) =
                (hex_value(bytes[index + 1]), hex_value(bytes[index + 2]))
            {
                output.push(high * 16 + low);
                index += 3;
                continue;
            }
        }
        output.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&output).into_owned()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[tauri::command]
fn open_external(url: String) -> Result<(), String> {
    let parsed = Url::parse(&url).map_err(|_| "That link is not valid.")?;
    if !matches!(parsed.scheme(), "http" | "https" | "mailto") {
        return Err("Only web and email links can be opened externally.".into());
    }
    platform::open_external(parsed.as_str())
}

#[tauri::command]
fn toggle_fullscreen(window: WebviewWindow) -> Result<(), String> {
    let fullscreen = window.is_fullscreen().map_err(|error| error.to_string())?;
    window
        .set_fullscreen(!fullscreen)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn force_close(_app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    save_preferences(&state)?;
    std::process::exit(0)
}

pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let portable = std::env::current_exe()
                .map(|path| is_portable_executable(&path))
                .unwrap_or(false);
            let preferences_path = if portable {
                None
            } else {
                Some(
                    app.path()
                        .resolve("preferences.json", BaseDirectory::AppConfig)
                        .map_err(|error| error.to_string())?,
                )
            };
            let preferences = preferences_path
                .as_deref()
                .map(load_preferences)
                .unwrap_or_default();
            let mut documents = DocumentsState::default();
            for path in initial_cli_paths() {
                if let Ok(source) = read_markdown_file(&path) {
                    documents.add_or_activate(path, source);
                }
            }
            let title = title_for(documents.active());
            app.manage(AppState {
                documents: Mutex::new(documents),
                preferences: Mutex::new(preferences.clone()),
                preferences_path,
            });
            if let Some(window) = app.get_webview_window("main") {
                window.set_title(&title)?;
                window.set_size(Size::Logical(LogicalSize::new(
                    preferences.window_width as f64,
                    preferences.window_height as f64,
                )))?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            initial_documents,
            activate_document,
            list_document_directory,
            close_document,
            get_preferences,
            update_preferences,
            render_source,
            choose_markdown,
            open_path,
            update_dirty,
            save_document,
            save_document_as,
            reload_document,
            open_relative_markdown,
            read_local_image,
            open_external,
            toggle_fullscreen,
            force_close,
        ])
        .build(tauri::generate_context!())
        .expect("FeatherMark failed to start");

    app.run(|app_handle, event| match event {
        RunEvent::WindowEvent { label, event, .. } if label == "main" => {
            let state = app_handle.state::<AppState>();
            let Some(window) = app_handle.get_webview_window("main") else {
                return;
            };
            match event {
                WindowEvent::Resized(size) => {
                    if let Ok(mut preferences) = state.preferences.lock() {
                        let scale = window.scale_factor().unwrap_or(1.0);
                        let logical = size.to_logical::<u32>(scale);
                        if logical.width >= 640 && logical.height >= 480 {
                            preferences.window_width = logical.width;
                            preferences.window_height = logical.height;
                        }
                    }
                }
                WindowEvent::DragDrop(DragDropEvent::Drop { paths, .. }) => {
                    if let Some(path) = paths.iter().find(|path| is_markdown_path(path)) {
                        let _ = window.emit("markdown-drop", path.to_string_lossy().into_owned());
                    } else {
                        let _ = window.emit("app-error", "Drop a .md or .markdown file.");
                    }
                }
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    let dirty = state
                        .documents
                        .lock()
                        .map(|documents| documents.any_dirty())
                        .unwrap_or(false);
                    if dirty {
                        let _ = window.emit("confirm-close", ());
                    } else {
                        let _ = save_preferences(&state);
                        std::process::exit(0);
                    }
                }
                WindowEvent::Destroyed => {
                    let _ = save_preferences(&state);
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        RunEvent::ExitRequested { api, .. } => {
            let state = app_handle.state::<AppState>();
            let dirty = state
                .documents
                .lock()
                .map(|documents| documents.any_dirty())
                .unwrap_or(false);
            if dirty {
                api.prevent_exit();
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.emit("confirm-close", ());
                }
            } else {
                let _ = save_preferences(&state);
                std::process::exit(0);
            }
        }
        #[cfg(target_os = "macos")]
        RunEvent::Opened { urls } => {
            let paths = urls
                .iter()
                .filter_map(|url| url.to_file_path().ok())
                .filter(|path| is_markdown_path(path))
                .collect::<Vec<_>>();
            if paths.is_empty() {
                return;
            }

            let state = app_handle.state::<AppState>();
            if let Ok(mut documents) = state.documents.lock() {
                for path in paths {
                    if let Ok(source) = read_markdown_file(&path) {
                        documents.add_or_activate(path, source);
                    }
                }
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.set_title(&title_for(documents.active()));
                    let _ = window.emit("documents-opened", ());
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            };
        }
        _ => {}
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_gfm_and_escapes_raw_html() {
        let source = "# Heading\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n- [x] done\n\n<script>alert(1)</script>";
        let html = render_markdown(source);
        assert!(html.contains("<table>"));
        assert!(html.contains("type=\"checkbox\""));
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn blocks_script_links_but_keeps_web_links() {
        let html = render_markdown("[bad](javascript:alert(1)) [good](https://example.com)");
        assert!(!html.contains("javascript:"));
        assert!(html.contains("https://example.com"));
    }

    #[test]
    fn document_dirty_state_tracks_saved_content() {
        let mut state = DocumentState::default();
        state.replace(PathBuf::from("test.md"), "saved".into());
        assert!(!state.update("saved".into()));
        assert!(state.update("changed".into()));
        state.mark_saved();
        assert!(!state.dirty);
    }

    #[test]
    fn only_markdown_extensions_are_accepted() {
        assert!(is_markdown_path(Path::new("README.md")));
        assert!(is_markdown_path(Path::new("notes.MARKDOWN")));
        assert!(!is_markdown_path(Path::new("notes.txt")));
    }

    #[test]
    fn file_loading_accepts_utf8_and_rejects_invalid_utf8() {
        let root =
            std::env::temp_dir().join(format!("feathermark-load-test-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let valid = root.join("valid.md");
        let invalid = root.join("invalid.md");
        fs::write(&valid, "Hello, 日本語 🪶").unwrap();
        fs::write(&invalid, [0xff, 0xfe, 0xfd]).unwrap();
        assert_eq!(read_markdown_file(&valid).unwrap(), "Hello, 日本語 🪶");
        assert_eq!(
            read_markdown_file(&invalid).unwrap_err(),
            "This file is not valid UTF-8."
        );
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn relative_images_cannot_escape_document_directory() {
        let root = std::env::temp_dir().join(format!("feathermark-test-{}", std::process::id()));
        let docs = root.join("docs");
        fs::create_dir_all(&docs).unwrap();
        fs::write(docs.join("doc.md"), "![image](image.png)").unwrap();
        fs::write(docs.join("image.png"), b"png").unwrap();
        fs::write(root.join("outside.png"), b"png").unwrap();
        let resolved = resolve_local_image(&docs.join("doc.md"), "image.png").unwrap();
        assert_eq!(resolved, docs.join("image.png").canonicalize().unwrap());
        assert!(resolve_local_image(&docs.join("doc.md"), "../outside.png").is_err());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn percent_decoding_handles_image_paths() {
        assert_eq!(percent_decode_simple("images/a%20b.png"), "images/a b.png");
    }

    #[test]
    fn directory_listing_is_shallow_and_markdown_only() {
        let root =
            std::env::temp_dir().join(format!("feathermark-directory-test-{}", std::process::id()));
        fs::create_dir_all(root.join("images")).unwrap();
        fs::write(root.join("one.md"), "# One").unwrap();
        fs::write(root.join("two.markdown"), "# Two").unwrap();
        fs::write(root.join("ignored.txt"), "ignored").unwrap();

        let listing = directory_listing_for(&root.join("one.md")).unwrap();
        assert!(listing
            .entries
            .iter()
            .any(|entry| entry.name == "images" && entry.is_directory));
        assert!(listing.entries.iter().any(|entry| entry.name == "one.md"));
        assert!(listing
            .entries
            .iter()
            .any(|entry| entry.name == "two.markdown"));
        assert!(!listing
            .entries
            .iter()
            .any(|entry| entry.name == "ignored.txt"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn tabs_are_deduplicated_and_keep_independent_dirty_state() {
        let root =
            std::env::temp_dir().join(format!("feathermark-tabs-test-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let first = root.join("first.md");
        let second = root.join("second.md");
        fs::write(&first, "# First").unwrap();
        fs::write(&second, "# Second").unwrap();

        let mut documents = DocumentsState::default();
        let first_id = documents.add_or_activate(first.clone(), "# First".into());
        let second_id = documents.add_or_activate(second, "# Second".into());
        documents
            .document_mut(first_id)
            .unwrap()
            .update("# Changed".into());
        let duplicate_id = documents.add_or_activate(first, "ignored reload".into());

        assert_eq!(duplicate_id, first_id);
        assert_eq!(documents.tabs.len(), 2);
        assert_eq!(documents.active_id, Some(first_id));
        assert!(documents.document(first_id).unwrap().dirty);
        assert!(!documents.document(second_id).unwrap().dirty);
        assert_eq!(documents.document(first_id).unwrap().source, "# Changed");
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn closing_the_active_tab_selects_a_neighbour() {
        let mut documents = DocumentsState::default();
        let first = documents.add_or_activate(PathBuf::from("first.md"), "first".into());
        let second = documents.add_or_activate(PathBuf::from("second.md"), "second".into());
        let third = documents.add_or_activate(PathBuf::from("third.md"), "third".into());

        documents.active_id = Some(second);
        assert!(documents.remove(second));
        assert_eq!(documents.active_id, Some(third));
        assert!(documents.remove(third));
        assert_eq!(documents.active_id, Some(first));
    }

    #[test]
    fn relative_markdown_links_resolve_but_non_markdown_links_do_not() {
        let root =
            std::env::temp_dir().join(format!("feathermark-links-test-{}", std::process::id()));
        fs::create_dir_all(root.join("docs")).unwrap();
        let current = root.join("docs").join("current.md");
        let target = root.join("docs").join("linked file.md");
        fs::write(&current, "[Linked](linked%20file.md)").unwrap();
        fs::write(&target, "# Linked").unwrap();

        assert_eq!(
            resolve_relative_markdown(&current, "linked%20file.md#section").unwrap(),
            target.canonicalize().unwrap()
        );
        assert!(resolve_relative_markdown(&current, "image.png").is_err());
        assert!(resolve_relative_markdown(&current, "https://example.com/doc.md").is_err());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn portable_mode_is_selected_only_by_the_distribution_filename() {
        assert!(is_portable_executable(Path::new(
            "FeatherMark-0.2.0-windows-x64-portable.exe"
        )));
        assert!(!is_portable_executable(Path::new("feathermark.exe")));
    }

    #[test]
    fn bundled_theme_names_are_validated() {
        for theme in THEME_IDS {
            assert!(theme_is_valid(theme));
        }
        assert!(!theme_is_valid("remote-theme"));
        assert!(!theme_is_valid(""));
    }

    #[test]
    fn new_users_start_with_the_dracula_theme() {
        assert_eq!(Preferences::default().theme, "dracula");
        assert!(theme_is_valid(DEFAULT_THEME));
    }
}
