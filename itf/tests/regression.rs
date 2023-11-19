#[test]
fn test_tuple() {
    let itf = r##"{
        "#tup" : [1, 2, 3]
    }"##;

    let _: [u8; 3] = itf::from_str(itf).unwrap();
    let _: (u8, u8, u8) = itf::from_str(itf).unwrap();
    let _: Vec<u8> = itf::from_str(itf).unwrap();
    let _: std::collections::HashSet<u8> = itf::from_str(itf).unwrap();
    let _: std::collections::BTreeSet<u8> = itf::from_str(itf).unwrap();
}

#[test]
fn test_set() {
    let itf = r##"{
        "#set" : [1, 2, 3]
    }"##;

    let _: [u8; 3] = itf::from_str(itf).unwrap();
    let _: (u8, u8, u8) = itf::from_str(itf).unwrap();
    let _: Vec<u8> = itf::from_str(itf).unwrap();
    let _: std::collections::HashSet<u8> = itf::from_str(itf).unwrap();
    let _: std::collections::BTreeSet<u8> = itf::from_str(itf).unwrap();
    let _: serde_json::Value = itf::from_str(itf).unwrap();
}

#[test]
fn test_num_bigint() {
    let itf = r##"[-1, [99]]"##;

    assert_eq!(itf::value::BigInt::new(-99), itf::from_str(itf).unwrap());
    assert_eq!(
        itf::value::Value::BigInt(itf::value::BigInt::new(-99)),
        itf::from_str(itf).unwrap()
    );
    assert_eq!(-99, itf::from_str::<i64>(itf).unwrap());
    assert_eq!(num_bigint::BigInt::from(-99), itf::from_str(itf).unwrap());

    assert!(itf::from_str::<u64>(itf).is_err());
}

#[test]
fn test_bigint_deser() {
    let itf = r##"{
        "#bigint": "-99"
    }"##;

    assert_eq!(itf::value::BigInt::new(-99), itf::from_str(itf).unwrap());
    assert_eq!(
        itf::value::Value::BigInt(itf::value::BigInt::new(-99)),
        itf::from_str(itf).unwrap()
    );
    assert_eq!(-99, itf::from_str::<i64>(itf).unwrap());
    assert_eq!(num_bigint::BigInt::from(-99), itf::from_str(itf).unwrap());

    assert!(itf::from_str::<u64>(itf).is_err());
}

#[test]
fn test_biguint_deser() {
    let itf = r##"{
        "#bigint": "99"
    }"##;

    assert_eq!(itf::value::BigInt::new(99), itf::from_str(itf).unwrap());
    assert_eq!(
        itf::value::Value::BigInt(itf::value::BigInt::new(99)),
        itf::from_str(itf).unwrap()
    );
    assert_eq!(99, itf::from_str::<i64>(itf).unwrap());
    assert_eq!(99, itf::from_str::<u64>(itf).unwrap());
    assert_eq!(num_bigint::BigInt::from(99), itf::from_str(itf).unwrap());

    assert!(itf::from_str::<num_bigint::BigUint>(itf).is_err());
}
