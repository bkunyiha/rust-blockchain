use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub api_key: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        let base_url = env::var("BITCOIN_API_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
        let api_key = env::var("BITCOIN_API_ADMIN_KEY")
            .unwrap_or_else(|_| "admin-secret".to_string());

        ApiConfig { base_url, api_key }
    }
}

impl ApiConfig {
    pub fn new(base_url: String, api_key: String) -> Self {
        ApiConfig { base_url, api_key }
    }
}
