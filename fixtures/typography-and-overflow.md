# Typography and overflow fixture

This original fixture checks the reading surface when ordinary prose meets awkward content. It mixes **strong text**, *emphasis*, ~~strikethrough~~, and `inline_code()` without changing the line height.

## Long inline code

Text before `C:\Users\example\Documents\a-very-long-project-name\artifacts\2026-08-23\build-output\feathermark-portable-release-candidate.md` text after. The code may wrap, but it must not push the document beyond the window or hide the toolbar.

An unbroken token should remain recoverable rather than clipping the whole page: `feathermark_build_identifier_0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz_end`.

## Long external link

[Open a deliberately long HTTPS destination](https://example.com/markdown/viewer/testing?fixture=typography-and-overflow&mode=preview&source=feathermark&value=0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ).

## Nested quotation and code

> A quoted introduction can span more than one line and should retain a clear left edge.
>
> ```rust
> fn is_lightweight(memory_mb: u64) -> bool {
>     memory_mb < 100
> }
> ```
>
> Text after the fenced block still belongs to the quotation.

## Repeated headings

### Status {#status-first}

The first repeated heading has an explicit, stable fragment identifier.

### Status {#status-second}

The second uses a different explicit identifier and should not collide with the first.

## Scripts and fallback fonts

Combining characters: Café and naïve.

Greek: Ελληνικά · Japanese: 日本語 · Korean: 한국어 · Arabic: العربية · Hindi: हिन्दी · Emoji: 🪶 ✅ ⚠️

## Reading rhythm

Good typography should disappear while you read it. This paragraph is intentionally longer than the others so the maximum line width, paragraph spacing, contrast, and approximately one-and-a-half line height can be judged together at several text sizes. Resize the window narrowly, then widen it again; the measure should remain comfortable rather than stretching from edge to edge.
