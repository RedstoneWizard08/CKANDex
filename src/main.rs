use git::clone_repo;
use server::run_server;

pub mod all;
pub mod git;
pub mod schemas;
pub mod server;

#[tokio::main]
pub async fn main() {
    clone_repo().await;
    run_server().await;
}
