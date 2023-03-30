use ckandex::{refresh_data, KSP};
use dotenv::dotenv;
use tokio::main;

#[main]
pub async fn main() {
    dotenv().ok();

    refresh_data(KSP::KSP2, "netkan-ksp2").await;
}
