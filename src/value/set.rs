use core::fmt;
use std::collections::BTreeSet;

/// A set of the form `{ "#set": [ <expr>, ..., <expr> ] }`.
///
/// A set is different from a list in that it does not assume any ordering of its elements.
/// However, it is only a syntax form in our format.
/// Apalache distinguishes between sets and lists and thus it will output sets in the set form.
/// Other tools may interpret sets as lists.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Set<V> {
    set: BTreeSet<V>,
}

impl<V> Set<V> {
    pub fn new() -> Self {
        Self {
            set: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, value: V) -> bool
    where
        V: Ord,
    {
        self.set.insert(value)
    }

    pub fn remove(&mut self, value: &V) -> bool
    where
        V: Ord,
    {
        self.set.remove(value)
    }

    pub fn contains(&self, value: &V) -> bool
    where
        V: Ord,
    {
        self.set.contains(value)
    }

    pub fn iter(&self) -> impl Iterator<Item = &V> {
        self.set.iter()
    }
}

impl<V> Default for Set<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> From<BTreeSet<V>> for Set<V> {
    fn from(set: BTreeSet<V>) -> Self {
        Self { set }
    }
}

impl<V> FromIterator<V> for Set<V>
where
    V: Ord,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
    {
        Self {
            set: iter.into_iter().collect(),
        }
    }
}

impl<V> IntoIterator for Set<V> {
    type Item = V;
    type IntoIter = std::collections::btree_set::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

impl<V> fmt::Debug for Set<V>
where
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.set.fmt(f)
    }
}

use serde::ser::{Serialize, SerializeMap, Serializer};
use serde::Deserialize;

impl<V> Serialize for Set<V>
where
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let elements = self.set.iter().collect::<Vec<_>>();

        let mut object = serializer.serialize_map(Some(1))?;
        object.serialize_entry("#set", &elements)?;
        object.end()
    }
}

impl<'de, V> Deserialize<'de> for Set<V>
where
    V: Deserialize<'de> + Ord,
{
    fn deserialize<D>(deserializer: D) -> Result<Set<V>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InnerSet<V: Ord> {
            #[serde(rename = "#set")]
            set: BTreeSet<V>,
        }

        let set = InnerSet::deserialize(deserializer)?.set;

        Ok(Set { set })
    }
}
