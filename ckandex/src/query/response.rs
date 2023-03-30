use serde::{Deserialize, Serialize};

use crate::Mod;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub netkans: Vec<Mod>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub frozen: Vec<Mod>,
}

impl QueryResponse {
    pub fn first(&self) -> Option<Mod> {
        if !self.netkans.is_empty() {
            return self.netkans.get(0).cloned();
        }

        if !self.frozen.is_empty() {
            return self.frozen.get(0).cloned();
        }

        return None;
    }
}
