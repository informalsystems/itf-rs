use itertools::Itertools;
use num_bigint::BigInt;
use thiserror::Error;

use crate::value::Value;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("invalid type")]
    InvalidType,

    #[error("field not found: {0}")]
    FieldNotFound(&'static str),
}

pub trait DecodeItfValue
where
    Self: Sized,
{
    fn decode(value: Value) -> Result<Self, DecodeError>;
}

macro_rules! decode {
    ($ty:ty, $cons:pat, $x:expr) => {
        impl DecodeItfValue for $ty {
            #[allow(irrefutable_let_patterns)]
            fn decode(value: Value) -> Result<Self, DecodeError> {
                if let $cons = value {
                    Ok($x)
                } else {
                    Err(DecodeError::InvalidType)
                }
            }
        }
    };
}

// macro_rules! decode_tuple {
//     ($ty:ty) => {
//         impl<T> DecodeItfValue for $ty
//         where
//             T: DecodeItfValue,
//         {
//             fn decode(value: Value) -> Result<Self, DecodeError> {
//                 if let Value::Tuple(t) = value {
//                     t.elements
//                         .into_iter()
//                         .map(T::decode)
//                         .collect::<Result<Vec<_>, _>>()?
//                         .into_iter()
//                         .collect_tuple()
//                         .ok_or(DecodeError::InvalidType)
//                 } else {
//                     Err(DecodeError::InvalidType)
//                 }
//             }
//         }
//     };
// }

decode!(Value, v, v);
decode!(i64, Value::Int(n), n);
decode!(BigInt, Value::BigInt(n), n.into_bigint());
decode!(bool, Value::Boolean(n), n);
decode!(String, Value::String(n), n);

// decode_tuple!((T,));
// decode_tuple!((T, T));
// decode_tuple!((T, T, T));
// decode_tuple!((T, T, T, T));
// decode_tuple!((T, T, T, T, T));
// decode_tuple!((T, T, T, T, T, T));
// decode_tuple!((T, T, T, T, T, T, T));
// decode_tuple!((T, T, T, T, T, T, T, T));
// decode_tuple!((T, T, T, T, T, T, T, T, T));
// decode_tuple!((T, T, T, T, T, T, T, T, T, T));
// decode_tuple!((T, T, T, T, T, T, T, T, T, T, T));
// decode_tuple!((T, T, T, T, T, T, T, T, T, T, T, T));

impl<T> DecodeItfValue for Vec<T>
where
    T: DecodeItfValue,
{
    fn decode(value: Value) -> Result<Self, DecodeError> {
        if let Value::List(l) = value {
            l.into_iter().map(T::decode).try_collect()
        } else {
            Err(DecodeError::InvalidType)
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
            s.into_iter().map(T::decode).try_collect()
        } else {
            Err(DecodeError::InvalidType)
        }
    }
}

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
                .try_collect(),

            Value::Record(m) => m
                .into_iter()
                .map(|(k, v)| {
                    K::decode(Value::String(k)).and_then(|k| V::decode(v).map(|v| (k, v)))
                })
                .try_collect(),

            _ => Err(DecodeError::InvalidType),
        }
    }
}
