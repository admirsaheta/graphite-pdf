# graphitepdf-render

Render command generation and the production PDF backend. The bridge between layout output and PDF bytes.

```toml
[dependencies]
graphitepdf-render = "0.2"
```

## Render commands

`RenderCommand` is a typed, serializable intermediate representation. Each command maps directly to a PDF page content operation:

```rust
pub enum RenderCommand {
    FillRect(FillRectOp),
    StrokeBorder(BorderRenderOp),
    DrawText(TextRenderOp),
    DrawImage(ImageRenderOp),
    DrawSvg(SvgRenderOp),
    PushTransform(TransformRenderOp),
    PopTransform,
    Clip(BoxRenderOp),
    Form(FormRenderOp),
    Debug(DebugRenderOp),
}
```

## From layout to render commands

```rust
use graphitepdf_render::RenderEngine;

let safe_doc = /* graphitepdf-layout output */;
let render_engine = RenderEngine::default();
let render_doc = render_engine.render(&safe_doc)?;

// inspect commands for a page
for command in &render_doc.pages[0].commands {
    match command {
        RenderCommand::DrawText(op) => println!("text: {:?}", op.text),
        RenderCommand::DrawImage(op) => println!("image at {:?}", op.bounds),
        _ => {}
    }
}
```

## End-to-end: layout → PDF

```rust
use graphitepdf_render::{render_to_bytes, render_to_file};
use graphitepdf_layout::Document;

let doc: Document = /* ... */;

// to Vec<u8>
let bytes = render_to_bytes(&doc).await?;

// to a file
render_to_file(&doc, "output.pdf").await?;
```

## `RendererSession` — incremental rendering

`RendererSession` caches layout state across updates. Useful when you update a document and want to re-render without recomputing unchanged pages:

```rust
use graphitepdf_render::RendererSession;

let mut session = RendererSession::new(doc);

// initial render
let snapshot = session.render_snapshot()?;
assert_eq!(snapshot.revision(), 0);
let bytes = session.to_bytes()?;

// mutate document
session.update(|doc| doc.add_page(new_page));

// re-render — only affected pages are recomputed
let snapshot = session.render_snapshot()?;
assert_eq!(snapshot.revision(), 1);
session.save("output.pdf")?;
```

## PDF backend

`PdfRenderBackend` converts `RenderDocument` → PDF/1.7 bytes using `graphitepdf-kit` for page content serialization, font registration, image embedding, and document structure assembly.

## Dependencies

`graphitepdf-errors`, `graphitepdf-font`, `graphitepdf-image`, `graphitepdf-kit`, `graphitepdf-layout`, `graphitepdf-primitives`, `graphitepdf-svg`, `graphitepdf-textkit`, `graphitepdf-utils`, `tokio`
