<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Foundational element tags and shared low-level types for the GraphitePDF ecosystem.</strong>
</p>

<p align="center">
  Geometry, color, units, and semantic document tags for layout and rendering pipelines.
</p>

# graphitepdf-primitives

This crate provides the common vocabulary used by higher-level layout, document, and
rendering layers without pulling in engine-specific behavior.

## Scope

`graphitepdf-primitives` contains:

- document element tag constants such as `Document`, `Page`, `View`, and `Text`
- geometry types such as `Bounds`, `Point`, and `Size`
- basic styling and measurement primitives such as `Color` and `Pt`

The crate is intended to stay small, explicit, and reusable across other crates.

## Installation

```bash
cargo add graphitepdf-primitives
```

## API Summary

| Category | Items |
| --- | --- |
| Element tags | `Document`, `Page`, `View`, `Text` |
| Geometry | `Bounds`, `Point`, `Size` |
| Units and color | `Pt`, `Color` |

## Example

```rust
use graphitepdf_primitives::{Bounds, Color, Document, Page, Point, Pt, Size, Text, View};

fn main() {
    assert_eq!(Document, "DOCUMENT");
    assert_eq!(Page, "PAGE");
    assert_eq!(View, "VIEW");
    assert_eq!(Text, "TEXT");

    let origin = Point::new(0.0, 0.0);
    let page_size = Size::new(595.0, 842.0);
    let bounds = Bounds::new(origin, page_size);
    let accent = Color::rgb(212, 88, 26);
    let font_size = Pt::new(12.0);

    assert_eq!(bounds.size.width, 595.0);
    assert_eq!(accent.red, 212);
    assert_eq!(font_size.value(), 12.0);
}
```

## Design Principles

- keep foundational APIs renderer-agnostic
- expose explicit data structures over implicit conversions
- avoid unnecessary dependencies
- make shared document and layout types easy to reuse

## Role In GraphitePDF

This crate sits near the base of the GraphitePDF stack. It exists so downstream
crates can share geometry, units, and semantic element tags without duplicating
definitions or inheriting unrelated runtime concerns.

## License

MIT
