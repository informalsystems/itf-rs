//! Library for consuming [Apalache ITF Traces](https://apalache.informal.systems/docs/adr/015adr-trace.html).
//!
//! ## Example
//!
//! **Trace:** [`MissionariesAndCannibals.itf.json`](../tests/fixtures/MissionariesAndCannibals.itf.json)
//!
//! ```rust
//! use serde::Deserialize;
//!
//! #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
//! enum Bank {
//!     #[serde(rename = "N")]
//!     North,
//!
//!     #[serde(rename = "W")]
//!     West,
//!
//!     #[serde(rename = "E")]
//!     East,
//!
//!     #[serde(rename = "S")]
//!     South,
//! }
//!
//! #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
//! enum Person {
//!     #[serde(rename = "c1_OF_PERSON")]
//!     Cannibal1,
//!
//!     #[serde(rename = "c2_OF_PERSON")]
//!     Cannibal2,
//!
//!     #[serde(rename = "m1_OF_PERSON")]
//!     Missionary1,
//!
//!     #[serde(rename = "m2_OF_PERSON")]
//!     Missionary2,
//! }
//!
//! #[derive(Clone, Debug, Deserialize)]
//! struct State {
//!     pub bank_of_boat: Bank,
//!     pub who_is_on_bank: ItfMap<Bank, ItfSet<Person>>,
//! }
//!
//! let data = include_str!("../tests/fixtures/MissionariesAndCannibals.itf.json");
//! let trace: Trace<State> = itf::trace_from_str(data).unwrap();
//!
//! dbg!(trace);
//! ```
//!
//! **Output:**
//!
//! ```rust
//! trace = Trace {
//!     meta: TraceMeta {
//!         description: None,
//!         source: Some(
//!             "MC_MissionariesAndCannibalsTyped.tla",
//!         ),
//!         var_types: {
//!             "bank_of_boat": "Str",
//!             "who_is_on_bank": "Str -> Set(PERSON)",
//!         },
//!         format: None,
//!         format_description: None,
//!         other: {},
//!     },
//!     params: [],
//!     vars: [
//!         "bank_of_boat",
//!         "who_is_on_bank",
//!     ],
//!     loop_index: None,
//!     states: [
//!         State {
//!             meta: StateMeta {
//!                 index: Some(
//!                     0,
//!                 ),
//!                 other: {},
//!             },
//!             value: State {
//!                 bank_of_boat: East,
//!                 who_is_on_bank: {
//!                     West: {},
//!                     East: {
//!                         Missionary2,
//!                         Cannibal1,
//!                         Cannibal2,
//!                         Missionary1,
//!                     },
//!                 },
//!             },
//!         },
//!         State {
//!             meta: StateMeta {
//!                 index: Some(
//!                     1,
//!                 ),
//!                 other: {},
//!             },
//!             value: State {
//!                 bank_of_boat: West,
//!                 who_is_on_bank: {
//!                     West: {
//!                         Missionary2,
//!                         Cannibal2,
//!                     },
//!                     East: {
//!                         Missionary1,
//!                         Cannibal1,
//!                     },
//!                 },
//!             },
//!         },
//!         State {
//!             meta: StateMeta {
//!                 index: Some(
//!                     2,
//!                 ),
//!                 other: {},
//!             },
//!             value: State {
//!                 bank_of_boat: East,
//!                 who_is_on_bank: {
//!                     West: {
//!                         Cannibal2,
//!                     },
//!                     East: {
//!                         Missionary2,
//!                         Cannibal1,
//!                         Missionary1,
//!                     },
//!                 },
//!             },
//!         },
//!         State {
//!             meta: StateMeta {
//!                 index: Some(
//!                     3,
//!                 ),
//!                 other: {},
//!             },
//!             value: State {
//!                 bank_of_boat: West,
//!                 who_is_on_bank: {
//!                     West: {
//!                         Missionary1,
//!                         Cannibal2,
//!                         Missionary2,
//!                     },
//!                     East: {
//!                         Cannibal1,
//!                     },
//!                 },
//!             },
//!         },
//!         State {
//!             meta: StateMeta {
//!                 index: Some(
//!                     4,
//!                 ),
//!                 other: {},
//!             },
//!             value: State {
//!                 bank_of_boat: East,
//!                 who_is_on_bank: {
//!                     East: {
//!                         Cannibal2,
//!                         Cannibal1,
//!                     },
//!                     West: {
//!                         Missionary1,
//!                         Missionary2,
//!                     },
//!                 },
//!             },
//!         },
//!         State {
//!             meta: StateMeta {
//!                 index: Some(
//!                     5,
//!                 ),
//!                 other: {},
//!             },
//!             value: State {
//!                 bank_of_boat: West,
//!                 who_is_on_bank: {
//!                     East: {},
//!                     West: {
//!                         Cannibal1,
//!                         Cannibal2,
//!                         Missionary1,
//!                         Missionary2,
//!                     },
//!                 },
//!             },
//!         },
//!     ],
//! }
//! ```

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

pub fn trace_from_str<S>(str: &str) -> Result<Trace<S>, Error>
where
    S: for<'de> Deserialize<'de>,
{
    let trace_value: Trace<Value> = serde_json::from_str(str)?;
    trace_value.decode()
}

pub fn trace_from_value<S>(value: serde_json::Value) -> Result<Trace<S>, Error>
where
    S: DeserializeOwned,
{
    let trace_value: Trace<Value> = serde_json::from_value(value)?;
    trace_value.decode()
}

pub fn from_str<S>(str: &str) -> Result<S, Error>
where
    S: for<'de> Deserialize<'de>,
{
    let value: Value = serde_json::from_str(str)?;
    let data = S::deserialize(value)?;
    Ok(data)
}

pub fn from_value<S>(value: serde_json::Value) -> Result<S, Error>
where
    S: DeserializeOwned,
{
    let trace_value: Value = serde_json::from_value(value)?;
    let s = S::deserialize(trace_value)?;
    Ok(s)
}
