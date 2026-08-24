# Contributing to FeatherMark

Thank you for helping improve FeatherMark. The project values focused changes that keep startup fast, memory use understandable, dependencies few, and the interface calm.

## Before opening an issue

- Search existing issues first.
- Test the latest release or current `main` branch.
- Remove private information from Markdown files and screenshots.
- Reduce rendering problems to the smallest Markdown example that still reproduces them.

Use the bug-report template for defects and the feature-request template for proposed changes. Security issues belong in the private process described in [SECURITY.md](SECURITY.md), not a public issue.

## Development setup

Windows is the currently verified development platform. Install Rust stable with the MSVC target, Visual Studio C++ Build Tools, Node.js LTS, and WebView2 Runtime.

```powershell
npm.cmd ci
npm.cmd run dev
```

Before submitting a pull request, run:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
node --check src/app.js
npm.cmd run check:themes
npm.cmd run build
```

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
