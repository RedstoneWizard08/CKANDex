use super::{asset::GitHubReleaseAsset, author::GitHubAuthor};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitHubReleaseSchema {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: i32,
    pub author: GitHubAuthor,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<GitHubReleaseAsset>,
    pub zipball_url: String,
    pub tarball_url: String,
    pub body: String,
}
