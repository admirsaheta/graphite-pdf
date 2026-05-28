<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Foundational low-level geometry, units, colors, and element tags for GraphitePDF.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--primitives-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-geometry_%7C_units_%7C_tags-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-primitives` provides the common low-level vocabulary used by higher-level GraphitePDF crates without pulling in layout or PDF-specific policy.

---

## Scope

`graphitepdf-primitives` contains:

- document element tags such as `Document`, `Page`, `View`, and `Text`
- geometry types such as `Bounds`, `Point`, and `Size`
- basic units and color types such as `Pt` and `Color`

---

## Installation

```bash
cargo add graphitepdf-primitives
```

---

## API Summary

| Category | Items |
| --- | --- |
| Element tags | `Document`, `Page`, `View`, `Text` |
| Geometry | `Bounds`, `Point`, `Size` |
| Units and color | `Pt`, `Color` |

---

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

---

## Design Principles

- keep foundational APIs renderer-agnostic
- expose explicit data structures over implicit conversions
- avoid unnecessary dependencies
- make shared document and layout types easy to reuse

---

## Role In GraphitePDF

This crate sits at the base of the GraphitePDF stack and gives downstream crates a stable set of shared low-level types.

---

## License

MIT
