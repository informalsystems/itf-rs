#![allow(dead_code)]

use std::collections::{BTreeSet, HashMap};

use serde::Deserialize;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
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
struct State {
    pub bank_of_boat: Bank,
    pub who_is_on_bank: HashMap<Bank, BTreeSet<Person>>,
}

#[test]
fn cannibals() {
    let data = include_str!("../tests/fixtures/MissionariesAndCannibals.itf.json");
    let trace = itf::trace_from_str::<State>(data).unwrap();

    dbg!(trace);
}
