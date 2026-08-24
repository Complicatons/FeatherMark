<p align="center">
  <img src="src-tauri/icons/feathermark.svg" width="112" height="112" alt="FeatherMark logo">
</p>

<h1 align="center">FeatherMark</h1>

<p align="center">
  A small, fast, security-conscious Markdown viewer for Windows, macOS, and Linux.
</p>

<p align="center">
  <a href="https://github.com/Complicatons/FeatherMark/actions/workflows/ci.yml"><img alt="CI" src="https://github.com/Complicatons/FeatherMark/actions/workflows/ci.yml/badge.svg"></a>
  <img alt="Release" src="https://img.shields.io/badge/release-0.2.0-36b99a">
  <img alt="Windows x64" src="https://img.shields.io/badge/Windows-x64-0078d4?logo=windows11">
  <img alt="macOS Intel and Apple Silicon" src="https://img.shields.io/badge/macOS-Intel_%2B_Apple_Silicon-111111?logo=apple">
  <img alt="Linux x64 and ARM64" src="https://img.shields.io/badge/Linux-x64_%2B_ARM64-fcc624?logo=linux&logoColor=111111">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-Tauri_2-b7410e?logo=rust">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-6e7781">
</p>

<p align="center">
  <a href="#download">Download</a> ·
  <a href="#why-feathermark">Why FeatherMark</a> ·
  <a href="#features">Features</a> ·
  <a href="#keyboard-shortcuts">Shortcuts</a> ·
  <a href="#build-from-source">Build</a> ·
  <a href="PORTFOLIO.md">Case study</a> ·
  <a href="#security">Security</a>
</p>

![FeatherMark showing two Markdown documents, a file panel, reading view, and document outline](docs/images/feathermark-reader.png)

FeatherMark opens local Markdown files quickly and gets out of the way. It is a viewer first, with a deliberately basic source editor for small corrections. There are no accounts, databases, cloud features, telemetry, bundled browser engines, background services, plugin systems, or automatic updates.

## Why FeatherMark

Many Markdown applications grow into note platforms, IDEs, or knowledge-management systems. FeatherMark explores the opposite direction: how small and focused can a practical desktop Markdown viewer remain while still handling the everyday details well?

That constraint shaped every decision. FeatherMark uses Rust for trusted file and state handling, the operating system's existing WebView instead of shipping a browser runtime, and a dependency-free interface. Editing is intentionally a single raw-source view. Tabs share one renderer. Remote content is blocked by default. Version 0.2.0 carries the same small application to Windows, Intel and Apple Silicon Macs, and x64 and ARM64 Linux systems.

### Engineering decisions

| Challenge | Decision | Result |
| --- | --- | --- |
| Keep the download small | Tauri 2 with each operating system's shared WebView | No bundled Chromium or background service |
| Render untrusted Markdown safely | Escape raw HTML, validate links in Rust, confine local image paths, and enforce a strict CSP | Documents cannot run arbitrary JavaScript or silently fetch remote images |
| Add tabs without turning the app into a browser | Keep document state in Rust and reuse one WebView and one rendered surface | Multiple files with a small incremental memory cost |
| Support quick corrections without becoming an editor suite | Plain-text source/preview toggle, debounced rendering, explicit saves, and dirty-state warnings | Useful editing with little interface or dependency overhead |
| Ship native packages without forking the app | Keep platform bundling in three small Tauri override files | Windows NSIS, macOS DMG, and Linux DEB/AppImage builds share one codebase |

> [!IMPORTANT]
> Version 0.2.0 packages are built and tested on GitHub-hosted Windows, macOS, and Linux runners. Windows has also received hands-on interface and installer testing. macOS and Linux packages have not yet received the same real-device manual UI pass and are not code-signed or notarized.

## Download

Download FeatherMark 0.2.0 from the repository's [**Releases** page](../../releases/latest). Choose the file matching your operating system and processor:

| Platform | Download | Best for |
| --- | --- | --- |
| Windows x64 | `FeatherMark-0.2.0-windows-x64-setup.exe` | Normal installation, **Open with**, and optional file associations |
| Windows x64 | `FeatherMark-0.2.0-windows-x64-portable.exe` | One-file use without installation or FeatherMark preference writes |
| macOS Apple Silicon | `FeatherMark-0.2.0-macos-aarch64.dmg` | M1, M2, M3, M4, and later Apple Silicon Macs |
| macOS Intel | `FeatherMark-0.2.0-macos-x64.dmg` | Intel-based Macs |
| Linux x64 | `FeatherMark-0.2.0-linux-x64.deb` | Debian, Ubuntu, and compatible x64 distributions |
| Linux x64 | `FeatherMark-0.2.0-linux-x64.AppImage` | Other x64 distributions |
| Linux ARM64 | `FeatherMark-0.2.0-linux-aarch64.deb` | Debian, Ubuntu, and compatible ARM64 systems |
| Linux ARM64 | `FeatherMark-0.2.0-linux-aarch64.AppImage` | Other ARM64 distributions |

