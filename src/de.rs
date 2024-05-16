//! Helpers for annotating types to deserialize from ITF values.

use serde::de::DeserializeOwned;

use crate::Value;

mod error;
pub use error::Error;

mod helpers;
pub use helpers::{As, Integer, Option, Result, Same};

mod deserializer;

#[doc(hidden)]
pub fn decode_value<T>(value: Value) -> std::result::Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(value)
}
