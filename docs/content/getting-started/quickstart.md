# Quick Start

Two paths to your first PDF: the **template DSL** (declarative, compile-time) or the **builder API** (programmatic, flexible). Both produce the same `Document` type and feed the same render pipeline.

## Option A — Template DSL

Add the dependency with the `template` feature:

```toml
[dependencies]
graphitepdf = { version = "0.2", features = ["template"] }
graphitepdf-render = "0.2"
```

Write your document:

```rust
use graphitepdf::{pdf, styles};
use graphitepdf_render::render_to_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = pdf! {
        <Document>
            <Page size="A4">
                <View style={styles! { background_color: "#F8FAFC" }}>
                    <Text style={styles! { font_size: 28.0, font_weight: bold, color: "#0F172A" }}>
                        "Hello from GraphitePDF"
                    </Text>
                    <Text style={styles! { font_size: 14.0, color: "#475569" }}>
                        "Generated with the pdf! template macro."
                    </Text>
                </View>
            </Page>
        </Document>
    };

    render_to_file(&document, "output.pdf")?;
    Ok(())
}
```

## Option B — Builder API

```toml
[dependencies]
graphitepdf-layout = "0.2"
graphitepdf-primitives = "0.2"
graphitepdf-render = "0.2"
```

```rust
use graphitepdf_layout::{Document, LayoutMetadata, LayoutStyle, Node, Page};
use graphitepdf_primitives::{Color, Pt, Size};
use graphitepdf_render::render_to_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = Document::new()
        .with_metadata(LayoutMetadata {
            title: Some("Hello PDF".into()),
            author: Some("ACME Corp".into()),
            ..Default::default()
        })
        .with_page(
            Page::new([
                Node::text("Hello from GraphitePDF")
                    .with_style(
                        LayoutStyle::new()
                            .with_font_size(Pt::new(28.0))
                            .with_color(Color::rgb(0x0F, 0x17, 0x2A)),
                    ),
            ])
            .with_size(Size::new(595.0, 842.0)),
        );

    render_to_file(&document, "output.pdf")?;
    Ok(())
}
```

## Run

```bash
cargo run
```

`output.pdf` appears in the working directory.

## Design system with `stylesheet!`

For multi-page documents, define named styles once and reference them everywhere:

```rust
use graphitepdf::{pdf, stylesheet, styles};

let ds = stylesheet! {
    .title   => { font_size: 24.0, font_weight: bold,     color: "#0F172A" },
    .body    => { font_size: 12.0, line_height: 18.0,     color: "#334155" },
    .caption => { font_size: 10.0,                        color: "#64748B" },
};

let doc = pdf! {
    <Document>
        <Page size="A4">
            <Text style={ds.title.clone()}>"Quarterly Report"</Text>
            <Text style={ds.body.clone()}>"Revenue grew 18% year-on-year."</Text>
            <Text style={ds.caption.clone()}>"Source: internal ledger, Q1 2026"</Text>
        </Page>
    </Document>
};
```

## Next steps

- Read the [Template DSL Overview](/templating/overview) and [Macro Reference](/templating/macros) for the full feature set
- Browse the **Crates** section for each layer of the pipeline
- See the `example/` directory in the repository for a four-page showcase generated with the full pipeline
- Check [docs.rs/graphitepdf](https://docs.rs/graphitepdf) for the complete type reference
