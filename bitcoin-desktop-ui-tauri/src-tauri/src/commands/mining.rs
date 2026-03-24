use crate::config::ApiConfig;
use crate::services::BitcoinApiService;
use serde_json::Value;
use std::sync::RwLock;
use tauri::State;

#[tauri::command]
pub async fn get_mining_info(config: State<'_, RwLock<ApiConfig>>) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::get_mining_info(&cfg).await
}

#[tauri::command]
pub async fn generate_blocks(
    address: String,
    nblocks: u32,
    maxtries: Option<u32>,
    config: State<'_, RwLock<ApiConfig>>,
) -> Result<Value, String> {
    let cfg = config.read().map_err(|e| format!("Lock error: {}", e))?;
    BitcoinApiService::generate_blocks(&cfg, &address, nblocks, maxtries).await
}
