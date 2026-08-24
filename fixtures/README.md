# FeatherMark test fixtures

These documents are original test material for manually checking FeatherMark. Open them from the release build, pass one on the command line, or drag one onto the window.

| Fixture | Purpose |
| --- | --- |
| `sample.md` | Fast smoke test for normal prose, lists, tasks, code, tables, a relative image, and escaped HTML. |
| `gfm-edge-cases.md` | Deep headings and lists, fenced code inside a list, alignment, footnotes, escapes, and Unicode. |
| `typography-and-overflow.md` | Long inline code and URLs, nested quote/code layout, duplicate headings, fallback fonts, and reading measure. |
| `links-and-images.md` | Fragment, web, email, and local links; ordinary, percent-encoded, and missing relative images. |
| `long-document.md` | Scrolling, redraw, reload, and general responsiveness on a longer file. |
| `utf8.md` | A compact UTF-8 loading check across several writing systems. |
| `unsafe.md` | Raw script markup, a JavaScript URL, a remote image, and an absolute local image that must all remain inert or blocked. |

## Short manual pass

1. Open `sample.md` and check every visible element, including the local image and raw HTML text.
2. Open `gfm-edge-cases.md`, resize the window, and confirm nested content stays within the reading column.
3. Open `typography-and-overflow.md`; test narrow and wide windows plus Ctrl++/Ctrl-/Ctrl+0.
4. Open `links-and-images.md`; follow the fragment, web, email, and local-document links and inspect all three image outcomes.
5. Open `unsafe.md`; confirm nothing runs, the unsafe link is blocked, and neither remote nor absolute images load.
6. Edit a disposable copy, wait for preview, save explicitly, then create another change and exercise the Open, Reload, and close warnings.

## Expected security outcomes

- Raw HTML is displayed as text rather than interpreted.
- `javascript:` and unsupported link schemes cannot be launched.
- Only `http`, `https`, and `mailto` links can leave the application.
- Images are loaded only from supported files contained by the opened document's directory.
- Remote, absolute, escaping, missing, and unsupported image paths fail safely.

The fixtures describe expected behaviour, not instructions to be executed by a Markdown viewer.
