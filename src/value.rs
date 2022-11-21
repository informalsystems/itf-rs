use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

macro_rules! try_from_value {
    ($ty:ty, $cons:pat, $x:expr) => {
        impl TryFrom<Value> for $ty {
            type Error = ();

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                if let $cons = value {
                    Ok($x)
                } else {
                    Err(())
                }
            }
        }
    };
}

macro_rules! try_tuple_from_value {
    ($ty:ty) => {
        impl<T> TryFrom<Value> for $ty
        where
            T: TryFrom<Value, Error = ()>,
        {
            type Error = ();

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                if let Value::Tuple(t) = value {
                    t.elements
                        .into_iter()
                        .map(T::try_from)
                        .collect::<Result<Vec<_>, ()>>()?
                        .into_iter()
                        .collect_tuple()
                        .ok_or(())
                } else {
                    Err(())
                }
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Int(i64),
    BigInt(i128),
    Boolean(bool),
    String(String),
    Map(Map),
    Set(Set),
    List(Vec<Value>),
    Tuple(Tuple),
    Record(HashMap<String, Value>),
    Unserializable(Unserializable),
}

try_from_value!(i64, Value::Int(n), n);
try_from_value!(i128, Value::BigInt(n), n);
try_from_value!(bool, Value::Boolean(n), n);
try_from_value!(String, Value::String(n), n);
try_from_value!(Vec<Value>, Value::List(n), n);

try_tuple_from_value!((T,));
try_tuple_from_value!((T, T));
try_tuple_from_value!((T, T, T));
try_tuple_from_value!((T, T, T, T));
try_tuple_from_value!((T, T, T, T, T));
try_tuple_from_value!((T, T, T, T, T, T));
try_tuple_from_value!((T, T, T, T, T, T, T));
try_tuple_from_value!((T, T, T, T, T, T, T, T));
try_tuple_from_value!((T, T, T, T, T, T, T, T, T));
try_tuple_from_value!((T, T, T, T, T, T, T, T, T, T));
try_tuple_from_value!((T, T, T, T, T, T, T, T, T, T, T));
try_tuple_from_value!((T, T, T, T, T, T, T, T, T, T, T, T));

impl<T> TryFrom<Value> for Vec<T>
where
    T: TryFrom<Value, Error = ()>,
{
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::List(l) = value {
            l.into_iter().map(T::try_from).try_collect()
        } else {
            Err(())
        }
    }
}

use std::hash::Hash;

impl<T> TryFrom<Value> for HashSet<T>
where
    T: Eq + Hash + TryFrom<Value, Error = ()>,
{
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Set(s) = value {
            s.into_iter().map(T::try_from).try_collect()
        } else {
            Err(())
        }
    }
}

impl<T> TryFrom<Value> for HashMap<String, T>
where
    T: TryFrom<Value, Error = ()>,
{
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Map(m) => m
                .map
                .into_iter()
                .map(|(k, v)| T::try_from(v).map(|v| (k, v)))
                .try_collect(),
            Value::Record(m) => m
                .into_iter()
                .map(|(k, v)| T::try_from(v).map(|v| (k, v)))
                .try_collect(),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unserializable {
    #[serde(rename = "#unserializable")]
    repr: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tuple {
    #[serde(rename = "#tup")]
    elements: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Map {
    #[serde(rename = "#map")]
    map: Vec<(String, Value)>,
}

impl Map {
    pub fn to_hashmap(self) -> HashMap<String, Value> {
        self.map.into_iter().collect()
    }
}

impl IntoIterator for Map {
    type Item = (String, Value);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Set {
    #[serde(rename = "#set")]
    set: Vec<Value>,
}

impl IntoIterator for Set {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}
