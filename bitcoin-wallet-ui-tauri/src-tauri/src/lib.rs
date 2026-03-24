mod commands;
mod config;
mod database;
mod models;
mod services;

use config::AppState;
use std::sync::RwLock;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_password = generate_database_password();
    if let Err(e) = database::init_database(&db_password) {
        eprintln!("Failed to initialize database: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .manage(RwLock::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            commands::wallet::create_wallet,
            commands::wallet::get_saved_wallets,
            commands::wallet::set_active_wallet,
            commands::wallet::delete_wallet_address,
            commands::wallet::update_wallet_label,
            commands::wallet::get_wallet_info,
            commands::wallet::get_balance,
            commands::wallet::send_transaction,
            commands::wallet::get_tx_history,
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::settings::check_connection,
            commands::health::health_check,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate_database_password() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();

    if let Ok(username) = std::env::var("USER") {
        username.hash(&mut hasher);
    } else if let Ok(username) = std::env::var("USERNAME") {
        username.hash(&mut hasher);
    }

    if let Some(home) = dirs::home_dir() {
        home.to_string_lossy().hash(&mut hasher);
    }

    "bitcoin-wallet-ui".hash(&mut hasher);

    format!("{:x}", hasher.finish())
}
