use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Int(i64),
    BigInt(i128),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tuple {
    #[serde(rename = "#tup")]
    pub(crate) elements: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Map {
    #[serde(rename = "#map")]
    pub(crate) map: Vec<(String, Value)>,
}

impl Map {
    pub fn to_hashmap(self) -> HashMap<String, Value> {
        self.map.into_iter().collect()
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

impl IntoIterator for Set {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}
