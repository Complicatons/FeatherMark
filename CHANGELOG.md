# Changelog

All notable changes to FeatherMark are documented here. The project follows [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Planned

- Publisher code signing and Apple notarization.
- Clean-machine and real-hardware verification across supported operating systems.

## [0.2.0] - 2026-08-24

### Added

- Native macOS DMGs for Apple Silicon and Intel Macs.
- Native Linux DEB and AppImage packages for x64 and ARM64 systems.
- Cross-platform CI on Windows, macOS, and Linux, plus five-platform release builds.
- Automatic release-asset completeness checks and a SHA-256 checksum manifest.
- macOS Finder file-open event handling and native Command-key shortcut labels.
- macOS and Linux installation, runtime, build, file-association, and unsigned-package guidance.

### Changed

- Moved Windows-only NSIS settings into a platform-specific Tauri configuration file.
- Updated the release workflow to publish only after all Windows, macOS, and Linux builds succeed.
- Bumped the application and package version to 0.2.0.

## [0.1.0] - 2026-08-23

### Added

- Secure GFM-style Markdown rendering with tables, task lists, footnotes, strikethrough, code blocks, and safe relative images.
- Lightweight multi-document tabs using one shared rendered view.
- Shallow file panel, heading outline, local Markdown navigation, and drag-and-drop opening.
- Plain-text edit mode, debounced preview, explicit Save and Save As, and unsaved-change warnings.
- System, light, and dark themes; text-size controls; full screen; and persistent minimal preferences.
- Windows x64 portable executable and per-user NSIS installer.
- Compact always-visible theme dropdown with ten bundled choices, Dracula as the first-run default, and automated contrast checks.
- `Ctrl+D` shortcut for quickly toggling Feather Light and Feather Dark.
- A lightweight document context menu with direct Edit/Preview, Save, Save As, Reload, and Open actions.
- An always-visible Edit/Preview button beside the tab controls.
- Optional `.md` and `.markdown` registration with a user-confirmed Windows Default Apps handoff.
- Installed-app entries in Windows **Open with** and the Markdown file context menu.
- Focused tests and Markdown fixtures for rendering, file loading, unsafe input, path resolution, tab state, and portable behaviour.
