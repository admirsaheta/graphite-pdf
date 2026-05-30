# GraphitePDF Architecture

## High-Level Overview

GraphitePDF is now a crate-centric Rust workspace. The primary architecture is no longer rooted in `src/`; it lives in the split crates under `crates/`.

The canonical crate-native pipeline is:

1. shared services such as `font`, `image`, `stylesheet`, `svg`, `math`, and `textkit` provide inputs and domain-specific processing
2. `layout` resolves pages, node geometry, inherited style, text layout, and asset sizing into a safe layout tree
3. `render` lowers that layout tree into typed render commands and provides the production PDF backend
4. `kit` emits PDF page content and serializes final PDF bytes

Supporting that pipeline are shared crates for errors, primitives, utilities, assets, and scene conversion:

- `errors`, `primitives`, `utils`
- `font`, `image`, `stylesheet`
- `svg`, `math`

Compatibility crates such as `document`, `style`, and `renderer`, along with the root crate and `src/`, still matter, but they sit above that canonical path as facade layers rather than as the place where the canonical architecture lives.

## Workspace Shape

```text
graphitepdf/
├── crates/
│   ├── errors/       # Shared Result/Error types
│   ├── primitives/   # Geometry, units, colors, common low-level types
│   ├── utils/        # Generic helpers
│   ├── font/         # Font descriptors, sources, registration, loading, fallback support
│   ├── image/        # Image sources, decoded assets, async resolution, caching
│   ├── stylesheet/   # CSS-like normalization, shorthands, media/unit resolution
│   ├── svg/          # SVG parser -> typed SvgNode tree
│   ├── math/         # LaTeX -> SVG scene conversion
│   ├── textkit/      # Attributed text, bidi, hyphenation, text layout
│   ├── style/        # Compatibility style facade over layout/style-system types
│   ├── document/     # Compatibility document facade over layout/render types
│   ├── layout/       # Canonical layout pipeline and safe layout tree
│   ├── render/       # Render commands + production PDF backend over kit
│   ├── renderer/     # Thin compatibility facade over render
│   └── kit/          # Low-level PDF content generation and file writing
└── src/
    ├── document/     # Root-crate compatibility wrappers
    ├── layout/       # Root-crate compatibility wrappers
    ├── render/       # Root-crate compatibility wrappers
    ├── renderer/     # Root-crate compatibility wrappers
    ├── style/        # Root-crate compatibility wrappers
    ├── textkit/      # Root-crate compatibility wrappers
    └── lib.rs        # Public facade and re-exports
```

## Architectural Layers

### 1. Foundation

- `crates/errors` defines the shared workspace error surface.
- `crates/primitives` defines reusable low-level data such as `Pt`, `Bounds`, `Point`, `Size`, and `Color`.
- `crates/utils` holds general helpers used by higher-level crates.

These crates sit at the bottom of the graph and should stay free of document, layout, render, or PDF policy.

### 2. Shared asset and style services

- `crates/font` owns `FontDescriptor`, `FontSource`, `FontStore`, standard fonts, loading, fallback lookup, and hyphenation hooks.
- `crates/image` owns image sources, decoded image assets, async resolution, and cache management.
- `crates/stylesheet` owns stylesheet flattening, shorthand expansion, unit resolution, media-query handling, and normalization.

These crates are shared domain services. They are not PDF-specific and they are not root-crate-only helpers.

### 3. Shared scene conversion

- `crates/svg` parses XML/SVG input into a typed `SvgNode` tree.
- `crates/math` renders LaTeX into SVG-shaped scene data using `graphitepdf-svg`.

`math` is therefore not a separate rendering backend. It is a scene producer that feeds the same downstream pipeline as SVG.

### 4. Canonical pipeline inputs

#### `textkit`

`crates/textkit` is the dedicated text subsystem. It owns:

- attributed text and text runs
- bidi and script heuristics
- font substitution
- hyphenation-aware breaking
- line and fragment layout

This crate is now the text engine that `layout` depends on rather than text behavior being embedded ad hoc in higher layers.

#### `layout`

`crates/layout` is the canonical layout layer. It integrates style, text, SVG, math, and image sizing into:

- `SafeLayoutDocument`
- `SafeLayoutPage`
- `SafeLayoutNode`

It also exposes the ordered layout pipeline steps, including style resolution, inheritance, text layout, SVG resolution, pagination, origin assignment, and z-index ordering.

This is now the authoritative geometric model for downstream rendering.

### 5. Render orchestration and PDF backend

#### `render`

`crates/render` is the main render layer. It lowers layout output into a typed `RenderDocument` made of `RenderCommand`s such as text, image, SVG, fill, border, transform, and debug operations.

Crucially, `render` also contains the production PDF path:

- `RenderEngine` builds render commands from `layout`
- `RendererSession` drives end-to-end document rendering
- `PdfRenderBackend` turns render commands into PDF bytes
- that backend uses `graphitepdf-kit` for page-content helpers and PDF writing

