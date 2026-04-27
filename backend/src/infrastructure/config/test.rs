#[derive(Debug, Clone)]
pub struct AdsbConfig {
    /// Base URL for the adsb.lol public API.
    pub base_url: String,
}

impl Default for AdsbConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.adsb.lol".to_string(),
        }
    }
}
