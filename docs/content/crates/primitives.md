# graphitepdf-primitives

Shared low-level data types used across every layer of GraphitePDF.

```toml
[dependencies]
graphitepdf-primitives = "0.2"
```

## Key types

### `Pt` — the unit of measurement

All measurements in GraphitePDF are in points (1 pt = 1/72 inch), matching the PDF coordinate space.

```rust
use graphitepdf_primitives::Pt;

let font_size = Pt::new(12.0);
let page_width = Pt::new(595.0);  // A4

// arithmetic
let double = font_size * 2.0;
let sum    = Pt::new(10.0) + Pt::new(5.0);
```

### `Size` — width × height

```rust
use graphitepdf_primitives::Size;

let a4     = Size::new(595.0, 842.0);   // A4 in points
let letter = Size::new(612.0, 792.0);   // US Letter
let square = Size::new(200.0, 200.0);
```

### `Point` — 2D coordinate

```rust
use graphitepdf_primitives::Point;

let origin  = Point::new(0.0, 0.0);
let shifted = Point::new(72.0, 144.0);  // 1 inch from left, 2 from top
```

### `Bounds` — origin + size

```rust
use graphitepdf_primitives::{Bounds, Point, Size};

let bounds = Bounds::new(Point::new(72.0, 72.0), Size::new(200.0, 50.0));
let (x, y, w, h) = (bounds.origin.x, bounds.origin.y,
                    bounds.size.width, bounds.size.height);
```

### `Color` — RGB color

```rust
use graphitepdf_primitives::Color;

let black  = Color::rgb(0x00, 0x00, 0x00);
let white  = Color::rgb(0xFF, 0xFF, 0xFF);
let orange = Color::rgb(0xD4, 0x58, 0x1A);  // graphitepdf rust accent
```

## Dependencies

No internal workspace dependencies.
