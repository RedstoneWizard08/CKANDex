use std::collections::HashMap;

use async_trait::async_trait;

#[async_trait]
pub trait ModResolver {
    fn should_resolve(&self, kref: String) -> bool;
    async fn resolve_url(&self, kref: String) -> Option<String>;

    fn merge_results(&self, other: &mut dyn ModResolver);
    fn accept_mods(&mut self, mods: ModSourceLists);
}

#[derive(Debug, Clone)]
pub struct ModSourceLists {
    pub avc: HashMap<String, String>,
    pub github: HashMap<String, String>,
    pub gitlab: HashMap<String, String>,
    pub netkan: HashMap<String, String>,
    pub direct: HashMap<String, String>,
    pub jenkins: HashMap<String, String>,
}
