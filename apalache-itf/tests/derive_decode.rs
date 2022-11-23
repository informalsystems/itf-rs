use std::collections::HashSet;

use apalache_itf::DecodeItfValue;

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

// #[test]
// #[allow(dead_code)]
// fn enum_with_mixed_variants() {
//     #[derive(DecodeItfValue)]
//     enum Mixed {
//         Foo { foo: i64, bar: bool },
//         Bar,
//     }
// }
