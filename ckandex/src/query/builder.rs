use crate::{Query, QueryFilter, QueryFilterContainer};

#[derive(Default)]
pub struct QueryBuilder {
    pub query: Query,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            query: Query::new(),
        }
    }

    pub fn add<F>(&mut self, filter: F) -> &mut Self
    where
        F: QueryFilter + 'static,
    {
        self.query.filters.push(QueryFilterContainer {
            inner: Box::new(filter),
        });

        self
    }

    pub fn build(&self) -> &Query {
        &self.query
    }
}
