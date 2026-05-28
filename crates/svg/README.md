<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Typed SVG parsing and conversion utilities for the GraphitePDF workspace.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--svg-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-svg_%7C_parser_%7C_scene-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-svg` parses SVG input into a typed `SvgNode` tree that downstream crates can inspect, size, transform, and render.

---

## Scope

`graphitepdf-svg` contains:

- `SvgNode` and `SvgNodeKind`
- `SvgProps` for normalized attribute storage
- `try_parse_svg()` and `parse_svg()` entry points
- typed tree output rather than raw XML strings

---

## Installation

```bash
cargo add graphitepdf-svg
```

---

## API Summary

| Category | Items |
| --- | --- |
| SVG tree | `SvgNode`, `SvgNodeKind`, `SvgProps` |
| Parsing | `try_parse_svg()`, `parse_svg()` |

---

## Example

```rust
use graphitepdf_svg::try_parse_svg;

fn main() -> graphitepdf_svg::Result<()> {
    let svg = try_parse_svg(
        r#"<svg width=\"10\" height=\"10\"><rect x=\"0\" y=\"0\" width=\"10\" height=\"10\" fill=\"black\"/></svg>"#,
    )?;

    assert_eq!(svg.children.len(), 1);
    Ok(())
}
```

---

## Design Principles

- expose parsed SVG as typed scene data
- keep parsing fallible and explicit
- avoid baking in PDF emission concerns
- make downstream sizing and rendering easier by normalizing structure early

---

## Role In GraphitePDF

This crate is the workspace's SVG scene layer. It feeds `image`, `math`, `layout`, `render`, and `kit`.

---

## License

MIT
