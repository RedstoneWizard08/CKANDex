use async_trait::async_trait;

use crate::NetKANSchema;

use super::common::{ModResolver, ModSourceLists};

#[derive(Debug, Clone)]
pub struct NetKANResolver {
    pub mods: ModSourceLists,
}

#[async_trait]
impl ModResolver for NetKANResolver {
    fn should_resolve(&self, kref: String) -> bool {
        return kref.starts_with("#/ckan/netkan/");
    }

    async fn resolve_url(&self, kref: String) -> Option<String> {
        let url = kref.replace("#/ckan/netkan/", "");
        let resp = reqwest::get(url).await.unwrap();

        let content = resp.text().await.unwrap();
        let data: NetKANSchema;

        if let Ok(json) = serde_json::from_str(&content) {
            data = json;
        } else if let Ok(yaml) = serde_yaml::from_str(&content) {
            data = yaml;
        } else {
            return None;
        }

        return data.kref;
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
