const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const $ = (selector) => document.querySelector(selector);
const $$ = (selector) => [...document.querySelectorAll(selector)];
const isMac = /Mac/.test(navigator.platform);
const primaryKeyLabel = isMac ? "⌘" : "Ctrl";

const elements = {
  open: $("#open-button"),
  emptyOpen: $("#empty-open-button"),
  newTab: $("#new-tab-button"),
  quickEdit: $("#quick-edit-button"),
  mode: $("#mode-button"),
  save: $("#save-button"),
  saveAs: $("#save-as-button"),
  close: $("#close-button"),
  reload: $("#reload-button"),
  fullscreen: $("#fullscreen-button"),
  about: $("#about-button"),
  aboutDialog: $("#about-dialog"),
  empty: $("#empty-state"),
  preview: $("#preview"),
  editor: $("#editor"),
  mainArea: $("#main-area"),
  contextMenu: $("#document-context-menu"),
  contextOpen: $("#context-open-button"),
  contextMode: $("#context-mode-button"),
  contextSave: $("#context-save-button"),
  contextSaveAs: $("#context-save-as-button"),
  contextReload: $("#context-reload-button"),
  status: $("#status"),
  dropOverlay: $("#drop-overlay"),
  workspace: $("#workspace"),
  explorerToggle: $("#explorer-toggle"),
  outlineToggle: $("#outline-toggle"),
  outlineFocus: $("#outline-focus-button"),
  outlineList: $("#outline-list"),
  expandOutline: $("#expand-outline-button"),
  collapseOutline: $("#collapse-outline-button"),
  fileTree: $("#file-tree"),
  fileSort: $("#file-sort"),
  refreshFiles: $("#refresh-files-button"),
  folderName: $("#folder-name"),
  documentPath: $("#document-path"),
  localIndicator: $("#local-indicator"),
  tabsContainer: $("#tabs-container"),
  themeSelect: $("#theme-select"),
};

let currentId = null;
let currentPath = null;
let currentFilename = "";
let savedSource = "";
let dirty = false;
let editing = false;
let explorerVisible = true;
let outlineVisible = true;
let directoryEntries = [];
const tabs = new Map();
const tabViewStates = new Map();
let preferences = { theme: "dracula", textSize: 17 };
let renderTimer;
let statusTimer;
let imageObjectUrls = [];

function showStatus(message, error = false) {
  clearTimeout(statusTimer);
  elements.status.textContent = message;
  elements.status.classList.toggle("error", error);
  elements.status.hidden = false;
  statusTimer = setTimeout(() => { elements.status.hidden = true; }, error ? 6000 : 2600);
}

function errorMessage(error) {
  if (typeof error === "string") return error;
  return error?.message || "Something went wrong.";
}

async function attempt(action) {
  try {
    return await action();
  } catch (error) {
    showStatus(errorMessage(error), true);
    return null;
  }
}

function closeMenus() {
  $$(".menu-popover").forEach((menu) => { menu.hidden = true; });
  elements.contextMenu.hidden = true;
  $$(".menu-trigger").forEach((button) => button.setAttribute("aria-expanded", "false"));
}

function showDocumentContextMenu(clientX, clientY) {
  closeMenus();
  elements.contextMenu.hidden = false;
  elements.contextMenu.style.left = "0px";
  elements.contextMenu.style.top = "0px";
  const bounds = elements.contextMenu.getBoundingClientRect();
  const left = Math.max(4, Math.min(clientX, window.innerWidth - bounds.width - 4));
  const top = Math.max(4, Math.min(clientY, window.innerHeight - bounds.height - 4));
  elements.contextMenu.style.left = `${left}px`;
  elements.contextMenu.style.top = `${top}px`;
  elements.contextMenu.querySelector("button:not(:disabled)")?.focus({ preventScroll: true });
}

function setupMenus() {
  $$(".menu-trigger").forEach((button) => {
    button.setAttribute("aria-expanded", "false");
    button.addEventListener("click", (event) => {
      event.stopPropagation();
      const menu = document.getElementById(button.dataset.menu);
      const opening = menu.hidden;
      closeMenus();
      menu.hidden = !opening;
      button.setAttribute("aria-expanded", String(opening));
    });
  });
  $$(".menu-popover button, .context-menu button").forEach((button) => button.addEventListener("click", closeMenus));
  window.addEventListener("pointerdown", (event) => {
    if (!event.target.closest(".menu-wrap, .context-menu")) closeMenus();
  });
  window.addEventListener("blur", closeMenus);
}

