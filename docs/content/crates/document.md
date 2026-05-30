# graphitepdf-document

Compatibility document facade. Provides a high-level `Document` / `Node` authoring API that lowers into the crate-native layout and render pipeline.

```toml
[dependencies]
graphitepdf-document = "0.2"
```

## Document model

### `Document`

```rust
use graphitepdf_document::{Document, Node, TextNode, ImageNode, PdfMetadata};

let mut doc = Document::new();
doc.set_metadata(PdfMetadata {
    title: Some("My Report".into()),
    author: Some("ACME".into()),
    ..Default::default()
});
```

### `Node` — content element

```rust
use graphitepdf_document::{Node, NodeKind, TextNode, ImageNode};
use graphitepdf_image::LocalImageSource;

// text node
let text = Node {
    kind: NodeKind::Text(TextNode {
        value: "Hello from graphitepdf".into(),
        ..Default::default()
    }),
    style: Default::default(),
    children: vec![],
};

// image node
let image = Node {
    kind: NodeKind::Image(ImageNode {
        src: LocalImageSource::new("/images/logo.png").into(),
        ..Default::default()
    }),
    style: Default::default(),
    children: vec![],
};
```

### `Style` — node appearance

The document crate uses `graphitepdf-style`'s `Style` type. See [graphitepdf-style](/docs/crates/style) for the full API.

## Rendering a document

```rust
use graphitepdf_document::Document;
use graphitepdf::{renderer, render};

let doc: Document = /* ... */;

// render to PDF bytes
let rendered = renderer::render_to_bytes(&doc).await?;
let bytes = rendered.to_bytes()?;
std::fs::write("output.pdf", bytes)?;
```

## Relationship to the crate-native API

`graphitepdf-document` is an adapter layer. Internally it:

1. Converts `Document` → `graphitepdf-layout::Document`
2. Calls the layout pipeline
3. Delegates rendering to `graphitepdf-render` and `graphitepdf-renderer`

If you want direct access to layout types without the compatibility wrapper, use `graphitepdf-layout` and `graphitepdf-render` directly.

## Dependencies

`graphitepdf-style`, `graphitepdf-image`, `graphitepdf-layout`, `graphitepdf-primitives`, `graphitepdf-render`, `graphitepdf-renderer`, `graphitepdf-textkit`
