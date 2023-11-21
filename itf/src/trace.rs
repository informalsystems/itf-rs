use std::collections::BTreeMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::state::State;
use crate::value::Value;

/// Metadata for an ITF [`Trace`].
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub format: Option<String>,

    #[serde(rename = "format-description")]
    pub format_description: Option<String>,

    #[serde(default)]
    pub source: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default, rename = "varTypes")]
    pub var_types: BTreeMap<String, String>,

    #[serde(default)]
    pub timestamp: Option<u64>,

    #[serde(flatten)]
    pub other: BTreeMap<String, String>,
}

/// An ITF trace over states of type `S`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trace<S> {
    #[serde(rename = "#meta")]
    pub meta: Meta,

    #[serde(default)]
    pub params: Vec<String>,

    #[serde(default)]
    pub vars: Vec<String>,

    #[serde(default, rename = "loop")]
    pub loop_index: Option<u64>,

    pub states: Vec<State<S>>,
}

impl Trace<serde_json::Value> {
    pub fn decode<S>(self) -> Result<Trace<S>, Error>
    where
        S: DeserializeOwned,
    {
        let states = self
            .states
            .into_iter()
            .map(|state| state.decode())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Trace {
            meta: self.meta,
            params: self.params,
            vars: self.vars,
            loop_index: self.loop_index,
            states,
        })
    }
}

impl Trace<Value> {
    pub fn decode<S>(self) -> Result<Trace<S>, Error>
    where
        S: DeserializeOwned,
    {
        let states = self
            .states
            .into_iter()
            .map(|state| state.decode())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Trace {
            meta: self.meta,
            params: self.params,
            vars: self.vars,
            loop_index: self.loop_index,
            states,
        })
    }
}

// use serde_json::value::RawValue;
//
// impl Trace<Box<RawValue>> {
//     pub fn decode<S>(self) -> Result<Trace<S>, Error>
//     where
//         S: DeserializeOwned,
//     {
//         let states = self
//             .states
//             .into_iter()
//             .map(|state| state.decode())
//             .collect::<Result<Vec<_>, _>>()?;
//
//         Ok(Trace {
//             meta: self.meta,
//             params: self.params,
//             vars: self.vars,
//             loop_index: self.loop_index,
//             states,
//         })
//     }
// }
//
// impl<'a> Trace<&'a RawValue> {
//     pub fn decode<S>(self) -> Result<Trace<S>, Error>
//     where
//         S: DeserializeOwned,
//     {
//         let states = self
//             .states
//             .into_iter()
//             .map(|state| state.decode())
//             .collect::<Result<Vec<_>, _>>()?;
//
//         Ok(Trace {
//             meta: self.meta,
//             params: self.params,
//             vars: self.vars,
//             loop_index: self.loop_index,
//             states,
//         })
//     }
// }
