use serde::{Deserialize, Serialize};

use crate::bigint::BigInt;
use crate::map::Map;
use crate::set::Set;
use crate::tuple::Tuple;
use crate::unserializable::Unserializable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// A JSON Boolean: either `false` or `true`.
    Bool(bool),

    /// A JSON string literal, e.g., `"hello"`.
    ///
    /// TLA+ strings are written as strings in this format.
    String(String),

    /// A JSON number literal, e.g., `42`.
    Number(i64),

    /// A big integer of the following form: { "#bigint": "[-][0-9]+" }.
    ///
    /// We are using this format, as many JSON parsers impose limits
    /// on integer values, see RFC7159.
    /// Big and small integers must be written in this format.
    BigInt(BigInt),

    /// A list of the form `[ <expr>, ..., <expr> ]`.
    ///
    /// A list is just a JSON array.
    /// TLA+ sequences are written as lists in this format.
    List(Vec<Value>),

    /// A tuple of the form `{ "#tup": [ <expr>, ..., <expr> ] }`.
    ///
    /// There is no strict rule about when to use sequences or tuples.
    /// Apalache differentiates between tuples and sequences, and it may produce both forms of expressions.
    Tuple(Tuple<Value>),

    /// A set of the form `{ "#set": [ <expr>, ..., <expr> ] }`.
    ///
    /// A set is different from a list in that it does not assume any ordering of its elements.
    /// However, it is only a syntax form in our format.
    /// Apalache distinguishes between sets and lists and thus it will output sets in the set form.
    /// Other tools may interpret sets as lists.
    Set(Set<Value>),

    /// A map of the form `{ "#map": [ [ <expr>, <expr> ], ..., [ <expr>, <expr> ] ] }`.
    ///
    /// That is, a map holds a JSON array of two-element arrays.
    /// Each two-element array p is interpreted as follows:
    ///   p[0] is the map key and p[1] is the map value.
    ///
    /// Importantly, a key may be an arbitrary expression.
    /// It does not have to be a string or an integer.
    /// TLA+ functions are written as maps in this format.
    Map(Map<Value, Value>),

    /// A record of the form `{ "field1": <expr>, ..., "fieldN": <expr> }`.
    ///
    /// A record is just a JSON object. Field names should not start with `#` and
    /// hence should not pose any collision with other constructs.
    /// TLA+ records are written as records in this format.
    Record(Map<String, Value>),

    /// An expression that cannot be serialized: `{ "#unserializable": "<string representation>" }`.
    ///
    /// For instance, the set of all integers is represented with `{ "#unserializable": "Int" }`.
    /// This should be a very rare expression, which should not occur in normal traces.
    /// Usually, it indicates some form of an error.
    Unserializable(Unserializable),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Bool,
    Number,
    String,
    BigInt,
    List,
    Record,
    Tuple,
    Set,
    Map,
    Unserializable,
    Float,
    Char,
    Enum,
}

// TODO: Display

impl Value {
    pub fn value_type(&self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::Number(_) => Type::Number,
            Value::String(_) => Type::String,
            Value::BigInt(_) => Type::BigInt,
            Value::List(_) => Type::List,
            Value::Record(_) => Type::Record,
            Value::Tuple(_) => Type::Tuple,
            Value::Set(_) => Type::Set,
            Value::Map(_) => Type::Map,
            Value::Unserializable(_) => Type::Unserializable,
        }
    }
}
