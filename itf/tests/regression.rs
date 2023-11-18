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
}