The installed version appears in Windows **Open with** and adds **Open with FeatherMark** to the standard file context menu for `.md` and `.markdown` documents. On Windows 11, classic desktop commands can appear under **Show more options**. The installer also asks whether to open Windows Default Apps so the user can confirm FeatherMark as the Markdown default; it never silently takes over file associations.

### Windows

**Standard installer**

1. Download `FeatherMark-0.2.0-windows-x64-setup.exe` from [Releases](../../releases/latest).
2. Run the installer. FeatherMark is installed for the current Windows user; administrator access is not required.
3. At the end, choose whether to open Windows Default Apps and select FeatherMark for `.md` and `.markdown` if desired.
4. Open Markdown files from the Start menu, Windows **Open with**, the file context menu, or by double-clicking an associated file.

**Portable edition**

1. Download `FeatherMark-0.2.0-windows-x64-portable.exe` from [Releases](../../releases/latest).
2. Keep `portable` in the filename and place the executable wherever you want.
3. Run it directly or drag a Markdown file onto it. Nothing is installed and FeatherMark does not register file associations or write its own preference file.

<p align="center">
  <img src="docs/images/default-app-prompt.png" width="416" alt="FeatherMark installer asking whether to open Windows Default Apps">
</p>

### macOS

1. Open **About This Mac** and check whether the processor is Apple Silicon or Intel.
2. Download the matching DMG, open it, and drag FeatherMark into **Applications**.
3. Because 0.2.0 is ad-hoc signed rather than Apple-notarized, the first launch may be blocked. Control-click FeatherMark in **Applications**, choose **Open**, then confirm **Open**. If macOS still blocks it, open **System Settings → Privacy & Security** and choose **Open Anyway** for FeatherMark.
4. To use FeatherMark for Markdown files, select an `.md` file in Finder, choose **File → Get Info**, select FeatherMark under **Open with**, and optionally choose **Change All**.

FeatherMark uses the WKWebView included with macOS. The package declares macOS 10.13 as its minimum deployment target, although 0.2.0 has only been build-tested on current GitHub macOS images.

### Linux

**Debian / Ubuntu package**

```bash
sudo apt install ./FeatherMark-0.2.0-linux-x64.deb
```

Use the `aarch64.deb` file instead on ARM64. Installing through `apt` resolves declared runtime dependencies and registers FeatherMark with the desktop application menu and `text/markdown` MIME type.

**AppImage**

```bash
chmod +x FeatherMark-0.2.0-linux-x64.AppImage
./FeatherMark-0.2.0-linux-x64.AppImage
```

Use the `aarch64.AppImage` file on ARM64. AppImages remain portable, but FeatherMark deliberately uses the distribution's WebKitGTK rather than bundling a browser engine. Install WebKitGTK 4.1 and FUSE through your distribution if they are absent. Desktop integration and default-file selection vary by environment; FeatherMark will appear as an option when installed from the DEB package, while AppImage users may use their desktop's **Open With** dialog.

### Verify a download

Every release includes `SHA256SUMS.txt`.

```powershell
Get-FileHash .\FeatherMark-0.2.0-windows-x64-setup.exe -Algorithm SHA256
```

```bash
sha256sum FeatherMark-0.2.0-macos-aarch64.dmg
sha256sum FeatherMark-0.2.0-linux-x64.AppImage
```

Compare the result with the matching line in `SHA256SUMS.txt` before bypassing an operating-system warning.

### Runtime requirements

- **Windows:** Windows 10 or 11 x64 and Microsoft WebView2 Runtime. The installer fetches Microsoft's bootstrapper when the shared runtime is missing.
- **macOS:** Intel or Apple Silicon Mac with WKWebView; the bundle declares macOS 10.13 or later.
- **Linux:** 64-bit x64 or ARM64 desktop with WebKitGTK 4.1. AppImage use may also require FUSE.

The Windows and macOS packages do not have paid publisher certificates, and the macOS DMGs are not notarized. Windows SmartScreen and macOS Gatekeeper may therefore show an unknown-publisher warning. Linux packages are also unsigned. The checksum manifest lets users verify that a download matches the artifact produced by the release workflow; publisher signing remains planned.

## Features

