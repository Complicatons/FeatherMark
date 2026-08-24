# FeatherMark release overview

> A focused Markdown viewer should make the document feel important and the application feel invisible.

## Project status

| Area | Status | Detail |
| :--- | :---: | :--- |
| Windows portable | **Ready** | One executable, no installation |
| Windows installer | **Ready** | Open with and optional file associations |
| macOS and Linux | **Ready** | Native packages for Intel, ARM64, and x64 |

## Release checklist

- [x] Secure GitHub-flavoured Markdown rendering
- [x] Relative images and local document links
- [x] Explicit saving and unsaved-change warnings
- [x] Accessible light and dark reading themes
- [x] Native Windows, macOS, and Linux packages
- [ ] Code signing for broad public distribution

## Configuration

```toml
[viewer]
theme = "dracula"
remote_images = false
autosave = false
```

Readable prose, `inline code`, **strong emphasis**, and useful structure stay clear without turning the viewer into an IDE.

English · Ελληνικά · 日本語 · 한국어 · العربية · हिन्दी · 🪶
