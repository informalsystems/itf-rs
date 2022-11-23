mod decode;
pub use decode::*;

mod meta;
pub use meta::*;

pub mod raw;

mod value;
pub use value::Value;

mod util;

pub use apalache_itf_derive::{DecodeItfValue, TryFromRawState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trace<State> {
    pub meta: TraceMeta,
    pub params: Vec<String>,
    pub vars: Vec<String>,
    pub r#loop: Option<u64>,
    pub states: Vec<State>,
}

pub fn parse_raw_trace<State>(raw_trace: raw::Trace) -> Result<Trace<State>, DecodeError>
where
    State: TryFrom<raw::State, Error = DecodeError>,
{
    let states = raw_trace
        .states
        .into_iter()
        .map(TryFrom::try_from)
        .collect::<Result<_, _>>()?;

    Ok(Trace {
        meta: raw_trace.meta,
        params: raw_trace.params,
        vars: raw_trace.vars,
        r#loop: raw_trace.r#loop,
        states,
    })
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

#[cfg(test)]
mod tests {}
