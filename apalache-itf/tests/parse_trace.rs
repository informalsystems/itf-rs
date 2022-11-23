use std::collections::{HashMap, HashSet};

use apalache_itf::{parse_raw_trace, raw, DecodeError, DecodeItfValue, TryFromRawState, Value};

#[test]
fn cannibals() {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, DecodeItfValue)]
    enum Dir {
        N,
        W,
        E,
        S,
    }

    #[derive(Clone, Debug, TryFromRawState)]
    #[allow(dead_code)]
    struct State {
        pub bank_of_boat: Dir,
        pub who_is_on_bank: HashMap<Dir, HashSet<String>>,
    }

    let data = include_str!("../tests/fixtures/MissionariesAndCannibals.itf.json");
    let raw_trace: raw::Trace = serde_json::from_str(data).unwrap();
    let trace = parse_raw_trace::<State>(raw_trace).unwrap();

    dbg!(trace);
}

#[test]
fn insufficent_success_9() {
    type Balance = HashMap<String, num_bigint::BigInt>;
    type Balances = HashMap<String, Balance>;

    #[derive(Copy, Clone, Debug)]
    enum Outcome {
        None,
        Success,
        DuplicateDenom,
        InsufficientFunds,
    }

    impl DecodeItfValue for Outcome {
        fn decode(value: Value) -> Result<Self, DecodeError> {
            match value {
                Value::String(s) => match s.as_str() {
                    "" => Ok(Self::None),
                    "SUCCESS" => Ok(Self::Success),
                    "DUPLICATE_DENOM" => Ok(Self::DuplicateDenom),
                    "INSUFFICIENT_FUNDS" => Ok(Self::InsufficientFunds),
                    _ => Err(DecodeError::InvalidType("string")),
                },
                _ => Err(DecodeError::InvalidType("string")),
            }
        }
    }

    #[derive(Clone, Debug, TryFromRawState)]
    #[allow(dead_code)]
    struct State {
        pub outcome: Outcome,
        pub balances: Balances,
        pub step: i64,
    }

    let data = include_str!("../tests/fixtures/TestInsufficientSuccess9.itf.json");
    let raw_trace: raw::Trace = serde_json::from_str(data).unwrap();
    let trace = parse_raw_trace::<State>(raw_trace).unwrap();
    dbg!(trace);
}
