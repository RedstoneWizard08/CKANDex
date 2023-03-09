use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitLabAsset {
    pub format: String,
    pub url: String,
}
