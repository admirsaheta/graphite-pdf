<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Reusable utility helpers for GraphitePDF and related Rust data-processing workflows.</strong>
</p>

<p align="center">
  Small, dependency-free APIs for composition, collection transforms, dynamic values, and parsing.
</p>

# graphitepdf-utils

This crate collects small, dependency-free helpers for array manipulation, function
composition, object-like value access, and string parsing. It is intended for internal
reuse across GraphitePDF crates, but it is also usable as a standalone utility crate.

## Scope

`graphitepdf-utils` includes:

- collection helpers such as `adjust`, `cast_array`, `drop_last`, `last`, `repeat`, `reverse`, and `without`
- function composition helpers such as `compose` and `async_compose`
- object and value helpers such as `get`, `pick`, `omit`, `map_values`, and `evolve`
- parsing and string helpers such as `parse_float`, `match_percent`, `capitalize`, and `upper_first`

## Installation

```bash
cargo add graphitepdf-utils
```

## API Summary

| Category | APIs |
| --- | --- |
| Collection helpers | `adjust`, `cast_array`, `drop_last`, `last`, `repeat`, `reverse`, `without`, `OneOrMany` |
| Function composition | `compose`, `async_compose` |
| Object and value helpers | `Object`, `Value`, `Keys`, `Path`, `get`, `pick`, `omit`, `map_values`, `evolve`, `Transform`, `TransformMap` |
| Parsing and string helpers | `is_nil`, `capitalize`, `upper_first`, `parse_float`, `match_percent`, `PercentMatch` |

## Design Principles

- keep the crate dependency-free
- prefer explicit behavior over framework-style abstractions
- keep helpers focused and composable
- support document, layout, and rendering pipelines without assuming a specific runtime

## Examples

### Compose functions

```rust
use graphitepdf_utils::compose;

let add_one = |x| x + 1;
let double = |x| x * 2;

let pipeline = compose(double, add_one);
assert_eq!(pipeline(5), 12);
```

### Transform object-like values

```rust
use graphitepdf_utils::{Object, TransformMap, Value, evolve};

let object: Object = [
    ("count".to_string(), Value::from(5_i32)),
    ("name".to_string(), Value::from("item")),
]
.into_iter()
.collect();

let transforms: TransformMap = [
    (
        "count".to_string(),
        Box::new(|value: &Value| match value {
            Value::Number(number) => Value::from(number + 1.0),
            other => other.clone(),
        }) as _,
    ),
]
.into_iter()
.collect();

let evolved = evolve(&object, &transforms);
assert_eq!(evolved.get("count"), Some(&Value::from(6.0_f64)));
```

### Parse numeric values

```rust
use graphitepdf_utils::parse_float;

assert_eq!(parse_float("3.14"), Some(3.14));
assert_eq!(parse_float("10px"), Some(10.0));
```

### Match percentages

```rust
use graphitepdf_utils::{PercentMatch, match_percent};

assert_eq!(
    match_percent("50%"),
    Some(PercentMatch {
        value: 50.0,
        percent: 0.5,
    })
);
```

## Role In GraphitePDF

This crate provides shared helper logic used by higher-level GraphitePDF crates. Keeping
these APIs isolated makes it easier to reuse common transforms and parsing logic without
pulling in rendering-specific dependencies.

## License

MIT
