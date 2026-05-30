# graphitepdf-style

Compatibility style facade. Maps `graphitepdf-stylesheet` types to a legacy `Style` shape and provides conversion to `graphitepdf-layout::LayoutStyle`.

```toml
[dependencies]
graphitepdf-style = "0.2"
```

## Key types

### `Style` — compatibility style record

```rust
use graphitepdf_style::{Style, StyleValue, JustifyContent, AlignItems, FlexDirection};

let style = Style {
    font_family: Some("Helvetica".into()),
    font_size: Some(14.0),
    font_weight: Some(700),
    color: Some([0xC4, 0xC4, 0xC0]),
    background_color: Some([0x1E, 0x1E, 0x1C]),
    padding: Some([16.0, 16.0, 16.0, 16.0]),  // top right bottom left
    margin: Some([8.0, 0.0, 8.0, 0.0]),
    width: Some(400.0),
    height: Some(100.0),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::FlexStart,
    flex_direction: FlexDirection::Column,
    ..Default::default()
};
```

### Re-exported font types

`graphitepdf-style` re-exports `FontDescriptor`, `FontSource`, `FontStyle`, `FontVariantWeight`, and `StandardFont` from `graphitepdf-font` so callers only need this one dependency for styled documents.

### `Stylesheet` and `StylesheetContainer`

Re-exported from `graphitepdf-stylesheet`. Use these to build reusable named style rules.

## Conversion to layout types

```rust
use graphitepdf_style::Style;
use graphitepdf_layout::LayoutStyle;

let compat_style = Style { font_size: Some(20.0), ..Default::default() };
let layout_style: LayoutStyle = LayoutStyle::from(compat_style);
```

## Who should use this

If you are building on the compatibility `Document` / `Node` API from `graphitepdf-document`, use `graphitepdf-style` for all styling. If you are building on the crate-native API directly, use `graphitepdf-layout::LayoutStyle` instead.

## Dependencies

`graphitepdf-font`, `graphitepdf-layout`, `graphitepdf-primitives`, `graphitepdf-stylesheet`
