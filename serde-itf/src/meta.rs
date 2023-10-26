use std::collections::BTreeMap;

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
    pub var_types: BTreeMap<String, String>,

    #[serde(default)]
    pub timestamp: Option<u64>,

    #[serde(flatten)]
    pub other: BTreeMap<String, String>,
}
