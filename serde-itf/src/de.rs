use std::fmt;
use std::marker::PhantomData;

use num_traits::ToPrimitive;
use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::{
    DeserializeOwned, DeserializeSeed, Deserializer, EnumAccess, Error as SerdeError,
    IntoDeserializer, VariantAccess, Visitor,
};

use crate::bigint::BigInt;
use crate::value::{Type, Value};

pub fn decode_value<T>(value: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(value.into_deserializer())
}

#[derive(Debug)]
pub enum Error {
    Custom(String),
    TypeMismatch(Type, Type),
    BigInt(BigInt, &'static str),
    UnsupportedType(&'static str),
    Number(i64, &'static str),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Custom(msg) => msg.fmt(f),

            Error::TypeMismatch(expected, actual) => {
                write!(f, "type mismatch: expected {expected:?}, found {actual:?}")
            }

            Error::BigInt(value, expected) => {
                write!(f, "cannot convert {value} to {expected}")
            }

            Error::Number(value, expected) => {
                write!(f, "cannot convert {value} to {expected}")
            }

            Error::UnsupportedType(ty) => write!(f, "unsupported type: {ty}"),
        }
    }
}

impl SerdeError for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl<'de> IntoDeserializer<'de, Error> for Value {
    type Deserializer = ValueDeserializer;

