#![allow(clippy::needless_return)]

pub mod cache;
pub mod error;
pub mod git;
pub mod query;
pub mod resolver;
pub mod schemas;

#[cfg(feature = "server")]
pub mod server;

pub use cache::*;
pub use error::*;
pub use git::*;
pub use query::*;
pub use resolver::*;
pub use schemas::ckan::*;
pub use schemas::frozen::*;
pub use schemas::netkan::*;

#[cfg(feature = "server")]
pub use server::*;

pub async fn refresh_data() {
    clone_repo().await;
}
