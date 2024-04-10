use super::filter::QueryFilter;
use crate::Mod;

pub struct IdFilter {
    pub filter: String,
}

pub struct NameFilter {
    pub filter: String,
}

impl IdFilter {
    pub fn new(filter: String) -> Self {
        Self { filter }
    }
}

impl NameFilter {
    pub fn new(filter: String) -> Self {
        Self { filter }
    }
}

impl QueryFilter for IdFilter {
    fn filter(&self, mods: Vec<Mod>) -> Vec<Mod> {
        let mut post = Vec::new();
        let filter = self.filter.to_lowercase();

        for item in mods {
            let item_id = item.id.to_lowercase();

            if item_id.contains(&filter) {
                post.push(item);
            }
        }

        post
    }
}

impl QueryFilter for NameFilter {
    fn filter(&self, mods: Vec<Mod>) -> Vec<Mod> {
        let mut post = Vec::new();
        let filter = self.filter.to_lowercase();

        for item in mods {
            let item_id = item.id.to_lowercase();
            let item_name = item.name.to_lowercase();

            // Sometimes it's in the ID.
            if item_name.contains(&filter) || item_id.contains(&filter) {
                post.push(item);
            }
        }

        post
    }
}
