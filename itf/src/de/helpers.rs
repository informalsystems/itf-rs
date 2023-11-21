use std::fmt::Display;

use num_bigint::BigInt;
use serde::de::Error;
use serde::Deserialize;
use serde_with::DeserializeAs;

pub struct Integer;

impl<'de, A> DeserializeAs<'de, A> for Integer
where
    A: TryFrom<BigInt>,
    <A as TryFrom<BigInt>>::Error: Display,
{
    fn deserialize_as<D>(deserializer: D) -> Result<A, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bigint = BigInt::deserialize(deserializer).map_err(D::Error::custom)?;
        A::try_from(bigint).map_err(D::Error::custom)
    }
}
