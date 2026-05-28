use crate::error::{GraphitePdfKitError, Result};
use std::io::Write;

#[cfg(feature = "tracing")]
use tracing::instrument;

/// PDF object types as specified in PDF 1.7 specification.
#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    /// Null object.
    Null,
    /// Boolean object.
    Boolean(bool),
    /// Integer object.
    Integer(i64),
    /// Real (floating-point) object.
    Real(f64),
    /// String object (binary data, typically 8-bit or Unicode).
    String(Vec<u8>),
    /// Name object.
    Name(String),
    /// Array object, containing other PDF objects.
    Array(Vec<Object>),
    /// Dictionary object, key-value pairs.
    Dict(Vec<(String, Object)>),
    /// Indirect reference object reference by object number.
    Ref(u64),
    /// Stream object, dictionary + data.
    Stream {
        dict: Vec<(String, Object)>,
        data: Vec<u8>,
    },
}

impl Object {
    /// Creates a new Name object.
    pub fn name(s: impl Into<String>) -> Self {
        Object::Name(s.into())
    }

    /// Creates a new Dictionary object from key-value pairs.
    pub fn dict(entries: impl IntoIterator<Item = (impl Into<String>, impl Into<Object>)>) -> Self {
        Object::Dict(
            entries
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }

    /// Creates a new Array object.
    pub fn array(items: impl IntoIterator<Item = impl Into<Object>>) -> Self {
        Object::Array(items.into_iter().map(|i| i.into()).collect())
    }

    /// Creates a new String object from bytes.
    pub fn string(s: impl AsRef<[u8]>) -> Self {
        Object::String(s.as_ref().to_vec())
    }

    /// Creates a new UTF-16BE encoded String object.
    pub fn string_utf16(s: &str) -> Self {
        let mut bytes = vec![0xFE, 0xFF]; // BOM
        for c in s.chars() {
            let code = c as u16;
            bytes.push((code >> 8) as u8);
            bytes.push((code & 0xFF) as u8);
        }
        Object::String(bytes)
    }

    /// Creates a new Integer object.
    pub fn integer(n: i64) -> Self {
        Object::Integer(n)
    }

    /// Creates a new Real object.
    pub fn real(n: f64) -> Self {
        Object::Real(n)
    }

    /// Inserts a key-value pair into a dictionary object.
    /// Returns an error if `self` is not a Dict.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<Object>) -> Result<()> {
        match self {
            Object::Dict(entries) => {
                let key_str = key.into();
                entries.retain(|(k, _)| k != &key_str);
                entries.push((key_str, value.into()));
                Ok(())
            }
            _ => Err(GraphitePdfKitError::InvalidObject(
                "Cannot insert into non-Dictionary object".to_string(),
            )),
        }
    }

    /// Gets a value from a dictionary object by key.
    /// Returns None if the object is not a Dict or key doesn't exist.
    pub fn get(&self, key: &str) -> Option<&Object> {
        match self {
            Object::Dict(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// Pushes an item to an array object.
    /// Returns an error if `self` is not an Array.
    pub fn push(&mut self, item: impl Into<Object>) -> Result<()> {
        match self {
            Object::Array(items) => {
                items.push(item.into());
                Ok(())
            }
            _ => Err(GraphitePdfKitError::InvalidObject(
                "Cannot push into non-Array object".to_string(),
            )),
        }
    }

    /// Writes the object to a writer as PDF syntax.
    #[cfg_attr(feature = "tracing", instrument(skip(writer)))]
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            Object::Null => writer.write_all(b"null")?,
            Object::Boolean(true) => writer.write_all(b"true")?,
            Object::Boolean(false) => writer.write_all(b"false")?,
            Object::Integer(n) => write!(writer, "{n}")?,
            Object::Real(n) => {
                if n.is_infinite() || n.is_nan() {
                    return Err(GraphitePdfKitError::InvalidObject(
                        "cannot serialize infinite or NaN real number".to_string(),
                    ));
                }
                write!(writer, "{n}")?;
            }
            Object::String(s) => {
                writer.write_all(b"(")?;
                for byte in s {
                    match byte {
                        b'(' | b')' | b'\\' => {
                            writer.write_all(b"\\")?;
                            writer.write_all(&[*byte])?;
                        }
                        0x08 => writer.write_all(b"\\b")?,
                        0x09 => writer.write_all(b"\\t")?,
                        0x0a => writer.write_all(b"\\n")?,
                        0x0c => writer.write_all(b"\\f")?,
                        0x0d => writer.write_all(b"\\r")?,
                        _ => writer.write_all(&[*byte])?,
                    }
                }
                writer.write_all(b")")?;
            }
            Object::Name(name) => {
                writer.write_all(b"/")?;
                for byte in name.bytes() {
                    match byte {
                        b'0'..=b'9'
                        | b'A'..=b'Z'
                        | b'a'..=b'z'
                        | b'_'
                        | b';'
                        | b':'
                        | b'@'
                        | b'&'
                        | b'$'
                        | b'#'
                        | b'%'
                        | b'+'
                        | b'-'
                        | b'.'
                        | b'!'
                        | b'~'
                        | b'*'
                        | b'/'
                        | b'?' => {
                            writer.write_all(&[byte])?;
                        }
                        _ => {
                            write!(writer, "#{:02x}", byte)?;
                        }
                    }
                }
            }
            Object::Array(items) => {
                writer.write_all(b"[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        writer.write_all(b" ")?;
                    }
                    item.write(writer)?;
                }
                writer.write_all(b"]")?;
            }
            Object::Dict(entries) => {
                writer.write_all(b"<<")?;
                for (i, (key, value)) in entries.iter().enumerate() {
                    if i > 0 {
                        writer.write_all(b"\n")?;
                    }
                    writer.write_all(b"/")?;
                    writer.write_all(key.as_bytes())?;
                    writer.write_all(b" ")?;
                    value.write(writer)?;
                }
                writer.write_all(b">>")?;
            }
            Object::Ref(n) => {
                write!(writer, "{n} 0 R")?;
            }
            Object::Stream { dict, data } => {
                let mut dict_with_length = dict.clone();
                dict_with_length.push(("Length".to_string(), Object::Integer(data.len() as i64)));
                Object::Dict(dict_with_length).write(writer)?;
                writer.write_all(b"\nstream\n")?;
                writer.write_all(data)?;
                writer.write_all(b"\nendstream")?;
            }
        }
        Ok(())
    }
}

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        Object::Boolean(value)
    }
}

impl From<i64> for Object {
    fn from(value: i64) -> Self {
        Object::Integer(value)
    }
}

impl From<i32> for Object {
    fn from(value: i32) -> Self {
        Object::Integer(value as i64)
    }
}

impl From<f64> for Object {
    fn from(value: f64) -> Self {
        Object::Real(value)
    }
}

impl From<f32> for Object {
    fn from(value: f32) -> Self {
        Object::Real(value as f64)
    }
}

impl From<&str> for Object {
    fn from(value: &str) -> Self {
        Object::string(value.as_bytes())
    }
}

impl From<String> for Object {
    fn from(value: String) -> Self {
        Object::string(value.into_bytes())
    }
}

impl<T: Into<Object>> From<Vec<T>> for Object {
    fn from(value: Vec<T>) -> Self {
        Object::Array(value.into_iter().map(|v| v.into()).collect())
    }
}

impl<'a, T: Into<Object> + Clone> From<&'a [T]> for Object {
    fn from(value: &'a [T]) -> Self {
        Object::Array(value.iter().map(|v| v.clone().into()).collect())
    }
}
