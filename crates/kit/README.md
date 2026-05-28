<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>A native Rust PDF generation library for the GraphitePDF ecosystem.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--kit-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-pdf_%7C_vector_%7C_text-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-kit` provides low-level but ergonomic PDF generation capabilities for Rust. It is designed for systems-level use and integration with the GraphitePDF layout engine.

---

## Scope

`graphitepdf-kit` contains:

- PDF document building with `DocumentBuilder` and `Page`
- Vector graphics (shapes, paths, colors) with `Canvas`
- Text rendering with standard fonts via `TextBuilder`
- Page sizing (A series, Letter, custom) with `PageSize`
- Metadata support with `Metadata`
- Compressed content streams via `flate2`

---

## Installation

```bash
cargo add graphitepdf-kit
```

---

## API Summary

| Category | Items |
| --- | --- |
| Document | `DocumentBuilder`, `Page` |
| Pages & Layout | `PageSize`, `PageMargins` |
| Text | `TextBuilder`, `TextAlignment` |
| Vector Graphics | `Canvas`, `Color`, `GraphicsState` |
| Metadata | `Metadata` |

---

## Example

```rust
use graphitepdf_kit::{
    DocumentBuilder,
    PageSize,
    TextBuilder,
    Canvas,
    Color,
    Metadata,
};

fn main() {
    // Build text content
    let text = TextBuilder::new()
        .font("F1", 24.0)
        .position(100.0, 700.0)
        .text("Hello from graphitepdf-kit!")
        .finish();

    // Build vector content
    let graphics = Canvas::new()
        .set_fill_color(Color::BLUE)
        .rect(100.0, 650.0, 200.0, 20.0)
        .fill()
        .set_stroke_color(Color::RED)
        .set_line_width(2.0)
        .rect(320.0, 650.0, 200.0, 20.0)
        .stroke()
        .finish();

    // Combine content
    let content = [text, graphics].concat();

    // Build and save document
    DocumentBuilder::new()
        .metadata(Metadata::new()
            .title("Sample PDF")
            .author("GraphitePDF")
            .subject("Rust PDF Generation")
        )
        .with_page(PageSize::A4, content)
        .save("output.pdf")
        .expect("Failed to write PDF");
}
```

---

## Design Principles

- Native Rust implementation with zero PDF library dependencies
- Explicit APIs over implicit magic
- Zero-cost abstractions where possible
- Minimal and focused dependencies
- Clear integration points for GraphitePDF layout engine

---

## Role In GraphitePDF

`graphitepdf-kit` is the core PDF generation crate used by the main GraphitePDF library to convert laid-out content into final PDF files. It can also be used standalone for basic PDF generation tasks.

---

## License

MIT
