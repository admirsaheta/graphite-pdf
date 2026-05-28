use std::collections::BTreeMap;

pub type Object = BTreeMap<String, Value>;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(Object),
}

impl Value {
    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Self::Object(object) => Some(object),
            _ => None,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Number(f64::from(value))
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Number(value as f64)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Number(f64::from(value))
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self::Number(value as f64)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Self::Array(value)
    }
}

impl From<Object> for Value {
    fn from(value: Object) -> Self {
        Self::Object(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Path<'a> {
    Key(&'a str),
    Keys(&'a [&'a str]),
}

impl<'a> Path<'a> {
    pub fn for_each(self, mut f: impl FnMut(&'a str)) {
        match self {
            Self::Key(key) => f(key),
            Self::Keys(keys) => {
                for key in keys {
                    f(key);
                }
            }
        }
    }
}

impl<'a> From<&'a str> for Path<'a> {
    fn from(value: &'a str) -> Self {
        Self::Key(value)
    }
}

impl<'a, const N: usize> From<&'a [&'a str; N]> for Path<'a> {
    fn from(value: &'a [&'a str; N]) -> Self {
        Self::Keys(value.as_slice())
    }
}

impl<'a> From<&'a [&'a str]> for Path<'a> {
    fn from(value: &'a [&'a str]) -> Self {
        Self::Keys(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keys<'a> {
    Key(&'a str),
    Many(&'a [&'a str]),
}

impl<'a> Keys<'a> {
    pub fn for_each(self, mut f: impl FnMut(&'a str)) {
        match self {
            Self::Key(key) => f(key),
            Self::Many(keys) => {
                for key in keys {
                    f(key);
                }
            }
        }
    }
}

impl<'a> From<&'a str> for Keys<'a> {
    fn from(value: &'a str) -> Self {
        Self::Key(value)
    }
}

impl<'a, const N: usize> From<&'a [&'a str; N]> for Keys<'a> {
    fn from(value: &'a [&'a str; N]) -> Self {
        Self::Many(value.as_slice())
    }
}

impl<'a> From<&'a [&'a str]> for Keys<'a> {
    fn from(value: &'a [&'a str]) -> Self {
        Self::Many(value)
    }
}