    fn into_deserializer(self) -> Self::Deserializer {
        ValueDeserializer { input: self }
    }
}

pub struct ValueDeserializer {
    input: Value,
}

impl<'de> Deserializer<'de> for ValueDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Bool(v) => visitor.visit_bool(v),
            Value::Number(v) => visitor.visit_i64(v),
            Value::String(v) => visitor.visit_string(v),
            Value::BigInt(v) => visitor.visit_i64(v.to_i64().unwrap()),
            Value::List(v) => visitor.visit_seq(SeqDeserializer::new(v.into_iter())),
            Value::Tuple(v) => visitor.visit_seq(SeqDeserializer::new(v.into_iter())),
            Value::Set(v) => visitor.visit_seq(SeqDeserializer::new(v.into_iter())),
            Value::Record(v) => visitor.visit_map(MapDeserializer::new(v.into_iter())),
            Value::Map(v) => visitor.visit_map(MapDeserializer::new(v.into_iter())),
            Value::Unserializable(_) => Err(Error::UnsupportedType("unserializable")),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Bool(v) => visitor.visit_bool(v),
            value => Err(Error::TypeMismatch(Type::Bool, value.value_type())),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Number(v) => visitor.visit_i8(v.to_i8().ok_or(Error::Number(v, "i8"))?),
            Value::BigInt(v) => visitor.visit_i8(v.to_i8().ok_or(Error::BigInt(v, "i8"))?),
            value => Err(Error::TypeMismatch(Type::BigInt, value.value_type())),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Number(v) => visitor.visit_i16(v.to_i16().ok_or(Error::Number(v, "i16"))?),
            Value::BigInt(v) => visitor.visit_i16(v.to_i16().ok_or(Error::BigInt(v, "i16"))?),
            value => Err(Error::TypeMismatch(Type::BigInt, value.value_type())),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Number(v) => visitor.visit_i32(v.to_i32().ok_or(Error::Number(v, "i32"))?),
            Value::BigInt(v) => visitor.visit_i32(v.to_i32().ok_or(Error::BigInt(v, "i32"))?),
            value => Err(Error::TypeMismatch(Type::BigInt, value.value_type())),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Number(v) => visitor.visit_i64(v.to_i64().ok_or(Error::Number(v, "i64"))?),
            Value::BigInt(v) => visitor.visit_i64(v.to_i64().ok_or(Error::BigInt(v, "i64"))?),
            value => Err(Error::TypeMismatch(Type::BigInt, value.value_type())),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Number(v) => visitor.visit_u8(v.to_u8().ok_or(Error::Number(v, "u8"))?),
            Value::BigInt(v) => visitor.visit_u8(v.to_u8().ok_or(Error::BigInt(v, "u8"))?),
            value => Err(Error::TypeMismatch(Type::BigInt, value.value_type())),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Number(v) => visitor.visit_u16(v.to_u16().ok_or(Error::Number(v, "u16"))?),
            Value::BigInt(v) => visitor.visit_u16(v.to_u16().ok_or(Error::BigInt(v, "u16"))?),
            value => Err(Error::TypeMismatch(Type::BigInt, value.value_type())),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Number(v) => visitor.visit_u32(v.to_u32().ok_or(Error::Number(v, "u32"))?),
            Value::BigInt(v) => visitor.visit_u32(v.to_u32().ok_or(Error::BigInt(v, "u32"))?),
            value => Err(Error::TypeMismatch(Type::BigInt, value.value_type())),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Number(v) => visitor.visit_u64(v.to_u64().ok_or(Error::Number(v, "u64"))?),
            Value::BigInt(v) => visitor.visit_u64(v.to_u64().ok_or(Error::BigInt(v, "u64"))?),
            value => Err(Error::TypeMismatch(Type::BigInt, value.value_type())),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Number(v) => visitor.visit_f32(v.to_f32().ok_or(Error::Number(v, "f32"))?),
            Value::BigInt(v) => visitor.visit_f32(v.to_f32().ok_or(Error::BigInt(v, "f32"))?),
            value => Err(Error::TypeMismatch(Type::Float, value.value_type())),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Number(v) => visitor.visit_f64(v.to_f64().ok_or(Error::Number(v, "f64"))?),
            Value::BigInt(v) => visitor.visit_f64(v.to_f64().ok_or(Error::BigInt(v, "f64"))?),
            value => Err(Error::TypeMismatch(Type::Float, value.value_type())),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::String(v) if v.len() == 1 => visitor.visit_char(v.chars().next().unwrap()),
            value => Err(Error::TypeMismatch(Type::Char, value.value_type())),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::String(v) => visitor.visit_str(&v),
            value => Err(Error::TypeMismatch(Type::String, value.value_type())),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("bytes"))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("unit"))
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::List(v) => visitor.visit_seq(SeqDeserializer::new(v.into_iter())),
            Value::Tuple(v) => visitor.visit_seq(SeqDeserializer::new(v.into_iter())),
            value => Err(Error::TypeMismatch(Type::List, value.value_type())),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Tuple(v) if v.len() == len => {
                visitor.visit_seq(SeqDeserializer::new(v.into_iter()))
            }
            value => Err(Error::TypeMismatch(Type::Tuple, value.value_type())),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Record(v) => visitor.visit_map(MapDeserializer::new(v.into_iter())),
            Value::Map(v) => visitor.visit_map(MapDeserializer::new(v.into_iter())),
            value => Err(Error::TypeMismatch(Type::Record, value.value_type())),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::String(v) => visitor.visit_enum(v.into_deserializer()),
            Value::Map(v) => visitor.visit_enum(Enum::new(Value::Map(v))),
            Value::Record(v) => visitor.visit_enum(Enum::new(Value::Record(v))),
            value => Err(Error::TypeMismatch(Type::Enum, value.value_type())),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct Enum<'de> {
    value: Value,
    marker: PhantomData<&'de ()>,
}

impl<'de> Enum<'de> {
    fn new(value: Value) -> Self {
        Enum {
            value,
            marker: PhantomData,
        }
    }
}

impl<'de> EnumAccess<'de> for Enum<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        dbg!(&self.value);
        let value = seed.deserialize(self.value.clone().into_deserializer())?;
        Ok((value, self))
    }
}

impl<'de> VariantAccess<'de> for Enum<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        match self.value {
            Value::String(_) => Ok(()),
            value => Err(Error::TypeMismatch(Type::String, value.value_type())),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.value.into_deserializer())
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        dbg!(_len);
        Deserializer::deserialize_seq(self.value.into_deserializer(), visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        dbg!(_fields);
        Deserializer::deserialize_map(self.value.into_deserializer(), visitor)
    }
}
