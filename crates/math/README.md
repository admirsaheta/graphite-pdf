<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>LaTeX-to-SVG math scene generation for the GraphitePDF workspace.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--math-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-math_%7C_latex_%7C_svg-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-math` converts LaTeX input into typed SVG scene data that can flow through the same downstream pipeline as ordinary SVG content.

---

## Scope

`graphitepdf-math` contains:

- `MathOptions` for inline/display, dimensions, color, and debug output
- `MathRender` for the original source, raw SVG, and parsed `SvgNode`
- `render_math()` and `render_math_with_options()`
- `MathDimension` for numeric or unit-bearing width/height values

---

## Installation

```bash
cargo add graphitepdf-math
```

---

## API Summary

| Category | Items |
| --- | --- |
| Options | `MathOptions`, `MathDimension` |
| Render result | `MathRender` |
| SVG integration | `SvgNode`, `SvgNodeKind` |
| Entry points | `render_math()`, `render_math_with_options()` |

---

## Example

```rust
use graphitepdf_math::{MathOptions, render_math_with_options};

fn main() -> graphitepdf_math::Result<()> {
    let render = render_math_with_options(
        r"\\int_0^1 x^2 \\, dx",
        &MathOptions::new().inline(false).width("160").color("#1f2937"),
    )?;

    assert!(render.raw_svg.contains("<svg"));
    Ok(())
}
```

---

## Design Principles

- keep math rendering separate from PDF output concerns
- reuse the SVG pipeline instead of inventing a parallel scene format
- expose display and inline behavior explicitly
- keep the public surface small and easy to integrate

---

## Role In GraphitePDF

This crate feeds `layout`, `render`, and `kit` indirectly through typed SVG output. It is the workspace's math scene producer.

---

## License

MIT
