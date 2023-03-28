use serde::{Serialize, Deserialize};

use crate::{CacheClient, Mod};

/// A query filter.
pub trait QueryFilter: Send + Sync {
    fn filter(&self, mods: Vec<Mod>) -> Vec<Mod>;
}

pub struct IdFilter {
    pub filter: String,
}

pub struct NameFilter {
    pub filter: String,
}

impl IdFilter {
    pub fn new(filter: String) -> Self {
        return Self { filter };
    }
}

impl NameFilter {
    pub fn new(filter: String) -> Self {
        return Self { filter };
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

        return post;
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

        return post;
    }
}

pub struct QueryFilterContainer {
    pub inner: Box<dyn QueryFilter>,
}

pub struct Query {
    pub filters: Vec<QueryFilterContainer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub netkans: Vec<Mod>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub frozen: Vec<Mod>,
}

impl QueryResponse {
    pub fn first(&self) -> Option<Mod> {
        if !self.netkans.is_empty() {
            return self.netkans.get(0).cloned();
        }

        if !self.frozen.is_empty() {
            return self.frozen.get(0).cloned();
        }

        return None;
    }
}

impl Default for Query {
    fn default() -> Self {
        return Self {
            filters: Vec::new(),
        };
    }
}

impl Query {
    pub fn new() -> Self {
        return Self {
            filters: Vec::new(),
        };
    }

    pub fn execute(&self, cache: CacheClient) -> QueryResponse {
        let netkans = cache.netkans.unwrap();
        let frozen = cache.frozen.unwrap();

        let netkans = netkans
            .iter()
            .map(|v| Mod::from_netkan(v.clone()))
            .collect::<Vec<Mod>>();

        let frozen = frozen
            .iter()
            .map(|v| Mod::from_frozen(v.clone()))
            .collect::<Vec<Mod>>();

        let mut res_netkan: Vec<Mod> = netkans;
        let mut res_frozen: Vec<Mod> = frozen;

        for filter in &self.filters {
            res_netkan = filter.inner.filter(res_netkan);
            res_frozen = filter.inner.filter(res_frozen);
        }

        return QueryResponse {
            netkans: res_netkan,
            frozen: res_frozen,
        };
    }
}

pub struct QueryBuilder {
    pub query: Query,
}

impl Default for QueryBuilder {
    fn default() -> Self {
        return Self {
            query: Query::new(),
        };
    }
}

impl QueryBuilder {
    pub fn new() -> Self {
        return Self {
            query: Query::new(),
        };
    }

    pub fn add<F>(&mut self, filter: F) -> &mut Self
    where
        F: QueryFilter + 'static,
    {
        self.query.filters.push(QueryFilterContainer {
            inner: Box::new(filter),
        });

        return self;
    }

    pub fn build(&self) -> &Query {
        return &self.query;
    }
}
