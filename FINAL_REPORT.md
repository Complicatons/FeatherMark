# FeatherMark 0.2.0 — cross-platform release report

FeatherMark 0.2.0 is the project's second public release and its first release for Windows, macOS, and Linux. The application remains one Rust/Tauri codebase with small platform-specific launch and bundle configuration.

## Release artifacts

The release workflow produces and checks all eight application downloads before publishing them:

| Platform | Architecture | Formats |
| --- | --- | --- |
| Windows | x64 | Portable EXE and per-user NSIS installer |
| macOS | Apple Silicon (ARM64) | DMG |
| macOS | Intel (x64) | DMG |
| Linux | ARM64 | DEB and AppImage |
| Linux | x64 | DEB and AppImage |

Every release also includes `SHA256SUMS.txt`, generated from the final files after all platform jobs succeed. The [v0.2.0 release](https://github.com/Complicatons/FeatherMark/releases/tag/v0.2.0) was published after the [five-platform release workflow](https://github.com/Complicatons/FeatherMark/actions/runs/32722360612) completed successfully.

The published 0.2.0 artifacts measured:

| Artifact | Size | SHA-256 |
| --- | ---: | --- |
| `FeatherMark-0.2.0-windows-x64-portable.exe` | 9,616,896 bytes (9.17 MiB) | `7ebe3cc79bc9085f587d939eb4e2f1528eb4be014546e3d27de3c50a5bfba113` |
| `FeatherMark-0.2.0-windows-x64-setup.exe` | 2,080,877 bytes (1.98 MiB) | `7e596ba17944d365cbe4274d8e8cbdcf645a98d85f31c7d218005da12feff800` |
| `FeatherMark-0.2.0-macos-aarch64.dmg` | 3,107,080 bytes (2.96 MiB) | `5a8f2a8979b69fa7ba10f9711c022f73125d0dbe0180069134ea96752882523e` |
| `FeatherMark-0.2.0-macos-x64.dmg` | 3,306,714 bytes (3.15 MiB) | `56a49ea8a8b2231dcc4c9f0685a79fdfa924bfa54c6be5425994b50893ee9e6d` |
| `FeatherMark-0.2.0-linux-aarch64.deb` | 3,267,516 bytes (3.12 MiB) | `a9785a063610dae23151e877722c857779b6fed665e879b2f047687f4c9530d9` |
| `FeatherMark-0.2.0-linux-aarch64.AppImage` | 79,067,656 bytes (75.40 MiB) | `1a96dd60661de3142bd7fcf58d3e0310fb9e948b4c5f17694bbe82ab5564c202` |
| `FeatherMark-0.2.0-linux-x64.deb` | 3,193,162 bytes (3.05 MiB) | `0b184f9288f31a586b1c127366fa9f9ceed3f80315dff8f0b42e6cd912cfa3ea` |
| `FeatherMark-0.2.0-linux-x64.AppImage` | 80,747,000 bytes (77.01 MiB) | `7a6ec2510f302467068df7b9b72673a9fe1149020688527a3f1b0c350c494448` |

The locally rebuilt Windows artifacts were within 0.5% of the GitHub-hosted files. The release's checksum manifest remains the authoritative checksum reference for downloaded artifacts.

## Startup and memory

The application architecture and runtime dependency set are unchanged from 0.1.0. On the measured Windows development host:

- First post-build launches reached a usable native window in approximately 0.64–0.79 seconds; one warm launch measured 0.08 seconds.
- The FeatherMark host used about 5.9 MiB private memory / 25.2 MiB working set with one fixture open.
- With two documents open, the host used about 6.0 MiB private memory / 25.3 MiB working set because inactive tabs retain text and state rather than another WebView.
- The complete WebView2 process tree used about 180.3 MiB aggregate private memory. Summed working sets double-count shared WebView2 pages and are not a useful unique-RAM figure.

These are practical machine-specific observations, not laboratory cold-cache benchmarks. Startup and memory have not yet been measured on physical macOS or Linux systems.

## Major runtime dependencies

FeatherMark declares six direct Rust runtime crates. The major components are:

1. Tauri/wry/tao for the native window, operating-system WebView, IPC, drag and drop, and packaging.
2. `pulldown-cmark` for Markdown parsing with tables, task lists, footnotes, and strikethrough.
3. Tauri's dialog plugin/rfd for native Open and Save dialogs.
4. `serde` and `serde_json` for command payloads and the small preferences file.
5. `url` for external-link parsing and scheme validation.

The frontend has no framework or runtime package dependency. Node and the Tauri CLI are build-time only.

| Platform | Shared runtime requirement |
| --- | --- |
| Windows | Microsoft WebView2 Runtime |
| macOS | WKWebView supplied by macOS |
| Linux | WebKitGTK 4.1 supplied by the distribution; AppImage use may also require FUSE |

## Verification performed

### Local Windows verification

- `node --check src/app.js`: passed.
- Bundled-theme contrast validation: all ten palettes passed the automated 4.5:1 floor for primary text, muted text, and links.
- `cargo fmt --check`: passed.
- `cargo test`: 14/14 focused tests passed, including UTF-8 loading, GFM rendering, raw-HTML escaping, unsafe-link blocking, relative-image confinement, Markdown link resolution, tab state, dirty state, portable detection, theme validation, and the Dracula default.
- `cargo clippy --all-targets -- -D warnings`: passed.
- Optimized Windows executable and NSIS installer: rebuilt successfully from the platform-specific Tauri configuration.
- Portable and installer staging: passed with exact versioned filenames and SHA-256 calculation.
- Production interface: launched with `fixtures/portfolio-showcase.md`, captured, and visually inspected in the default Dracula theme. The current three-pane layout, tabs, nearby Edit button, rendering, table, checklist, code, and outline remained correct.
- PowerShell release and packaging scripts: parsed successfully.
- Tauri base, Windows, macOS, and Linux JSON configuration files: parsed successfully.

The 0.1.0 hands-on Windows installer audit remains relevant because the Windows shell hooks are unchanged: it verified **Open with**, `.md` and `.markdown` registration, quoted open commands, the optional Default Apps handoff, preservation of existing defaults, and complete uninstall cleanup.

### GitHub-hosted cross-platform verification

The CI workflow compiles, tests, lints, and bundles on Windows x64, macOS Apple Silicon, and Linux x64. The tag workflow additionally uses native macOS Intel and Linux ARM64 runners, then blocks publication unless all eight expected assets are present. Release-run links and final artifact sizes are available from the repository's Actions and Releases pages.

## Security and packaging decisions

- Raw HTML remains escaped; Markdown cannot run JavaScript.
- Unsafe link schemes and remote images remain blocked.
- Relative image paths remain confined to the opened document's directory.
- Platform file associations come from Tauri's bundle metadata. Windows retains its isolated optional shell integration and never silently takes over defaults.
- macOS receives Finder open-file events while the app is running and refreshes its shared document state.
- macOS labels Command-key shortcuts in the interface and supports the standard Control-Command-F full-screen shortcut.
- The release workflow uses native hosted runners rather than experimental cross-compilation and generates a checksum manifest from the assembled files.

## Known limitations

- Windows and macOS do not yet carry paid publisher certificates, and macOS packages are not notarized. Linux packages are also unsigned. SmartScreen, Gatekeeper, or distribution tools can therefore warn on first launch.
- Windows has the strongest real-interface and installer verification. macOS and Linux 0.2.0 packages are natively compiled and automatically tested, but their dialogs, typography, drag and drop, file associations, edit/save flow, and desktop integration still need a full manual pass on physical systems.
- The macOS bundle declares 10.13 as its minimum system version, but this release was only built on current GitHub macOS images.
- Linux desktop behavior varies by distribution and desktop environment. The DEB package provides application and MIME registration; AppImage integration is intentionally left to the user's desktop tooling.
- Remote images and raw SVG remain intentionally blocked. Tabs are not restored between launches. Syntax highlighting, Mermaid, math, recursive vaults, recent files, live reload, plugins, and automatic updates remain outside FeatherMark's focused scope.
