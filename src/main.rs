mod application;
mod domain;
mod infrastructure;

use std::sync::Arc;

use application::use_cases::FetchAndPrintAircraft;

use crate::infrastructure::{config::test::AdsbConfig, http::adsb_client::AdsbHttpClient};

#[tokio::main]
async fn main() {
    // ── Logging ──────────────────────────────────────────────────────────────
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("adsb_tracker=debug")),
        )
        .init();

    // ── Composition root ─────────────────────────────────────────────────────
    // Infrastructure adapter fulfils the domain port.
    let config = AdsbConfig::default();
    let gateway = Arc::new(AdsbHttpClient::new(config));

    // Use-case receives the gateway through the port (trait object).
    let use_case = FetchAndPrintAircraft::new(gateway);

    // ── Execute ──────────────────────────────────────────────────────────────
    // Fetch all aircraft within 100 NM of Hamburg, Germany.
    let lat = 51.1657; // Mittelpunkt Deutschland
    let lon = 10.4515;
    let radius_nm = 400;

    if let Err(e) = use_case.execute(lat, lon, radius_nm).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
