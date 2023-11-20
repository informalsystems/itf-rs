use serde::Deserialize;

#[test]
fn test_tuple() {
    let itf = serde_json::json!({"#tup": [1, 2, 3]});

    let _: [u8; 3] = itf::from_value(itf.clone()).unwrap();
    let _: (u8, u8, u8) = itf::from_value(itf.clone()).unwrap();
    let _: Vec<u8> = itf::from_value(itf.clone()).unwrap();
    let _: std::collections::HashSet<u8> = itf::from_value(itf.clone()).unwrap();
    let _: std::collections::BTreeSet<u8> = itf::from_value(itf.clone()).unwrap();
}

#[test]
fn test_set() {
    let itf = serde_json::json!({"#set": [1, 2, 3]});

    let _: [u8; 3] = itf::from_value(itf.clone()).unwrap();
    let _: (u8, u8, u8) = itf::from_value(itf.clone()).unwrap();
    let _: Vec<u8> = itf::from_value(itf.clone()).unwrap();
    let _: std::collections::HashSet<u8> = itf::from_value(itf.clone()).unwrap();
    let _: std::collections::BTreeSet<u8> = itf::from_value(itf.clone()).unwrap();
    let _: serde_json::Value = itf::from_value(itf.clone()).unwrap();
}

#[test]
fn test_num_bigint() {
    let itf = serde_json::json!([-1, [99]]);

    // successful cases
    assert_eq!(
        num_bigint::BigInt::from(-99),
        itf::from_value::<num_bigint::BigInt>(itf.clone()).unwrap()
    );

    // unsuccessful cases
    assert!(itf::from_value::<i64>(itf.clone()).is_err());
    assert!(itf::from_value::<u64>(itf.clone()).is_err());
    assert!(itf::from_value::<itf::value::BigInt>(itf.clone()).is_err());
    assert!(!matches!(
        itf::from_value::<itf::Value>(itf.clone()).unwrap(),
        itf::Value::BigInt(_),
    ));
}

#[test]
fn test_bigint_deser() {
    let itf = serde_json::json!({"#bigint": "-99"});

    // successful cases
    assert_eq!(-99, itf::from_value::<i64>(itf.clone()).unwrap());
    assert_eq!(
        num_bigint::BigInt::from(-99),
        itf::from_value(itf.clone()).unwrap()
    );

    // unsuccessful cases
    assert!(itf::from_value::<u64>(itf.clone()).is_err());
    assert!(itf::from_value::<itf::value::BigInt>(itf.clone()).is_err());
    assert!(!matches!(
        itf::from_value::<itf::Value>(itf.clone()).unwrap(),
        itf::Value::BigInt(_),
    ));
}

#[test]
fn test_biguint_deser() {
    let itf = serde_json::json!({"#bigint": "99"});

    // successful cases
    assert_eq!(99, itf::from_value::<i64>(itf.clone()).unwrap());
    assert_eq!(99, itf::from_value::<u64>(itf.clone()).unwrap());
    assert_eq!(
        num_bigint::BigInt::from(99),
        itf::from_value(itf.clone()).unwrap()
    );

    // unsuccessful cases
    assert!(itf::from_value::<num_bigint::BigUint>(itf.clone()).is_err());
    assert!(itf::from_value::<itf::value::BigInt>(itf.clone()).is_err());
    assert!(!matches!(
        itf::from_value::<itf::Value>(itf.clone()).unwrap(),
        itf::Value::BigInt(_),
    ));
}

#[test]
fn test_itf_value_equivalent() {
    let itf = serde_json::json!({
        "bool": true,
        "number": -99,
        "str": "hello",
        "list": [1, 2, 3],
        "record": {"a": 1, "b": 2, "c": 3},
    });

    let value = serde_json::from_value::<itf::Value>(itf.clone()).unwrap();
    assert_eq!(value.clone(), itf::Value::deserialize(value).unwrap());
}

#[test]
#[should_panic]
fn test_itf_value_noneq() {
    // Deserialized Value loses the type information
    let itf = serde_json::json!({
        "bigint": {"#bigint": "-999"},
        "tuple": {"#tup": [1, 2, 3]},
        "set": {"#set": [1, 2, 3]},
        "map": {"#map": [["1", 3], ["2", 4]]},
    });

    let value = serde_json::from_value::<itf::Value>(itf.clone()).unwrap();
    assert_eq!(value.clone(), itf::Value::deserialize(value).unwrap());
}

#[test]
#[should_panic]
fn test_map_with_non_str_key() {
    // MapSerializer accepts only string keys
    let itf = serde_json::json!({
        "map": {"#map": [[1, 3], [2, 4]]},
    });

    let value = serde_json::from_value::<itf::Value>(itf).unwrap();
    itf::Value::deserialize(value).unwrap();
}
