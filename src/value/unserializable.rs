/// An expression that cannot be serialized: `{ "#unserializable": "<string representation>" }`.
///
/// For instance, the set of all integers is represented with `{ "#unserializable": "Int" }`.
/// This should be a very rare expression, which should not occur in normal traces.
/// Usually, it indicates some form of an error.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unserializable(String);

use serde::ser::{Serialize, SerializeMap, Serializer};
use serde::Deserialize;

impl Serialize for Unserializable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut object = serializer.serialize_map(Some(1))?;
        object.serialize_entry("#unserializable", &self.0)?;
        object.end()
    }
}

impl<'de> Deserialize<'de> for Unserializable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner {
            #[serde(rename = "#unserializable")]
            string: String,
        }

        let inner = Inner::deserialize(deserializer)?.string;

        Ok(Self(inner))
    }
}
