use super::asset::GitLabAsset;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitLabAssets {
    pub count: i32,
    pub sources: Vec<GitLabAsset>,
    pub links: Vec<Value>,
}
