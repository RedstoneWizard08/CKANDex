use ckandex::{run_server, clone_repo};
use dotenv::dotenv;
use tokio::main;

#[main]
pub async fn main() {
    dotenv().ok();

    clone_repo().await;
    run_server().await;
}
