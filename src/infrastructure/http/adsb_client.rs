use async_trait::async_trait;
use reqwest::Client;
use tracing::debug;

use crate::application::ports::AdsbGateway;
use crate::domain::aircraft::AdsbResponse;
use crate::domain::error::DomainError;
use crate::infrastructure::config::test::AdsbConfig;

pub struct AdsbHttpClient {
    client: Client,
    config: AdsbConfig,
}

impl AdsbHttpClient {
    pub fn new(config: AdsbConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait]
impl AdsbGateway for AdsbHttpClient {
    async fn fetch_military(&self) -> Result<AdsbResponse, DomainError> {
        let url = format!("{}/v2/mil", self.config.base_url);

        debug!(%url, "Fetching military aircraft");

        let raw = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::DataUnavailable(e.to_string()))?
            .error_for_status()
            .map_err(|e| DomainError::DataUnavailable(e.to_string()))?
            .text()
            .await
            .map_err(|e| DomainError::DataUnavailable(e.to_string()))?;

        let response = serde_json::from_str::<AdsbResponse>(&raw)
            .map_err(|e| DomainError::DataUnavailable(e.to_string()))?;

        Ok(response)
    }
}
