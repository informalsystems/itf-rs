use std::collections::{HashMap, HashSet};

use apalache_itf::{parse_raw_trace, raw, DecodeError, DecodeItfValue, TryFromRawState};

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

    let data = include_str!("../tests/fixtures/MissionariesAndCannibals.json");
    let raw_trace: raw::Trace = serde_json::from_str(data).unwrap();
    let trace = parse_raw_trace::<State>(raw_trace).unwrap();

    dbg!(trace);
}
