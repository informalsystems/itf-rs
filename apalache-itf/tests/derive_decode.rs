use std::collections::{HashMap, HashSet};

use apalache_itf::DecodeItfValue;
use num_bigint::BigInt;

#[test]
#[allow(dead_code)]
fn struct_with_named_fields() {
    #[derive(DecodeItfValue)]
    struct Named {
        foo: i64,
        bar: Box<Named>,
        baz: HashMap<BigInt, HashSet<String>>,
    }
}

#[test]
#[allow(dead_code)]
fn struct_with_unnamed_fields() {
    #[derive(DecodeItfValue)]
    struct Unnamed(i64, Box<Unnamed>, HashMap<BigInt, HashSet<String>>);
}

#[test]
#[allow(dead_code)]
fn enum_with_unit_variants() {
    #[derive(DecodeItfValue)]
    enum Unit {
        Foo,
        Bar,
        Baz,
    }
}

#[test]
#[allow(dead_code)]
fn enum_with_named_variants() {
    #[derive(DecodeItfValue)]
    enum Named {
        Foo {
            foo: i64,
            bar: bool,
        },
        Bar {
            bar: HashSet<i64>,
        },
        Baz {
            toto: (bool, i64, String),
            tata: Vec<Named>,
        },
    }
}

#[test]
#[allow(dead_code)]
fn enum_with_mixed_variants() {
    #[derive(DecodeItfValue)]
    enum Mixed {
        Foo { foo: i64, bar: bool },
        Bar,
    }
}
