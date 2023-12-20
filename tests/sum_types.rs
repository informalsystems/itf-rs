use num_bigint::BigInt;

use serde::Deserialize;
use serde_json::json;

use itf::de::{As, Integer, Same};

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

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(tag = "tag", content = "value")]
enum Enum {
    Foo,
    Bar(String),
    Baz((String, bool)),
    #[serde(with = "As::<(Same, Integer, Same)>")]
    FooBar(String, BigInt, bool),
}

#[test]
#[allow(clippy::disallowed_names)]
fn test_deserialize_enum() {
    let foo_itf = json!({
        "tag": "Foo",
        "value": {},
    });

    let foo = itf::from_value::<Enum>(foo_itf).unwrap();
    assert_eq!(foo, Enum::Foo);

    let bar_itf = json!({
        "tag": "Bar",
        "value": "hello",
    });

    let bar = itf::from_value::<Enum>(bar_itf).unwrap();
    assert_eq!(bar, Enum::Bar("hello".to_string()));

    let baz_itf = json!({
        "tag": "Baz",
        "value": { "#tup": ["hello", true] },
    });

    let baz = itf::from_value::<Enum>(baz_itf).unwrap();
    assert_eq!(baz, Enum::Baz(("hello".to_string(), true)));

    let foobar_itf = json!({
        "tag": "FooBar",
        "value": { "#tup": ["hello", { "#bigint": "42" }, true] },
    });
    let foobar = itf::from_value::<Enum>(foobar_itf).unwrap();
    assert_eq!(foobar, Enum::FooBar("hello".to_string(), 42.into(), true));
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(tag = "tag", content = "value")]
enum EnumRecords {
    None,
    OptionA { x: String },
    OptionB { x: String, y: bool },
}

#[test]
fn test_deserialize_record_enum() {
    let option_a_itf = json!({
        "tag": "OptionA",
        "value": {"x": "hello"},
    });

    let a = itf::from_value::<EnumRecords>(option_a_itf).unwrap();
    assert_eq!(
        a,
        EnumRecords::OptionA {
            x: "hello".to_string()
        }
    );

    let option_b_itf = json!({
        "tag": "OptionB",
        "value": {"x": "hello", "y": true},
    });
    let b = itf::from_value::<EnumRecords>(option_b_itf).unwrap();
    assert_eq!(
        b,
        EnumRecords::OptionB {
            x: "hello".to_string(),
            y: true
        }
    );

    let option_none_itf = json!({
        "tag": "None",
        "value": {},
    });
    let no_args = itf::from_value::<EnumRecords>(option_none_itf).unwrap();
    assert_eq!(no_args, EnumRecords::None);
}
