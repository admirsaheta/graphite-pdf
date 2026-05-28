<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>The canonical layout engine and safe layout tree for the GraphitePDF workspace.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--layout-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-layout_%7C_pagination_%7C_safe_tree-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-layout` is the canonical layout layer for GraphitePDF.

It combines page sizing, style resolution, inheritance, asset sizing, text layout, SVG and math resolution, pagination, origin assignment, and z-index ordering into a safe layout tree used by downstream rendering.

---

## Scope

`graphitepdf-layout` contains:

- authoring types such as `Document`, `Page`, `Node`, and `LayoutStyle`
- the ordered layout pipeline in `ORDERED_PIPELINE`
- `SafeLayoutDocument`, `SafeLayoutPage`, and `SafeLayoutNode`
- `LayoutEngine` for turning document input into positioned layout output

---

## Installation

```bash
cargo add graphitepdf-layout
```

---

## API Summary

| Category | Items |
| --- | --- |
| Authoring types | `Document`, `Page`, `Node`, `NodeKind`, `LayoutMetadata` |
| Style and spacing | `LayoutStyle`, `EdgeInsets` |
| Pipeline | `ORDERED_PIPELINE`, `LayoutPipelineStep`, `LayoutEngine` |
| Safe output | `SafeLayoutDocument`, `SafeLayoutPage`, `SafeLayoutNode`, `SafeNodeKind`, `SafeLayoutStyle`, `SafeFont` |
| Legacy compatibility | `LayoutDocument`, `LayoutPage`, `LayoutNode`, `LayoutContent` |

---

## Example

```rust
use graphitepdf_font::{FontSource, StandardFont};
use graphitepdf_layout::{Document, EdgeInsets, LayoutEngine, LayoutStyle, Node, Page};
use graphitepdf_primitives::{Color, Pt, Size};
use graphitepdf_textkit::{TextBlock, TextSpan};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let title = TextBlock::from(TextSpan::new("GraphitePDF layout")?);

    let document = Document::new().with_page(
        Page::new([
            Node::text(title).with_style(
                LayoutStyle::new()
                    .with_font_family("Helvetica")
                    .with_font_source(FontSource::standard(StandardFont::HelveticaBold))
                    .with_font_size(Pt::new(18.0))
                    .with_color(Color::rgb(0x17, 0x2b, 0x4d)),
            ),
        ])
        .with_size(Size::new(420.0, 180.0))
        .with_style(LayoutStyle::new().with_padding(EdgeInsets::all(Pt::new(24.0)))),
    );

    let layout = LayoutEngine::new().layout_document(&document)?;
    assert_eq!(layout.pages().len(), 1);
    Ok(())
}
```

---

## Design Principles

- keep layout policy centralized in one canonical crate
- represent downstream-safe geometry and style explicitly
- make pipeline stages inspectable and testable
- integrate text, image, SVG, and math handling without binding directly to PDF output

---

## Role In GraphitePDF

This crate is the bridge between authoring inputs and rendering. Everything that needs positioned, paginated, and inherited scene data should flow through `graphitepdf-layout`.

---

## License

MIT
