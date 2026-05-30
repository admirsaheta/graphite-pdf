# Quick Start

This guide walks through producing a simple PDF document with GraphitePDF.

## 1. Add the dependency

```toml
[dependencies]
graphitepdf = "0.1"
```

## 2. Create a document

```rust
use graphitepdf::document::Document;
use graphitepdf::layout::{Block, Size};
use graphitepdf::renderer::PdfRenderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::new();

    let page = doc.add_page(Size::A4);

    let block = Block::builder()
        .text("Hello from GraphitePDF")
        .font_size(24.0)
        .build();

    page.push(block);

    let bytes = PdfRenderer::new().render(&doc)?;
    std::fs::write("output.pdf", bytes)?;

    Ok(())
}
```

## 3. Run

```bash
cargo run
```

You will find `output.pdf` in the working directory.

## Next steps

- Browse the individual **Crates** in the sidebar to understand each layer
- Check the **API reference** on [docs.rs](https://docs.rs/graphitepdf) for full type documentation
- See the `examples/` directory in the repository for complete working examples