function revokeImages() {
  imageObjectUrls.forEach((url) => URL.revokeObjectURL(url));
  imageObjectUrls = [];
}

function blockedImage(label) {
  const span = document.createElement("span");
  span.className = "muted";
  span.textContent = `[${label}]`;
  return span;
}

async function hydrateImages(container) {
  const images = [...container.querySelectorAll("img[data-local-image]")];
  await Promise.all(images.map(async (image) => {
    const path = image.dataset.localImage;
    if (!path || /^(?:[a-z][a-z0-9+.-]*:|\/|\\)/i.test(path)) {
      image.replaceWith(blockedImage("Remote or absolute image blocked"));
      return;
    }
    try {
      const payload = await invoke("read_local_image", { id: currentId, path });
      const url = URL.createObjectURL(new Blob([new Uint8Array(payload.bytes)], { type: payload.mimeType }));
      imageObjectUrls.push(url);
      image.src = url;
    } catch {
      image.replaceWith(blockedImage("Image unavailable"));
    }
  }));
}

function headingSlug(text) {
  return text.trim().toLowerCase()
    .replace(/[^\p{L}\p{N}\s-]/gu, "")
    .replace(/\s+/g, "-")
    .replace(/-+/g, "-") || "section";
}

function buildOutline() {
  const headings = [...elements.preview.querySelectorAll("h1, h2, h3, h4, h5, h6")];
  elements.outlineList.replaceChildren();
  if (!headings.length) {
    const empty = document.createElement("p");
    empty.className = "panel-empty";
    empty.textContent = currentPath ? "This document has no headings." : "Headings appear here.";
    elements.outlineList.append(empty);
    elements.outlineFocus.disabled = true;
    return;
  }

  const usedIds = new Set();
  headings.forEach((heading) => {
    let id = heading.id || headingSlug(heading.textContent);
    const base = id;
    let suffix = 2;
    while (usedIds.has(id) || (document.getElementById(id) && document.getElementById(id) !== heading)) {
      id = `${base}-${suffix++}`;
    }
    heading.id = id;
    usedIds.add(id);

    const button = document.createElement("button");
    button.type = "button";
    button.className = "outline-entry";
    button.dataset.level = heading.tagName.slice(1);
    button.title = heading.textContent;
    button.textContent = heading.textContent;
    button.addEventListener("click", () => {
      heading.scrollIntoView({ behavior: "smooth", block: "start" });
      elements.preview.focus({ preventScroll: true });
    });
    elements.outlineList.append(button);
  });
  elements.outlineFocus.disabled = false;
}

function setRenderedHtml(html) {
  revokeImages();
  const template = document.createElement("template");
  template.innerHTML = html;
  for (const image of template.content.querySelectorAll("img")) {
    const source = image.getAttribute("src") || "";
    image.removeAttribute("src");
    image.dataset.localImage = source;
    image.setAttribute("loading", "lazy");
  }
  elements.preview.replaceChildren(template.content);
  buildOutline();
  hydrateImages(elements.preview);
}

function rememberActiveView() {
  if (currentId === null) return;
  tabViewStates.set(currentId, {
    editing,
    previewScroll: elements.preview.scrollTop,
    editorScroll: elements.editor.scrollTop,
    selectionStart: elements.editor.selectionStart,
    selectionEnd: elements.editor.selectionEnd,
  });
}

function renderTabs() {
  elements.tabsContainer.replaceChildren();
  for (const tab of tabs.values()) {
    const wrapper = document.createElement("div");
    wrapper.className = `document-tab${tab.id === currentId ? " active" : ""}`;
    wrapper.dataset.id = String(tab.id);

    const select = document.createElement("button");
    select.type = "button";
    select.className = "tab-select";
    select.role = "tab";
    select.ariaSelected = String(tab.id === currentId);
    select.title = tab.path;
    select.textContent = `${tab.dirty ? "● " : ""}${tab.filename}`;
    select.addEventListener("click", () => activateTab(tab.id));

    const close = document.createElement("button");
    close.type = "button";
    close.className = "tab-close";
    close.title = `Close ${tab.filename}`;
    close.ariaLabel = `Close ${tab.filename}`;
    close.textContent = "×";
    close.addEventListener("click", (event) => {
      event.stopPropagation();
      closeTab(tab.id);
    });

    wrapper.addEventListener("auxclick", (event) => {
      if (event.button === 1) closeTab(tab.id);
    });
    wrapper.addEventListener("contextmenu", async (event) => {
      event.preventDefault();
      const { clientX, clientY } = event;
      if (tab.id !== currentId && !(await activateTab(tab.id))) return;
      showDocumentContextMenu(clientX, clientY);
    });
    wrapper.append(select, close);
    elements.tabsContainer.append(wrapper);
  }
  elements.tabsContainer.querySelector(".document-tab.active")?.scrollIntoView({ inline: "nearest" });
}

