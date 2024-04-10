use crate::{CacheClient, Mod, QueryFilterContainer, QueryResponse};

#[derive(Default)]
pub struct Query {
    pub filters: Vec<QueryFilterContainer>,
}

impl Query {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn filter(&mut self, filter: QueryFilterContainer) -> &mut Self {
        self.filters.push(filter);
        self
    }

    pub fn execute(&self, cache: &CacheClient) -> QueryResponse {
        let netkans = cache.netkans.clone().unwrap();
        let frozen = cache.frozen.clone().unwrap();

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

        QueryResponse {
            netkans: res_netkan,
            frozen: res_frozen,
        }
    }
}
