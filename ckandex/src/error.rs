use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum CKANError {
    CacheNotFound,
    UnresolvableKref,
    UnknownMod,
    UnknownDescriptorFormat,
    NoAsset,
    UnknownArtifact,
    InvalidCommit,
}
