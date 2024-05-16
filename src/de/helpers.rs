use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

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

/// Helper for `serde` to deserialize types isomorphic to [`std::option::Option`].
///
/// To be used in conjunction with [`As`].
///
/// ## Example
///
/// ```rust
/// use serde::Deserialize;
/// use serde_json::json;
///
/// use itf::de::{self, As};
///
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct FooOption {
///     #[serde(with = "As::<de::Option::<_>>")]
///     foo: Option<u64>,
/// }
///
/// let some_itf = json!({
///     "foo": {
///         "tag": "Some",
///         "value": 42,
///     }
/// });
///
/// let some_foo = itf::from_value::<FooOption>(some_itf).unwrap();
/// assert_eq!(some_foo, FooOption { foo: Some(42) });
///
/// let none_itf = json!({
///     "foo": {
///         "tag": "None",
///         "value": {},
///     }
/// });
///
/// let none_foo = itf::from_value::<FooOption>(none_itf).unwrap();
/// assert_eq!(none_foo, FooOption { foo: None });
/// ```
pub type Option<T> = serde_with::FromInto<QuintOption<T>>;

/// A type isomorphic to [`std::option::Option`].
///
/// Can either be used directly or with [`As`] together with [`Option`].
#[derive(
    Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(tag = "tag", content = "value")]
pub enum QuintOption<T> {
    #[default]
    None,
    Some(T),
}

impl<T> From<std::option::Option<T>> for QuintOption<T> {
    fn from(opt: std::option::Option<T>) -> Self {
        match opt {
            Some(value) => QuintOption::Some(value),
            None => QuintOption::None,
        }
    }
}

impl<T> From<QuintOption<T>> for std::option::Option<T> {
    fn from(opt: QuintOption<T>) -> Self {
        match opt {
            QuintOption::Some(value) => Some(value),
            QuintOption::None => None,
        }
    }
}

/// Helper for `serde` to deserialize types isomorphic to [`std::result::Result`].
///
/// To be used in conjunction with [`As`].
///
/// ## Example
///
/// ```rust
/// use serde::Deserialize;
/// use serde_json::json;
///
/// use itf::de::{self, As};
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct FooResult {
///     #[serde(with = "As::<de::Result::<_, _>>")]
///     foo: Result<u64, u64>,
/// }
///
/// let ok_itf = json!({
///     "foo": {
///         "tag": "Ok",
///         "value": 42,
///     }
/// });
///
/// let ok = itf::from_value::<FooResult>(ok_itf).unwrap();
/// assert_eq!(ok.foo, Ok(42));
///
/// let err_itf = json!({
///     "foo": {
///         "tag": "Err",
///         "value": 42,
///     }
/// });
///
/// let err = itf::from_value::<FooResult>(err_itf).unwrap();
/// assert_eq!(err.foo, Err(42));
/// ```
pub type Result<T, E> = serde_with::FromInto<QuintResult<T, E>>;

/// A type isomorphic to [`std::result::Result`].
///
/// Can either be used directly or with [`As`] together with [`Result`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "tag", content = "value")]
pub enum QuintResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> From<std::result::Result<T, E>> for QuintResult<T, E> {
    fn from(opt: std::result::Result<T, E>) -> Self {
        match opt {
            Ok(value) => QuintResult::Ok(value),
            Err(e) => QuintResult::Err(e),
        }
    }
}

impl<T, E> From<QuintResult<T, E>> for std::result::Result<T, E> {
    fn from(opt: QuintResult<T, E>) -> Self {
        match opt {
            QuintResult::Ok(value) => Ok(value),
            QuintResult::Err(e) => Err(e),
        }
    }
}
