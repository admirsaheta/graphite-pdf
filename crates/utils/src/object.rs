use std::collections::{BTreeMap, BTreeSet};

use crate::value::{Keys, Object, Path, Value};

pub type Transform = Box<dyn Fn(&Value) -> Value + Send + Sync + 'static>;
pub type TransformMap = BTreeMap<String, Transform>;

pub fn get<'a>(value: &Value, path: impl Into<Path<'a>>, fallback: Value) -> Value {
    let mut current = value;
    let mut missing = false;

    path.into().for_each(|segment| {
        if missing {
            return;
        }

        current = match current {
            Value::Object(object) => match object.get(segment) {
                Some(value) => value,
                None => {
                    missing = true;
                    current
                }
            },
            _ => {
                missing = true;
                current
            }
        };
    });

    if missing {
        fallback
    } else {
        current.clone()
    }
}

pub fn map_values<F>(object: &Object, mut mapper: F) -> Object
where
    F: FnMut(&Value, &str) -> Value,
{
    object
        .iter()
        .map(|(key, value)| (key.clone(), mapper(value, key)))
        .collect()
}

pub fn pick<'a>(keys: impl Into<Keys<'a>>, object: &Object) -> Object {
    let mut result = Object::new();

    keys.into().for_each(|key| {
        if let Some(value) = object.get(key) {
            result.insert(key.to_string(), value.clone());
        }
    });

    result
}

pub fn omit<'a>(keys: impl Into<Keys<'a>>, object: &Object) -> Object {
    let mut excluded: BTreeSet<&str> = BTreeSet::new();
    keys.into().for_each(|key| {
        excluded.insert(key);
    });

    object
        .iter()
        .filter(|(key, _)| !excluded.contains(key.as_str()))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

pub fn evolve(object: &Object, transforms: &TransformMap) -> Object {
    object
        .iter()
        .map(|(key, value)| match transforms.get(key) {
            Some(transform) => (key.clone(), transform(value)),
            None => (key.clone(), value.clone()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn object(entries: [(&str, Value); 2]) -> Object {
        entries
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect()
    }

    #[test]
    fn gets_nested_value_from_path() {
        let nested = Value::from(object([
            ("b", Value::from(1_i32)),
            ("c", Value::from("ignored")),
        ]));
        let value = Value::from(
            [("a".to_string(), nested)]
                .into_iter()
                .collect::<Object>(),
        );

        assert_eq!(get(&value, &["a", "b"], Value::Null), Value::from(1_i32));
        assert_eq!(get(&value, &["a", "missing"], Value::from(0_i32)), Value::from(0_i32));
    }

    #[test]
    fn maps_and_filters_object_values() {
        let object = object([("a", Value::from(1_i32)), ("b", Value::from(2_i32))]);

        let doubled = map_values(&object, |value, _| match value {
            Value::Number(number) => Value::from(number * 2.0),
            other => other.clone(),
        });

        assert_eq!(pick("a", &doubled).len(), 1);
        assert_eq!(pick(&["a"], &doubled).len(), 1);
        assert_eq!(omit("b", &doubled).len(), 1);
        assert_eq!(omit(&["b"], &doubled).len(), 1);
    }

    #[test]
    fn evolves_values_with_matching_transformers() {
        let object = object([("count", Value::from(5_i32)), ("name", Value::from("item"))]);
        let transforms: TransformMap = [
            (
                "count".to_string(),
                Box::new(|value: &Value| match value {
                    Value::Number(number) => Value::from(number + 1.0),
                    other => other.clone(),
                }) as Transform,
            ),
            (
                "name".to_string(),
                Box::new(|value: &Value| match value {
                    Value::String(text) => Value::from(text.to_uppercase()),
                    other => other.clone(),
                }) as Transform,
            ),
        ]
        .into_iter()
        .collect();

        let evolved = evolve(&object, &transforms);

        assert_eq!(evolved.get("count"), Some(&Value::from(6.0_f64)));
        assert_eq!(evolved.get("name"), Some(&Value::from("ITEM")));
    }
}
