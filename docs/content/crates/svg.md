# graphitepdf-svg

SVG parser that produces a typed `SvgNode` scene tree for use in the GraphitePDF pipeline.

```toml
[dependencies]
graphitepdf-svg = "0.2"
```

## What it does

Parses SVG XML into a structured `SvgNode` tree. This tree is not rendered here — it is passed downstream to `graphitepdf-render` and `graphitepdf-kit`, which turn it into PDF vector content.

## Key types

```rust
pub enum SvgNodeKind {
    Svg,
    Group,
    Rect,
    Circle,
    Ellipse,
    Line,
    Polyline,
    Polygon,
    Path,
    Text,
    Image,
    Use,
    Defs,
    ClipPath,
    LinearGradient,
    RadialGradient,
    Stop,
    Unknown(String),
}

pub struct SvgNode {
    pub kind: SvgNodeKind,
    pub props: SvgProps,        // BTreeMap<String, String>
    pub children: Vec<SvgNode>,
}
```

## Parsing

```rust
use graphitepdf_svg::{parse_svg, try_parse_svg};

let svg_bytes: &[u8] = include_bytes!("icon.svg");
let node = parse_svg(svg_bytes)?;

// or fallible without panic:
let maybe = try_parse_svg(svg_bytes);
```

## Integration

`graphitepdf-image` resolves SVG byte sources into `Image::Svg(SvgImage)` values. `graphitepdf-math` produces `SvgNode` trees from LaTeX. Both feed the same downstream vector rendering path.

## Dependencies

`graphitepdf-primitives`, `graphitepdf-errors`
