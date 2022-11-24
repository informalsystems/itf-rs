use crate::TraceMeta;

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
