use crate::config::ApiConfig;
use crate::services::BitcoinApiService;
use serde_json::Value;
use std::sync::RwLock;
use tauri::State;

#[tauri::command]
pub async fn get_mempool(config: State<'_, RwLock<ApiConfig>>) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::get_mempool(&cfg).await
}

#[tauri::command]
pub async fn get_mempool_transaction(
    txid: String,
    config: State<'_, RwLock<ApiConfig>>,
) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::get_mempool_transaction(&cfg, &txid).await
}

#[tauri::command]
pub async fn get_all_transactions(config: State<'_, RwLock<ApiConfig>>) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::get_all_transactions(&cfg).await
}

#[tauri::command]
pub async fn get_address_transactions(
    address: String,
    config: State<'_, RwLock<ApiConfig>>,
) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::get_address_transactions(&cfg, &address).await
}
