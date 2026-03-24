use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub active_wallet: Option<String>,
    pub base_url: String,
    pub api_key: String,
}

impl Default for AppState {
    fn default() -> Self {
        // Try to load settings from database
        let (base_url, api_key) = match crate::database::load_settings() {
            Ok(settings) => (settings.base_url, settings.api_key),
            Err(_) => (
                "http://127.0.0.1:8080".to_string(),
                std::env::var("BITCOIN_API_WALLET_KEY")
                    .unwrap_or_else(|_| "wallet-secret".to_string()),
            ),
        };
        Self {
            active_wallet: None,
            base_url,
            api_key,
        }
    }
}

impl AppState {
    pub fn api_config(&self) -> bitcoin_api::ApiConfig {
        bitcoin_api::ApiConfig {
            base_url: self.base_url.clone(),
            api_key: Some(self.api_key.clone()),
        }
    }
}
