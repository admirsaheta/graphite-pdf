<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Compatibility style facade types for GraphitePDF document-facing APIs.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--style-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-style_%7C_compat_%7C_adapters-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-style` provides compatibility-oriented style types that can be derived from resolved stylesheet data and converted into `graphitepdf_layout::LayoutStyle`.

---

## Scope

`graphitepdf-style` contains:

- the compatibility `Style` type
- compatibility layout enums such as `FlexDirection`, `JustifyContent`, and `AlignItems`
- re-exported font and stylesheet types used by style-facing APIs
- conversion between resolved stylesheet values and layout-facing style data

---

## Installation

```bash
cargo add graphitepdf-style
```

---

## API Summary

| Category | Items |
| --- | --- |
| Style facade | `Style`, `EdgeInsets` |
| Layout-like enums | `FlexDirection`, `JustifyContent`, `AlignItems` |
| Font re-exports | `FontDescriptor`, `FontSource`, `FontStyle`, `FontVariantWeight`, `StandardFont` |
| Stylesheet re-exports | `Stylesheet`, `StylesheetContainer`, `StylesheetMap`, `StyleValue` |

---

## Example

```rust
use graphitepdf_style::{Style, StyleValue, Stylesheet, StylesheetContainer};

fn main() {
    let stylesheet = Stylesheet::new(StyleValue::Object(
        [
            ("fontFamily".to_string(), "Inter".into()),
            ("fontSize".to_string(), 14.0.into()),
            ("paddingTop".to_string(), 12.0.into()),
        ]
        .into_iter()
        .collect(),
    ));

    let style = Style::from_stylesheet(&StylesheetContainer::new(595.0, 842.0), &stylesheet);
    let layout_style = style.to_layout_style();

    assert_eq!(layout_style.font_family.as_deref(), Some("Inter"));
}
```

---

## Design Principles

- keep compatibility style behavior explicit
- reuse the canonical layout style model rather than diverging from it
- expose font and stylesheet inputs through a simpler facade
- avoid becoming a second independent style engine

---

## Role In GraphitePDF

This crate is an adapter used by `document`, the root facade, and compatibility-facing code. The canonical layout crate still owns downstream positioning and rendering behavior.

---

## License

MIT
