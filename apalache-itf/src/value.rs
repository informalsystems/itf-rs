use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Int(i64),
    BigInt(BigInt),
    Boolean(bool),
    String(String),
    Map(Map),
    Set(Set),
    List(Vec<Value>),
    Tuple(Tuple),
    Record(HashMap<String, Value>),
    Unserializable(Unserializable),
}

impl Value {
    pub fn as_int(&self) -> Option<i64> {
        if let Self::Int(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_big_int(&self) -> Option<&BigInt> {
        if let Self::BigInt(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        if let Self::Boolean(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        if let Self::String(v) = self {
            Some(v.as_str())
        } else {
            None
        }
    }

    pub fn as_map(&self) -> Option<&Map> {
        if let Self::Map(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_set(&self) -> Option<&Set> {
        if let Self::Set(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_list(&self) -> Option<&[Value]> {
        if let Self::List(v) = self {
            Some(v.as_slice())
        } else {
            None
        }
    }

    pub fn as_tuple(&self) -> Option<&Tuple> {
        if let Self::Tuple(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_record(&self) -> Option<&HashMap<String, Value>> {
        if let Self::Record(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_unserializable(&self) -> Option<&Unserializable> {
        if let Self::Unserializable(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_int(self) -> Result<i64, Self> {
        if let Self::Int(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_big_int(self) -> Result<BigInt, Self> {
        if let Self::BigInt(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_boolean(self) -> Result<bool, Self> {
        if let Self::Boolean(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_string(self) -> Result<String, Self> {
        if let Self::String(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_map(self) -> Result<Map, Self> {
        if let Self::Map(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_set(self) -> Result<Set, Self> {
        if let Self::Set(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_list(self) -> Result<Vec<Value>, Self> {
        if let Self::List(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_tuple(self) -> Result<Tuple, Self> {
        if let Self::Tuple(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_record(self) -> Result<HashMap<String, Value>, Self> {
        if let Self::Record(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_unserializable(self) -> Result<Unserializable, Self> {
        if let Self::Unserializable(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the value is [`Int`].
    ///
    /// [`Int`]: Value::Int
    #[must_use]
    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(..))
    }

    /// Returns `true` if the value is [`BigInt`].
    ///
    /// [`BigInt`]: Value::BigInt
    #[must_use]
    pub fn is_big_int(&self) -> bool {
        matches!(self, Self::BigInt(..))
    }

    /// Returns `true` if the value is [`Boolean`].
    ///
    /// [`Boolean`]: Value::Boolean
    #[must_use]
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(..))
    }

    /// Returns `true` if the value is [`String`].
    ///
    /// [`String`]: Value::String
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(..))
    }

    /// Returns `true` if the value is [`Map`].
    ///
    /// [`Map`]: Value::Map
    #[must_use]
    pub fn is_map(&self) -> bool {
        matches!(self, Self::Map(..))
    }

    /// Returns `true` if the value is [`Set`].
    ///
    /// [`Set`]: Value::Set
    #[must_use]
    pub fn is_set(&self) -> bool {
        matches!(self, Self::Set(..))
    }

    /// Returns `true` if the value is [`List`].
    ///
    /// [`List`]: Value::List
    #[must_use]
    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(..))
    }

    /// Returns `true` if the value is [`Tuple`].
    ///
    /// [`Tuple`]: Value::Tuple
    #[must_use]
    pub fn is_tuple(&self) -> bool {
        matches!(self, Self::Tuple(..))
    }

    /// Returns `true` if the value is [`Record`].
    ///
    /// [`Record`]: Value::Record
    #[must_use]
    pub fn is_record(&self) -> bool {
        matches!(self, Self::Record(..))
    }

    /// Returns `true` if the value is [`Unserializable`].
    ///
    /// [`Unserializable`]: Value::Unserializable
    #[must_use]
    pub fn is_unserializable(&self) -> bool {
        matches!(self, Self::Unserializable(..))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unserializable {
    #[serde(rename = "#unserializable")]
    pub(crate) repr: String,
}

impl Unserializable {
    pub fn as_str(&self) -> &str {
        self.repr.as_str()
    }
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BigInt {
    #[serde(rename = "#bigint")]
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) value: num_bigint::BigInt,
}

impl BigInt {
    pub fn into_bigint(self) -> num_bigint::BigInt {
        self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tuple {
    #[serde(rename = "#tup")]
    pub(crate) elements: Vec<Value>,
}

impl Tuple {
    pub fn as_slice(&self) -> &[Value] {
        self.elements.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Map {
    #[serde(rename = "#map")]
    pub(crate) map: Vec<(String, Value)>,
}

impl Map {
    pub fn as_slice(&self) -> &[(String, Value)] {
        self.map.as_slice()
    }
}

impl IntoIterator for Map {
    type Item = (String, Value);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Set {
    #[serde(rename = "#set")]
    pub(crate) set: Vec<Value>,
}

impl Set {
    pub fn as_slice(&self) -> &[Value] {
        self.set.as_slice()
    }
}

impl IntoIterator for Set {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}