This means the workspace now does have a real crate-native `layout -> render -> kit -> PDF` path. That was previously a planned end state; it is now implemented.

### 6. Low-level PDF emission

`crates/kit` is the low-level PDF engine. It owns:

- PDF object and file serialization
- PDF page and document writing
- text, vector, image, and SVG page-content helpers
- font registration helpers for PDF emission

`kit` should stay focused on PDF mechanics. It is not the place for stylesheet resolution, document authoring policy, or high-level layout decisions.

### 7. Compatibility and facade crates

#### `style`

`crates/style` is currently a compatibility style facade, not a canonical pipeline layer. It projects stylesheet data into a legacy `Style` type, but it also depends on `graphitepdf-layout` for shared types such as `EdgeInsets` and for conversion into `graphitepdf_layout::LayoutStyle`.

So `style` does not sit below `layout` in the dependency graph. It sits above or beside the canonical layout crate as an adapter for legacy API shapes.

#### `document`

`crates/document` is also a compatibility-facing facade. It provides compatibility-friendly `Document`, `Node`, `TextNode`, and `ImageNode` types, lowers them into `graphitepdf-layout` types, and also implements rendering entry points by depending on `graphitepdf-render` and `graphitepdf-renderer`.

This crate is therefore a front door and adapter layer over the canonical pipeline, not the core pipeline itself.

#### `renderer`

`crates/renderer` is currently a thin facade over `graphitepdf-render`. It exists as an API tier and compatibility surface, but the substantive rendering logic lives in `crates/render`.

## Dependency Direction

The dependency direction is best understood like this:

```text
root crate
  -> document, renderer, render, layout, style, textkit, kit, font, image, stylesheet, svg, math, primitives, errors, utils

document
  -> style, image, layout, render, renderer, textkit, primitives

renderer
  -> render, layout, kit, font, primitives, errors

style
  -> layout, stylesheet, font, primitives

render
  -> layout, kit, font, image, svg, textkit, primitives, utils, errors

layout
  -> textkit, stylesheet, image, math, svg, font, primitives, errors

kit
  -> font, image, math, svg, errors

textkit
  -> font, primitives, errors

image
  -> svg, errors

math
  -> svg, primitives, errors

svg
  -> primitives, errors

stylesheet
  -> errors

font
  -> errors
```

Important practical consequences:

- `layout` is the canonical bridge between shared inputs and rendering.
- `render` is the canonical bridge between layout output and `kit`.
- `renderer` is an API shell over `render`, not a separate rendering engine.
- `style` and `document` are compatibility adapters, not canonical layers that everything else must flow through.
- `src/` and the root crate sit above the crate graph as a facade layer.

## Role Of `src/`

The role of `src/` is intentionally reduced.

Today `src/` primarily does three things:

1. re-exports the split crates through the root `graphitepdf` crate
2. preserves compatibility modules such as `src/document`, `src/layout`, `src/render`, `src/renderer`, `src/style`, and `src/textkit`
3. offers a convenient top-level facade for users who want one dependency instead of many

What `src/` is not anymore:

- the canonical home of the document model
- the canonical layout engine
- the canonical render engine
- the canonical PDF backend

Those responsibilities now belong to the crates under `crates/`.

## Current Mental Model

The cleanest current mental model is:

- `font`, `image`, and `stylesheet` provide shared domain services
- `svg` and `math` provide typed scene data
- `textkit` provides the canonical text subsystem that feeds layout
- `layout` produces the canonical positioned scene graph
- `render` turns that graph into backend-friendly render commands and a working PDF backend
- `kit` performs low-level PDF emission
- `style`, `document`, `renderer`, the root crate, and `src/` provide convenience and compatibility surfaces

## Remaining Post-Port Gaps

Most of the post-port architectural gaps are now closed, including the last PDF-boundary image seam.

- Concrete `Image` assets render all the way through `render` into `kit`.
- Unresolved `ImageSource` values now follow one canonical source-to-asset resolution step inside the PDF backend before final encoding.
- The remaining work is now mostly iterative polish, coverage expansion, and narrowing behavior differences between the compatibility facade and the crate-native surfaces.

So the missing seam is no longer "is there a real PDF backend?" There is. It is also no longer "where is the single canonical source-to-asset image resolution step?" That path now exists in the render-to-kit backend.

## Summary

GraphitePDF now has the intended crate-centric layering:

- shared foundations in `errors`, `primitives`, and `utils`
- shared asset/style services in `font`, `image`, and `stylesheet`
- shared scene conversion in `svg` and `math`
- canonical text and layout inputs in `textkit`
- canonical layout in `layout`
- canonical rendering and PDF backend integration in `render`
- low-level PDF emission in `kit`
- facade layers in `style`, `document`, `renderer`, the root crate, and `src/`

That is the architecture the workspace actually implements today.
