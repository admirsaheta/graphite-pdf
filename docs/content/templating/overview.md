# Template DSL — Overview

GraphitePDF ships a JSX-like proc-macro layer on top of the builder API. It compiles away entirely at build time — the output is identical to hand-written builder calls.

Enable it by adding the `template` feature:

```toml
[dependencies]
graphitepdf = { version = "0.2", features = ["template"] }
graphitepdf-template-macros = "0.2"
```

## The three macros

| Macro | Purpose |
| --- | --- |
| `pdf!` | Assemble a `Document` from declarative markup |
| `styles!` | Produce a `LayoutStyle` from named property literals |
| `stylesheet!` | Produce a typed struct of named `LayoutStyle` values |

All three are proc macros that run at compile time. No reflection, no runtime parsing, no stringly-typed configuration survives to the binary.

## How `pdf!` works

The macro accepts a tree of elements and compiles each one to a builder call:

```rust
use graphitepdf::{pdf, styles};

let document = pdf! {
    <Document>
        <Page size="A4">
            <View style={styles! { background_color: "#F8FAFC" }}>
                <Text style={styles! { font_size: 24.0, color: "#0F172A" }}>
                    "Hello, GraphitePDF"
                </Text>
            </View>
        </Page>
    </Document>
};
```

This expands to roughly:

```rust
let document = Document::new().with_page(
    Page::new([
        Node::view([
            Node::text(TextBlock::from(TextSpan::new("Hello, GraphitePDF").unwrap()))
                .with_style(LayoutStyle::new()
                    .with_font_size(Pt::new(24.0))
                    .with_color(Color::rgb(0x0F, 0x17, 0x2A))),
        ])
        .with_style(LayoutStyle::new()
            .with_background_color(Color::rgb(0xF8, 0xFA, 0xFC))),
    ])
    .with_size(into_pdf_size(PageSize::A4)),
);
```

No allocation overhead. No virtual dispatch. The compiler sees builder calls.

## Mixing markup and Rust

Any `{ expression }` child inside `pdf!` is spliced in directly. The expression must produce something that implements the relevant conversion trait (`IntoLayoutNodeChildren`, `IntoLayoutPageChildren`, or `Into<Document>`). `Option<_>`, `Vec<_>`, arrays, and single values all work:

```rust
let extra_panels: Vec<Node> = build_panels();

let doc = pdf! {
    <Document>
        <Page size="A4">
            {extra_panels}
            <Text>"Static footer"</Text>
        </Page>
    </Document>
};
```

## When to use the template DSL vs the builder API

| Scenario | Recommendation |
| --- | --- |
| Declarative document structure, mostly static markup | `pdf!` + `styles!` |
| Programmatic node generation, loops, helpers | Builder API (`Node::view`, `Node::text`, …) |
| Reusable design tokens shared across pages | `stylesheet!` |
| Low-level PDF construction, kit API | `graphitepdf-kit` directly |

The two approaches compose freely — `pdf!` nodes are the same `Node` / `Page` types as builder-constructed ones. A common pattern is to define a design system with `stylesheet!` and the page structure with `pdf!`, while complex content blocks are helper functions that return `Vec<Node>`.

## Relationship to the layout engine

The template macros produce a `Document` that is fed directly to `LayoutEngine::layout_document`. They do not bypass any part of the pipeline — styles, pagination, SVG resolution, math rendering, and text layout all apply identically regardless of whether nodes were created by a macro or the builder API.

```
pdf! / styles! / stylesheet!
       │
       ▼ compile-time expansion
  Document (LayoutMetadata + Vec<Page>)
       │
       ▼
  LayoutEngine::layout_document
       │
       ▼
  SafeLayoutDocument
       │
       ▼
  render_to_file / render_to_bytes
```
