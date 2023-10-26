#![allow(dead_code)]

use std::collections::HashMap;

use serde::Deserialize;
use serde_itf::value::BigInt;

type Balance = HashMap<String, BigInt>;
type Balances = HashMap<String, Balance>;

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
struct Coin {
    amount: BigInt,
    denom: String,
}

#[derive(Clone, Debug, Deserialize)]
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
struct State {
    action: Action,
    outcome: Outcome,
    balances: Balances,
    step: i64,
}

#[test]
#[ignore]
fn deserialize() {
    let data = include_str!("../tests/fixtures/TestInsufficientSuccess9.itf.json");
    let trace = serde_itf::trace_from_str::<State>(data).unwrap();

    dbg!(trace);
}
