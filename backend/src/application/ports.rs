use async_trait::async_trait;

use crate::domain::{
    aircraft::{AdsbResponse, Aircraft},
    error::DomainError,
    position::{AircraftSummary, Position},
};

/// Outbound port: anything that can fetch live ADS-B data must implement this.
#[async_trait]
pub trait AdsbGateway: Send + Sync {
    /// Fetch all aircraft globally flagged as military (dbFlags & 1).
    async fn fetch_military(&self) -> Result<AdsbResponse, DomainError>;
}

/// Outbound port: persistent storage for aircraft and their positions.
pub trait AircraftRepository: Send + Sync {
    fn insert_aircraft(&self, aircraft: &Aircraft) -> Result<(), DomainError>;
    fn insert_position(&self, aircraft: &Aircraft, timestamp: i64) -> Result<(), DomainError>;

    fn list_aircraft_summaries(&self) -> Result<Vec<AircraftSummary>, DomainError>;

    fn list_positions(
        &self,
        icao: &str,
        from_ts: i64,
        to_ts: i64,
    ) -> Result<Vec<Position>, DomainError>;

    /// Returns ISO-8601 dates (YYYY-MM-DD) on which `icao` has at least one
    /// position, expressed in the timezone identified by `tz_offset_seconds`.
    fn list_active_days(
        &self,
        icao: &str,
        tz_offset_seconds: i32,
    ) -> Result<Vec<String>, DomainError>;
}
