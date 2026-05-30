# Architecture Overview

GraphitePDF is a crate-centric Rust workspace. There is no single monolithic engine — the stack is split into focused crates that compose cleanly.

## The pipeline

```text
Your code
  │
  ▼
Document / Node tree
  │  graphitepdf-document (compat facade)
  │  graphitepdf-layout   (authoring types)
  ▼
Layout computation
  │  graphitepdf-textkit  → line breaking, bidi, hyphenation
  │  graphitepdf-stylesheet → cascade, shorthand expansion
  │  graphitepdf-font     → metrics, fallback, registration
  │  graphitepdf-image    → decode, cache, size
  │  graphitepdf-svg      → parse → SvgNode tree
  │  graphitepdf-math     → LaTeX → SvgNode scene
  ▼
SafeLayoutDocument / SafeLayoutPage / SafeLayoutNode
  │  graphitepdf-layout
  ▼
Render commands
  │  graphitepdf-render   (RenderEngine)
  ▼
RenderDocument / RenderPage / Vec<RenderCommand>
  │  graphitepdf-render   (PdfRenderBackend)
  ▼
PDF page content (via graphitepdf-kit)
  │  graphitepdf-kit
  ▼
PDF bytes  →  write to file / buffer / stream
```

## Layers

### 1. Foundation

`errors`, `primitives`, `utils` — no document, layout, render, or PDF policy. Everything else builds on these.

### 2. Shared services

| Crate | Responsibility |
| --- | --- |
| `font` | FontDescriptor, FontSource, FontStore, standard fonts, fallback |
| `image` | sources, decoded assets, async resolution, LRU cache |
| `stylesheet` | cascade, shorthand expansion, unit resolution |

### 3. Scene conversion

| Crate | Responsibility |
| --- | --- |
| `svg` | XML/SVG → typed `SvgNode` tree |
| `math` | LaTeX → SVG-shaped scene data via mathjax |

`math` is not a separate renderer — it produces the same `SvgNode` scenes that feed the standard SVG path.

### 4. Text engine

`textkit` is the dedicated text subsystem. It owns attributed text, bidi and script heuristics, font substitution, hyphenation-aware line breaking, and fragment layout. Layout depends on it, not the other way around.

### 5. Layout

`layout` is the canonical geometric model. It integrates style, text, SVG, math, and image sizing into `SafeLayoutDocument` / `SafeLayoutPage` / `SafeLayoutNode`. These types are the authoritative positioned scene graph consumed by rendering.

### 6. Render and PDF emission

`render` lowers layout output into typed `RenderCommand`s — text, image, SVG, fill, border, transform, debug — and also contains the concrete `PdfRenderBackend` that emits PDF page content via `kit`.

`kit` is the low-level PDF engine: object graph, page writing, text, vector, image, and font helpers. It is not the place for layout or document authoring decisions.

### 7. Compatibility surfaces

`style`, `document`, and `renderer` are adapter layers over the canonical pipeline. They preserve a legacy-friendly API shape and serve as the front door for users who prefer a higher-level API over the crate-native surface.

The root `graphitepdf` crate and `src/` sit above everything as a facade — one dependency that re-exports the full stack.

## Dependency directions

```text
root → document → renderer → render → layout → textkit → font
                                              → stylesheet
                                              → image → svg
                                              → math  → svg
                                              → svg → primitives, errors
                    kit → font, image, math, svg, errors
         style → layout, stylesheet, font, primitives
         font, image, stylesheet, svg → errors
         primitives, utils, errors → (nothing internal)
```
