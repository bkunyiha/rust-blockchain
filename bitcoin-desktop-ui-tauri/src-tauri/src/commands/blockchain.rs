use crate::config::ApiConfig;
use crate::services::BitcoinApiService;
use serde_json::Value;
use std::sync::RwLock;
use tauri::State;

#[tauri::command]
pub async fn get_blockchain_info(config: State<'_, RwLock<ApiConfig>>) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::get_blockchain_info(&cfg).await
}

#[tauri::command]
pub async fn get_latest_blocks(config: State<'_, RwLock<ApiConfig>>) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::get_latest_blocks(&cfg).await
}

#[tauri::command]
pub async fn get_all_blocks(config: State<'_, RwLock<ApiConfig>>) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::get_all_blocks(&cfg).await
}

#[tauri::command]
pub async fn get_block_by_hash(
    hash: String,
    config: State<'_, RwLock<ApiConfig>>,
) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::get_block_by_hash(&cfg, &hash).await
}
