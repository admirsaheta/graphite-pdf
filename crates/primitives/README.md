![GraphitePDF primitives logo](https://user-images.githubusercontent.com/5600341/27505816-c8bc37aa-587f-11e7-9a86-08a2d081a8b9.png)

# `graphitepdf-primitives`

Primitive element type constants and shared foundational types for `graphitepdf`,
modeled after `@react-pdf/primitives`.

## Installation

```bash
cargo add graphitepdf-primitives
```

## Usage

```rust
use graphitepdf_primitives::{Document, Page, Text, View};

fn main() {
    assert_eq!(View, "VIEW");
    assert_eq!(Text, "TEXT");
    assert_eq!(Page, "PAGE");
    assert_eq!(Document, "DOCUMENT");
}
```

This crate provides:

- primitive element tags such as `Document`, `Page`, `View`, and `Text`
- shared geometric primitives such as `Bounds`, `Point`, and `Size`
- basic units and colors such as `Pt` and `Color`

## License

MIT
