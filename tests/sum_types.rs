use num_bigint::BigInt;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(tag = "tag", content = "value")]
enum IntOption {
    Some(BigInt),
    None,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct State {
    value: IntOption,
}

#[test]
fn parse_trace() {
    let trace =
        itf::trace_from_str::<State>(include_str!("./fixtures/SumTypes0.itf.json")).unwrap();

    assert_eq!(trace.states[0].value.value, IntOption::None);
    assert_eq!(trace.states[1].value.value, IntOption::Some(40.into()));
    assert_eq!(trace.states[2].value.value, IntOption::Some(41.into()));
}

#[test]
fn test_deserialize_some() {
    let some_itf = json!({
        "tag": "Some",
        "value": {"#bigint": "1"},
    });

    let some = itf::from_value::<IntOption>(some_itf).unwrap();
    assert_eq!(some, IntOption::Some(1.into()));
}

#[test]
fn test_deserialize_none() {
    let none_itf = json!({
        "tag": "None",
        "value": {},
    });

    let none = itf::from_value::<IntOption>(none_itf).unwrap();
    assert_eq!(none, IntOption::None);
}
