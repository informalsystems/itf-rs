//! Library for consuming [Apalache ITF Traces](https://apalache.informal.systems/docs/adr/015adr-trace.html).
//!
//! ## Example
//!
//! **Trace:** [`MissionariesAndCannibals.itf.json`](./apalache-itf/tests/fixtures/MissionariesAndCannibals.itf.json)
//!
//! ```rust
//! use serde::Deserialize;
//!
//! use itf::{trace_from_str, ItfMap, ItfSet};
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
//! let trace: Trace<State> = trace_from_str(data).unwrap();
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

mod util;

mod meta;
pub use meta::*;

mod itf;
pub use itf::*;

mod trace;
use serde::{de::DeserializeOwned, Deserialize};
pub use trace::*;

use serde_json::Result;

pub fn trace_from_str<'a, State>(s: &'a str) -> Result<Trace<State>>
where
    State: Deserialize<'a>,
{
    serde_json::from_str(s)
}

pub fn trace_from_slice<'a, State>(s: &'a [u8]) -> Result<Trace<State>>
where
    State: Deserialize<'a>,
{
    serde_json::from_slice(s)
}

pub fn trace_from_value<State>(v: serde_json::Value) -> Result<Trace<State>>
where
    State: DeserializeOwned,
{
    serde_json::from_value(v)
}

pub fn trace_from_reader<State, R>(r: R) -> Result<Trace<State>>
where
    State: DeserializeOwned,
    R: std::io::Read,
{
    serde_json::from_reader(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
    enum Bank {
        #[serde(rename = "N")]
        North,
        #[serde(rename = "W")]
        West,
        #[serde(rename = "E")]
        East,
        #[serde(rename = "S")]
        South,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
    enum Person {
        #[serde(rename = "c1_OF_PERSON")]
        Cannibal1,
        #[serde(rename = "c2_OF_PERSON")]
        Cannibal2,
        #[serde(rename = "m1_OF_PERSON")]
        Missionary1,
        #[serde(rename = "m2_OF_PERSON")]
        Missionary2,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[allow(dead_code)]
    struct State {
        pub bank_of_boat: Bank,
        pub who_is_on_bank: ItfMap<Bank, ItfSet<Person>>,
    }

    const DATA: &str = include_str!("../tests/fixtures/MissionariesAndCannibals.itf.json");

    #[test]
    fn from_str() {
        let _trace = trace_from_str::<State>(DATA).unwrap();
    }

    #[test]
    fn from_slice() {
        let _trace = trace_from_slice::<State>(DATA.as_bytes()).unwrap();
    }

    #[test]
    fn from_value() {
        let value = serde_json::from_str(DATA).unwrap();
        let _trace = trace_from_value::<State>(value).unwrap();
    }

    #[test]
    fn from_reader() {
        let _trace = trace_from_reader::<State, _>(DATA.as_bytes()).unwrap();
    }
}
