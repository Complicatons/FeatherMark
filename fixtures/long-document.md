# Long-document fixture

This moderately long document is intentionally repetitive. It is large enough to exercise ordinary scrolling without pretending FeatherMark is a 100,000-line IDE.

## Section one

Markdown viewers should preserve a calm reading width even when the window is wide. This paragraph contains enough words to wrap across several lines and expose typography, spacing, and overflow problems.

```text
Code stays horizontally scrollable when a line is wider than the reading column: 0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ
```

## Section two

> A blockquote can span multiple lines while maintaining a clear relationship to the surrounding prose. It should remain readable in light and dark themes.

| Case | Expected result |
| --- | --- |
| Narrow table | Hugs its content within the reading column |
| Wide content | Scrolls inside the table instead of widening the page |

## Section three

1. First item
2. Second item
3. Third item
4. Fourth item
5. Fifth item

## Section four

- [x] Headings
- [x] Paragraphs
- [x] Code blocks
- [x] Tables
- [x] Task lists
- [x] Horizontal rules

---

End of the scrolling fixture.
