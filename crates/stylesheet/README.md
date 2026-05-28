<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>CSS-like stylesheet flattening, media-query resolution, and normalization for GraphitePDF.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--stylesheet-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-stylesheets_%7C_units_%7C_resolution-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-stylesheet` provides CSS-like style maps, flattening, media-query resolution, unit normalization, and property expansion for GraphitePDF.

---

## Scope

`graphitepdf-stylesheet` contains:

- `Style`, `StyleValue`, and `Stylesheet`
- `Container` and `Orientation` for environment-dependent resolution
- `flatten()`, `resolve_media_queries()`, `resolve_style()`, and `resolve_styles()`
- color normalization helpers such as `transform_color()`

---

## Installation

```bash
cargo add graphitepdf-stylesheet
```

---

## API Summary

| Category | Items |
| --- | --- |
| Style model | `Style`, `SafeStyle`, `ExpandedStyle`, `StyleValue` |
| Resolution context | `Container`, `Orientation` |
| Stylesheets | `Stylesheet` |
| Resolution helpers | `flatten()`, `resolve_media_queries()`, `resolve_style()`, `resolve_styles()`, `transform_color()` |

---

## Example

```rust
use graphitepdf_stylesheet::{Container, Style, Stylesheet};

fn main() {
    let mut style = Style::new();
    style.insert("width".to_string(), "50%".into());
    style.insert("padding".to_string(), 12.0.into());
    style.insert("backgroundColor".to_string(), "rgb(212, 88, 26)".into());

    let stylesheet = Stylesheet::new(style);
    let resolved = stylesheet.resolve(&Container::new(400.0, 200.0));

    assert!(resolved.contains_key("width"));
    assert!(resolved.contains_key("paddingTop"));
}
```

---

## Design Principles

- keep stylesheet behavior deterministic and data-oriented
- separate CSS-like resolution from layout and rendering crates
- expose flattening and resolution as ordinary functions
- keep the crate useful outside of PDF-specific code paths

---

## Role In GraphitePDF

This crate sits in the shared-services layer and feeds resolved style data into `layout`, `style`, `document`, and the root facade.

---

## License

MIT
