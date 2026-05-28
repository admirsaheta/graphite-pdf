<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>High-level rendering facade and output-oriented helpers for GraphitePDF.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--renderer-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-session_%7C_output_%7C_facade-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-renderer` provides the higher-level rendering facade for GraphitePDF and re-exports the rendering surface from `graphitepdf-render`.

---

## Scope

`graphitepdf-renderer` contains:

- re-exported render-session and backend APIs from `graphitepdf-render`
- output-oriented helpers for bytes, writers, and files
- examples that demonstrate the split `layout -> render -> renderer` flow

---

## Installation

```bash
cargo add graphitepdf-renderer
```

---

## API Summary

| Category | Items |
| --- | --- |
| Re-exported render API | `RenderCommand`, `RenderEngine`, `RendererSession`, `RenderBackend` |
| Output helpers | `render_to_bytes()`, `render_to_writer()`, `render_to_file()` |
| Session flow | `DocumentContainer`, `RenderSnapshot`, `RendererSession` |

---

## Example

```rust
use graphitepdf_font::{FontSource, StandardFont};
use graphitepdf_layout::{Document, LayoutStyle, Node, Page};
use graphitepdf_primitives::Pt;
use graphitepdf_renderer::render_to_bytes;
use graphitepdf_textkit::{TextBlock, TextSpan};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = TextBlock::from(TextSpan::new("Renderer facade")?);
    let document = Document::new().with_page(
        Page::new([
            Node::text(body).with_style(
                LayoutStyle::new()
                    .with_font_family("Helvetica")
                    .with_font_source(FontSource::standard(StandardFont::Helvetica))
                    .with_font_size(Pt::new(12.0)),
            ),
        ]),
    );

    let pdf = render_to_bytes(&document)?;
    assert!(pdf.starts_with(b"%PDF-1.7"));
    Ok(())
}
```

---

## Design Principles

- keep the crate thin and output-focused
- mirror the underlying render APIs instead of inventing a second rendering engine
- provide a friendly entry point for callers that want session-style rendering
- stay aligned with the canonical render crate rather than diverging from it

---

## Role In GraphitePDF

This crate sits above `graphitepdf-render` as a convenience facade. The substantive rendering logic still lives in `render`.

---

## License

MIT
