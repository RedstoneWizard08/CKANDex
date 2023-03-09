use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::netkan::{Dependency, InstallConfig};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FrozenSchema {
    pub spec_version: String,
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub ksp_version: Option<String>,

    #[serde(rename = "abstract")]
    pub short_description: Option<String>,

    #[serde(rename = "$kref")]
    pub kref: Option<String>,
    
    pub resources: Option<HashMap<String, String>>,
    pub license: Option<String>,
    pub tags: Option<Vec<String>>,
    pub depends: Option<Vec<Dependency>>,
    pub conflicts: Option<Vec<Dependency>>,
    pub install: Option<Vec<InstallConfig>>,

    #[serde(rename = "replaced-by")]
    pub replaced_by: Option<Dependency>,
}
