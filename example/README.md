# GraphitePDF Template Example

This project generates a four-page PDF using the GraphitePDF templating language and render engine.

## What It Uses

- `graphitepdf` for the `pdf!`, `styles!`, and `stylesheet!` macros
- `graphitepdf-layout` for direct `Node` construction inside template expressions
- `graphitepdf-render` for `render_to_file`
- `graphitepdf-image` for a data-uri image source
- `graphitepdf-svg` for parsed SVG nodes
- `graphitepdf-math` for math layout nodes
- `graphitepdf-textkit` and `graphitepdf-font` for direct text block construction
- `graphitepdf-primitives` for `Color` and `Pt`
- `graphitepdf-stylesheet` for raw stylesheet interop
- `graphitepdf-kit` for typed page sizes

## Run

```bash
cargo run -p graphitepdf-template-example
```

The generated file is written to `example/output/template-showcase.pdf`.
