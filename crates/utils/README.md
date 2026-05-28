<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Reusable utility helpers for GraphitePDF and related Rust data-processing workflows.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--utils-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-utils_%7C_values_%7C_parsing-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-utils` collects small, dependency-free helpers for collection transforms, composition, object-like values, and string parsing.

---

## Scope

`graphitepdf-utils` includes:

- collection helpers such as `adjust`, `cast_array`, `drop_last`, `last`, `repeat`, `reverse`, and `without`
- composition helpers such as `compose` and `async_compose`
- object and value helpers such as `get`, `pick`, `omit`, `map_values`, and `evolve`
- parsing and string helpers such as `parse_float`, `match_percent`, `capitalize`, and `upper_first`

---

## Installation

```bash
cargo add graphitepdf-utils
```

---

## API Summary

| Category | Items |
| --- | --- |
| Collection helpers | `adjust`, `cast_array`, `drop_last`, `last`, `repeat`, `reverse`, `without`, `OneOrMany` |
| Function composition | `compose`, `async_compose` |
| Object and value helpers | `Object`, `Value`, `Keys`, `Path`, `get`, `pick`, `omit`, `map_values`, `evolve`, `Transform`, `TransformMap` |
| Parsing and string helpers | `is_nil`, `capitalize`, `upper_first`, `parse_float`, `match_percent`, `PercentMatch` |

---

## Example

```rust
use graphitepdf_utils::{compose, match_percent, parse_float};

fn main() {
    let add_one = |x| x + 1;
    let double = |x| x * 2;
    let pipeline = compose(double, add_one);

    assert_eq!(pipeline(5), 12);
    assert_eq!(parse_float("10px"), Some(10.0));
    assert_eq!(match_percent("50%").map(|value| value.percent), Some(0.5));
}
```

---

## Design Principles

- keep the crate dependency-free
- prefer explicit helpers over framework-style abstractions
- keep utilities focused and composable
- support higher-level GraphitePDF crates without assuming a specific runtime

---

## Role In GraphitePDF

This crate provides shared helper logic used by higher-level GraphitePDF crates. Keeping these APIs isolated makes it easier to reuse common transforms and parsing logic without pulling in rendering-specific dependencies.

---

## License

MIT
