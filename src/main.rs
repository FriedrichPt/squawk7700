mod application;
mod domain;
mod infrastructure;
use application::use_cases::FetchAndStoreAircraft;
use std::sync::Arc;

use crate::infrastructure::{
    config::test::AdsbConfig, db::sqlite_repository::SqliteRepository,
    http::adsb_client::AdsbHttpClient,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("squawk7700=debug")),
        )
        .init();

    let config = AdsbConfig::default();
    let gateway = Arc::new(AdsbHttpClient::new(config));

    let repository = Arc::new(SqliteRepository::new("squawk7700.db").unwrap_or_else(|e| {
        eprintln!("Failed to open database: {e}");
        std::process::exit(1);
    }));

    let use_case = FetchAndStoreAircraft::new(gateway, repository);

    let lat = 51.1657;
    let lon = 10.4515;
    let radius_nm = 4000;

    if let Err(e) = use_case.execute(lat, lon, radius_nm).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