function updateDocumentChrome() {
  const hasDocument = currentId !== null;
  elements.localIndicator.hidden = !hasDocument;
  elements.documentPath.textContent = currentPath || "No document open";
  [elements.mode, elements.save, elements.saveAs, elements.close, elements.reload]
    .forEach((control) => { control.disabled = !hasDocument; });
  elements.quickEdit.disabled = !hasDocument;
  elements.quickEdit.textContent = editing ? "Preview" : "Edit";
  elements.quickEdit.title = editing ? `Show preview (${primaryKeyLabel}+E)` : `Edit source (${primaryKeyLabel}+E)`;
  elements.contextMode.disabled = !hasDocument;
  elements.contextMode.querySelector("span").textContent = editing ? "Return to preview" : "Edit source";
  [elements.contextSave, elements.contextSaveAs, elements.contextReload]
    .forEach((control) => { control.disabled = !hasDocument; });
  elements.outlineFocus.disabled = !hasDocument || !elements.outlineList.querySelector("button");
  elements.mode.querySelector("span").textContent = editing ? "Preview" : "Edit source";
  if (hasDocument) {
    const tab = tabs.get(currentId);
    if (tab) tabs.set(currentId, { ...tab, path: currentPath, filename: currentFilename, dirty });
  }
  renderTabs();
}

function applyNoDocument() {
  currentId = null;
  currentPath = null;
  currentFilename = "";
  savedSource = "";
  dirty = false;
  editing = false;
  directoryEntries = [];
  elements.preview.replaceChildren();
  elements.preview.hidden = true;
  elements.editor.hidden = true;
  elements.empty.hidden = false;
  elements.folderName.textContent = "FeatherMark";
  elements.fileTree.innerHTML = '<p class="panel-empty">Open a file to show its folder.</p>';
  buildOutline();
  updateDocumentChrome();
}

function applyDocument(documentPayload) {
  if (!documentPayload) return;
  currentId = documentPayload.id;
  currentPath = documentPayload.path;
  currentFilename = documentPayload.filename;
  savedSource = documentPayload.savedSource;
  dirty = documentPayload.dirty;
  tabs.set(documentPayload.id, {
    id: documentPayload.id,
    path: documentPayload.path,
    filename: documentPayload.filename,
    dirty: documentPayload.dirty,
  });
  const view = tabViewStates.get(documentPayload.id);
  editing = view?.editing ?? false;
  elements.editor.value = documentPayload.source;
  setRenderedHtml(documentPayload.html);
  elements.empty.hidden = true;
  elements.preview.hidden = editing;
  elements.editor.hidden = !editing;
  updateDocumentChrome();
  refreshDirectory();
  requestAnimationFrame(() => {
    elements.preview.scrollTop = view?.previewScroll ?? 0;
    elements.editor.scrollTop = view?.editorScroll ?? 0;
    if (view) elements.editor.setSelectionRange(view.selectionStart, view.selectionEnd);
  });
}

function applySavedDocument(documentPayload) {
  if (!documentPayload) return;
  currentPath = documentPayload.path;
  currentFilename = documentPayload.filename;
  savedSource = documentPayload.savedSource;
  dirty = documentPayload.dirty;
  setRenderedHtml(documentPayload.html);
  updateDocumentChrome();
  refreshDirectory();
}

function confirmDiscard(action) {
  if (!dirty) return true;
  return window.confirm(`Discard unsaved changes and continue ${action}?\n\nChoose Cancel to return and save them first.`);
}

async function chooseOpen() {
  rememberActiveView();
  applyDocument(await attempt(() => invoke("choose_markdown")));
}

async function openPath(path) {
  rememberActiveView();
  applyDocument(await attempt(() => invoke("open_path", { path })));
}

async function activateTab(id) {
  if (id === currentId) return true;
  if (!tabs.has(id)) return false;
  rememberActiveView();
  const payload = await attempt(() => invoke("activate_document", { id }));
  if (!payload) return false;
  applyDocument(payload);
  return true;
}

