use thiserror::Error;

use crate::value::Value;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("invalid type, expected '{0}'")]
    InvalidType(&'static str),

    #[error("field not found: {0}")]
    FieldNotFound(&'static str),

    #[error("unknown tag: '{0}'")]
    UnknownTag(&'static str),

    #[error("unknown variant: '{0}'")]
    UnknownVariant(String),
}

pub trait DecodeItfValue
where
    Self: Sized,
{
    fn decode(value: Value) -> Result<Self, DecodeError>;
}

macro_rules! decode {
    ($name:expr, $ty:ty, $cons:pat, $x:expr) => {
        impl DecodeItfValue for $ty {
            #[allow(irrefutable_let_patterns)]
            fn decode(value: Value) -> Result<Self, DecodeError> {
                if let $cons = value {
                    Ok($x)
                } else {
                    Err(DecodeError::InvalidType($name))
                }
            }
        }
    };
}

// FIXME: do this properly without cloning
macro_rules! decode_tuple {
    ($($ty:ident)+) => {
        impl<$($ty ,)+> DecodeItfValue for ($($ty ,)+)
        where
            $($ty: DecodeItfValue,)+
        {
            #[allow(unused_assignments, non_snake_case)]
            fn decode(value: Value) -> Result<Self, DecodeError> {
                if let Value::Tuple(t) = value {
                    let mut i = 0;
                    $(
                        let $ty = <$ty as DecodeItfValue>::decode(t.elements[i].clone())?;
                        i += 1;
                    )+
                    Ok(($($ty,)+))
                } else {
                    Err(DecodeError::InvalidType("tuple"))
                }
            }
        }
    };
}

decode!("value", Value, v, v);
decode!("int", i64, Value::Int(n), n);
decode!("boolean", bool, Value::Boolean(n), n);
decode!("string", String, Value::String(n), n);

impl DecodeItfValue for num_bigint::BigInt {
    fn decode(value: Value) -> Result<Self, DecodeError> {
        match value {
            Value::BigInt(n) => Ok(n.into_bigint()),
            Value::Int(n) => Ok(num_bigint::BigInt::from(n)),
            _ => Err(DecodeError::InvalidType("bigint")),
        }
    }
}

decode_tuple!(A B);
decode_tuple!(A B C);
decode_tuple!(A B C D);
decode_tuple!(A B C D E);
decode_tuple!(A B C D E F);
decode_tuple!(A B C D E F G);
decode_tuple!(A B C D E F G H);
decode_tuple!(A B C D E F G H I);
decode_tuple!(A B C D E F G H I J);
decode_tuple!(A B C D E F G H I J K);
decode_tuple!(A B C D E F G H I J K L);

impl<T> DecodeItfValue for Vec<T>
where
    T: DecodeItfValue,
{
    fn decode(value: Value) -> Result<Self, DecodeError> {
        if let Value::List(l) = value {
            l.into_iter().map(T::decode).collect::<Result<_, _>>()
        } else {
            Err(DecodeError::InvalidType("list"))
        }
    }
}

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

impl<T> DecodeItfValue for HashSet<T>
where
    T: Eq + Hash + DecodeItfValue,
{
    fn decode(value: Value) -> Result<Self, DecodeError> {
        if let Value::Set(s) = value {
            s.into_iter().map(T::decode).collect::<Result<_, _>>()
        } else {
            Err(DecodeError::InvalidType("set"))
        }
    }
}

// TODO: Specialize for the case where K = String
impl<K, V> DecodeItfValue for HashMap<K, V>
where
    K: DecodeItfValue + Hash + Eq,
    V: DecodeItfValue,
{
    fn decode(value: Value) -> Result<Self, DecodeError> {
        match value {
            Value::Map(m) => m
                .map
                .into_iter()
                .map(|(k, v)| {
                    K::decode(Value::String(k)).and_then(|k| V::decode(v).map(|v| (k, v)))
                })
                .collect::<Result<_, _>>(),

            Value::Record(m) => m
                .into_iter()
                .map(|(k, v)| {
                    K::decode(Value::String(k)).and_then(|k| V::decode(v).map(|v| (k, v)))
                })
                .collect::<Result<_, _>>(),

            _ => Err(DecodeError::InvalidType("map")),
        }
    }
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn list() {
        let value: Value = serde_json::from_value(json!([1, 2, 3])).unwrap();
        let list = <Vec<i64>>::decode(value).unwrap();

        assert_eq!(list, vec![1, 2, 3]);
    }

    #[test]
    fn tuple() {
        let value: Value = serde_json::from_value(json!({
            "#tup": [1, "Hello", true]
        }))
        .unwrap();

        let (a, b, c) = <(i64, String, bool)>::decode(value).unwrap();
        assert_eq!(a, 1);
        assert_eq!(b.as_str(), "Hello");
        assert_eq!(c, true);
    }
}
