use core::fmt;
use core::hash::{Hash, Hasher};
use std::borrow::Cow;
use std::fmt::Debug;

use crate::index::Index;

/// Represents any valid JSON value.
///
/// # Example
/// ```
/// use std::io;
/// use serde_json_borrow::Value;
/// fn main() -> io::Result<()> {
///     let data = r#"{"bool": true, "key": "123"}"#;
///     let value: Value = serde_json::from_str(&data)?;
///     assert_eq!(value.get("bool"), &Value::Bool(true));
///     assert_eq!(value.get("key"), &Value::Str("123".into()));
///     Ok(())
/// }
/// ```
#[derive(Clone, Eq, PartialEq)]
pub enum Value<'ctx> {
    /// Represents a JSON null value.
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// #
    /// let v = Value::Null;
    /// ```
    Null,

    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// #
    /// let v = Value::Bool(true);
    /// ```
    Bool(bool),

    /// Represents a JSON number, whether integer or floating point.
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// #
    /// let v = Value::Number(12.5.into());
    /// ```
    Number(Number),

    /// Represents a JSON string.
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// #
    /// let v = Value::Str("ref".into());
    /// ```
    Str(Cow<'ctx, str>),

    /// Represents a JSON array.
    Array(Vec<Value<'ctx>>),

    /// Represents a JSON object.
    ///
    /// By default the map is backed by a Vec. Allows very fast deserialization.
    /// Ideal when wanting to iterate over the values, in contrast to look up by key.
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// #
    /// let v = Value::Object([("key", Value::Str("value".into()))].into_iter().collect());
    /// ```
    Object(Vec<(&'ctx str, Value<'ctx>)>),
}

impl<'ctx> Value<'ctx> {
    /// Index into a `serde_json_borrow::Value` using the syntax `value.get(0)` or
    /// `value.get("k")`.
    ///
    /// Returns `Value::Null` if the type of `self` does not match the type of
    /// the index, for example if the index is a string and `self` is an array
    /// or a number. Also returns `Value::Null` if the given key does not exist
    /// in the map or the given index is not within the bounds of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// #
    /// let json_obj = r#"
    /// {
    ///     "x": {
    ///         "y": ["z", "zz"]
    ///     }
    /// }
    /// "#;
    ///
    /// let data: Value = serde_json::from_str(json_obj).unwrap();
    ///
    /// assert_eq!(data.get("x").get("y").get(0), &Value::Str("z".into()));
    /// assert_eq!(data.get("x").get("y").get(1), &Value::Str("zz".into()));
    /// assert_eq!(data.get("x").get("y").get(2), &Value::Null);
    ///
    /// assert_eq!(data.get("a"), &Value::Null);
    /// assert_eq!(data.get("a").get("b"), &Value::Null);
    /// ```
    #[inline]
    pub fn get<I: Index<'ctx>>(&'ctx self, index: I) -> &'ctx Value<'ctx> {
        static NULL: Value = Value::Null;
        index.index_into(self).unwrap_or(&NULL)
    }

    /// Returns true if `Value` is Value::Null.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns true if `Value` is Value::Array.
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    /// Returns true if `Value` is Value::Object.
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    /// Returns true if `Value` is Value::Bool.
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// Returns true if `Value` is Value::Number.
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// Returns true if `Value` is Value::Str.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::Str(_))
    }

    /// Returns true if the Value is an integer between i64::MIN and i64::MAX.
    /// For any Value on which is_i64 returns true, as_i64 is guaranteed to return the integer
    /// value.
    pub fn is_i64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i64(),
            _ => false,
        }
    }

    /// Returns true if the Value is an integer between zero and u64::MAX.
    /// For any Value on which is_u64 returns true, as_u64 is guaranteed to return the integer
    /// value.
    pub fn is_u64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u64(),
            _ => false,
        }
    }

    /// Returns true if the Value is a f64 number.
    pub fn is_f64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_f64(),
            _ => false,
        }
    }

    /// If the Value is an Array, returns an iterator over the elements in the array.
    pub fn iter_array(&self) -> Option<impl Iterator<Item = &Value<'_>>> {
        match self {
            Value::Array(arr) => Some(arr.iter()),
            _ => None,
        }
    }

    /// If the Value is an Object, returns an iterator over the elements in the object.
    pub fn iter_object(&self) -> Option<impl Iterator<Item = &(&str, Value<'_>)>> {
        match self {
            Value::Object(arr) => Some(arr.iter()),
            _ => None,
        }
    }

    /// If the Value is a Boolean, returns the associated bool. Returns None otherwise.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// If the Value is a String, returns the associated str. Returns None otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(text) => Some(text),
            _ => None,
        }
    }

    /// If the Value is an integer, represent it as i64 if possible. Returns None otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Number(n) => n.as_i64(),
            _ => None,
        }
    }

    /// If the Value is an integer, represent it as u64 if possible. Returns None otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Number(n) => n.as_u64(),
            _ => None,
        }
    }

    /// If the Value is a number, represent it as f64 if possible. Returns None otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => n.as_f64(),
            _ => None,
        }
    }
}

