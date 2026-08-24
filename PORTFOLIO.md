# FeatherMark — portfolio case study

## One-line summary

FeatherMark is a small, fast, security-conscious Markdown viewer built with Rust and Tauri 2 for Windows, macOS, and Linux.

![FeatherMark using the Dracula theme with its file panel, Markdown reading view, outline, and nearby Edit control](docs/images/feathermark-reader.png)

## Short portfolio description

I designed and built FeatherMark to answer a simple question: can a desktop Markdown viewer remain genuinely lightweight without feeling unfinished?

Instead of creating another notes platform or Electron application, I focused on the core reading experience. FeatherMark opens local Markdown files quickly, renders GitHub-flavoured Markdown in a clean desktop interface, resolves safe relative images, supports lightweight tabs and outlines, and provides a deliberately basic raw-source editor for quick corrections.

The Windows portable build is approximately 9.13 MiB and the installer approximately 1.98 MiB. The application host reached a usable window in roughly 0.64–0.79 seconds on the test machine and used about 25 MiB working set at idle. It relies on the shared WebView2 runtime rather than bundling Chromium.

## The problem

Many Markdown tools are excellent at writing, project management, or knowledge organisation, but those capabilities add startup cost, memory use, interface complexity, and large downloads. I wanted a focused utility for opening a `.md` file as naturally as opening an image or PDF.

The difficult part was not simply rendering Markdown. A practical viewer also needs secure handling of untrusted files, relative images, file associations, command-line opening, drag and drop, useful navigation, explicit saving, unsaved-change protection, themes, and packaging that behaves correctly across different desktop operating systems.

## The solution

FeatherMark combines a compact Rust host with Tauri 2 and the operating system's WebView. Rust owns file access, document state, path validation, preferences, Markdown parsing, and platform integration. The interface is dependency-free HTML, CSS, and JavaScript.

The product deliberately avoids accounts, cloud storage, databases, plugins, telemetry, rich-text editing, automatic updates, background services, and a bundled browser engine.

## Notable engineering work

- Designed a tab model that stores multiple document states while reusing a single WebView and rendered surface.
- Treated Markdown as untrusted input by escaping raw HTML, blocking unsafe URL schemes, validating external links in Rust, applying a restrictive Content Security Policy, and preventing relative image paths from escaping the document directory.
- Built portable and per-user installer workflows, including optional Windows `.md` and `.markdown` registration without silently replacing the user's default application.
- Implemented explicit Save and Save As flows, debounced live preview, independent dirty state per document, and warnings before destructive actions.
- Added ten accessible reading themes with automated contrast validation and Dracula as the first-run default.
- Isolated Windows-specific shell integration and used platform-specific Tauri overrides to ship native NSIS, DMG, DEB, and AppImage packages without forking the core viewer.
- Built a five-platform GitHub release matrix for Windows x64, Intel and Apple Silicon macOS, and x64 and ARM64 Linux, with a completeness gate and generated SHA-256 manifest.
- Created focused fixtures and automated tests for file loading, GFM rendering, unsafe HTML, unsafe links, image confinement, tab state, dirty-state handling, and preferences.

## Outcome

- Portable Windows executable: approximately 9.13 MiB.
- Per-user Windows installer: approximately 1.98 MiB.
- Application host idle working set: approximately 25 MiB on the measured machine.
- Measured startup to a usable window: approximately 0.64–0.79 seconds.
- Fourteen focused Rust tests, automated Rust linting, JavaScript syntax checking, and theme contrast validation.
- No accounts, telemetry, database, bundled Chromium runtime, or background service.

Measurements are machine-specific. WebView2 subprocesses use additional shared memory; the repository's `FINAL_REPORT.md` records the complete figures and limitations.

## Technology

Rust, Tauri 2, pulldown-cmark, HTML, CSS, JavaScript, NSIS, DMG, DEB, AppImage, GitHub Actions, WebView2, WKWebView, and WebKitGTK.

## My role

Product definition, interface design, Rust and frontend implementation, Markdown security model, platform integration, cross-platform packaging, automated tests, fixtures, performance measurement, and release documentation.

## What I learned

“Lightweight” is not one measurement. Using the system WebView keeps the download and application host small, but the browser subprocesses still have a real memory cost. I chose to report both figures rather than present only the flattering one.

The project also reinforced that packaging is part of product design. File associations, portable-mode behaviour, unsaved-change handling, installer prompts, and useful failure messages matter as much as the Markdown parser when a utility is meant to feel dependable.

Finally, a small scope needs active protection. Each useful feature was weighed against startup cost, dependency count, interface clutter, and whether it moved FeatherMark toward becoming a general-purpose editor.

## Current status

Windows x64 is built and tested as both a portable application and installer. Version 0.2.0 adds native Intel and Apple Silicon macOS DMGs plus x64 and ARM64 Linux DEB/AppImage packages. All are built and tested on native GitHub-hosted runners; Windows has the strongest hands-on verification, while physical Mac and Linux UI passes remain pending. The packages do not yet carry paid publisher certificates or Apple notarization.

## Suggested portfolio card

**FeatherMark**<br>
A lightweight Rust Markdown viewer for Windows, macOS, and Linux. Secure local rendering, ten themes, basic source editing, native file integration, and platform packages without bundled Chromium.

`Rust` `Tauri 2` `Windows` `macOS` `Linux` `Desktop application` `Security`

## Suggested repository description

> A tiny, fast, security-conscious Markdown viewer for Windows, macOS, and Linux, built with Rust and Tauri 2.

## Links

- [Source code and documentation](https://github.com/Complicatons/FeatherMark)
- [Cross-platform downloads](https://github.com/Complicatons/FeatherMark/releases/latest)
- [Detailed measurements and limitations](FINAL_REPORT.md)
