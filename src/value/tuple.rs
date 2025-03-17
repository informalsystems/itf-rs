use core::fmt;

/// A tuple of the form `{ "#tup": [ <expr>, ..., <expr> ] }`.
///
/// There is no strict rule about when to use sequences or tuples.
/// Apalache differentiates between tuples and sequences, and it may produce both forms of expressions.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tuple<V> {
    elements: Vec<V>,
}

impl<V> Tuple<V> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &V> {
        self.elements.iter()
    }
}

impl<V> Default for Tuple<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> From<Vec<V>> for Tuple<V> {
    fn from(elements: Vec<V>) -> Self {
        Self { elements }
    }
}

impl<V> FromIterator<V> for Tuple<V> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
    {
        Self {
            elements: iter.into_iter().collect(),
        }
    }
}

impl<V> IntoIterator for Tuple<V> {
    type Item = V;
    type IntoIter = std::vec::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<V> fmt::Debug for Tuple<V>
where
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.elements.fmt(f)
    }
}

use serde::ser::{Serialize, SerializeMap, Serializer};
use serde::Deserialize;

impl<V> Serialize for Tuple<V>
where
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let elements = self.elements.iter().collect::<Vec<_>>();

        let mut object = serializer.serialize_map(Some(1))?;
        object.serialize_entry("#tup", &elements)?;
        object.end()
    }
}

impl<'de, V> Deserialize<'de> for Tuple<V>
where
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Tuple<V>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InnerTuple<V> {
            #[serde(rename = "#tup")]
            elements: Vec<V>,
        }

        let elements = InnerTuple::deserialize(deserializer)?.elements;

        Ok(Tuple { elements })
    }
}
