# graphitepdf-utils

Lightweight utility functions shared across the GraphitePDF workspace.

```toml
[dependencies]
graphitepdf-utils = "0.2"
```

## Modules

### `array` — slice and array helpers

```rust
use graphitepdf_utils::{last, drop_last, reverse, without, OneOrMany};

let tail = drop_last(&[1, 2, 3]);  // &[1, 2]
let rev  = reverse(&[1, 2, 3]);    // vec![3, 2, 1]

// OneOrMany<T> — ergonomic enum for single-item or multi-item APIs
let single: OneOrMany<i32> = OneOrMany::One(42);
let multi:  OneOrMany<i32> = OneOrMany::Many(vec![1, 2, 3]);
```

### `compose` — function composition

```rust
use graphitepdf_utils::compose;

let double_then_add_one = compose(|x: i32| x + 1, |x: i32| x * 2);
assert_eq!(double_then_add_one(3), 7);
```

### `string` — string helpers

```rust
use graphitepdf_utils::{capitalize, upper_first, parse_float};

let s = capitalize("hello world");  // "Hello World"
let f = parse_float("3.14");        // Ok(3.14_f32)
```

### `value` — dynamic value types

`Value`, `Object`, `Keys`, and `Path` provide a lightweight dynamically-typed value system used internally for style property maps.

## Dependencies

No internal workspace dependencies.
