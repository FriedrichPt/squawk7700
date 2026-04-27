use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::application::ports::AircraftRepository;
use crate::domain::error::DomainError;

use super::handlers;

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn AircraftRepository>,
}

pub async fn run(addr: SocketAddr, state: AppState) -> Result<(), DomainError> {
    let app = Router::new()
        .route("/api/health", get(handlers::health))
        .route("/api/aircraft", get(handlers::list_aircraft))
        .route("/api/aircraft/{icao}/days", get(handlers::list_days))
        .route("/api/aircraft/{icao}/track", get(handlers::get_track))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    info!("API listening on http://{addr}");

    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| DomainError::DatabaseError(format!("bind {addr}: {e}")))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| DomainError::DatabaseError(format!("serve: {e}")))?;

    Ok(())
}
