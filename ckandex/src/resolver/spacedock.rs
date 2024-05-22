use crate::CKANError;

use super::common::{ModResolver, ModSourceLists};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone)]
pub struct SpaceDockResolver {
    pub mods: ModSourceLists,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceDockSchema {
    pub friendly_version: String,
    pub game_version: String,
    pub id: u64,
    pub created: String,
    pub download_path: String,
    pub changelog: Option<String>,
    pub downloads: u64,
}

#[async_trait]
impl ModResolver for SpaceDockResolver {
    fn should_resolve(&self, kref: String) -> bool {
        kref.starts_with("#/ckan/spacedock/")
    }

    async fn resolve_url(&self, kref: String, _: String) -> Result<String, CKANError> {
        let url = kref.replace("#/ckan/spacedock/", "https://spacedock.info/api/mod/") + "/latest";
        let resp = reqwest::get(url).await.unwrap();

        let content = resp.text().await.unwrap();
        let data = serde_json::from_str::<SpaceDockSchema>(&content);

        if let Ok(data) = data {
            return Ok("https://spacedock.info".to_string() + &data.download_path);
        }

        return Err(CKANError::UnresolvableKref);
    }

    fn merge_results(&self, other: &mut dyn ModResolver) {
        other.accept_mods(self.mods.clone());
    }

    fn accept_mods(&mut self, mods: ModSourceLists) {
        mods.avc.iter().for_each(|(k, v)| {
            self.mods.avc.insert(k.clone(), v.clone()).unwrap();
        });

        mods.spacedock.iter().for_each(|(k, v)| {
            self.mods.spacedock.insert(k.clone(), v.clone()).unwrap();
        });

        mods.github.iter().for_each(|(k, v)| {
            self.mods.github.insert(k.clone(), v.clone()).unwrap();
        });

        mods.gitlab.iter().for_each(|(k, v)| {
            self.mods.gitlab.insert(k.clone(), v.clone()).unwrap();
        });

        mods.netkan.iter().for_each(|(k, v)| {
            self.mods.netkan.insert(k.clone(), v.clone()).unwrap();
        });

        mods.direct.iter().for_each(|(k, v)| {
            self.mods.direct.insert(k.clone(), v.clone()).unwrap();
        });

        mods.jenkins.iter().for_each(|(k, v)| {
            self.mods.jenkins.insert(k.clone(), v.clone()).unwrap();
        });
    }
}
