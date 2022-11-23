pub mod serde {
    pub mod display_from_str {
        use std::{fmt::Display, str::FromStr};

        use serde::{de, Deserialize, Deserializer, Serializer};

        pub fn serialize<S, T>(t: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
            T: Display,
        {
            serializer.serialize_str(t.to_string().as_str())
        }

        pub fn deserialize<'de, D, T, E>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
            T: FromStr<Err = E>,
            E: Display,
        {
            let s = String::deserialize(deserializer)?;
            FromStr::from_str(&s).map_err(de::Error::custom)
        }
    }
}
