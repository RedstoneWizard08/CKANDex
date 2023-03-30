use crate::Mod;

pub trait QueryFilter: Send + Sync {
    fn filter(&self, mods: Vec<Mod>) -> Vec<Mod>;
}

pub struct QueryFilterContainer {
    pub inner: Box<dyn QueryFilter>,
}
