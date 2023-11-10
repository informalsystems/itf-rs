use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use num_bigint::BigInt;
use serde::{de::DeserializeOwned, Deserialize};

pub type ItfMap<K, V> = Itf<HashMap<K, V>>;
pub type ItfSet<T> = Itf<HashSet<T>>;
pub type ItfTuple<T> = Itf<T>;
pub type ItfBigInt = Itf<BigInt>;
pub type ItfInt = i64;
pub type ItfBool = bool;
pub type ItfString = String;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Itf<T>(T);

impl<T> fmt::Debug for Itf<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> fmt::Display for Itf<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Itf<T> {
    pub fn value(self) -> T {
        self.0
    }
}

impl<T> Deref for Itf<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Itf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'de, T> Deserialize<'de> for Itf<HashSet<T>>
where
    T: Eq + Hash + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct Set<T> {
            #[serde(rename = "#set")]
            set: Vec<T>,
        }

        let set = Set::<T>::deserialize(deserializer)?;
        Ok(Self(set.set.into_iter().collect()))
    }
}

impl<'de, K, V> Deserialize<'de> for Itf<HashMap<K, V>>
where
    K: Eq + Hash + DeserializeOwned,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct Map<K, V> {
            #[serde(rename = "#map")]
            elements: Vec<(K, V)>,
        }

        let map = Map::<K, V>::deserialize(deserializer)?;
        Ok(Self(map.elements.into_iter().collect()))
    }
}

impl<'de> Deserialize<'de> for Itf<BigInt> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct BI {
            #[serde(rename = "#bigint", with = "crate::util::serde::display_from_str")]
            value: num_bigint::BigInt,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum IntOrBigInt {
            Int(i64),
            BigInt(BI),
        }

        IntOrBigInt::deserialize(deserializer)
            .map(|ib| match ib {
                IntOrBigInt::Int(n) => BigInt::from(n),
                IntOrBigInt::BigInt(b) => b.value,
            })
            .map(Itf)
    }
}

#[derive(Deserialize)]
struct Tup {
    #[serde(rename = "#tup")]
    elements: Vec<serde_json::Value>,
}

macro_rules! deserialize_itf_tuple {
    ($len:literal, $($n:literal $ty:ident)+) => {
        impl<'de, $($ty ,)+> Deserialize<'de> for Itf<($($ty ,)+)>
        where
            $($ty: DeserializeOwned,)+
        {
            #[allow(non_snake_case)]
            fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
            where
                De: serde::Deserializer<'de>,
            {
                let mut elements = Tup::deserialize(deserializer).map(|t| t.elements)?;

                if elements.len() != $len {
                    return Err(serde::de::Error::custom(format_args!(
                        "expected tuple with {} elements but found {}", $len, elements.len()
                    )));
                }

                $(
                    let $ty: $ty = serde_json::from_value(std::mem::take(&mut elements[$n]))
                        .map_err(|e| serde::de::Error::custom(e))?;
                )+

                Ok(Itf(($($ty,)+)))
            }
        }
    };
}

deserialize_itf_tuple!(2,  0 A 1 B);
deserialize_itf_tuple!(3,  0 A 1 B 2 C);
deserialize_itf_tuple!(4,  0 A 1 B 2 C 3 D);
deserialize_itf_tuple!(5,  0 A 1 B 2 C 3 D 4 E);
deserialize_itf_tuple!(6,  0 A 1 B 2 C 3 D 4 E 5 F);
deserialize_itf_tuple!(7,  0 A 1 B 2 C 3 D 4 E 5 F 6 G);
// deserialize_itf_tuple!(8,  0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H);
// deserialize_itf_tuple!(9,  0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I);
// deserialize_itf_tuple!(10, 0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I 9 J);
// deserialize_itf_tuple!(11, 0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I 9 J 10 K);
// deserialize_itf_tuple!(12, 0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I 9 J 10 K 11 L);

