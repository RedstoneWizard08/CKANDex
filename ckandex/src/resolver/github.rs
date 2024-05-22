use crate::{schemas::github::release::GitHubReleaseSchema, CKANError};
use reqwest::header;

use super::common::{ModResolver, ModSourceLists};

#[derive(Default, Debug, Clone)]
pub struct GitHubResolver {
    pub mods: ModSourceLists,
}

#[async_trait]
impl ModResolver for GitHubResolver {
    fn should_resolve(&self, kref: String) -> bool {
        kref.starts_with("#/ckan/github/")
    }

    async fn resolve_url(&self, kref: String, token: String) -> Result<String, CKANError> {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap(),
        );
        headers.insert(
            "User-Agent",
            header::HeaderValue::from_str("CKANDex Resolver").unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let kref_url = kref.replace("#/ckan/github/", "");
        let url = format!("https://api.github.com/repos/{}/releases/latest", kref_url);
        let resp = client.get(url).send().await.unwrap();

        let content = resp.text().await.unwrap();
        let data = serde_json::from_str::<GitHubReleaseSchema>(&content);

        if let Ok(data) = data {
            let assets = data.assets;
            let asset = assets.iter().find(|v| v.name.ends_with(".zip"));

            if let Some(asset) = asset {
                return Ok(asset.clone().browser_download_url);
            }
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
