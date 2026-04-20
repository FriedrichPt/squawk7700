use std::sync::Arc;

use tracing::info;

use crate::{application::ports::AdsbGateway, domain::error::DomainError};

pub struct FetchAndPrintAircraft {
    gateway: Arc<dyn AdsbGateway>,
}

impl FetchAndPrintAircraft {
    pub fn new(gateway: Arc<dyn AdsbGateway>) -> Self {
        Self { gateway }
    }

    /// Fetch aircraft near the given position and print each one to stdout.
    pub async fn execute(&self, lat: f64, lon: f64, radius_nm: u32) -> Result<(), DomainError> {
        let response = self.gateway.fetch_by_location(lat, lon, radius_nm).await?;

        info!(
            total = response.aircraft.len(),
            "Received aircraft snapshot"
        );

        for aircraft in response.aircraft.iter().filter(|a| a.is_military()) {
            //println!("{:#?}", aircraft);
            let country = aircraft.registration_country().unwrap_or("Unknown");
            //println!("[{}] {:?}", country, aircraft);
            println!("{:#?}", country);
        }

        Ok(())
    }
}
