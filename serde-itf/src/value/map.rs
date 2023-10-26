use core::fmt;
use std::collections::BTreeMap;

/// A map of the form `{ "#map": [ [ <expr>, <expr> ], ..., [ <expr>, <expr> ] ] }`.
///
/// That is, a map holds a JSON array of two-element arrays.
/// Each two-element array p is interpreted as follows:
/// - `p[0]` is the map key
/// - `p[1]` is the map value
///
/// Importantly, a key may be an arbitrary expression.
/// It does not have to be a string or an integer.
///
/// TLA+ functions are written as maps in this format.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Map<K, V> {
    map: BTreeMap<K, V>,
}

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl<K, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::btree_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<K, V> fmt::Debug for Map<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.fmt(f)
    }
}

use serde::ser::{Serialize, SerializeMap, Serializer};
use serde::Deserialize;

use crate::value::Value;

/// Serialize into a JSON object of this form:
///
///```ignore
/// {
///   "#map": [
///     [ <key>, <value> ],
///     ...,
///     [ <key>, <value> ]
///  ]
/// }
/// ```
impl<K, V> Serialize for Map<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // let mut pairs = serializer.serialize_seq(Some(self.map.len()))?;
        //
        // for (key, value) in &self.map {
        //     let mut pair = serializer.serialize_tuple(2)?;
        //     pair.serialize_element(key)?;
        //     pair.serialize_element(value);
        // }
        //
        // let pairs = pairs.end()?;

        let pairs = self.map.iter().collect::<Vec<_>>();

        let mut object = serializer.serialize_map(Some(1))?;
        object.serialize_entry("#map", &pairs)?;
        object.end()
    }
}

impl<'de, V> Deserialize<'de> for Map<Value, V>
where
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Map<Value, V>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InnerMap<V> {
            #[serde(rename = "#map")]
            map: Vec<(Value, V)>,
        }

        let map = InnerMap::deserialize(deserializer)?
            .map
            .into_iter()
            .collect();

        Ok(Map { map })
    }
}

impl<'de, V> Deserialize<'de> for Map<String, V>
where
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Map<String, V>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = BTreeMap::<String, V>::deserialize(deserializer)?;
        Ok(Map { map })
    }
}
