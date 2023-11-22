use num_bigint::BigInt;

pub use serde_with::{As, Same};

/// Helper for `serde` to deserialize a `BigInt` to
/// any type which implements `TryFrom<num_bigint::BigInt>`.
///
/// To be used in conjunction with [`As`].
///
/// ## Example
///
/// ```rust
/// use std::collections::HashMap;
///
/// use num_bigint::BigInt;
/// use serde::Deserialize;
///
/// use itf::Trace;
/// use itf::de::{As, Integer};
///
/// let json = serde_json::json!([
///     {
///         "_foo": {"#map": [[{"#bigint": "1"}, {"#bigint": "2"}]]},
///         "typ": "Foo",
///     },
///     {
///         "_bar": [[[{"#bigint": "1"}, {"#bigint": "2"}]]],
///         "typ": "Bar",
///     }
/// ]);
///
/// // Deserialize as `num_bigint::BigInt`
/// #[derive(Deserialize, Debug)]
/// #[serde(tag = "typ")]
/// enum FooBarBigInt {
///     Foo { _foo: HashMap<BigInt, BigInt> },
///     Bar { _bar: Vec<Vec<(BigInt, BigInt)>> },
/// }
/// itf::from_value::<Vec<FooBarBigInt>>(json.clone()).unwrap();
///
/// // Deserialize as `i64`
/// #[derive(Deserialize, Debug)]
/// #[serde(tag = "typ")]
/// enum FooBarInt {
///     // try to deserialize _foo as i64, instead of BigInt
///     Foo {
///         #[serde(with = "As::<HashMap<Integer, Integer>>")]
///         _foo: HashMap<i64, i64>,
///     },
///     Bar {
///         #[serde(with = "As::<Vec<Vec<(Integer, Integer)>>>")]
///         _bar: Vec<Vec<(i64, i64)>>,
///     },
/// }
/// itf::from_value::<Vec<FooBarInt>>(json.clone()).unwrap();
///
/// // Deserialize as a mix
/// #[derive(Deserialize, Debug)]
/// #[serde(tag = "typ")]
/// enum FooBarMixInt {
///     // try to deserialize _foo as i64, instead of BigInt
///     Foo {
///         #[serde(with = "As::<HashMap<Integer, Integer>>")]
///         _foo: HashMap<i64, BigInt>,
///     },
///     Bar {
///         #[serde(with = "As::<Vec<Vec<(Integer, Integer)>>>")]
///         _bar: Vec<Vec<(BigInt, u64)>>,
///     },
/// }
/// itf::from_value::<Vec<FooBarMixInt>>(json.clone()).unwrap();
/// ```
pub type Integer = serde_with::TryFromInto<BigInt>;
