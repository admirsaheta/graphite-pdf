const INTRO: &str = r#"# Introduction

**GraphitePDF** is a Rust-native PDF generation engine built for layout, composition, and rendering pipelines. It exposes a clean, modular workspace of crates that can be used independently or together through the `graphitepdf` facade.

## What it is

GraphitePDF provides a full stack for producing PDF output from Rust:

- **Layout engine** — box model, flex-like flow, block/inline composition
- **Text pipeline** — font loading, shaping, glyph metrics, text layout (`textkit`)
- **Image handling** — raster embedding, format support
- **SVG support** — vector embedding in document flow
- **Style system** — CSS-influenced styling, cascade, stylesheet resolution
- **Math primitives** — coordinate types, affine transforms, geometry
- **Render pipeline** — page rendering and output assembly
- **PDF assembly** — low-level PDF object graph via `kit`

All crates share strict type safety, zero-copy where possible, and are designed with Rust 1.85+ edition 2024 semantics.

## Design philosophy

GraphitePDF is not a wrapper around a C library. It is a ground-up Rust implementation that prioritizes:

- **Correctness** — precise layout math, accurate font metrics
- **Composability** — crates can be used at any level of the stack
- **Performance** — zero allocations in hot paths, arena-friendly types
- **Ergonomics** — clear APIs, strong types, no stringly-typed configuration

## Crate ecosystem

| Crate | Role |
| --- | --- |
| `graphitepdf` | Facade — one import for the full stack |
| `graphitepdf-document` | Document model, pages, content tree |
| `graphitepdf-layout` | Box model layout engine |
| `graphitepdf-render` | Page render pipeline |
| `graphitepdf-renderer` | Output assembly and PDF serialization |
| `graphitepdf-style` | Style types and property system |
| `graphitepdf-stylesheet` | Cascade, specificity, style resolution |
| `graphitepdf-textkit` | Font loading, shaping, text layout |
| `graphitepdf-font` | Font types and metrics |
| `graphitepdf-image` | Image embedding |
| `graphitepdf-svg` | SVG embedding |
| `graphitepdf-math` | Coordinate types, transforms, geometry |
| `graphitepdf-primitives` | Shared primitive types |
| `graphitepdf-errors` | Error types across the ecosystem |
| `graphitepdf-kit` | Low-level PDF construction helpers |
| `graphitepdf-utils` | Shared utility functions |
"#;

const INSTALLATION: &str = r#"# Installation

GraphitePDF requires **Rust 1.85** or later and uses the 2024 edition.

## Adding the facade crate

Add `graphitepdf` to your `Cargo.toml` for one-import access to the full stack:

```toml
[dependencies]
graphitepdf = "0.1"
```

## Using individual crates

If you only need part of the stack, depend on individual crates directly:

```toml
[dependencies]
graphitepdf-layout    = "0.1"
graphitepdf-textkit   = "0.1"
graphitepdf-renderer  = "0.1"
```

## Workspace setup

For a workspace that uses GraphitePDF across multiple crates, define it once in `[workspace.dependencies]`:

```toml
[workspace.dependencies]
graphitepdf = "0.1"
```

Then in each member:

```toml
[dependencies]
graphitepdf = { workspace = true }
```

## Feature flags

The top-level crate exposes feature flags for optional components. Check the [Cargo.toml](https://github.com/admirsaheta/graphitepdf/blob/main/Cargo.toml) for the current list.

## Minimum Supported Rust Version

| MSRV | Rust edition |
| --- | --- |
| 1.85 | 2024 |

The MSRV is tested in CI and only bumped with a minor version bump.
"#;

const QUICKSTART: &str = r#"# Quick Start

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
"#;

pub fn get_static_content(section: &str, page: &str) -> Option<&'static str> {
    match (section, page) {
        ("getting-started", "introduction") => Some(INTRO),
        ("getting-started", "installation") => Some(INSTALLATION),
        ("getting-started", "quickstart") => Some(QUICKSTART),
        _ => None,
    }
}

pub fn github_raw_url(section: &str, page: &str) -> Option<String> {
    if section == "crates" {
        let crate_name = page;
        Some(format!(
            "https://raw.githubusercontent.com/admirsaheta/graphitepdf/main/crates/{}/README.md",
            crate_name
        ))
    } else {
        None
    }
}

pub fn github_edit_url(section: &str, page: &str) -> String {
    if section == "crates" {
        format!(
            "https://github.com/admirsaheta/graphitepdf/edit/main/crates/{}/README.md",
            page
        )
    } else {
        format!(
            "https://github.com/admirsaheta/graphitepdf/edit/main/docs/content/{}/{}.md",
            section, page
        )
    }
}

#[derive(PartialEq)]
pub struct NavSection {
    pub label: &'static str,
    pub items: &'static [NavItem],
}

#[derive(PartialEq)]
pub struct NavItem {
    pub label: &'static str,
    pub section: &'static str,
    pub page: &'static str,
}

pub const NAV: &[NavSection] = &[
    NavSection {
        label: "Getting Started",
        items: &[
            NavItem {
                label: "Introduction",
                section: "getting-started",
                page: "introduction",
            },
            NavItem {
                label: "Installation",
                section: "getting-started",
                page: "installation",
            },
            NavItem {
                label: "Quick Start",
                section: "getting-started",
                page: "quickstart",
            },
        ],
    },
    NavSection {
        label: "Crates",
        items: &[
            NavItem {
                label: "document",
                section: "crates",
                page: "document",
            },
            NavItem {
                label: "errors",
                section: "crates",
                page: "errors",
            },
            NavItem {
                label: "font",
                section: "crates",
                page: "font",
            },
            NavItem {
                label: "image",
                section: "crates",
                page: "image",
            },
            NavItem {
                label: "kit",
                section: "crates",
                page: "kit",
            },
            NavItem {
                label: "layout",
                section: "crates",
                page: "layout",
            },
            NavItem {
                label: "math",
                section: "crates",
                page: "math",
            },
            NavItem {
                label: "primitives",
                section: "crates",
                page: "primitives",
            },
            NavItem {
                label: "render",
                section: "crates",
                page: "render",
            },
            NavItem {
                label: "renderer",
                section: "crates",
                page: "renderer",
            },
            NavItem {
                label: "style",
                section: "crates",
                page: "style",
            },
            NavItem {
                label: "stylesheet",
                section: "crates",
                page: "stylesheet",
            },
            NavItem {
                label: "svg",
                section: "crates",
                page: "svg",
            },
            NavItem {
                label: "textkit",
                section: "crates",
                page: "textkit",
            },
            NavItem {
                label: "utils",
                section: "crates",
                page: "utils",
            },
        ],
    },
];
