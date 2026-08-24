# Changelog

All notable changes to FeatherMark are documented here. The project follows [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Compact always-visible theme dropdown with ten bundled choices.
- GitHub, Nord, Solarized, Sepia, and Dracula reading palettes without additional runtime dependencies.
- `Ctrl+D` shortcut for quickly toggling Feather Light and Feather Dark.
- Automated contrast checks for primary text, muted text, and links across every palette.
- Dracula as the default theme for new installations and portable launches.
- A lightweight document context menu with direct Edit/Preview, Save, Save As, Reload, and Open actions.
- The always-visible Edit/Preview button moved beside the tab controls so it remains easy to reach.

### Planned

- Code signing for public Windows releases.
- Clean-machine verification on supported Windows 10 and Windows 11 versions.
- Native packaging investigation for macOS and Linux.

## [0.1.0] - 2026-08-23

### Added

- Secure GFM-style Markdown rendering with tables, task lists, footnotes, strikethrough, code blocks, and safe relative images.
- Lightweight multi-document tabs using one shared rendered view.
- Shallow file panel, heading outline, local Markdown navigation, and drag-and-drop opening.
- Plain-text edit mode, debounced preview, explicit Save and Save As, and unsaved-change warnings.
- System, light, and dark themes; text-size controls; full screen; and persistent minimal preferences.
- Windows x64 portable executable and per-user NSIS installer.
- Optional `.md` and `.markdown` registration with a user-confirmed Windows Default Apps handoff.
- Installed-app entries in Windows **Open with** and the Markdown file context menu.
- Focused tests and Markdown fixtures for rendering, file loading, unsafe input, path resolution, tab state, and portable behaviour.
