# Contributing to FeatherMark

Thank you for helping improve FeatherMark. The project values focused changes that keep startup fast, memory use understandable, dependencies few, and the interface calm.

## Before opening an issue

- Search existing issues first.
- Test the latest release or current `main` branch.
- Remove private information from Markdown files and screenshots.
- Reduce rendering problems to the smallest Markdown example that still reproduces them.

Use the bug-report template for defects and the feature-request template for proposed changes. Security issues belong in the private process described in [SECURITY.md](SECURITY.md), not a public issue.

## Development setup

FeatherMark is built on Windows, macOS, and Linux. Install Rust stable and Node.js LTS on every platform. Windows additionally needs Visual Studio C++ Build Tools and WebView2; macOS needs Xcode Command Line Tools; Linux needs WebKitGTK 4.1 development libraries, libappindicator, librsvg, `patchelf`, and `xdg-utils`.

```powershell
npm.cmd ci
npm.cmd run dev
```

On macOS and Linux use `npm ci` and `npm run dev`. The complete platform prerequisite commands are in [README.md](README.md#build-from-source).

Before submitting a pull request, run:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
node --check src/app.js
npm.cmd run check:themes
npm.cmd run build
```

Use `npm` instead of `npm.cmd` outside Windows. Pull requests also run this verification on Windows x64, macOS Apple Silicon, and Linux x64.

For rendering or file-handling changes, add or update a focused Rust test and, when useful, a minimal file under `fixtures/`.

## Scope

Good contributions include correctness fixes, accessibility improvements, security hardening, small performance improvements, packaging fixes, and carefully scoped Markdown compatibility work.

Features that turn FeatherMark into an IDE, note platform, vault manager, cloud service, or rich-text editor are unlikely to be accepted. New dependencies need a clear size, maintenance, and security justification.

## Pull requests

- Keep each pull request focused on one change.
- Explain the user-visible outcome and the trade-offs.
- List the checks you ran and anything you could not verify.
- Include before-and-after screenshots for visible interface changes.
- Do not commit `node_modules`, `src-tauri/target`, `dist`, credentials, private fixtures, or generated local state.
- Update `README.md`, `FINAL_REPORT.md`, or `CHANGELOG.md` when the public behaviour or release status changes.

By contributing, you agree that your contribution is licensed under the project's MIT License.
