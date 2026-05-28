<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Typed render-command generation and the production PDF backend for GraphitePDF.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--render-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-render_%7C_backend_%7C_pdf-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-render` lowers layout output into typed render commands and also provides the concrete PDF backend used in the production crate-native pipeline.

---

## Scope

`graphitepdf-render` contains:

- `RenderDocument`, `RenderPage`, and `RenderCommand`
- typed operations for text, images, SVG, fills, borders, transforms, forms, and debug overlays
- `RenderEngine` for lowering safe layout output into render commands
- `RendererSession`, `PdfRenderBackend`, and `render_to_*` helpers for end-to-end PDF generation

---

## Installation

```bash
cargo add graphitepdf-render
```

---

## API Summary

| Category | Items |
| --- | --- |
| Render model | `RenderDocument`, `RenderPage`, `RenderCommand`, `RenderContext` |
| Operations | `TextRenderOp`, `ImageRenderOp`, `SvgRenderOp`, `FillRectOp`, `BorderRenderOp`, `TransformRenderOp` |
| Engines | `RenderEngine`, `RendererSession`, `Renderer`, `PdfRenderBackend` |
| Output helpers | `render_to_bytes()`, `render_to_writer()`, `render_to_file()` |
| Utilities | `fit_object()`, `parse_color()`, `parse_transform()`, `parse_view_box()`, `resolve_svg_size()` |

---

## Example

```rust
use graphitepdf_font::{FontSource, StandardFont};
use graphitepdf_layout::{Document, LayoutEngine, LayoutStyle, Node, Page};
use graphitepdf_primitives::Pt;
use graphitepdf_render::{RenderEngine, render_to_bytes};
use graphitepdf_textkit::{TextBlock, TextSpan};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let block = TextBlock::from(TextSpan::new("Render me")?);
    let document = Document::new().with_page(
        Page::new([
            Node::text(block).with_style(
                LayoutStyle::new()
                    .with_font_family("Helvetica")
                    .with_font_source(FontSource::standard(StandardFont::Helvetica))
                    .with_font_size(Pt::new(12.0)),
            ),
        ]),
    );

    let layout = LayoutEngine::new().layout_document(&document)?;
    let render_document = RenderEngine::new().build(&layout)?;
    assert_eq!(render_document.pages.len(), 1);

    let pdf = render_to_bytes(&document)?;
    assert!(pdf.starts_with(b"%PDF-1.7"));
    Ok(())
}
```

---

## Design Principles

- keep render commands typed and inspectable
- separate layout decisions from backend emission details
- make the production PDF path available without forcing callers through the root crate
- reuse `kit` for low-level PDF mechanics rather than re-implementing them here

---

## Role In GraphitePDF

This crate is the canonical bridge from layout output to final PDF generation. It is the core of the production `layout -> render -> kit -> PDF` path.

---

## License

MIT
