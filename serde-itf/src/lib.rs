use serde::de::DeserializeOwned;

pub mod bigint;
pub mod de;
pub mod error;
pub mod map;
pub mod meta;
pub mod set;
pub mod state;
pub mod trace;
pub mod tuple;
pub mod unserializable;
pub mod value;

use error::Error;
use trace::Trace;
use value::Value;

pub fn from_str<S>(s: &str) -> Result<Trace<S>, Error>
where
    S: DeserializeOwned,
{
    let value = serde_json::from_str(s)?;
    from_value(value)
}

pub fn from_value<S>(value: serde_json::Value) -> Result<Trace<S>, Error>
where
    S: DeserializeOwned,
{
    let trace_value: Trace<Value> = serde_json::from_value(value)?;
    trace_value.decode()
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use crate::value::Value;

    use super::*;

    use serde::Deserialize;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
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

    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
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
        pub who_is_on_bank: BTreeMap<Bank, BTreeSet<Person>>,
    }

    #[test]
    fn de_cannibals() -> Result<(), Error> {
        let path = format!(
            "{}/../itf/tests/fixtures/MissionariesAndCannibals.itf.json",
            env!("CARGO_MANIFEST_DIR")
        );

        let fixture = std::fs::read_to_string(path)?;
        let trace: Trace<State> = crate::from_str(&fixture)?;
        dbg!(trace);

        Ok(())
    }

    #[test]
    fn de_consensus() -> Result<(), Error> {
        let path = format!(
            "{}/../itf/tests/fixtures/DecideNonProposerTest0.itf.json",
            env!("CARGO_MANIFEST_DIR")
        );

        let fixture = std::fs::read_to_string(path)?;
        let trace: Trace<Value> = crate::from_str(&fixture)?;
        dbg!(trace);

        Ok(())
    }
}
