use crate::{CacheClient, Mod, QueryFilterContainer, QueryResponse};

pub struct Query {
    pub filters: Vec<QueryFilterContainer>,
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
