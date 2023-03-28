use super::namespace::GitLabNamespace;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitLabProject {
    pub id: i32,
    pub description: Option<String>,
    pub name: String,
    pub name_with_namespace: String,
    pub path: String,
    pub path_with_namespace: String,
    pub created_at: String,
    pub default_branch: String,
    pub tag_list: Vec<String>,
    pub topics: Vec<String>,
    pub ssh_url_to_repo: String,
    pub http_url_to_repo: String,
    pub readme_url: Option<String>,
    pub forks_count: i32,
    pub avatar_url: Option<String>,
    pub star_count: i32,
    pub last_activity_at: String,
    pub namespace: GitLabNamespace,
}
