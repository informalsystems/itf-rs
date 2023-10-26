use serde::de::DeserializeOwned;
use serde::Deserialize;

pub mod de;
pub mod error;
pub mod meta;
pub mod state;
pub mod trace;
pub mod value;

use error::Error;
use trace::Trace;
use value::Value;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;
    use serde::Deserialize;
    use std::collections::{BTreeSet, HashMap};

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
        pub who_is_on_bank: HashMap<Bank, BTreeSet<Person>>,
    }

    #[test]
    fn de_cannibals() -> Result<(), Error> {
        let path = format!(
            "{}/../itf/tests/fixtures/MissionariesAndCannibals.itf.json",
            env!("CARGO_MANIFEST_DIR")
        );

        let fixture = std::fs::read_to_string(path)?;
        let trace: Trace<State> = crate::trace_from_str(&fixture)?;
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
        let trace: Trace<Value> = crate::trace_from_str(&fixture)?;
        dbg!(trace);

        Ok(())
    }
}
