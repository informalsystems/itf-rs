use std::collections::BTreeMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::value::Value;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub index: Option<u64>,

    #[serde(flatten)]
    pub other: BTreeMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State<S> {
    #[serde(rename = "#meta")]
    pub meta: Meta,

    #[serde(flatten)]
    pub value: S,
}

impl State<serde_json::Value> {
    pub fn decode<S>(self) -> Result<State<S>, Error>
    where
        S: DeserializeOwned,
    {
        let value: Value = serde_json::from_value(self.value)?;
        let inner: S = crate::de::decode_value(value)?;

        Ok(State {
            meta: self.meta,
            value: inner,
        })
    }
}

impl State<Value> {
    pub fn decode<S>(self) -> Result<State<S>, Error>
    where
        S: DeserializeOwned,
    {
        let inner: S = crate::de::decode_value(self.value)?;

        Ok(State {
            meta: self.meta,
            value: inner,
        })
    }
}

// use serde_json::value::RawValue;
//
// impl State<Box<RawValue>> {
//     pub fn decode<S>(self) -> Result<State<S>, Error>
//     where
//         S: DeserializeOwned,
//     {
//         let value: Value = serde_json::from_str(self.value.get())?;
//         dbg!(&value);
//         let inner: S = crate::de::decode_value(value)?;
//
//         Ok(State {
//             meta: self.meta,
//             value: inner,
//         })
//     }
// }
//
// impl<'a> State<&'a RawValue> {
//     pub fn decode<S>(self) -> Result<State<S>, Error>
//     where
//         S: DeserializeOwned,
//     {
//         let value: Value = serde_json::from_str(self.value.get())?;
//         dbg!(&value);
//         let inner: S = crate::de::decode_value(value)?;
//
//         Ok(State {
//             meta: self.meta,
//             value: inner,
//         })
//     }
// }
