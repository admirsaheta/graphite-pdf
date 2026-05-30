# graphitepdf-kit

Low-level PDF generation: document building, page content, text, vector graphics, images, font registration, and file writing.

```toml
[dependencies]
graphitepdf-kit = "0.2"
```

## Features

| Feature | Default | Description |
| --- | --- | --- |
| `std-fonts` | ✓ | Standard PDF font support |
| `vector` | ✓ | Canvas and path drawing |
| `tables` | ✓ | Table helpers |
| `images` | ✓ | PNG and JPEG embedding (enables `jpeg-decoder`, `png`) |
| `parallel` | ✗ | Parallel page rendering via `rayon` |
| `fonts-engine` | ✗ | TTF parsing via `ttf-parser` |
| `full` | ✗ | All features |

## Building a document

```rust
use graphitepdf_kit::{DocumentBuilder, PageSize, Metadata};

let metadata = Metadata::new()
    .title("Annual Report")
    .author("ACME Corp")
    .subject("Finance");

let (builder, font) = DocumentBuilder::new().add_font(
    graphitepdf_font::StandardFont::Helvetica
);

let text = TextBuilder::new()
    .font(&font, 12.0)
    .position(72.0, 720.0)
    .text("Hello from graphitepdf-kit")
    .finish();

let mut buf = Vec::new();
builder
    .with_metadata(metadata)
    .with_page(PageSize::A4, text)
    .write(&mut buf)?;
```

## Page sizes

```rust
pub enum PageSize {
    A0, A1, A2, A3, A4, A5, A6,
    Letter, Legal, Tabloid,
    Custom(f32, f32),  // width, height in points
}
```

## Vector graphics

```rust
use graphitepdf_kit::{Canvas, Color};

let canvas = Canvas::new()
    .set_fill_color(Color::rgb(0xD4, 0x58, 0x1A))
    .rect(72.0, 700.0, 200.0, 40.0)
    .fill()
    .set_stroke_color(Color::rgb(0, 0, 0))
    .set_line_width(1.5)
    .rect(72.0, 640.0, 200.0, 40.0)
    .stroke()
    .finish();
```

## Font registration

```rust
// Standard fonts — no embedding required
let (doc, font_name) = DocumentBuilder::new()
    .add_font(graphitepdf_font::StandardFont::HelveticaBold);
// font_name is e.g. "F2"

// Custom fonts via FontRegistry
let mut registry = FontRegistry::with_default_font();
let name = registry.register(graphitepdf_font::StandardFont::CourierBold);
```

## Role in the pipeline

`kit` is the lowest layer in GraphitePDF. `graphitepdf-render`'s `PdfRenderBackend` uses `kit` for all PDF serialization. If you want direct PDF generation without the layout engine, `kit` gives you that.

## Dependencies

`graphitepdf-errors`, `graphitepdf-font`, `graphitepdf-image`, `graphitepdf-math`, `graphitepdf-svg`, `flate2`
