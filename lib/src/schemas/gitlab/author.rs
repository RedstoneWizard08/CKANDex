use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitLabAuthor {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub state: String,
    pub avatar_url: String,
    pub web_url: String,
}
