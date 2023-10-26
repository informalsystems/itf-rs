use core::fmt;
use std::ops::Deref;

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BigInt(num_bigint::BigInt);

impl Deref for BigInt {
    type Target = num_bigint::BigInt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for BigInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("BigInt", 1)?;
        s.serialize_field("#bigint", &self.to_string())?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for BigInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct BigInt {
            #[serde(rename = "#bigint")]
            bigint: String,
        }

        let inner = BigInt::deserialize(deserializer)?;
        let bigint = inner.bigint.parse().map_err(serde::de::Error::custom)?;
        Ok(Self(bigint))
    }
}
