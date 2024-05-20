#[derive(Debug)]
pub enum CKANError {
    CacheNotFound,
    UnresolvableKref,
    UnknownMod,
    UnknownDescriptorFormat,
    NoAsset,
    UnknownArtifact,
    InvalidCommit,
    FastForwardOnly,

    Io(std::io::Error),
    Json(serde_json::Error),
    Yaml(serde_yaml::Error),
    Git(git2::Error),
}

impl From<std::io::Error> for CKANError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for CKANError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<serde_yaml::Error> for CKANError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::Yaml(value)
    }
}

impl From<git2::Error> for CKANError {
    fn from(value: git2::Error) -> Self {
        Self::Git(value)
    }
}
