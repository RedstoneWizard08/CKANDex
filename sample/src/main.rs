use ckandex::{run_server, update_repo};
use tokio::main;

#[main]
pub async fn main() {
    update_repo().await;
    run_server().await;
}
