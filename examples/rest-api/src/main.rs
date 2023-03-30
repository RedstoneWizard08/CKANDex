use ckandex::{refresh_data, run_server, KSP};
use dotenv::dotenv;
use tokio::main;

#[main]
pub async fn main() {
    dotenv().ok();

    refresh_data(KSP::KSP2, "netkan-ksp2").await;
    run_server("netkan-ksp2".to_string()).await;
}
