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

pub enum KSP {
    KSP1,
    KSP2,
}

pub async fn refresh_data(game: KSP, dir: &str) {
    let repo = match game {
        KSP::KSP1 => KSP1_REPO_INFO,
        KSP::KSP2 => KSP2_REPO_INFO,
    };

    clone_repo(repo, dir).await;
}
