pub mod serde {
    pub mod display_from_str {
        use std::{fmt::Display, str::FromStr};

        use serde::{de, Deserialize, Deserializer};

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
