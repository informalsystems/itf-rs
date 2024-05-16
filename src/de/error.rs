use std::fmt;

/// Error type for deserialization.
#[derive(Debug)]
pub enum Error {
    Custom(String),
    UnsupportedType(&'static str),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Custom(msg) => msg.fmt(f),
            Error::UnsupportedType(ty) => write!(f, "unsupported type: {ty}"),
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}
