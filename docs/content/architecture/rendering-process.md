# Rendering Process

How GraphitePDF turns a document description into a PDF file.

## Overview

The pipeline has six discrete stages. Each stage has a single input type and a single output type. Stages are independent — you can stop at any point and inspect the intermediate result.

```text
Stage 1  Input            Document / Node tree
         ↓
Stage 2  Style            Cascade → resolved LayoutStyle per node
         ↓
Stage 3  Text layout      TextEngine → lines, fragments, metrics
         ↓
Stage 4  Layout           Box model → positioned SafeLayoutDocument
         ↓
Stage 5  Render           RenderEngine → Vec<RenderCommand>
         ↓
Stage 6  PDF emission     PdfRenderBackend + kit → PDF bytes
```

---

## Stage 1 — Input

You describe a document using the authoring types from `graphitepdf-layout` or the higher-level compat types from `graphitepdf-document`.

```rust
use graphitepdf_layout::{Document, Page, Node, LayoutStyle};
use graphitepdf_primitives::{Pt, Size};

let mut doc = Document::new();
doc.add_page(
    Page::new([
        Node::text("Hello, GraphitePDF")
            .with_style(LayoutStyle::new().with_font_size(Pt::new(24.0))),
    ])
    .with_size(Size::A4),
);
```

The document tree is just data at this point. No computation has happened.

---

## Stage 2 — Style resolution

`graphitepdf-stylesheet` resolves the cascade for each node:

- shorthand properties are expanded (`margin` → `margin-top`, `margin-right`, …)
- inherited values flow from parent to child
- unit values (`em`, `%`) are resolved to absolute `Pt`
- conflicting declarations are resolved by specificity

The output is a resolved `LayoutStyle` attached to each node — a flat, concrete set of values with no inheritance chains or relative units remaining.

---

## Stage 3 — Text layout

For every text node, `graphitepdf-textkit` runs the full text pipeline:

1. **Attribution** — spans are merged into a single `AttributedString` with per-character attributes
2. **Script detection** — Unicode script and bidi categories are assigned per character
3. **Run segmentation** — the string is split at script or direction boundaries
4. **Font substitution** — each run finds a concrete font; fallbacks are resolved via `graphitepdf-font`
5. **Line breaking** — runs are broken into tokens; tokens are packed into lines respecting the container width, with optional hyphenation
6. **Fragment layout** — each line fragment gets a `TextRect`, baseline, and direction

The result is a `TextLayout` — a fully positioned set of `LineFragment`s that the layout engine can place.

---

## Stage 4 — Layout computation

`graphitepdf-layout` runs the ordered pipeline across every node in the document tree:

| Step | What happens |
| --- | --- |
| Style resolution | inherited values finalized |
| Asset sizing | images and SVG measured at their display size |
| Text layout | text nodes receive their `TextLayout` |
| Box computation | margins, padding, borders, width, height resolved |
| Pagination | nodes are distributed across pages |
| Origin assignment | absolute `(x, y)` coordinates assigned to every node |
| Z-index ordering | stacking order finalised |

The output is a `SafeLayoutDocument` — a fully positioned, paginated scene graph where every coordinate is in absolute `Pt` space. No further geometry computation is needed downstream.

---

## Stage 5 — Render commands

`graphitepdf-render::RenderEngine` walks the `SafeLayoutDocument` and emits typed `RenderCommand`s:

| Command | Meaning |
| --- | --- |
| `DrawText` | text fragment with font, size, position, color |
| `DrawImage` | raster image with bounds |
| `DrawSvg` | vector scene with transform |
| `FillRect` | solid rectangle (background) |
| `StrokeBorder` | border edge |
| `PushTransform` | affine transform scope |
| `PopTransform` | end transform scope |
| `Clip` | clipping region |

The `RenderDocument` is a pure, serializable data structure. It has no PDF-specific types and can be consumed by any backend.

---

## Stage 6 — PDF emission

`PdfRenderBackend` turns the `RenderDocument` into PDF bytes:

1. **Font registration** — each font used in the document is embedded or referenced via `graphitepdf-kit::FontRegistry`
2. **Image encoding** — PNG images are embedded as-is; JPEG images use DCT compression
3. **Page content streams** — each page's commands are serialized to PDF page content using `graphitepdf-kit::Canvas` and `TextBuilder`
4. **PDF structure** — the document catalog, page tree, resource dictionaries, and cross-reference table are assembled by `graphitepdf-kit::DocumentBuilder`
5. **Output** — the final PDF/1.7 byte stream is written to a `Vec<u8>`, a file, or any `Write` implementor

```rust
use graphitepdf_render::{RenderEngine, render_to_bytes};

let layout_doc = /* ... stage 4 output ... */;
let bytes: Vec<u8> = render_to_bytes(&layout_doc)?;
std::fs::write("output.pdf", bytes)?;
```

---

## End-to-end in one call

For the common case, `RendererSession` drives the full pipeline:

```rust
use graphitepdf_render::RendererSession;
use graphitepdf_layout::{Document, Page, Node, LayoutStyle};
use graphitepdf_primitives::{Pt, Size};

let mut doc = Document::new();
doc.add_page(
    Page::new([Node::text("Hello")])
        .with_size(Size::A4),
);

let bytes = RendererSession::new(doc)
    .render_snapshot()?
    .to_bytes()?;
```
