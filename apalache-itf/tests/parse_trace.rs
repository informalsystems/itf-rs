use std::collections::{HashMap, HashSet};

use apalache_itf::{parse_raw_trace, raw, DecodeError, DecodeItfValue, TryFromRawState};

#[test]
fn cannibals() {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, DecodeItfValue)]
    enum Bank {
        #[itf(rename = "N")]
        North,
        #[itf(rename = "W")]
        West,
        #[itf(rename = "E")]
        East,
        #[itf(rename = "S")]
        South,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, DecodeItfValue)]
    enum Person {
        #[itf(rename = "c1_OF_PERSON")]
        Cannibal1,
        #[itf(rename = "c2_OF_PERSON")]
        Cannibal2,
        #[itf(rename = "m1_OF_PERSON")]
        Missionary1,
        #[itf(rename = "m2_OF_PERSON")]
        Missionary2,
    }

    #[derive(Clone, Debug, TryFromRawState)]
    #[allow(dead_code)]
    struct State {
        #[itf(rename = "bank_of_boat")]
        pub boat_is_on_bank: Bank,
        pub who_is_on_bank: HashMap<Bank, HashSet<Person>>,
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

    #[derive(Copy, Clone, Debug, DecodeItfValue)]
    enum Outcome {
        #[itf(rename = "")]
        None,
        #[itf(rename = "SUCCESS")]
        Success,
        #[itf(rename = "DUPLICATE_DENOM")]
        DuplicateDenom,
        #[itf(rename = "INSUFFICIENT_FUNDS")]
        InsufficientFunds,
    }

    // #[derive(Clone, Debug, DecodeItfValue)]
    // #[allow(dead_code)]
    // struct Action {
    //     tag: String,
    //     balances: Balances,
    // }

    #[derive(Clone, Debug, TryFromRawState)]
    #[allow(dead_code)]
    struct State {
        // action: Balances,
        outcome: Outcome,
        balances: Balances,
        step: i64,
    }

    let data = include_str!("../tests/fixtures/TestInsufficientSuccess9.itf.json");
    let raw_trace: raw::Trace = serde_json::from_str(data).unwrap();
    let trace = parse_raw_trace::<State>(raw_trace).unwrap();
    dbg!(trace);
}