async function closeTab(id) {
  const tab = tabs.get(id);
  if (!tab) return;
  if (tab.dirty && !window.confirm(`Close ${tab.filename} and discard its unsaved changes?`)) return;
  const wasActive = id === currentId;
  if (wasActive) rememberActiveView();
  const result = await attempt(() => invoke("close_document", { id, discard: tab.dirty }));
  if (!result) return;
  tabs.delete(id);
  tabViewStates.delete(id);
  if (wasActive) {
    if (result.active) applyDocument(result.active);
    else applyNoDocument();
  } else {
    renderTabs();
  }
}

function closeCurrent() {
  if (currentId !== null) closeTab(currentId);
}

function cycleTab(direction) {
  const ids = [...tabs.keys()];
  if (ids.length < 2 || currentId === null) return;
  const currentIndex = ids.indexOf(currentId);
  const nextIndex = (currentIndex + direction + ids.length) % ids.length;
  activateTab(ids[nextIndex]);
}

function toggleMode() {
  if (!currentPath) return;
  editing = !editing;
  elements.preview.hidden = editing;
  elements.editor.hidden = !editing;
  if (editing) {
    elements.editor.focus();
  } else {
    updatePreviewNow();
    elements.preview.focus();
  }
  updateDocumentChrome();
}

async function updatePreviewNow() {
  clearTimeout(renderTimer);
  if (!currentPath) return;
  const html = await attempt(() => invoke("render_source", { source: elements.editor.value }));
  if (html !== null) setRenderedHtml(html);
}

function onEditorInput() {
  dirty = elements.editor.value !== savedSource;
  updateDocumentChrome();
  invoke("update_dirty", { id: currentId, source: elements.editor.value }).catch((error) => showStatus(errorMessage(error), true));
  clearTimeout(renderTimer);
  renderTimer = setTimeout(updatePreviewNow, 160);
}

async function save(asNew = false) {
  if (!currentPath) return;
  const command = asNew ? "save_document_as" : "save_document";
  const payload = await attempt(() => invoke(command, { id: currentId, source: elements.editor.value }));
  if (!payload) return;
  applySavedDocument(payload);
  showStatus("Saved");
}

async function reload() {
  if (!currentPath || !confirmDiscard("reloading from disk")) return;
  const payload = await attempt(() => invoke("reload_document", { id: currentId }));
  applyDocument(payload);
  if (payload) showStatus("Reloaded");
}

function renderDirectory() {
  elements.fileTree.replaceChildren();
  if (!directoryEntries.length) {
    elements.fileTree.innerHTML = '<p class="panel-empty">No folders or Markdown files here.</p>';
    return;
  }
  const direction = elements.fileSort.value === "name-desc" ? -1 : 1;
  const entries = [...directoryEntries].sort((a, b) => {
    if (a.isDirectory !== b.isDirectory) return a.isDirectory ? -1 : 1;
    return a.name.localeCompare(b.name, undefined, { sensitivity: "base" }) * direction;
  });
  for (const entry of entries) {
    const button = document.createElement("button");
    button.type = "button";
    button.className = `file-entry ${entry.isDirectory ? "directory-entry" : ""}`;
    if (!entry.isDirectory && entry.path.toLowerCase() === currentPath?.toLowerCase()) button.classList.add("current");
    button.disabled = entry.isDirectory;
    button.title = entry.path;
    const icon = document.createElement("span");
    icon.className = "entry-icon";
    icon.textContent = entry.isDirectory ? "›" : "◩";
    const name = document.createElement("span");
    name.className = "entry-name";
    name.textContent = entry.name;
    button.append(icon, name);
    if (!entry.isDirectory) button.addEventListener("click", () => openPath(entry.path));
    elements.fileTree.append(button);
  }
}

async function refreshDirectory() {
  if (!currentPath) return;
  const listing = await attempt(() => invoke("list_document_directory", { id: currentId }));
  if (!listing) return;
  elements.folderName.textContent = listing.rootName;
  elements.folderName.title = listing.rootPath;
  directoryEntries = listing.entries;
  renderDirectory();
}

function applyPreferences() {
  document.documentElement.dataset.theme = preferences.theme;
  document.documentElement.style.setProperty("--font-size", `${preferences.textSize}px`);
  elements.themeSelect.value = preferences.theme;
}

function persistPreferences() {
  invoke("update_preferences", { theme: preferences.theme, textSize: preferences.textSize }).catch(() => {});
}

