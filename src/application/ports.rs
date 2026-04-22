use async_trait::async_trait;

use crate::domain::{
    aircraft::{AdsbResponse, Aircraft},
    error::DomainError,
};

/// Outbound port: anything that can fetch live ADS-B data must implement this.
#[async_trait]
pub trait AdsbGateway: Send + Sync {
    /// Fetch all aircraft globally flagged as military (dbFlags & 1).
    async fn fetch_military(&self) -> Result<AdsbResponse, DomainError>;
}

/// Outbound port: persistent storage for aircraft and their positions.
pub trait AircraftRepository: Send + Sync {
    /// Insert the aircraft record if it doesn't exist yet (ignore duplicates).
    fn insert_aircraft(&self, aircraft: &Aircraft) -> Result<(), DomainError>;

    /// Append a position snapshot for the given ICAO.
    fn insert_position(&self, aircraft: &Aircraft, timestamp: i64) -> Result<(), DomainError>;
}
