# GFM edge-case fixture

This original FeatherMark fixture collects layout cases that often expose renderer defects.

## Heading depth

### Third level

#### Fourth level

##### Fifth level

###### Sixth level

## Nested content

- Parent item with **strong text** and ~~removed text~~
  1. Ordered child with *emphasis*
  2. Another ordered child
     - [x] Finished nested task
     - [ ] Pending nested task
       - A third nesting level with `inline_code()`

1. A list item containing a fenced block:

   ```json
   {
     "viewer": "FeatherMark",
     "autosave": false
   }
   ```

2. Content after the fenced block must not overlap it.

## Table alignment and wrapping

| Left | Centre | Right |
| :--- | :----: | ----: |
| short | `C:\a\deliberately\long\path\that\must\remain\inside\its\cell\file.md` | 42 |
| **bold** | [safe link](https://example.com/docs?q=markdown) | 1,024 |

## Footnote

FeatherMark enables GFM-style footnotes for useful technical prose.[^detail]

[^detail]: Footnote content can include **formatting** and `code`.

## Escapes and punctuation

Literal markers: \*not emphasis\*, \# not a heading, and an em dash — beside “curly quotes”.

## Unicode fallback

English · Ελληνικά · 日本語 · 한국어 · العربية · हिन्दी · 🪶

## Raw HTML safety

<section data-test="raw-html">This tag must be visible as text, not interpreted.</section>
