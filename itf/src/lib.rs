#![doc = include_str!("../../README.md")]

use serde::de::DeserializeOwned;
use serde::Deserialize;

pub mod de;
pub mod error;
pub mod state;
pub mod trace;

pub use error::Error;
pub use state::State;
pub use trace::Trace;

#[doc(hidden)]
pub mod value;

#[doc(hidden)]
pub use value::Value;

/// Deserialize a [`Trace`] over states `S` from an ITF JSON string.
pub fn trace_from_str<S>(str: &str) -> Result<Trace<S>, Error>
where
    S: for<'de> Deserialize<'de>,
{
    let trace_value: Trace<Value> = serde_json::from_str(str)?;
    trace_value.decode()
}

/// Deserialize a [`Trace`] over states `S` from an ITF JSON [`serde_json::Value`].
pub fn trace_from_value<S>(value: serde_json::Value) -> Result<Trace<S>, Error>
where
    S: DeserializeOwned,
{
    let trace_value: Trace<Value> = serde_json::from_value(value)?;
    trace_value.decode()
}

/// Deserialize an ITF-encoded expression `S` from an ITF JSON string.
pub fn from_str<S>(str: &str) -> Result<S, Error>
where
    S: for<'de> Deserialize<'de>,
{
    let value: Value = serde_json::from_str(str)?;
    let data = S::deserialize(value)?;
    Ok(data)
}

/// Deserialize an ITF-encoded expression `S` from an ITF JSON [`serde_json::Value`].
pub fn from_value<S>(value: serde_json::Value) -> Result<S, Error>
where
    S: DeserializeOwned,
{
    let trace_value: Value = serde_json::from_value(value)?;
    let s = S::deserialize(trace_value)?;
    Ok(s)
}
