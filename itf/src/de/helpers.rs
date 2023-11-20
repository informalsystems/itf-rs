use std::fmt::Display;

use num_bigint::BigInt;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

pub fn from_bigint<'de, A, D>(deserializer: D) -> Result<A, D::Error>
where
    D: Deserializer<'de>,
    A: TryFrom<BigInt>,
    <A as TryFrom<BigInt>>::Error: Display,
{
    let bigint = BigInt::deserialize(deserializer).map_err(D::Error::custom)?;
    A::try_from(bigint).map_err(D::Error::custom)
}