impl<T> From<T> for ItfBigInt
where
    BigInt: From<T>,
{
    fn from(t: T) -> Self {
        Itf(BigInt::from(t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_set() {
        let json = json!({
            "#set": [1, 2, 3, 4]
        });

        let set: ItfSet<ItfInt> = serde_json::from_value(json).unwrap();
        let elems = [1_i64, 2, 3, 4].into_iter().collect::<HashSet<_>>();

        assert_eq!(set.0, elems);
    }

    #[test]
    fn deserialize_map() {
        let json = json!({
            "#map": [["hello", 1], ["world", 2]]
        });

        let set: ItfMap<ItfString, ItfInt> = serde_json::from_value(json).unwrap();
        let elems = [("hello".to_string(), 1), ("world".to_string(), 2)]
            .into_iter()
            .collect::<HashMap<_, _>>();

        assert_eq!(set.0, elems);
    }

    #[test]
    fn deserialize_bigint_int() {
        let json = json!(1024);

        let bigint: ItfBigInt = serde_json::from_value(json).unwrap();
        assert_eq!(bigint.0, BigInt::from(1024));
    }

    #[test]
    fn deserialize_bigint() {
        let json = json!({
            "#bigint": "1234567891011121314151617181920"
        });

        let bigint: ItfBigInt = serde_json::from_value(json).unwrap();
        assert_eq!(bigint.0, "1234567891011121314151617181920".parse().unwrap());
    }

    #[test]
    #[should_panic(expected = "expected tuple with 3 elements but found 2")]
    fn deserialize_tuple_wrong_cardinality() {
        let json = json!({
            "#tup": [
                { "#bigint": "1234567891011121314151617181920" },
                1234,
            ]
        });

        let _tuple: ItfTuple<(ItfBigInt, ItfInt, ItfString)> =
            serde_json::from_value(json).unwrap();
    }

    #[test]
    fn deserialize_tuple_2() {
        let json = json!({
            "#tup": [
                { "#bigint": "1234567891011121314151617181920" },
                1234,
            ]
        });

        let mut tuple: ItfTuple<(ItfBigInt, ItfInt)> = serde_json::from_value(json).unwrap();

        assert_eq!(
            tuple.deref().0,
            Itf("1234567891011121314151617181920".parse().unwrap()),
        );

        assert_eq!(tuple.deref().1, 1234);
        assert_eq!(tuple.deref_mut().1, 1234);
    }

    #[test]
    fn deserialize_tuple3() {
        let json = json!({
            "#tup": [
                { "#bigint": "1234567891011121314151617181920" },
                1234,
                "Hello world",
            ]
        });

        let tuple: ItfTuple<(ItfBigInt, ItfInt, ItfString)> = serde_json::from_value(json).unwrap();
        let tuple = tuple.value();

        assert_eq!(
            tuple,
            (
                Itf("1234567891011121314151617181920".parse().unwrap()),
                1234,
                "Hello world".to_string(),
            )
        );
    }

    #[test]
    fn deserialize_tuple4() {
        let json = json!({
            "#tup": [
                { "#bigint": "1234567891011121314151617181920" },
                1234,
                "Hello world",
                true
            ]
        });

        let tuple: ItfTuple<(ItfBigInt, ItfInt, ItfString, ItfBool)> =
            serde_json::from_value(json).unwrap();

        assert_eq!(
            tuple.0,
            (
                Itf("1234567891011121314151617181920".parse().unwrap()),
                1234,
                "Hello world".to_string(),
                true
            )
        );
    }

    #[test]
    fn deserialize_tuple5() {
        let json = json!({
            "#tup": [
                { "#bigint": "1234567891011121314151617181920" },
                1234,
                "Hello world",
                true,
                { "#set": [1, 2, 3] }
            ]
        });

        let tuple: ItfTuple<(ItfBigInt, ItfInt, ItfString, ItfBool, ItfSet<ItfInt>)> =
            serde_json::from_value(json).unwrap();

        assert_eq!(
            tuple.0,
            (
                Itf("1234567891011121314151617181920".parse().unwrap()),
                1234,
                "Hello world".to_string(),
                true,
                Itf([1, 2, 3].into_iter().collect())
            )
        );
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn deserialize_tuple6() {
        let json = json!({
            "#tup": [
                { "#bigint": "1234567891011121314151617181920" },
                1234,
                "Hello world",
                true,
                { "#set": [1, 2, 3] },
                { "#map": [[1, true], [2, false], [3, true]] }
            ]
        });

        let tuple: ItfTuple<(
            ItfBigInt,
            ItfInt,
            ItfString,
            ItfBool,
            ItfSet<ItfInt>,
            ItfMap<ItfInt, ItfBool>,
        )> = serde_json::from_value(json).unwrap();

        assert_eq!(
            tuple.0,
            (
                Itf("1234567891011121314151617181920".parse().unwrap()),
                1234,
                "Hello world".to_string(),
                true,
                Itf([1, 2, 3].into_iter().collect()),
                Itf([(1, true), (2, false), (3, true)].into_iter().collect())
            )
        );
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn deserialize_tuple7() {
        let json = json!({
            "#tup": [
                { "#bigint": "1234567891011121314151617181920" },
                1234,
                "Hello world",
                true,
                { "#set": [ 1, 2, 3 ] },
                { "#map": [ [1, true], [2, false], [3, true] ] },
                { "#tup": [ { "#bigint": "1" }, "hello"] },
            ]
        });

        let tuple: ItfTuple<(
            ItfBigInt,
            ItfInt,
            ItfString,
            ItfBool,
            ItfSet<ItfInt>,
            ItfMap<ItfInt, ItfBool>,
            ItfTuple<(ItfBigInt, ItfString)>,
        )> = serde_json::from_value(json).unwrap();

        assert_eq!(
            tuple.0,
            (
                Itf("1234567891011121314151617181920".parse().unwrap()),
                1234,
                "Hello world".to_string(),
                true,
                Itf([1, 2, 3].into_iter().collect()),
                Itf([(1, true), (2, false), (3, true)].into_iter().collect()),
                Itf((Itf(BigInt::from(1)), "hello".to_string()))
            )
        );
    }

    #[test]
    fn display() {
        let s = "1234567891011121314151617181920";
        let itf: ItfBigInt = Itf(s.parse().unwrap());
        assert_eq!(format!("{}", itf), s.to_string());
    }
}
