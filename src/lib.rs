use itertools::Itertools;
use meta::TraceMeta;

pub mod meta;
pub mod raw;
pub mod value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trace<State> {
    pub meta: TraceMeta,
    pub params: Vec<String>,
    pub vars: Vec<String>,
    pub r#loop: Option<u64>,
    pub states: Vec<State>,
}

impl<State> Default for Trace<State> {
    fn default() -> Self {
        Self {
            meta: Default::default(),
            params: Default::default(),
            vars: Default::default(),
            r#loop: Default::default(),
            states: Default::default(),
        }
    }
}

impl<State> TryFrom<raw::Trace> for Trace<State>
where
    State: TryFrom<raw::State>,
{
    type Error = State::Error;

    fn try_from(raw: raw::Trace) -> Result<Self, Self::Error> {
        let states = raw
            .states
            .into_iter()
            .map(TryFrom::try_from)
            .try_collect()?;

        Ok(Self {
            meta: raw.meta,
            params: raw.params,
            vars: raw.vars,
            r#loop: raw.r#loop,
            states,
        })
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use std::collections::{HashMap, HashSet};

    use crate::{raw, value::Value, Trace};

    #[test]
    fn cannibals() {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        enum Dir {
            N,
            W,
            E,
            S,
        }

        #[derive(Clone, Debug)]
        #[allow(dead_code)]
        struct State {
            pub bank_of_boat: Dir,
            pub who_is_on_bank: HashMap<Dir, HashSet<String>>,
        }

        impl TryFrom<Value> for Dir {
            type Error = ();

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    Value::String(dir) if dir == "N" => Ok(Dir::N),
                    Value::String(dir) if dir == "W" => Ok(Dir::W),
                    Value::String(dir) if dir == "E" => Ok(Dir::E),
                    Value::String(dir) if dir == "S" => Ok(Dir::S),
                    _ => Err(()),
                }
            }
        }

        impl TryFrom<raw::State> for State {
            type Error = ();

            fn try_from(mut raw: raw::State) -> Result<Self, Self::Error> {
                let bank_of_boat = raw.values.remove("bank_of_boat").ok_or(())?;
                let who_is_on_bank = raw.values.remove("who_is_on_bank").ok_or(())?;

                let bank_of_boat = Dir::try_from(bank_of_boat)?;
                let who_is_on_bank = HashMap::<String, HashSet<String>>::try_from(who_is_on_bank)?;
                let who_is_on_bank = who_is_on_bank
                    .into_iter()
                    .map(|(k, v)| Dir::try_from(Value::String(k)).map(|k| (k, v)))
                    .try_collect()?;

                Ok(State {
                    bank_of_boat,
                    who_is_on_bank,
                })
            }
        }

        let data = include_str!("../tests/fixtures/MissionariesAndCannibals.json");
        let raw_trace = serde_json::from_str::<raw::Trace>(data).unwrap();
        let trace = Trace::<State>::try_from(raw_trace).unwrap();
        dbg!(trace);
    }
}
