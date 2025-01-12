use std::borrow::Cow;

use base64::Engine;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Base64Bytes(pub Vec<u8>);

impl Serialize for Base64Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = base64::prelude::BASE64_STANDARD.encode(&self.0);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Base64Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
        base64::prelude::BASE64_STANDARD
            .decode(&*s)
            .map_err(serde::de::Error::custom)
            .map(Base64Bytes)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Integer(pub i64);

impl Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}
impl<'a> Deserialize<'a> for Integer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum NumberOrInt {
            Int(i64),
            Float(f64),
        }

        let num = NumberOrInt::deserialize(deserializer)?;
        let value = match num {
            NumberOrInt::Int(i) => i,
            NumberOrInt::Float(f) => {
                if f.is_infinite() || f.is_nan() {
                    return Err(serde::de::Error::custom(format!(
                        "float value {} is not a finite number",
                        f
                    )));
                }
                if f > i64::MAX as f64 || f < i64::MIN as f64 {
                    return Err(serde::de::Error::custom(format!(
                        "float value {} is outside of i64 range",
                        f
                    )));
                }
                f as i64
            }
        };
        Ok(Integer(value))
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct Empty(#[allow(unused)] Map<String, Value>);

pub struct Tag<T>(T);

pub trait TagData: Default {
    const TAG: &'static str;
}

impl<T: TagData> Serialize for Tag<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(T::TAG)
    }
}

impl<'de, T: TagData> Deserialize<'de> for Tag<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        if s != T::TAG {
            return Err(serde::de::Error::custom(format!("expected tag {}", T::TAG)));
        }
        Ok(Tag(T::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_base64_bytes() {
        let bytes = Base64Bytes(vec![1, 2, 3, 4, 5]);
        let json = json!(bytes);
        assert_eq!(json, json!("AQIDBAU="));

        let bytes: Base64Bytes = serde_json::from_value(json).unwrap();
        assert_eq!(bytes.0, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_integer_serialize() {
        let integer = Integer(42);
        let json = json!(integer);
        assert_eq!(json, json!(42));
    }

    #[test]
    fn test_integer_deserialize_int() {
        let json = json!(42);
        let integer: Integer = serde_json::from_value(json).unwrap();
        assert_eq!(integer.0, 42);
    }
    #[test]
    fn test_integer_deserialize_float() {
        let json = json!(42.0);
        let integer: Integer = serde_json::from_value(json).unwrap();
        assert_eq!(integer.0, 42);
    }
}
