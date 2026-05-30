# Compatibility

What GraphitePDF supports, partially supports, and does not yet support.

---

## PDF standard

| Feature | Status | Notes |
| --- | --- | --- |
| PDF version | ✓ | PDF 1.7 |
| Linearized PDF | ✗ | Not yet |
| PDF/A | ✗ | Planned |
| PDF/X | ✗ | Planned |
| Encryption | ✗ | Not implemented |
| Digital signatures | ✗ | Not implemented |
| Interactive forms (AcroForm) | ✗ | Not implemented |
| JavaScript actions | ✗ | Out of scope |
| Named destinations | ✗ | Planned |
| Outlines / bookmarks | ✗ | Planned |
| Page labels | ✗ | Planned |

---

## Layout

| Feature | Status | Notes |
| --- | --- | --- |
| Block flow | ✓ | |
| Padding | ✓ | per-edge via `EdgeInsets` |
| Margin | ✓ | per-edge |
| Border | ✓ | width, color |
| Width / height | ✓ | absolute `Pt` values |
| Min / max width / height | ⚠ | partial |
| Flex layout | ⚠ | flex direction and alignment, no wrapping |
| Grid layout | ✗ | Not planned |
| Absolute positioning | ✓ | |
| Z-index stacking | ✓ | |
| Overflow: hidden | ⚠ | clip regions supported |
| Overflow: scroll | ✗ | Not applicable (static PDF) |
| Page breaks | ✓ | automatic pagination |
| Explicit page breaks | ⚠ | in progress |

---

## Text

| Feature | Status | Notes |
| --- | --- | --- |
| Unicode text | ✓ | Full UTF-8 |
| Left-to-right (LTR) | ✓ | |
| Right-to-left (RTL) | ✓ | bidi via `textkit` |
| Mixed bidi | ✓ | per-run direction |
| Arabic script | ✓ | |
| Latin script | ✓ | |
| CJK | ⚠ | renders, no line-break rules yet |
| Font size | ✓ | |
| Line height | ✓ | |
| Letter spacing | ✗ | Planned |
| Text alignment: left | ✓ | |
| Text alignment: center | ✓ | |
| Text alignment: right | ✓ | |
| Text alignment: justify | ✓ | full justification with whitespace expansion |
| Text decoration | ⚠ | underline supported |
| Text transform | ✗ | Planned |
| Hyphenation | ✓ | optional, configurable per engine |
| Line breaking | ✓ | greedy best-fit |
| Orphan / widow control | ✗ | Planned |
| Superscript / subscript | ✗ | Planned |
| Inline code spans | ✗ | Planned |

---

## Fonts

| Feature | Status | Notes |
| --- | --- | --- |
| Standard PDF fonts (14) | ✓ | Helvetica, Times, Courier families + Symbol + ZapfDingbats |
| Custom TTF | ✓ | via `FontSource::local` or `FontSource::remote` |
| Custom OTF | ⚠ | parsed via ttf-parser; complex OTF features limited |
| Font fallback | ✓ | multi-level fallback chain in `textkit` |
| Font embedding | ✓ | subset embedding planned; currently full embedding |
| Font subsetting | ✗ | Planned |
| Variable fonts | ✗ | Planned |
| Web fonts (remote URL) | ✓ | async fetch via `graphitepdf-font` |
| Color fonts | ✗ | Not planned |

---

## Images

| Format | Status | Notes |
| --- | --- | --- |
| PNG | ✓ | lossless, full alpha channel |
| JPEG | ✓ | DCT-compressed, EXIF orientation applied |
| SVG | ✓ | parsed to `SvgNode` scene, rendered as vector |
| WebP | ✗ | Planned |
| GIF | ✗ | Not planned |
| AVIF | ✗ | Not planned |
| Remote image (URL) | ✓ | async fetch, LRU cache |
| Local file | ✓ | |
| Data URI | ✓ | `data:image/png;base64,...` |
| Inline bytes | ✓ | `Vec<u8>` |

---

## SVG

| Feature | Status | Notes |
| --- | --- | --- |
| Basic shapes | ✓ | rect, circle, ellipse, line, polyline, polygon, path |
| Fill | ✓ | solid color |
| Stroke | ✓ | color, width, linecap, linejoin |
| Transforms | ✓ | translate, scale, rotate, matrix |
| Gradients | ✗ | Planned |
| Patterns | ✗ | Not planned |
| Filters / effects | ✗ | Not planned |
| Text in SVG | ✗ | Planned |
| `<use>` / `<symbol>` | ✗ | Planned |
| `<image>` inside SVG | ✗ | Planned |
| viewBox | ✓ | width/height derived from viewBox |
| Clip paths | ✗ | Planned |
| Opacity | ✓ | fill-opacity, stroke-opacity |

---

## Math / LaTeX

| Feature | Status | Notes |
| --- | --- | --- |
| LaTeX rendering | ✓ | via `mathjax-svg-rs` → SVG scene |
| Inline math | ✓ | |
| Display math | ✓ | |
| Custom macros | ✗ | Not supported |
| MathML | ✗ | Not planned |

---

## Colors

| Feature | Status | Notes |
| --- | --- | --- |
| RGB | ✓ | `Color::rgb(r, g, b)` |
| RGBA (transparency) | ✗ | PDF transparency groups not yet wired |
| CMYK | ✗ | Planned |
| Named CSS colors | ✗ | Use `Color::rgb` directly |
| HSL | ✗ | Use `Color::rgb` directly |

---

## Page sizes

All `PageSize` values are in `Pt` (1/72 inch). Common presets:

| Size | Width | Height |
| --- | --- | --- |
| A4 | 595 pt | 842 pt |
| A3 | 842 pt | 1191 pt |
| A5 | 420 pt | 595 pt |
| Letter | 612 pt | 792 pt |
| Legal | 612 pt | 1008 pt |
| Tabloid | 792 pt | 1224 pt |
| Custom | any | any |

Portrait and landscape are both supported via `PageOrientation`.

---

## Status legend

| Symbol | Meaning |
| --- | --- |
| ✓ | Fully supported |
| ⚠ | Partial — works for common cases, edge cases missing |
| ✗ | Not supported |
