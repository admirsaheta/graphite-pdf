![GraphitePDF utilities logo](https://user-images.githubusercontent.com/5600341/27505816-c8bc37aa-587f-11e7-9a86-08a2d081a8b9.png)

# `graphitepdf-utils`

Lightweight utility functions for `graphitepdf` and related Rust workflows.

This crate provides a compact, dependency-free toolkit for collection
transforms, string handling, function composition, and dynamic object-like value
access. The APIs are designed for idiomatic Rust usage while staying practical
for document, layout, and rendering pipelines.

## Installation

```bash
cargo add graphitepdf-utils
```

## Rust Usage Notes

- `compose` and `async_compose` compose two functions at a time and can be nested
  for longer pipelines
- object-oriented helpers such as `get`, `map_values`, `pick`, `omit`, and
  `evolve` operate on the crate's `Value` and `Object` types
- `pick` and `omit` accept either a single key or multiple keys
- `parse_float` returns `Option<f32>` to model parsing failure explicitly
- `is_nil` works on `Option<T>` and treats `None` as nil

## Functions

### `adjust`

```rust
use graphitepdf_utils::adjust;

assert_eq!(adjust(1, |x| x * 2, &[1, 2, 3]), vec![1, 4, 3]);
assert_eq!(adjust(-1, |x| x + 10, &[1, 2, 3]), vec![1, 2, 13]);
```

### `async_compose`

```rust
use graphitepdf_utils::async_compose;
use std::future::ready;

let add_async = |x| ready(x + 1);
let double_async = |x| ready(x * 2);

let function = async_compose(double_async, add_async);
```

### `capitalize`

```rust
use graphitepdf_utils::capitalize;

assert_eq!(capitalize("hello world"), "Hello World");
```

### `cast_array`

```rust
use graphitepdf_utils::cast_array;

assert_eq!(cast_array("foo"), vec!["foo"]);
assert_eq!(cast_array::<&str>(vec!["foo"]), vec!["foo"]);
```

### `compose`

```rust
use graphitepdf_utils::compose;

let add_one = |x| x + 1;
let double = |x| x * 2;

let function = compose(double, add_one);
assert_eq!(function(5), 12);
```

### `drop_last`

```rust
use graphitepdf_utils::drop_last;

assert_eq!(drop_last(&[1, 2, 3][..]), vec![1, 2]);
assert_eq!(drop_last("hello"), "hell");
```

### `evolve`

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

### `get`

```rust
use graphitepdf_utils::{Object, Value, get};

let nested: Object = [("b".to_string(), Value::from(1_i32))].into_iter().collect();
let root = Value::from([("a".to_string(), Value::from(nested))].into_iter().collect::<Object>());

assert_eq!(get(&root, &["a", "b"], Value::from(0_i32)), Value::from(1_i32));
```

### `is_nil`

```rust
use graphitepdf_utils::is_nil;

assert!(is_nil(&None::<i32>));
assert!(!is_nil(&Some(0)));
```

### `last`

```rust
use graphitepdf_utils::last;

assert_eq!(last(&[1, 2, 3][..]), Some(3));
assert_eq!(last("abc"), Some('c'));
```

### `map_values`

```rust
use graphitepdf_utils::{Object, Value, map_values};

let object: Object = [
    ("a".to_string(), Value::from(1_i32)),
    ("b".to_string(), Value::from(2_i32)),
]
.into_iter()
.collect();

let mapped = map_values(&object, |value, _| match value {
    Value::Number(number) => Value::from(number * 2.0),
    other => other.clone(),
});
```

### `match_percent`

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

### `omit`

```rust
use graphitepdf_utils::{Object, Value, omit};

let object: Object = [
    ("a".to_string(), Value::from(1_i32)),
    ("b".to_string(), Value::from(2_i32)),
]
.into_iter()
.collect();

let omitted = omit("b", &object);
assert_eq!(omitted.contains_key("b"), false);
```

### `parse_float`

```rust
use graphitepdf_utils::parse_float;

assert_eq!(parse_float("3.14"), Some(3.14));
assert_eq!(parse_float("10px"), Some(10.0));
```

### `pick`

```rust
use graphitepdf_utils::{Object, Value, pick};

let object: Object = [
    ("a".to_string(), Value::from(1_i32)),
    ("b".to_string(), Value::from(2_i32)),
]
.into_iter()
.collect();

let selected = pick("a", &object);
assert_eq!(selected.contains_key("a"), true);
```

### `repeat`

```rust
use graphitepdf_utils::repeat;

assert_eq!(repeat("a", 3), vec!["a", "a", "a"]);
```

### `reverse`

```rust
use graphitepdf_utils::reverse;

assert_eq!(reverse(&[1, 2, 3][..]), vec![3, 2, 1]);
```

### `upper_first`

```rust
use graphitepdf_utils::upper_first;

assert_eq!(upper_first("hello"), "Hello");
```

### `without`

```rust
use graphitepdf_utils::without;

assert_eq!(without(&[2, 4], &[1, 2, 3, 4, 5]), vec![1, 3, 5]);
```

## License

MIT
