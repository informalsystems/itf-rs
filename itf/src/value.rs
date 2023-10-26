use serde::{Deserialize, Serialize};

mod bigint;
mod map;
mod set;
mod tuple;
mod unserializable;

pub use bigint::BigInt;
pub use map::Map;
pub use set::Set;
pub use tuple::Tuple;
pub use unserializable::Unserializable;

// TODO: Display

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// A JSON Boolean: either `false` or `true`.
    Bool(bool),

    /// A JSON number literal, e.g., `42`.
    Number(i64),

    /// A JSON string literal, e.g., `"hello"`.
    ///
    /// TLA+ strings are written as strings in this format.
    String(String),

    /// A big integer of the following form: `{ "#bigint": "[-][0-9]+" }`.
    ///
    /// We are using this format, as many JSON parsers impose limits
    /// on integer values, see RFC7159.
    ///
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
    ///   `p[0]` is the map key and `p[1]` is the map value.
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
