use super::author::GitHubAuthor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitHubReleaseAsset {
    pub url: String,
    pub id: i32,
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub uploader: GitHubAuthor,
    pub content_type: String,
    pub state: String,
    pub size: i32,
    pub download_count: i32,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}
