use crate::{Query, QueryFilter, QueryFilterContainer};

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
