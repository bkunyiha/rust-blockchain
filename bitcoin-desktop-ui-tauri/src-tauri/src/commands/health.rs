use crate::config::ApiConfig;
use crate::services::BitcoinApiService;
use serde_json::Value;
use std::sync::RwLock;
use tauri::State;

#[tauri::command]
pub async fn health_check(config: State<'_, RwLock<ApiConfig>>) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::health_check(&cfg).await
}

#[tauri::command]
pub async fn liveness_check(config: State<'_, RwLock<ApiConfig>>) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::liveness_check(&cfg).await
}

#[tauri::command]
pub async fn readiness_check(config: State<'_, RwLock<ApiConfig>>) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::readiness_check(&cfg).await
}
