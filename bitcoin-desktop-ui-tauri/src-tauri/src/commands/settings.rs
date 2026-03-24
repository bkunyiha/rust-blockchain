use crate::config::ApiConfig;
use crate::services::BitcoinApiService;
use serde_json::{json, Value};
use std::sync::RwLock;
use tauri::State;

#[tauri::command]
pub fn get_config(config: State<'_, RwLock<ApiConfig>>) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    Ok(json!({
        "base_url": cfg.base_url,
        "api_key": cfg.api_key
    }))
}

#[tauri::command]
pub fn update_config(
    base_url: String,
    api_key: String,
    config: State<'_, RwLock<ApiConfig>>,
) -> Result<Value, String> {
    let mut cfg = config.write().map_err(|e| format!("Lock error: {}", e))?;
    cfg.base_url = base_url;
    cfg.api_key = api_key;
    Ok(json!({
        "success": true,
        "base_url": cfg.base_url,
        "api_key": cfg.api_key
    }))
}

#[tauri::command]
pub async fn check_connection(config: State<'_, RwLock<ApiConfig>>) -> Result<bool, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    match BitcoinApiService::health_check(&cfg).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
