# graphitepdf-layout

The canonical layout engine. Resolves style, text, assets, and geometry into a fully positioned, paginated scene graph.

```toml
[dependencies]
graphitepdf-layout = "0.2"
```

## Authoring types

### `Document`

```rust
use graphitepdf_layout::{Document, Page, Node, LayoutStyle, LayoutMetadata};
use graphitepdf_primitives::{Pt, Size};

let doc = Document::new()
    .with_metadata(LayoutMetadata {
        title: Some("My PDF".into()),
        author: Some("ACME".into()),
        ..Default::default()
    })
    .with_page(
        Page::new([
            Node::text("Hello, PDF")
                .with_style(LayoutStyle::new().with_font_size(Pt::new(20.0))),
        ])
        .with_size(Size::new(595.0, 842.0)),
    );
```

### `Node` — the content element

```rust
Node::text("Some text")
Node::box_node()            // empty container
Node::view(children)        // group of nodes
Node::image_source(source)  // image from any ImageSource
```

### `LayoutStyle` — node appearance

```rust
use graphitepdf_layout::LayoutStyle;
use graphitepdf_primitives::{Pt, Color};

let style = LayoutStyle::new()
    .with_width(Pt::new(400.0))
    .with_height(Pt::new(100.0))
    .with_padding(EdgeInsets::all(Pt::new(16.0)))
    .with_margin(EdgeInsets::horizontal(Pt::new(20.0)))
    .with_background_color(Color::rgb(0x1E, 0x1E, 0x1C))
    .with_color(Color::rgb(0xC4, 0xC4, 0xC0))
    .with_font_family("Helvetica")
    .with_font_size(Pt::new(14.0))
    .with_line_height(Pt::new(20.0))
    .with_z_index(10);
```

### `EdgeInsets` — per-edge spacing

```rust
use graphitepdf_layout::EdgeInsets;
use graphitepdf_primitives::Pt;

EdgeInsets::all(Pt::new(16.0))
EdgeInsets::horizontal(Pt::new(20.0))
EdgeInsets::vertical(Pt::new(8.0))
EdgeInsets::new(top, right, bottom, left)
```

## Layout pipeline

The pipeline runs in a fixed order of steps exposed as `LayoutPipelineStep`:

| Step | Description |
| --- | --- |
| `ResolveStyle` | cascade, inheritance, shorthand expansion |
| `ResolveAssets` | image sizing, SVG dimensions |
| `LayoutText` | text engine, line breaking, fragment positioning |
| `ComputeBoxModel` | margin, padding, border, width, height |
| `Paginate` | distribute nodes across pages |
| `AssignOrigins` | absolute (x, y) for every node |
| `OrderByZIndex` | sort stacking layers |

The output, `SafeLayoutDocument`, contains `SafeLayoutPage` and `SafeLayoutNode` — fully resolved, coordinate-stamped, and pagination-aware.

## Using the pipeline directly

```rust
use graphitepdf_layout::{LayoutEngine, LayoutMetadata};

let engine = LayoutEngine::default();
let safe_doc = engine.layout(doc)?;
```

## Dependencies

`graphitepdf-errors`, `graphitepdf-font`, `graphitepdf-image`, `graphitepdf-math`, `graphitepdf-primitives`, `graphitepdf-stylesheet`, `graphitepdf-svg`, `graphitepdf-textkit`
