use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_either::SingleOrVec;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ModKind {
    #[serde(rename = "package")]
    PACKAGE,

    #[serde(rename = "metapackage")]
    METAPACKAGE,

    #[serde(rename = "dlc")]
    DLC,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Dependency {
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InstallConfig {
    pub find: Option<String>,
    pub file: Option<String>,
    pub install_to: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetKANSchema {
    pub spec_version: String,
    pub name: Option<String>,
    pub identifier: Option<String>,
    pub kind: Option<ModKind>,

    #[serde(rename = "abstract")]
    pub short_description: Option<String>,

    pub description: Option<String>,
    pub comment: Option<String>,
    pub author: Option<SingleOrVec<String>>,
    pub download: Option<String>,
    pub license: Option<SingleOrVec<String>>,

    #[serde(rename = "$kref")]
    pub kref: Option<String>,

    #[serde(rename = "$vref")]
    pub vref: Option<String>,

    pub resources: Option<HashMap<String, String>>,
    pub tags: Option<Vec<String>>,

    pub depends: Option<Vec<Dependency>>,
    pub conflicts: Option<Vec<Dependency>>,

    pub install: Option<Vec<InstallConfig>>,
}
