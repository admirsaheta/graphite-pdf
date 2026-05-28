<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Shared workspace error types for GraphitePDF crates and facade layers.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--errors-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-errors_%7C_result_%7C_shared-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-errors` defines the common `Result<T>` alias and `GraphitePdfError` enum used across the GraphitePDF workspace.

---

## Scope

`graphitepdf-errors` contains:

- `Result<T>` as the standard workspace result alias
- `GraphitePdfError` for I/O, layout, rendering, encoding, and validation failures
- a minimal dependency footprint centered on `thiserror`

---

## Installation

```bash
cargo add graphitepdf-errors
```

---

## API Summary

| Category | Items |
| --- | --- |
| Result alias | `Result<T>` |
| Shared error enum | `GraphitePdfError` |

---

## Example

```rust
use graphitepdf_errors::{GraphitePdfError, Result};

fn validate_page_width(width: f32) -> Result<()> {
    if width <= 0.0 {
        return Err(GraphitePdfError::InvalidPageSize(format!(
            "width must be positive, got {width}"
        )));
    }

    Ok(())
}

fn main() {
    assert!(validate_page_width(595.0).is_ok());
    assert!(validate_page_width(0.0).is_err());
}
```

---

## Design Principles

- keep the shared error surface small and predictable
- support workspace-wide consistency for fallible APIs
- avoid pulling unrelated policy into the lowest layer
- remain useful both for internal crates and external consumers

---

## Role In GraphitePDF

This crate lives at the bottom of the workspace and provides a shared error vocabulary for layout, rendering, asset loading, and PDF generation flows.

---

## License

MIT
