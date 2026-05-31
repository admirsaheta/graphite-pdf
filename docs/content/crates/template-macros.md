# graphitepdf-template-macros

Proc-macro crate that provides the `pdf!`, `styles!`, and `stylesheet!` macros. This crate has no runtime code — it is a pure `proc-macro = true` library.

```toml
[dependencies]
graphitepdf-template-macros = "0.2"
```

In most cases you access these macros through the `graphitepdf` facade with the `template` feature rather than depending on this crate directly.

## Macros

### `pdf!`

Compiles a JSX-like element tree into a `graphitepdf_layout::Document`:

```rust
use graphitepdf::{pdf, styles};

let document = pdf! {
    <Document>
        <Page size="A4">
            <View style={styles! { background_color: "#F8FAFC" }}>
                <Text style={styles! { font_size: 20.0, color: "#0F172A" }}>
                    "Welcome to GraphitePDF"
                </Text>
            </View>
        </Page>
    </Document>
};
```

Supported elements: `Document`, `Page`, `View`, `Text`, `Image`. Any `{ expr }` child is spliced in verbatim. See [Macro Reference](/templating/macros) for the full element and attribute specification.

### `styles!`

Produces a `LayoutStyle` from a compact property list. All supported color literals are resolved to `Color` values at compile time:

```rust
let card_style = styles! {
    width: 400.0,
    background_color: "#FFFFFF",
    font_size: 14.0,
    font_weight: bold,
};
```

### `stylesheet!`

Produces an anonymous struct with one `LayoutStyle` field per named entry. Useful for defining a design system once and referencing it by name:

```rust
let theme = stylesheet! {
    .h1 => { font_size: 28.0, font_weight: bold, color: "#0F172A" },
    .h2 => { font_size: 20.0, font_weight: semi_bold, color: "#1E293B" },
    .p  => { font_size: 12.0, line_height: 18.0, color: "#334155" },
};

// Fields are plain LayoutStyle
node.with_style(theme.h1.clone());
```

## Compile-time guarantees

- **Duplicate keys** in `styles!` or `stylesheet!` are a compile error.
- **Unknown keys** in `styles!` are a compile error.
- **Mismatched closing tags** in `pdf!` are a compile error.
- **Color hex strings** are validated and converted to `Color::rgb(…)` at macro expansion time — invalid hex is a compile error.

## Dependencies

`proc-macro2`, `quote`, `syn`

## See also

- [Template DSL Overview](/templating/overview)
- [Macro Reference](/templating/macros)
- [graphitepdf-template](/crates/template)
