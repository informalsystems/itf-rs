use core::fmt;
use std::collections::BTreeMap;

use crate::value::Value;

/// A record of the form `{ "field1": <expr>, ..., "fieldN": <expr> }`.
///
/// A record is just a JSON object. Field names should not start with `#` and
/// hence should not pose any collision with other constructs.
/// TLA+ records are written as records in this format.

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Record {
    map: BTreeMap<String, Value>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        self.map.insert(key, value)
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.map.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.map.get(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.map.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl From<BTreeMap<String, Value>> for Record {
    fn from(map: BTreeMap<String, Value>) -> Self {
        Self { map }
    }
}

impl FromIterator<(String, Value)> for Record {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (String, Value)>,
    {
        Self {
            map: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Record {
    type Item = (String, Value);
    type IntoIter = std::collections::btree_map::IntoIter<String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.fmt(f)
    }
}

use serde::ser::{Serialize, SerializeMap, Serializer};
use serde::Deserialize;

/// Serialize into a JSON object of this form:
///
///```ignore
/// {
///   "field1": <value>,
///   ...
///   "fieldN": <value>,
/// }
/// ```
impl Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.map.len()))?;
        for (k, v) in &self.map {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Record {
    fn deserialize<D>(deserializer: D) -> Result<Record, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = BTreeMap::<String, Value>::deserialize(deserializer)?;
        Ok(Record { map })
    }
}
