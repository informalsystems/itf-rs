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
