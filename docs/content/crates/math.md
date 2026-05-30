# graphitepdf-math

LaTeX to SVG scene conversion for embedding math in PDF documents.

```toml
[dependencies]
graphitepdf-math = "0.2"
```

## What it does

Converts LaTeX math strings into `SvgNode` scene trees using `mathjax-svg-rs`. The output is the same scene type used by `graphitepdf-svg`, so it feeds directly into the standard vector rendering path.

## Key types

```rust
pub enum MathDimension {
    Width(f32),
    Height(f32),
}

pub struct MathOptions {
    pub display: bool,       // display vs inline mode
    pub font_size: f32,
    pub color: Option<String>,
}
```

## Usage

```rust
use graphitepdf_math::{render_math, render_math_with_options, MathOptions};
use graphitepdf_svg::SvgNode;

// inline math
let scene: SvgNode = render_math("E = mc^2")?;

// display math with options
let scene = render_math_with_options(
    r"\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}",
    MathOptions { display: true, font_size: 16.0, color: None },
)?;
```

The resulting `SvgNode` can be embedded in a layout `Node` as an SVG image.

## Requirements

Requires Rust **1.88** or later — `mathjax-svg-rs` 0.4.0 depends on `boa_engine` 0.21 which sets this minimum.

## Dependencies

`graphitepdf-svg`, `graphitepdf-primitives`, `graphitepdf-errors`, `mathjax-svg-rs = "=0.4.0"`
