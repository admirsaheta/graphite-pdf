<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Rust facade crate for creating PDF files with the GraphitePDF stack.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-facade_%7C_layout_%7C_pdf-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf` is the wrapper crate for the GraphitePDF ecosystem.

Use it when you want one dependency that exposes:

- compatibility-facing document and style APIs
- the canonical layout, render, and renderer pipeline
- low-level `kit` PDF helpers
- shared types for fonts, images, SVG, math, primitives, and stylesheet resolution

The implementation is split across the workspace crates, but this package is the convenient front door when you want the GraphitePDF stack in one import path.

---

## Which Crate?

Use `graphitepdf` if you want to create PDFs through the crate facade.

If you only need part of the stack, you may want one of the lower-level crates instead:

- `graphitepdf-kit` for low-level PDF writing and page-content generation
- `graphitepdf-layout` for canonical page and node layout
- `graphitepdf-render` for typed render commands and the production PDF backend
- `graphitepdf-svg`, `graphitepdf-image`, `graphitepdf-font`, or `graphitepdf-textkit` for focused subsystem access

---

## Installation

```bash
cargo add graphitepdf
```

---

## How It Works

Build a document, run it through the facade, and save the returned PDF bytes:

```rust
use graphitepdf::{
    Color, EdgeInsets, FontSource, LayoutStyle, Node, Page, Pt, RendererSession, Size,
    StandardFont, TextBlock, TextSpan,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let title = TextBlock::from(TextSpan::new("Hello GraphitePDF")?);

    let document = graphitepdf::layout::Document::new().with_page(
        Page::new([
            Node::text(title).with_style(
                LayoutStyle::new()
                    .with_font_family("Helvetica")
                    .with_font_source(FontSource::standard(StandardFont::HelveticaBold))
                    .with_font_size(Pt::new(20.0))
                    .with_color(Color::rgb(0x17, 0x2b, 0x4d)),
            ),
        ])
        .with_size(Size::new(420.0, 220.0))
        .with_style(LayoutStyle::new().with_padding(EdgeInsets::all(Pt::new(24.0)))),
    );

    let mut session = RendererSession::new(document);
    let pdf = session.to_bytes()?;
    std::fs::write("example.pdf", pdf)?;

    Ok(())
}
```

---

## What You Get

The root crate re-exports the major GraphitePDF surfaces:

- document-facing types such as `Document`, `Node`, `TextNode`, `ImageNode`, and `Style`
- canonical pipeline crates such as `layout`, `render`, and `renderer`
- low-level PDF helpers from `kit`
- shared subsystems such as `font`, `image`, `svg`, `math`, `stylesheet`, `primitives`, and `textkit`

This makes `graphitepdf` a good default when you want ergonomic access to the whole stack without wiring individual crates together yourself.

---

## Architecture

The root crate is a facade, not the canonical implementation layer.

The underlying stack is organized around:

1. shared services such as `font`, `image`, `stylesheet`, `svg`, `math`, and `textkit`
2. `layout` for safe positioned layout output
3. `render` for typed render commands and backend integration
4. `kit` for low-level PDF emission

For the detailed crate graph and responsibilities, see `architecture/README.md`.

---

## Design Principles

- provide one ergonomic crate surface for the GraphitePDF stack
- keep the facade explicit about the underlying crate boundaries
- separate document modeling, layout, rendering, and PDF emission
- preserve access to lower-level crates when callers need more control

---

## License

MIT