impl<'ctx> std::fmt::Debug for Value<'ctx> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => formatter.write_str("Null"),
            Value::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Value::Number(number) => match number.n {
                N::PosInt(n) => write!(formatter, "Number({:?})", n),
                N::NegInt(n) => write!(formatter, "Number({:?})", n),
                N::Float(n) => write!(formatter, "Number({:?})", n),
            },
            Value::Str(string) => write!(formatter, "Str({:?})", string),
            Value::Array(vec) => {
                formatter.write_str("Array ")?;
                Debug::fmt(vec, formatter)
            }
            Value::Object(map) => {
                formatter.write_str("Object ")?;
                Debug::fmt(map, formatter)
            }
        }
    }
}

/// Represents a JSON number, whether integer or floating point.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    n: N,
}

#[derive(Copy, Clone)]
enum N {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
}

impl Number {
    /// If the `Number` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            N::PosInt(v) => Some(v),
            _ => None,
        }
    }
    /// If the `Number` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            N::PosInt(n) => {
                if n <= i64::max_value() as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            }
            N::NegInt(v) => Some(v),
            _ => None,
        }
    }

    /// Represents the number as f64 if possible. Returns None otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            N::PosInt(n) => Some(n as f64),
            N::NegInt(n) => Some(n as f64),
            N::Float(n) => Some(n),
        }
    }

    /// Returns true if the `Number` is a f64.
    pub fn is_f64(&self) -> bool {
        matches!(self.n, N::Float(_))
    }

    /// Returns true if the `Number` is a u64.
    pub fn is_u64(&self) -> bool {
        matches!(self.n, N::PosInt(_))
    }

    /// Returns true if the `Number` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    pub fn is_i64(&self) -> bool {
        match self.n {
            N::PosInt(v) => v <= i64::max_value() as u64,
            N::NegInt(_) => true,
            N::Float(_) => false,
        }
    }
}

impl PartialEq for N {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (N::PosInt(a), N::PosInt(b)) => a == b,
            (N::NegInt(a), N::NegInt(b)) => a == b,
            (N::Float(a), N::Float(b)) => a == b,
            _ => false,
        }
    }
}

// Implementing Eq is fine since any float values are always finite.
impl Eq for N {}

impl Hash for N {
    fn hash<H: Hasher>(&self, h: &mut H) {
        match *self {
            N::PosInt(i) => i.hash(h),
            N::NegInt(i) => i.hash(h),
            N::Float(f) => {
                if f == 0.0f64 {
                    // There are 2 zero representations, +0 and -0, which
                    // compare equal but have different bits. We use the +0 hash
                    // for both so that hash(+0) == hash(-0).
                    0.0f64.to_bits().hash(h);
                } else {
                    f.to_bits().hash(h);
                }
            }
        }
    }
}

impl From<u64> for Number {
    fn from(val: u64) -> Self {
        Self { n: N::PosInt(val) }
    }
}

impl From<i64> for Number {
    fn from(val: i64) -> Self {
        Self { n: N::NegInt(val) }
    }
}

impl From<f64> for Number {
    fn from(val: f64) -> Self {
        Self { n: N::Float(val) }
    }
}

impl From<Number> for serde_json::value::Number {
    fn from(num: Number) -> Self {
        match num.n {
            N::PosInt(n) => n.into(),
            N::NegInt(n) => n.into(),
            N::Float(n) => serde_json::value::Number::from_f64(n).unwrap(),
        }
    }
}

impl<'ctx> From<Value<'ctx>> for serde_json::Value {
    fn from(val: Value) -> Self {
        match val {
            Value::Null => serde_json::Value::Null,
            Value::Bool(val) => serde_json::Value::Bool(val),
            Value::Number(val) => serde_json::Value::Number(val.into()),
            Value::Str(val) => serde_json::Value::String(val.to_string()),
            Value::Array(vals) => {
                serde_json::Value::Array(vals.into_iter().map(|val| val.into()).collect())
            }
            Value::Object(vals) => serde_json::Value::Object(
                vals.into_iter()
                    .map(|(key, val)| (key.to_owned(), val.into()))
                    .collect(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn number_test() -> io::Result<()> {
        let data = r#"{"val1": 123.5, "val2": 123, "val3": -123}"#;
        let value: Value = serde_json::from_str(data)?;
        assert!(value.get("val1").is_f64());
        assert!(!value.get("val1").is_u64());
        assert!(!value.get("val1").is_i64());

        assert!(!value.get("val2").is_f64());
        assert!(value.get("val2").is_u64());
        assert!(value.get("val2").is_i64());

        assert!(!value.get("val3").is_f64());
        assert!(!value.get("val3").is_u64());
        assert!(value.get("val3").is_i64());

        assert!(value.get("val1").as_f64().is_some());
        assert!(value.get("val2").as_f64().is_some());
        assert!(value.get("val3").as_f64().is_some());

        assert!(value.get("val1").as_u64().is_none());
        assert!(value.get("val2").as_u64().is_some());
        assert!(value.get("val3").as_u64().is_none());

        assert!(value.get("val1").as_i64().is_none());
        assert!(value.get("val2").as_i64().is_some());
        assert!(value.get("val3").as_i64().is_some());

        Ok(())
    }
}
