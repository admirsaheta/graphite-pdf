# Introduction

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
