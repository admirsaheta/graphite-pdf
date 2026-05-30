# graphitepdf-errors

Shared error and result types for the entire GraphitePDF workspace.

## When to use

Add this crate when you need to match on or construct `GraphitePdfError` values, or when you want the `Result<T>` alias without pulling in a heavier crate.

```toml
[dependencies]
graphitepdf-errors = "0.2"
```

## Key types

```rust
pub type Result<T> = std::result::Result<T, GraphitePdfError>;

pub enum GraphitePdfError {
    Io(io::Error),
    InvalidObject(String),
    FontError(String),
    ImageError(String),
    InvalidPageSize(String),
    EncodingError(String),
    CompressionError(String),
    InvalidArgument(String),
    InvalidDocument(String),
    Layout(String),
    Render(String),
    UnsupportedFeature(&'static str),
}
```

## Usage

```rust
use graphitepdf_errors::{GraphitePdfError, Result};

fn load(path: &str) -> Result<Vec<u8>> {
    std::fs::read(path).map_err(GraphitePdfError::Io)
}
```

All variants implement `std::error::Error` via `thiserror`.

## Dependencies

No internal workspace dependencies. Safe to use as a leaf crate.
