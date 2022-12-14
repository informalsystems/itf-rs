use serde::Deserialize;

use crate::{StateMeta, TraceMeta};

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

    #[serde(default, rename = "loop")]
    pub loop_index: Option<u64>,

    pub states: Vec<State<S>>,
}

impl<S> Default for Trace<S> {
    fn default() -> Self {
        Self {
            meta: Default::default(),
            params: Default::default(),
            vars: Default::default(),
            loop_index: Default::default(),
            states: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_default() {
        #[derive(Debug, PartialEq, Eq)]
        struct S; // no need for `Default derive`

        let t: Trace<S> = Trace::default();
        assert_eq!(t.meta, TraceMeta::default());
        assert_eq!(t.params, Vec::<String>::new());
        assert_eq!(t.vars, Vec::<String>::new());
        assert_eq!(t.loop_index, None);
        assert_eq!(t.states, Vec::<State<S>>::new());
    }
}
