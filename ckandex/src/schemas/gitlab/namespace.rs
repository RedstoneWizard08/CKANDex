use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitLabNamespace {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub kind: String,
    pub full_path: String,
    pub parent_id: Option<String>,
    pub avatar_url: String,
    pub web_url: String,
}
