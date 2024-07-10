use thiserror::Error;

#[derive(Debug, Error)]
pub enum CKANError {
    #[error("CKAN cache not found!")]
    CacheNotFound,

    #[error("Unresolvable KREF!")]
    UnresolvableKref,

    #[error("Unknown mod!")]
    UnknownMod,

    #[error("Unknown NetKAN descriptor format!")]
    UnknownDescriptorFormat,

    #[error("Could not find an asset!")]
    NoAsset,

    #[error("Could not find an artifact!")]
    UnknownArtifact,

    #[error("An invalid commit was found!")]
    InvalidCommit,

    #[error("Fast-forward changes only!")]
    FastForwardOnly,

    #[error("An error occured")]
    Io(#[from] std::io::Error),

    #[error("An error occured")]
    Json(#[from] serde_json::Error),

    #[error("An error occured")]
    Yaml(#[from] serde_yml::Error),

    #[error("An error occured")]
    Git(#[from] git2::Error),
}
