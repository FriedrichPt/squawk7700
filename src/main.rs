mod application;
mod domain;
mod infrastructure;
use application::use_cases::FetchAndStoreAircraft;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

use crate::infrastructure::{
    config::test::AdsbConfig, db::sqlite_repository::SqliteRepository,
    http::adsb_client::AdsbHttpClient,
};

const POLL_INTERVAL: Duration = Duration::from_secs(30);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("squawk7700=info")),
        )
        .init();

    let config = AdsbConfig::default();
    let gateway = Arc::new(AdsbHttpClient::new(config));

    let repository = Arc::new(SqliteRepository::new("squawk7700.db").unwrap_or_else(|e| {
        eprintln!("Failed to open database: {e}");
        std::process::exit(1);
    }));

    let use_case = FetchAndStoreAircraft::new(gateway, repository);

    info!("Starting poller (interval: {}s)", POLL_INTERVAL.as_secs());

    let mut interval = time::interval(POLL_INTERVAL);
    loop {
        interval.tick().await;
        if let Err(e) = use_case.execute().await {
            error!("Fetch failed: {e}");
        }
    }
}