- Fast native-window startup with a compact Rust host.
- GitHub-flavoured Markdown essentials: headings, emphasis, lists, links, images, blockquotes, inline and fenced code, tables, task lists, footnotes, strikethrough, and horizontal rules.
- Lightweight tabs backed by one shared rendered view rather than one WebView per document.
- Shallow file panel and generated document outline.
- Relative Markdown links and safe local images resolved from the document directory.
- Always-visible theme dropdown with Dracula as the first-run default, plus System, Feather Light/Dark, GitHub Light/Dark, Nord, Solarized Light/Dark, and Sepia palettes.
- Right-click document and tab menus with direct Edit/Preview, Save, Save As, Reload, and Open actions.
- Plain-text source mode with debounced preview and explicit Save / Save As.
- Unsaved-change warnings before destructive navigation or closing.
- File picker, command-line opening, drag and drop, and operating-system file associations. Windows also provides **Open with** registration and a Markdown context-menu command.
- Clean maximum-width reading layout with Unicode-friendly system font fallbacks.

![FeatherMark rendering a release overview with a table, task checklist, code block, and outline in the default Dracula theme](docs/images/markdown-rendering.png)

<p align="center">
  <img src="docs/images/context-menu.png" alt="FeatherMark document context menu with Open, Edit, Save, Save As, and Reload actions">
</p>

<p align="center"><em>Core document actions stay close at hand without turning the reading view into a toolbar-heavy editor.</em></p>

FeatherMark intentionally does **not** include rich-text editing, recursive vault management, session restoration, sync, accounts, Git integration, AI features, export systems, Mermaid, LaTeX, plugins, or an updater.

## Keyboard shortcuts

| Shortcut | Action |
| --- | --- |
| <kbd>Ctrl</kbd> + <kbd>O</kbd> | Open a Markdown file |
| <kbd>Ctrl</kbd> + <kbd>T</kbd> | Open another document |
| <kbd>Ctrl</kbd> + <kbd>R</kbd> | Reload from disk |
| <kbd>Ctrl</kbd> + <kbd>E</kbd> | Toggle source and preview |
| <kbd>Ctrl</kbd> + <kbd>D</kbd> | Toggle Feather Light and Feather Dark |
| <kbd>Ctrl</kbd> + <kbd>S</kbd> | Save |
| <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>S</kbd> | Save As |
| <kbd>Ctrl</kbd> + <kbd>W</kbd> | Close the current document |
| <kbd>Ctrl</kbd> + <kbd>Tab</kbd> | Next tab |
| <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Tab</kbd> | Previous tab |
| <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>E</kbd> | Toggle the file panel |
| <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>O</kbd> | Toggle the outline |
| <kbd>Ctrl</kbd> + <kbd>+</kbd> / <kbd>-</kbd> / <kbd>0</kbd> | Adjust or reset text size |
| <kbd>F11</kbd> | Toggle full screen |

On macOS, use <kbd>Command</kbd> in place of <kbd>Ctrl</kbd>; FeatherMark changes its in-app shortcut labels automatically. The standard macOS full-screen shortcut <kbd>Control</kbd> + <kbd>Command</kbd> + <kbd>F</kbd> is supported alongside <kbd>F11</kbd>.

The **Edit** button sits directly beside the tab controls. The rendered document and tabs also have a right-click menu for common document actions.

## How it stays small

FeatherMark uses:

- **Rust** for file access, state, path validation, preferences, and platform integration.
- **Tauri 2 / wry / tao** for the native window and operating-system WebView.
- **pulldown-cmark** for Markdown parsing.
- **Tauri dialog plugin** for native Open and Save dialogs.
- A dependency-free HTML, CSS, and JavaScript interface.

FeatherMark shares the platform WebView instead of bundling Chromium: WebView2 on Windows, WKWebView on macOS, and WebKitGTK on Linux. The 0.1.0 Windows portable executable measured 9.13 MiB and reached a usable window in roughly 0.64–0.79 seconds on the development host. Updated 0.2.0 artifact sizes and cross-platform build evidence are recorded in [FINAL_REPORT.md](FINAL_REPORT.md); runtime memory and startup have not been measured on physical Mac or Linux systems yet.

## Security

Markdown files are treated as untrusted input.

- Raw embedded HTML is escaped and displayed as text.
- Markdown cannot execute arbitrary JavaScript.
- Unsafe link schemes such as `javascript:` are blocked.
- External links are opened only after Rust validates `http`, `https`, or `mailto`.
- Remote and absolute images are blocked.
- Relative PNG, JPEG, GIF, WebP, and BMP images are loaded only when their resolved path stays inside the Markdown document's directory.
- A restrictive Content Security Policy blocks remote scripts, connections, and image loads.
- Invalid UTF-8, unsupported extensions, files over 16 MiB, and I/O failures produce compact errors rather than crashes.

