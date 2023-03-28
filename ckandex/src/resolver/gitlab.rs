use crate::{
    schemas::gitlab::{project::GitLabProject, release::GitLabRelease},
    CKANError,
};
use async_trait::async_trait;

use super::common::{ModResolver, ModSourceLists};

#[derive(Default, Debug, Clone)]
pub struct GitLabResolver {
    pub mods: ModSourceLists,
}

#[async_trait]
impl ModResolver for GitLabResolver {
    fn should_resolve(&self, kref: String) -> bool {
        return kref.starts_with("#/ckan/gitlab/");
    }

    async fn resolve_url(&self, kref: String, _: String) -> Result<String, CKANError> {
        let kref_url = kref.replace("#/ckan/gitlab/", "");
        let mut kref_spl = kref_url.split('/');

        let username = kref_spl.next().unwrap();
        let repo = kref_spl.next().unwrap();

        let url = format!("https://gitlab.com/api/v4/users/{}/projects", username);
        let resp = reqwest::get(url).await.unwrap();

        let content = resp.text().await.unwrap();
        let data = serde_json::from_str::<Vec<GitLabProject>>(&content).unwrap();
        let project = data.iter().find(|p| p.path == repo).unwrap();

        let url = format!("https://gitlab.com/api/v4/projects/{}/releases", project.id);
        let resp = reqwest::get(url).await.unwrap();

        let content = resp.text().await.unwrap();
        let data = serde_json::from_str::<Vec<GitLabRelease>>(&content).unwrap();

        let asset = data
            .get(0)
            .unwrap()
            .assets
            .sources
            .iter()
            .find(|s| s.format == *"zip");

        if let Some(asset) = asset {
            return Ok(asset.url.clone());
        }

        return Err(CKANError::NoAsset);
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
