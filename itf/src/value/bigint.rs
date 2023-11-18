use core::fmt;

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

/// A big integer of the following form: `{ "#bigint": "[-][0-9]+" }`.
///
/// We are using this format, as many JSON parsers impose limits
/// on integer values, see RFC7159.
///
/// Big and small integers must be written in this format.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BigInt(num_bigint::BigInt);

impl BigInt {
    pub fn new(value: impl Into<num_bigint::BigInt>) -> Self {
        Self(value.into())
    }

    pub fn get(&self) -> &num_bigint::BigInt {
        &self.0
    }

    pub fn into_inner(self) -> num_bigint::BigInt {
        self.0
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
        #[serde(untagged)]
        enum BigInt {
            // deserialized serde_json::Value
            Itf {
                #[serde(rename = "#bigint")]
                bigint: String,
            },
            // deserialized itf::Value
            BigInt(i64, Vec<u32>),
        }

        match BigInt::deserialize(deserializer)? {
            BigInt::Itf { bigint } => {
                let bigint: num_bigint::BigInt =
                    bigint.parse().map_err(serde::de::Error::custom)?;
                Ok(Self::new(bigint))
            }
            BigInt::BigInt(sign, digits) => {
                let sign = match sign.cmp(&0) {
                    std::cmp::Ordering::Less => num_bigint::Sign::Minus,
                    std::cmp::Ordering::Equal => num_bigint::Sign::NoSign,
                    std::cmp::Ordering::Greater => num_bigint::Sign::Plus,
                };
                Ok(Self::new(num_bigint::BigInt::new(sign, digits)))
            }
        }
    }
}
