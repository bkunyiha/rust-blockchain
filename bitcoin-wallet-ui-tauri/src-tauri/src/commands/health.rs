use crate::config::AppState;
use crate::services::bitcoin_api::BitcoinApiService;
use std::sync::RwLock;
use tauri::State;

#[tauri::command]
pub async fn health_check(state: State<'_, RwLock<AppState>>) -> Result<bool, String> {
    let cfg = state.read().map_err(|e| e.to_string())?.api_config();
    BitcoinApiService::health_check(cfg).await
}
