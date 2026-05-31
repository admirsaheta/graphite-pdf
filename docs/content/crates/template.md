# graphitepdf-template

Runtime support layer for the `pdf!` macro DSL. Exposes the `PdfNode` enum and re-exports the proc macros from `graphitepdf-template-macros`.

```toml
[dependencies]
graphitepdf = { version = "0.2", features = ["template"] }
```

When the `template` feature is enabled on the `graphitepdf` facade, this crate is wired in automatically. Direct use is rarely needed.

## What this crate provides

### `PdfNode`

The intermediate value type that `pdf!` nodes resolve to before the macro extracts the final `Document`:

```rust
pub enum PdfNode {
    Document(Document),
    Page(Page),
    Node(Node),
    Fragment(Vec<PdfNode>),
    Empty,
}
```

You do not construct `PdfNode` values directly — they are produced internally by the `pdf!` macro expansion.

### Re-exports

```rust
pub use graphitepdf_template_macros::{pdf, stylesheet, styles};
pub use graphitepdf_kit::PageSize;
pub use graphitepdf_layout::{LayoutMetadata, LayoutStyle};
```

### `__private` module

Conversion helpers used by macro-generated code. Not part of the public API — names and types may change across minor versions.

| Symbol | Role |
| --- | --- |
| `into_pdf_size` | Convert `PageSize`, `(f32, f32)`, or `Size` to `Size` |
| `into_layout_nodes` | Convert `Node`, `Vec<Node>`, `Option<Node>`, etc. |
| `into_layout_pages` | Convert `Page`, `Vec<Page>`, `Option<Page>`, etc. |
| `text_node_from_str` | Build a `Node::text` from a `&str` (used for string literal children) |
| `text_node_from_string` | Build a `Node::text` from a `String` |

## Dependencies

`graphitepdf-image`, `graphitepdf-kit`, `graphitepdf-layout`, `graphitepdf-primitives`, `graphitepdf-template-macros`, `graphitepdf-textkit`

## See also

- [Template DSL Overview](/templating/overview)
- [Macro Reference](/templating/macros)
- [graphitepdf-template-macros](/crates/template-macros)
