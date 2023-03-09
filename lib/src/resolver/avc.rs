use super::common::{ModResolver, ModSourceLists};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use fancy_regex::Regex;

#[derive(Default, Debug, Clone)]
pub struct AVCResolver {
    pub mods: ModSourceLists,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SemVer {
    #[serde(rename = "MAJOR")]
    pub major: i32,

    #[serde(rename = "MINOR")]
    pub minor: i32,

    #[serde(rename = "PATCH")]
    pub patch: i32,

    #[serde(rename = "BUILD")]
    pub build: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SemVerSmall {
    #[serde(rename = "MAJOR")]
    pub major: i32,

    #[serde(rename = "MINOR")]
    pub minor: i32,

    #[serde(rename = "PATCH")]
    pub patch: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AVCSchema {
    #[serde(rename = "NAME")]
    pub name: String,

    #[serde(rename = "URL")]
    pub url: String,

    #[serde(rename = "DOWNLOAD")]
    pub download: String,

    #[serde(rename = "VERSION")]
    pub version: SemVer,

    #[serde(rename = "KSP_VERSION_MIN")]
    pub ksp_version_min: Option<SemVerSmall>,

    #[serde(rename = "KSP_VERSION_MAX")]
    pub ksp_version_max: Option<SemVerSmall>,
}

#[async_trait]
impl ModResolver for AVCResolver {
    fn should_resolve(&self, kref: String) -> bool {
        return kref.starts_with("#/ckan/ksp-avc/");
    }

    async fn resolve_url(&self, kref: String, _: String) -> Option<String> {
        let url = kref.replace("#/ckan/ksp-avc/", "");
        let resp = reqwest::get(url).await.unwrap();

        let regex = Regex::new(r#"\,(?!\s*?[\{\[\"\'\w])"#).unwrap();
        let content = resp.text().await.unwrap();
        let content_json = regex.replace(&content, "");
        let data = serde_json::from_str::<AVCSchema>(&content_json);

        if let Ok(data) = data {
            return Some(data.download);
        }

        return None;
    }

    fn merge_results(&self, other: &mut dyn ModResolver) {
        other.accept_mods(self.mods.clone());
    }

    fn accept_mods(&mut self, mods: ModSourceLists) {
        mods.avc.iter().for_each(|(k, v)| {
            self.mods.avc.insert(k.clone(), v.clone()).unwrap();
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
