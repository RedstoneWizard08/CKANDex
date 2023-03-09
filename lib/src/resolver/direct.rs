use async_trait::async_trait;
use super::common::{ModResolver, ModSourceLists};

#[derive(Default, Debug, Clone)]
pub struct DirectResolver {
    pub mods: ModSourceLists,
}

#[async_trait]
impl ModResolver for DirectResolver {
    fn should_resolve(&self, kref: String) -> bool {
        return kref.starts_with("#/ckan/http/");
    }

    async fn resolve_url(&self, kref: String, _: String) -> Option<String> {
        return Some(kref.replace("#/ckan/http/", ""));
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
