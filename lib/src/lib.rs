pub mod all;
pub mod git;

pub mod resolver;
pub mod schemas;

pub use schemas::frozen::*;
pub use schemas::netkan::*;

pub use all::*;
pub use git::*;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "server")]
pub use server::*;

pub async fn refresh_data() {
    clone_repo().await;
}
