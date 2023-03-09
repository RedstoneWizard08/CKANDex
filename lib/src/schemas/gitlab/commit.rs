use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitLabCommit {
    pub id: String,
    pub short_id: String,
    pub created_at: String,
    pub parent_ids: Vec<String>,
    pub title: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub authored_date: String,
    pub committer_name: String,
    pub committer_email: String,
    pub committed_date: String,
    pub trailers: Value,
    pub web_url: String,
}
