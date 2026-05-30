# graphitepdf-stylesheet

CSS-influenced stylesheet engine: cascade, shorthand expansion, unit resolution, and media query handling.

```toml
[dependencies]
graphitepdf-stylesheet = "0.2"
```

## Key types

```rust
pub type Style       = IndexMap<String, StyleValue>;
pub type SafeStyle   = Style;      // post-cascade, all values resolved
pub type ExpandedStyle = Style;    // shorthands expanded, not yet resolved

pub enum StyleValue {
    String(String),
    Number(f32),
    Bool(bool),
    Array(Vec<StyleValue>),
    Object(IndexMap<String, StyleValue>),
    Null,
}
```

## Stylesheet container

```rust
use graphitepdf_stylesheet::{Stylesheet, StylesheetContainer, StylesheetMap};

// define reusable styles
let mut sheet = Stylesheet::new();
sheet.insert("heading".into(), {
    let mut s = Style::new();
    s.insert("font-size".into(), StyleValue::Number(24.0));
    s.insert("font-weight".into(), StyleValue::String("bold".into()));
    s
});

let container = StylesheetContainer::new(sheet);
```

## Cascade

The cascade resolves inheritance from parent to child nodes, expands shorthands like `margin` into `margin-top` / `margin-right` / `margin-bottom` / `margin-left`, and resolves relative units (`em`, `%`) to absolute `Pt` values.

## Integration

`graphitepdf-layout` drives cascade resolution across the full document tree. `graphitepdf-style` provides a compatibility facade that maps these types to the legacy `Style` shape.

## Dependencies

`graphitepdf-errors`
