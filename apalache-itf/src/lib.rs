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
