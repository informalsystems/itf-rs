use std::collections::HashMap;

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

    assert_eq!(
        itf::value::BigInt::new(-99),
        itf::from_value(itf.clone()).unwrap()
    );
    assert_eq!(
        itf::value::Value::BigInt(itf::value::BigInt::new(-99)),
        itf::from_value(itf.clone()).unwrap()
    );
    assert_eq!(-99, itf::from_value::<i64>(itf.clone()).unwrap());
    assert_eq!(
        num_bigint::BigInt::from(-99),
        itf::from_value(itf.clone()).unwrap()
    );

    assert!(itf::from_value::<u64>(itf.clone()).is_err());
}

#[test]
fn test_bigint_deser() {
    let itf = serde_json::json!({"#bigint": "-99"});

    assert_eq!(
        itf::value::BigInt::new(-99),
        itf::from_value(itf.clone()).unwrap()
    );
    assert_eq!(
        itf::value::Value::BigInt(itf::value::BigInt::new(-99)),
        itf::from_value(itf.clone()).unwrap()
    );
    assert_eq!(-99, itf::from_value::<i64>(itf.clone()).unwrap());
    assert_eq!(
        num_bigint::BigInt::from(-99),
        itf::from_value(itf.clone()).unwrap()
    );

    assert!(itf::from_value::<u64>(itf.clone()).is_err());
}

#[test]
fn test_biguint_deser() {
    let itf = serde_json::json!({"#bigint": "99"});

    assert_eq!(
        itf::value::BigInt::new(99),
        itf::from_value(itf.clone()).unwrap()
    );
    assert_eq!(
        itf::value::Value::BigInt(itf::value::BigInt::new(99)),
        itf::from_value(itf.clone()).unwrap()
    );
    assert_eq!(99, itf::from_value::<i64>(itf.clone()).unwrap());
    assert_eq!(99, itf::from_value::<u64>(itf.clone()).unwrap());
    assert_eq!(
        num_bigint::BigInt::from(99),
        itf::from_value(itf.clone()).unwrap()
    );

    assert!(itf::from_value::<num_bigint::BigUint>(itf.clone()).is_err());
}