function changeTextSize(delta) {
  preferences.textSize = delta === 0 ? 17 : Math.min(24, Math.max(13, preferences.textSize + delta));
  applyPreferences();
  persistPreferences();
  showStatus(`Text size: ${preferences.textSize}px`);
}

function toggleLightDarkTheme() {
  const darkThemes = new Set(["dark", "github-dark", "nord", "solarized-dark", "dracula"]);
  const systemIsDark = window.matchMedia?.("(prefers-color-scheme: dark)").matches;
  const currentlyDark = preferences.theme === "system" ? systemIsDark : darkThemes.has(preferences.theme);
  preferences.theme = currentlyDark ? "light" : "dark";
  applyPreferences();
  persistPreferences();
  showStatus(`Theme: ${elements.themeSelect.selectedOptions[0].textContent}`);
}

function setPanel(kind, visible) {
  if (kind === "explorer") explorerVisible = visible;
  else outlineVisible = visible;
  elements.workspace.classList.toggle("explorer-hidden", !explorerVisible);
  elements.workspace.classList.toggle("outline-hidden", !outlineVisible);
  elements.explorerToggle.querySelector("span").textContent = `${explorerVisible ? "✓ " : ""}File panel`;
  elements.outlineToggle.querySelector("span").textContent = `${outlineVisible ? "✓ " : ""}Outline`;
}

function isExternalLink(href) {
  return /^(https?:|mailto:)/i.test(href);
}

elements.preview.addEventListener("click", (event) => {
  const link = event.target.closest("a");
  if (!link) return;
  event.preventDefault();
  const href = link.getAttribute("href") || "";
  if (href.startsWith("#") && href !== "#blocked-link") {
    document.getElementById(decodeURIComponent(href.slice(1)))?.scrollIntoView({ behavior: "smooth" });
  } else if (isExternalLink(href)) {
    attempt(() => invoke("open_external", { url: href }));
  } else if (href === "#blocked-link") {
    showStatus("Unsafe link blocked", true);
  } else {
    const newTab = event.ctrlKey || event.metaKey;
    if (!newTab && !confirmDiscard("following this local link")) return;
    rememberActiveView();
    if (!newTab) tabViewStates.delete(currentId);
    const fragment = href.includes("#") ? decodeURIComponent(href.split("#").slice(1).join("#")) : "";
    attempt(() => invoke("open_relative_markdown", {
      id: currentId,
      target: href,
      newTab,
    })).then((payload) => {
      if (!payload) return;
      applyDocument(payload);
      if (fragment) requestAnimationFrame(() => document.getElementById(fragment)?.scrollIntoView());
    });
  }
});

setupMenus();
elements.open.addEventListener("click", chooseOpen);
elements.emptyOpen.addEventListener("click", chooseOpen);
elements.newTab.addEventListener("click", chooseOpen);
elements.mode.addEventListener("click", toggleMode);
elements.save.addEventListener("click", () => save(false));
elements.saveAs.addEventListener("click", () => save(true));
elements.close.addEventListener("click", closeCurrent);
elements.quickEdit.addEventListener("click", toggleMode);
elements.contextOpen.addEventListener("click", chooseOpen);
elements.contextMode.addEventListener("click", toggleMode);
elements.contextSave.addEventListener("click", () => save(false));
elements.contextSaveAs.addEventListener("click", () => save(true));
elements.contextReload.addEventListener("click", reload);
elements.reload.addEventListener("click", reload);
elements.fullscreen.addEventListener("click", () => attempt(() => invoke("toggle_fullscreen")));
elements.about.addEventListener("click", () => elements.aboutDialog.showModal());
elements.editor.addEventListener("input", onEditorInput);
elements.refreshFiles.addEventListener("click", refreshDirectory);
elements.fileSort.addEventListener("change", renderDirectory);
elements.explorerToggle.addEventListener("click", () => setPanel("explorer", !explorerVisible));
elements.outlineToggle.addEventListener("click", () => setPanel("outline", !outlineVisible));
elements.outlineFocus.addEventListener("click", () => {
  setPanel("outline", true);
  elements.outlineList.querySelector("button")?.focus();
});
elements.expandOutline.addEventListener("click", () => $$(".outline-entry").forEach((entry) => { entry.hidden = false; }));
elements.collapseOutline.addEventListener("click", () => $$(".outline-entry").forEach((entry) => { entry.hidden = Number(entry.dataset.level) > 2; }));
elements.themeSelect.addEventListener("change", () => {
  preferences.theme = elements.themeSelect.value;
  applyPreferences();
  persistPreferences();
  showStatus(`Theme: ${elements.themeSelect.selectedOptions[0].textContent}`);
});