Please report vulnerabilities privately using GitHub's **Report a vulnerability** option. See [SECURITY.md](SECURITY.md) for the disclosure policy.

## Build from source

### Windows prerequisites

1. Rust stable with the MSVC target.
2. Visual Studio C++ Build Tools.
3. Node.js LTS. Node is used only by the Tauri build CLI, not at runtime.
4. WebView2 Runtime.

```powershell
git clone <your-feathermark-repository-url>
cd FeatherMark
npm.cmd ci
npm.cmd run dev
```

Run the checks and create both Windows packages:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
node --check src/app.js
npm.cmd run check:themes
npm.cmd run build
npm.cmd run package:windows
```

The two upload-ready artifacts are staged in `dist/windows`. That directory is intentionally ignored by Git: publish binaries as GitHub Release assets instead of adding them to source history.

### macOS prerequisites

Install Xcode Command Line Tools, Rust stable, and Node.js LTS:

```bash
xcode-select --install
rustup update stable
npm ci
npm run dev
```

Create the DMG using an ad-hoc identity for an unsigned local build:

```bash
APPLE_SIGNING_IDENTITY="-" npm run build
```

The DMG is written below `src-tauri/target/release/bundle/dmg/`.

### Linux prerequisites

For Debian or Ubuntu development systems:

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf xdg-utils
rustup update stable
npm ci
npm run dev
```

Run the same checks shown in the Windows section with `npm` instead of `npm.cmd`, then use `npm run build`. The DEB and AppImage are written below `src-tauri/target/release/bundle/`.

## Publishing a release

The repository includes two GitHub Actions workflows:

- `ci.yml` tests, lints, and builds every pull request and push to `main` on Windows x64, macOS Apple Silicon, and Linux x64.
- `release.yml` validates a `v*` tag, builds all eight Windows/macOS/Linux assets across five native GitHub runners, verifies the complete asset set, writes `SHA256SUMS.txt`, and publishes the release only after every build succeeds.

Keep the version in `package.json`, `src-tauri/tauri.conf.json`, and `src-tauri/Cargo.toml` aligned, then push a matching tag:

```powershell
git tag v0.2.0
git push origin v0.2.0
```

Release notes live in `.github/release-notes/v0.2.0.md`. Review that file before tagging; the workflow publishes it verbatim with the verified artifacts.

## Project structure

```text
src/                         Dependency-free interface
src-tauri/src/               Rust application and platform isolation
src-tauri/tauri.*.conf.json  Platform-specific bundle targets
src-tauri/windows/           Windows-only installer hooks
src-tauri/icons/             Source logo and generated application icons
fixtures/                    Markdown and image test fixtures
scripts/                     Packaging, release staging, and Windows QA helpers
docs/images/                 Images used by this README
.github/workflows/           Continuous integration and release publishing
```

## Platform status

| Platform | Status | Runtime |
| --- | --- | --- |
| Windows x64 | Built, automated-tested, and manually exercised; portable and NSIS packages | WebView2 |
| macOS Apple Silicon | Native DMG built and automated-tested on GitHub; manual hardware UI pass pending | WKWebView |
| macOS Intel | Native DMG built and automated-tested on GitHub; manual hardware UI pass pending | WKWebView |
| Linux x64 | DEB and AppImage built and automated-tested on GitHub; manual desktop UI pass pending | WebKitGTK 4.1 |
| Linux ARM64 | DEB and AppImage built and automated-tested on GitHub; manual hardware UI pass pending | WebKitGTK 4.1 |

Platform-specific launch code, bundle configuration, and Windows shell registration remain isolated. The Markdown renderer, document state, editor, security rules, and interface are shared across all desktop builds.

## Contributing

Focused fixes that preserve FeatherMark's small scope are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request. For behaviour reports, include the Markdown fixture that reproduces the problem and avoid attaching private documents.

For a concise project case study suitable for a personal website or portfolio index, see [PORTFOLIO.md](PORTFOLIO.md).

## Acknowledgements

The three-pane visual direction and several edge-case fixture categories were informed by [aydiler/md-viewer](https://github.com/aydiler/md-viewer), an MIT-licensed Rust viewer used as a comparison source. FeatherMark has its own implementation, security model, editing flow, packaging, and branding.

## License

FeatherMark is available under the [MIT License](LICENSE).

<p align="center">Designed and built as a focused desktop software project by <a href="https://github.com/Complicatons">Complicated</a>.</p>
