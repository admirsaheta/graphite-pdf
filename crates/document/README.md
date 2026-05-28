<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Compatibility-friendly document facade types for the GraphitePDF crate ecosystem.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--document-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-document_%7C_compat_%7C_adapters-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-document` provides compatibility-oriented `Document`, `Node`, `TextNode`, and `ImageNode` types that can be lowered into the canonical GraphitePDF layout and render pipeline.

---

## Scope

`graphitepdf-document` contains:

- compatibility `Document` and `Node` authoring types
- text and image node helpers for content modeling
- conversion into `graphitepdf_layout::Document`, `Page`, and `Node`
- compatibility with renderer sessions through `RendererDocumentSource`

---

## Installation

```bash
cargo add graphitepdf-document
```

---

## API Summary

| Category | Items |
| --- | --- |
| Document facade | `Document`, `PdfMetadata` |
| Nodes | `Node`, `NodeKind`, `TextNode`, `ImageNode` |
| Image compatibility | `ImageSource` |
| Pipeline integration | `Document::to_layout_document()` |

---

## Example

```rust
use graphitepdf_document::{Document, Node, NodeKind, Style, TextNode};

fn main() {
    let title = Node::new(NodeKind::Text(TextNode::new("Hello GraphitePDF")), Style::default());
    let page = Node::new(NodeKind::View { children: vec![title] }, Style::default());

    let document = Document::new().add_page(page);
    let layout_document = document.to_layout_document();

    assert_eq!(layout_document.pages().len(), 1);
}
```

---

## Design Principles

- preserve compatibility-friendly document authoring shapes
- keep lowering into the canonical crates explicit
- avoid duplicating layout or rendering policy inside the facade
- make it easy to bridge user-facing APIs into the crate-native pipeline

---

## Role In GraphitePDF

This crate sits above the canonical pipeline as an adapter layer. It is useful when callers want a simpler document facade, but the real work still flows through `layout`, `render`, `renderer`, and `kit`.

---

## License

MIT
