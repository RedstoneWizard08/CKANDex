use serde::{Deserialize, Serialize};

use crate::{kref::resolve_kref, CKANError, FrozenSchema, NetKANSchema};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum ModType {
    NETKAN,
    FROZEN,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mod {
    pub id: String,
    pub name: String,
    pub kind: ModType,
    pub kref: String,
    pub download: Option<String>,
    pub description: Option<String>,

    token: Option<String>,
}

unsafe impl Send for Mod {}
unsafe impl Sync for Mod {}

impl Mod {
    pub fn from_netkan(netkan: NetKANSchema) -> Self {
        Self {
            id: netkan.identifier.unwrap(),
            name: netkan.name.unwrap_or_default(),
            kind: ModType::NETKAN,
            kref: netkan.kref.unwrap_or_default(),
            download: None,
            token: None,
            description: netkan.short_description,
        }
    }

    pub fn from_frozen(frozen: FrozenSchema) -> Self {
        Self {
            id: frozen.identifier.unwrap(),
            name: frozen.name.unwrap_or_default(),
            kind: ModType::FROZEN,
            kref: frozen.kref.unwrap_or_default(),
            download: None,
            token: None,
            description: frozen.short_description,
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub async fn resolve_download(&mut self) -> Result<String, CKANError> {
        let download = resolve_kref(
            self.kref.clone(),
            self.token.clone().unwrap_or("".to_string()),
        )
        .await;

        if let Ok(url) = download {
            self.download = Some(url.clone());

            return Ok(url);
        }

        Err(CKANError::UnresolvableKref)
    }
}
