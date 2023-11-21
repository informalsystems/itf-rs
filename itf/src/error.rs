/// Error type for the library.
#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Decode(crate::de::Error),
}

impl From<serde_json::Error> for Error {
    fn from(v: serde_json::Error) -> Self {
        Self::Json(v)
    }
}

impl From<crate::de::Error> for Error {
    fn from(v: crate::de::Error) -> Self {
        Self::Decode(v)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::Decode(e) => write!(f, "decoding error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Json(e) => Some(e),
            Error::Decode(e) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}
