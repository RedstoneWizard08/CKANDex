use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitLabEvidence {
    pub sha: String,
    pub filepath: String,
    pub collected_at: String,
}
