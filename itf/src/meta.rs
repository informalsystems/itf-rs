use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceMeta {
    #[serde(default)]
    pub format: Option<String>,

    #[serde(rename = "format-description")]
    pub format_description: Option<String>,

    #[serde(default)]
    pub source: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default, rename = "varTypes")]
    pub var_types: HashMap<String, String>,

    #[serde(default)]
    pub timestamp: Option<u64>,

    #[serde(flatten)]
    pub other: HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateMeta {
    #[serde(default)]
    pub index: Option<u64>,

    #[serde(flatten)]
    pub other: HashMap<String, String>,
}
