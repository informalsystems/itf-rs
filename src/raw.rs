use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    meta::{StateMeta, TraceMeta},
    value::Value,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trace {
    #[serde(rename = "#meta")]
    pub meta: TraceMeta,

    #[serde(default)]
    pub params: Vec<String>,

    #[serde(default)]
    pub vars: Vec<String>,

    #[serde(default)]
    pub states: Vec<State>,

    #[serde(default)]
    pub r#loop: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    #[serde(rename = "#meta")]
    pub meta: StateMeta,

    #[serde(flatten)]
    pub values: HashMap<String, Value>,
}
