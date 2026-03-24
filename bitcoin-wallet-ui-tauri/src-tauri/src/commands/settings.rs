use crate::config::AppState;
use crate::database;
use crate::services::bitcoin_api::BitcoinApiService;
use std::sync::RwLock;
use tauri::State;

#[tauri::command]
pub async fn get_settings() -> Result<database::Settings, String> {
    database::load_settings().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings(
    base_url: String,
    api_key: String,
    state: State<'_, RwLock<AppState>>,
) -> Result<(), String> {
    let settings = database::Settings {
        base_url: base_url.clone(),
        api_key: api_key.clone(),
    };
    database::save_settings_db(&settings).map_err(|e| e.to_string())?;

    // Update in-memory state
    let mut app_state = state.write().map_err(|e| e.to_string())?;
    app_state.base_url = base_url;
    app_state.api_key = api_key;

    Ok(())
}

#[tauri::command]
pub async fn check_connection(state: State<'_, RwLock<AppState>>) -> Result<bool, String> {
    let cfg = state.read().map_err(|e| e.to_string())?.api_config();
    BitcoinApiService::health_check(cfg).await
}
