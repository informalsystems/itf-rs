#![allow(dead_code)]

use std::collections::BTreeMap;
use std::result::Result as StdResult;

use serde::de::IntoDeserializer;
use serde::Deserialize;

type Address = String;
type Value = String;
type Step = String;
type Round = i64;
type Height = i64;

#[derive(Clone, Debug, Deserialize)]
enum Timeout {
    #[serde(rename = "timeoutPrevote")]
    Prevote,

    #[serde(rename = "timeoutPrecommit")]
    Precommit,

    #[serde(rename = "timeoutPropose")]
    Propose,
}

#[derive(Clone, Debug, Deserialize)]
struct State {
    system: System,

    #[serde(rename = "_Event")]
    event: Event,

    #[serde(rename = "_Result")]
    result: Result,
}

#[derive(Clone, Debug, Deserialize)]
struct System(BTreeMap<Address, ConsensusState>);

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "name")]
enum Event {
    Initial,
    NewRound {
        height: Height,
        round: Round,
    },
    Proposal {
        height: Height,
        round: Round,
        value: Value,
    },
    ProposalAndPolkaAndValid {
        height: Height,
        round: Round,
        value: Value,
    },
    ProposalAndCommitAndValid {
        height: Height,
        round: Round,
        value: Value,
    },
    NewHeight {
        height: Height,
        round: Round,
    },
    NewRoundProposer {
        height: Height,
        round: Round,
        value: Value,
    },
    PolkaNil {
        height: Height,
        round: Round,
        value: Value,
    },
    PolkaAny {
        height: Height,
        round: Round,
        value: Value,
    },
    PrecommitAny {
        height: Height,
        round: Round,
        value: Value,
    },
    TimeoutPrevote {
        height: Height,
        round: Round,
    },
    TimeoutPrecommit {
        height: Height,
        round: Round,
        value: Value,
    },
    TimeoutPropose {
        height: Height,
        round: Round,
        value: Value,
    },
    ProposalInvalid {
        height: Height,
        round: Round,
    },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Result {
    name: String,
    #[serde(deserialize_with = "proposal_or_none")]
    proposal: Option<Proposal>,
    #[serde(deserialize_with = "vote_message_or_none")]
    vote_message: Option<VoteMessage>,
    #[serde(deserialize_with = "empty_string_as_none")]
    timeout: Option<Timeout>,
    #[serde(deserialize_with = "empty_string_as_none")]
    decided: Option<Value>,
    #[serde(deserialize_with = "minus_one_as_none")]
    skip_round: Option<Round>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Proposal {
    src: Address,
    height: Height,
    round: Round,
    proposal: Value,
    valid_round: Round,
}

impl Proposal {
    fn is_empty(&self) -> bool {
        self.src.is_empty()
            && self.proposal.is_empty()
            && self.height == -1
            && self.round == -1
            && self.valid_round == -1
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VoteMessage {
    src: Address,
    height: Height,
    round: Round,
    step: Step,
    id: Value,
}

impl VoteMessage {
    fn is_empty(&self) -> bool {
        self.src.is_empty()
            && self.id.is_empty()
            && self.height == -1
            && self.round == -1
            && self.step.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsensusState {
    p: Address,
    height: Height,
    round: Round,
    step: Step,

    #[serde(deserialize_with = "minus_one_as_none")]
    locked_round: Option<Round>,
    #[serde(deserialize_with = "empty_string_as_none")]
    locked_value: Option<Value>,
    #[serde(deserialize_with = "minus_one_as_none")]
    valid_round: Option<Round>,
    #[serde(deserialize_with = "empty_string_as_none")]
    valid_value: Option<Value>,
}

fn empty_string_as_none<'de, D, T>(de: D) -> StdResult<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

fn minus_one_as_none<'de, D, T>(de: D) -> StdResult<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<i64>::deserialize(de)?;
    match opt {
        None | Some(-1) => Ok(None),
        Some(i) => T::deserialize(i.into_deserializer()).map(Some),
    }
}

fn proposal_or_none<'de, D>(de: D) -> StdResult<Option<Proposal>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let proposal = Proposal::deserialize(de)?;
    if proposal.is_empty() {
        Ok(None)
    } else {
        Ok(Some(proposal))
    }
}

fn vote_message_or_none<'de, D>(de: D) -> StdResult<Option<VoteMessage>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let vote_message = VoteMessage::deserialize(de)?;
    if vote_message.is_empty() {
        Ok(None)
    } else {
        Ok(Some(vote_message))
    }
}

#[test]
fn deserialize() {
    let data = include_str!("../tests/fixtures/DecideNonProposerTest0.itf.json");
    let trace = serde_itf::trace_from_str::<State>(data).unwrap();
    dbg!(trace);
}
