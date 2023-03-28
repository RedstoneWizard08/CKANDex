use super::{
    assets::GitLabAssets, author::GitLabAuthor, commit::GitLabCommit, evidence::GitLabEvidence,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitLabRelease {
    pub name: String,
    pub tag_name: String,
    pub description: String,
    pub created_at: String,
    pub released_at: String,
    pub upcoming_release: bool,
    pub author: GitLabAuthor,
    pub commit: GitLabCommit,
    pub commit_path: String,
    pub tag_path: String,
    pub assets: GitLabAssets,
    pub evidences: Vec<GitLabEvidence>,
    pub _links: HashMap<String, String>,
}
