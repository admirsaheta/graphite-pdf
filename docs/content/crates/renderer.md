# graphitepdf-renderer

Thin API facade over `graphitepdf-render`. Exists for backwards compatibility and as a stable public surface for users who depended on the pre-split API.

```toml
[dependencies]
graphitepdf-renderer = "0.2"
```

## What it exports

`graphitepdf-renderer` re-exports the full public surface of `graphitepdf-render`:

```rust
pub use graphitepdf_render::*;
```

This means `RenderEngine`, `RendererSession`, `RenderDocument`, `RenderCommand`, `render_to_bytes`, `render_to_file`, and all related types are available through this crate.

## When to use this vs `graphitepdf-render`

If you are starting a new project, depend on `graphitepdf-render` directly — it is the canonical crate.

Use `graphitepdf-renderer` if:
- You have existing code that imports from `graphitepdf_renderer`
- You want a stable re-export surface that shields you from internal crate splits

Both crates will track the same version and the same API.

## Custom backends

`graphitepdf-renderer` also exposes the `RenderBackend` trait and `NoopRenderBackend` for implementing alternative output backends:

```rust
use graphitepdf_renderer::{RenderBackend, RenderDocument, NoopRenderBackend};

struct MyBackend;

impl RenderBackend for MyBackend {
    type Output = String;
    fn render(&self, doc: &RenderDocument) -> graphitepdf_errors::Result<Self::Output> {
        Ok(format!("{} pages", doc.pages.len()))
    }
}
```

## Dependencies

`graphitepdf-render`, `graphitepdf-errors`, `graphitepdf-font`, `graphitepdf-kit`, `graphitepdf-layout`, `graphitepdf-primitives`
