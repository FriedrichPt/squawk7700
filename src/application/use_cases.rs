use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use tracing::info;

use crate::application::ports::{AdsbGateway, AircraftRepository};
use crate::domain::error::DomainError;

pub struct FetchAndStoreAircraft {
    gateway: Arc<dyn AdsbGateway>,
    repository: Arc<dyn AircraftRepository>,
}

impl FetchAndStoreAircraft {
    pub fn new(gateway: Arc<dyn AdsbGateway>, repository: Arc<dyn AircraftRepository>) -> Self {
        Self {
            gateway,
            repository,
        }
    }

    pub async fn execute(&self, lat: f64, lon: f64, radius_nm: u32) -> Result<(), DomainError> {
        let response = self.gateway.fetch_by_location(lat, lon, radius_nm).await?;

        info!(
            total = response.aircraft.len(),
            "Received aircraft snapshot"
        );

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .as_secs() as i64;

        for aircraft in response.aircraft.iter().filter(|a| a.is_german_military()) {
            info!(
                icao = %aircraft.hex,
                callsign = ?aircraft.flight,
                aircraft_type = ?aircraft.t,
                registration = ?aircraft.r,
                "German military aircraft"
            );

            self.repository.upsert_aircraft(aircraft)?;

            if aircraft.has_position() {
                self.repository.insert_position(aircraft, now)?;
            }
        }

        Ok(())
    }
}
