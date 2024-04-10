use crate::Mod;

pub trait QueryFilter: Send + Sync {
    fn filter(&self, mods: Vec<Mod>) -> Vec<Mod>;
}

pub struct QueryFilterContainer {
    pub inner: Box<dyn QueryFilter>,
}

impl<T: QueryFilter + 'static> From<T> for QueryFilterContainer {
    fn from(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}
