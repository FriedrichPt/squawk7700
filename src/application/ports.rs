use async_trait::async_trait;

use crate::domain::{aircraft::AdsbResponse, error::DomainError};

/// Outbound port: anything that can fetch live ADS-B data must implement this.
#[async_trait]
pub trait AdsbGateway: Send + Sync {
    /// Fetch all aircraft within `radius_nm` nautical miles of the given coordinates.
    async fn fetch_by_location(
        &self,
        lat: f64,
        lon: f64,
        radius_nm: u32,
    ) -> Result<AdsbResponse, DomainError>;

    /// Fetch a single aircraft by its ICAO hex identifier.
    async fn fetch_by_icao(&self, icao: &str) -> Result<AdsbResponse, DomainError>;
}
