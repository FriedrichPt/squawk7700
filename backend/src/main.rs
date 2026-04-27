mod application;
mod domain;
mod infrastructure;

use std::net::SocketAddr;
use std::sync::Arc;

use tracing::{error, info};

use crate::infrastructure::db::sqlite_repository::SqliteRepository;
use crate::infrastructure::web::{self, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::new("squawk7700=info,tower_http=info")
            }),
        )
        .init();

    let db_path =
        std::env::var("SQUAWK_DB_PATH").unwrap_or_else(|_| "../squawk7700.db".to_string());
    let addr: SocketAddr = std::env::var("SQUAWK_API_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8181".to_string())
        .parse()
        .unwrap_or_else(|e| {
            eprintln!("Invalid SQUAWK_API_ADDR: {e}");
            std::process::exit(1);
        });

    info!(db = %db_path, "Opening database (read-only intent)");

    let repository = Arc::new(SqliteRepository::new(&db_path).unwrap_or_else(|e| {
        eprintln!("Failed to open database '{db_path}': {e}");
        std::process::exit(1);
    }));

    let state = AppState { repo: repository };

    if let Err(e) = web::run(addr, state).await {
        error!("API server stopped: {e}");
        std::process::exit(1);
    }
}
