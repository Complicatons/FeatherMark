# FeatherMark 0.1.0 — Windows build report

Measured on Windows 25H2, build 26200.9168, with WebView2 Runtime 151.0.4129.101.

## Release artifacts

| Artifact | Size | SHA-256 |
| --- | ---: | --- |
| `dist/windows/FeatherMark-0.1.0-windows-x64-portable.exe` | 9,574,912 bytes (9.13 MiB) | `5EA8554E02B50B85EC44722B258FE1B573346E44DC9257EF1ECDBA3F882DA557` |
| `dist/windows/FeatherMark-0.1.0-windows-x64-setup.exe` | 2,079,740 bytes (1.98 MiB) | `80B22B5BE544EE61EB7BAF202431E18CE0E4AE5E746CC82977641C1E25993DDF` |

The portable file is the optimized application itself and can be hosted as a single download. It does not install or register anything and suppresses FeatherMark preference writes while its filename contains `portable`; the shared WebView2 runtime may still maintain its normal Local AppData cache.

The per-user installer registers `.md` and `.markdown` as supported file types, adds FeatherMark to Windows **Open with**, and installs an **Open with FeatherMark** secondary context-menu verb for both extensions. It also displays an optional Yes/No question at the end of installation. Choosing Yes opens Windows Default Apps for the user's confirmation. It does not and cannot silently override the user's existing default.

## Startup and memory

- First post-build launches reached a usable native window in approximately 0.64–0.79 seconds. A warm launch measured 0.08 seconds. These are stopwatch-to-window measurements, not laboratory cold-cache benchmarks.
- At idle with the representative fixture open, the FeatherMark host used about 5.9 MiB private memory / 25.2 MiB working set.
- With two real documents open in separate tabs, the host used about 6.0 MiB private memory / 25.3 MiB working set. Inactive tabs retain text and small state records, not another WebView or rendered page.
- The complete WebView2 process tree used about 180.3 MiB aggregate private memory. Summed working sets were about 363.8 MiB, but that figure double-counts shared WebView2 pages and should not be read as entirely unique RAM.

WebView2 dominates memory use. This is the main cost of choosing the smallest practical executable and mature browser-quality Markdown presentation without bundling a runtime. A native custom renderer could lower runtime memory, but it would materially increase executable size, rendering code, dependencies, and implementation risk.

## Major runtime dependencies

FeatherMark declares six Rust runtime crates; the major components are:

1. Tauri/wry/tao — window, WebView2 integration, IPC, drag-and-drop, and packaging.
2. `pulldown-cmark` — Markdown parsing with tables, task lists, footnotes, and strikethrough.
3. Tauri dialog plugin/rfd — native open and save dialogs.
4. `serde`/`serde_json` — small command payloads and the preferences file.
5. `url` — external-link scheme validation.

The frontend has no framework or package dependency. Node and the Tauri CLI are build-time only. Windows requires the shared WebView2 Runtime; macOS would use WKWebView; Linux would require WebKitGTK.

## Verified on Windows

- `cargo test`: 14/14 focused tests passed, including tab deduplication, independent dirty state, neighbour selection after closing, relative Markdown links, UTF-8 loading, shallow folder filtering, GFM rendering, raw-HTML escaping, unsafe-link blocking, relative-image confinement/decoding, portable-mode detection, bundled-theme validation, and the Dracula first-run default.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `node --check src/app.js`: passed.
- `npm run check:themes`: all bundled palettes passed the automated 4.5:1 contrast floor for primary text, muted text, and links against the reading surface.
- Optimized Rust executable and NSIS installer: built successfully; Tauri's NSIS verification passed.
- Portable package: generated as one file and launched with `fixtures/sample.md`; its title displayed `sample.md — FeatherMark`. The distribution folder remained a single executable, no file-association registry keys were created, and the existing FeatherMark preferences file was not modified.
- Production executable: launched with `fixtures/sample.md` as a quoted command-line path; title displayed `sample.md — FeatherMark`.
- Actual interface: visually inspected from an off-screen capture of the production portable window with a real fixture. The compact theme dropdown fit beside the document path and local-content indicator, while the tab, file panel, generated outline, rendered content, and top Edit button remained usable. Evidence is in `docs/images/feathermark-reader.png`.
- Installer: walked through the actual native wizard, verified the optional default-app prompt visually in `docs/images/default-app-prompt.png`, and chose No so the host's defaults were not changed. A repeatable silent-install audit confirmed the registered application, both extension mappings, **Open with** application registration, both context-menu verbs, their quoted open commands, and that existing defaults remained unchanged. Silent uninstall removed every FeatherMark registration and the installed files.
- Lifecycle: ten consecutive close messages sent to the actual Tauri window all exited within three seconds, with no FeatherMark host left running.
- Package configuration: reviewed for NSIS generation, WebView2 bootstrap behavior, no production console, isolated Windows association hooks, and portable preference behavior.
- Dependency audit: `npm install` reported zero npm vulnerabilities. Direct Rust dependencies were reviewed and an unnecessary external-launch crate was removed.

## Limits and remaining verification

- Windows automation successfully exercised the native installer, but could not reliably acquire foreground input in the WebView application. Therefore clicking the Edit button, opening and using the new context menu, edit typing/save, middle-click closing, Ctrl+Tab, Save As, drag-and-drop, theme switching, link launch, full screen, and visible dirty-tab prompts were code-reviewed and partly covered by focused state/render/path tests, but were not all driven end-to-end in the production window. The README and fixtures provide a short manual pass for these controls.
- Normal and percent-encoded local relative image paths were visually confirmed in `docs/images/relative-images.png`; missing-image fallback remains part of the manual fixture pass.
- The binaries are unsigned. A browser download can therefore trigger Windows SmartScreen or an unknown-publisher warning; production code signing remains necessary for a polished public release.
- The installer/default-app flow was verified on this Windows 25H2 host, not yet on clean Windows 10 and Windows 11 virtual machines. On older supported Windows versions, the Settings page may be less targeted and require the user to search for FeatherMark manually.
- The classic **Open with FeatherMark** verb is registered for both extensions. Windows 11 can place classic Win32 verbs under **Show more options** rather than its compact first-level menu; a first-level Windows 11 command would require a substantially heavier packaged shell extension.
- macOS and Linux were not built or run. Their webview packages, dialogs, file-open behavior, associations, typography, and installers remain unverified.
- Remote images and raw SVG are intentionally blocked. The file panel is deliberately shallow, and tabs are not restored between launches. Syntax highlighting, Mermaid, math, recursive folders/vaults, recent files, and live reload remain intentionally out of scope.