elements.mainArea.addEventListener("contextmenu", (event) => {
  if (event.target === elements.editor) return;
  event.preventDefault();
  showDocumentContextMenu(event.clientX, event.clientY);
});

elements.contextMenu.addEventListener("keydown", (event) => {
  const buttons = [...elements.contextMenu.querySelectorAll("button:not(:disabled)")];
  const current = buttons.indexOf(document.activeElement);
  let next = current;
  if (event.key === "ArrowDown") next = (current + 1) % buttons.length;
  else if (event.key === "ArrowUp") next = (current - 1 + buttons.length) % buttons.length;
  else if (event.key === "Home") next = 0;
  else if (event.key === "End") next = buttons.length - 1;
  else return;
  event.preventDefault();
  buttons[next]?.focus();
});

window.addEventListener("keydown", (event) => {
  const ctrl = event.ctrlKey || event.metaKey;
  if (ctrl && event.key === "Tab") { event.preventDefault(); cycleTab(event.shiftKey ? -1 : 1); }
  else if (ctrl && event.key.toLowerCase() === "t") { event.preventDefault(); chooseOpen(); }
  else if (ctrl && event.shiftKey && event.key.toLowerCase() === "e") { event.preventDefault(); setPanel("explorer", !explorerVisible); }
  else if (ctrl && event.shiftKey && event.key.toLowerCase() === "o") { event.preventDefault(); setPanel("outline", !outlineVisible); }
  else if (ctrl && event.key.toLowerCase() === "o") { event.preventDefault(); chooseOpen(); }
  else if (ctrl && event.key.toLowerCase() === "r") { event.preventDefault(); reload(); }
  else if (ctrl && event.key.toLowerCase() === "e") { event.preventDefault(); toggleMode(); }
  else if (ctrl && event.key.toLowerCase() === "d") { event.preventDefault(); toggleLightDarkTheme(); }
  else if (ctrl && event.shiftKey && event.key.toLowerCase() === "s") { event.preventDefault(); save(true); }
  else if (ctrl && event.key.toLowerCase() === "s") { event.preventDefault(); save(false); }
  else if (ctrl && event.key.toLowerCase() === "w") { event.preventDefault(); closeCurrent(); }
  else if (ctrl && (event.key === "+" || event.key === "=")) { event.preventDefault(); changeTextSize(1); }
  else if (ctrl && event.key === "-") { event.preventDefault(); changeTextSize(-1); }
  else if (ctrl && event.key === "0") { event.preventDefault(); changeTextSize(0); }
  else if (event.key === "F11" || (isMac && event.metaKey && event.ctrlKey && event.key.toLowerCase() === "f")) { event.preventDefault(); attempt(() => invoke("toggle_fullscreen")); }
  else if (event.key === "Escape") closeMenus();
});

function adaptShortcutLabels() {
  if (!isMac) return;
  $$('kbd').forEach((label) => {
    label.textContent = label.textContent.split("Ctrl").join("⌘");
    if (label.textContent === "F11") label.textContent = "⌃⌘F";
  });
  $$('[title*="Ctrl+"]').forEach((element) => {
    element.title = element.title.split("Ctrl").join("⌘");
  });
}

async function syncDocumentsFromBackend() {
  const initial = await invoke("initial_documents");
  tabs.clear();
  for (const tab of initial.tabs) tabs.set(tab.id, tab);
  if (initial.active) applyDocument(initial.active);
  else applyNoDocument();
}

async function initialize() {
  adaptShortcutLabels();
  preferences = await invoke("get_preferences");
  applyPreferences();
  setPanel("explorer", true);
  setPanel("outline", true);

  await listen("documents-opened", syncDocumentsFromBackend);
  await listen("markdown-drop", (event) => openPath(event.payload));
  await listen("app-error", (event) => showStatus(event.payload, true));
  await listen("confirm-close", async () => {
    if (window.confirm("You have unsaved changes. Close FeatherMark and discard them?")) {
      await attempt(() => invoke("force_close"));
    }
  });
  await listen("tauri://drag-enter", () => { elements.dropOverlay.hidden = false; });
  await listen("tauri://drag-leave", () => { elements.dropOverlay.hidden = true; });
  await listen("tauri://drag-drop", () => { elements.dropOverlay.hidden = true; });
  await syncDocumentsFromBackend();
}

initialize().catch((error) => showStatus(errorMessage(error), true));
