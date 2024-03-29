use serde::de::DeserializeOwned;

use crate::Value;

mod error;
pub use error::Error;

mod helpers;
pub use helpers::{As, Integer, Same};

mod deserializer;

#[doc(hidden)]
pub fn decode_value<T>(value: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(value)
}
