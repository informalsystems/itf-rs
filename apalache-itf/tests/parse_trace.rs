use std::collections::HashMap;

use apalache_itf::{Itf, ItfMap, ItfSet, StateMeta, TraceMeta};
use num_bigint::BigInt;
use serde::Deserialize;

#[test]
fn cannibals() {
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

    let data = include_str!("../tests/fixtures/MissionariesAndCannibals.itf.json");
    let trace: Trace<State> = serde_json::from_str(data).unwrap();

    dbg!(trace);
}

#[test]
fn insufficent_success_9() {
    type Balance = Itf<HashMap<String, Itf<BigInt>>>;
    type Balances = Itf<HashMap<String, Balance>>;

    #[derive(Copy, Clone, Debug, Deserialize)]
    enum Outcome {
        #[serde(rename = "")]
        None,
        #[serde(rename = "SUCCESS")]
        Success,
        #[serde(rename = "DUPLICATE_DENOM")]
        DuplicateDenom,
        #[serde(rename = "INSUFFICIENT_FUNDS")]
        InsufficientFunds,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[allow(dead_code)]
    struct Coin {
        amount: Itf<BigInt>,
        denom: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[allow(dead_code)]
    #[serde(tag = "tag")]
    enum Action {
        #[serde(rename = "init")]
        Init { balances: Balances },

        #[serde(rename = "send")]
        Send {
            receiver: String,
            sender: String,
            coins: Vec<Coin>,
        },
    }

    #[derive(Clone, Debug, Deserialize)]
    #[allow(dead_code)]
    struct State {
        action: Action,
        outcome: Outcome,
        balances: Balances,
        step: i64,
    }

    let data = include_str!("../tests/fixtures/TestInsufficientSuccess9.itf.json");
    let trace: Trace<State> = serde_json::from_str(data).unwrap();
    dbg!(trace);
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct State<S> {
    #[serde(rename = "#meta")]
    pub meta: StateMeta,

    #[serde(flatten)]
    pub value: S,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Trace<S> {
    #[serde(rename = "#meta")]
    pub meta: TraceMeta,

    #[serde(default)]
    pub params: Vec<String>,

    #[serde(default)]
    pub vars: Vec<String>,

    #[serde(default)]
    pub r#loop: Option<u64>,

    pub states: Vec<State<S>>,
}
