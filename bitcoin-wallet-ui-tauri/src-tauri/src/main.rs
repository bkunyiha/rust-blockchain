#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod database;
mod models;
mod services;

use config::AppState;
use std::sync::RwLock;
use tracing_subscriber;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    // Initialize database with SQLCipher encryption
    let db_password = generate_database_password();
    if let Err(e) = database::init_database(&db_password) {
        eprintln!("Failed to initialize database: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .manage(RwLock::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            // Wallet commands
            commands::wallet::create_wallet,
            commands::wallet::get_saved_wallets,
            commands::wallet::set_active_wallet,
            commands::wallet::delete_wallet_address,
            commands::wallet::update_wallet_label,
            commands::wallet::get_wallet_info,
            commands::wallet::get_balance,
            commands::wallet::send_transaction,
            commands::wallet::get_tx_history,
            // Settings commands
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::settings::check_connection,
            // Health commands
            commands::health::health_check,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Generate a secure database password - EXACT replica from bitcoin-wallet-ui-iced
fn generate_database_password() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();

    // Use username
    if let Ok(username) = std::env::var("USER") {
        username.hash(&mut hasher);
    } else if let Ok(username) = std::env::var("USERNAME") {
        username.hash(&mut hasher);
    }

    // Use home directory
    if let Some(home) = dirs::home_dir() {
        home.to_string_lossy().hash(&mut hasher);
    }

    // Use application name
    "bitcoin-wallet-ui".hash(&mut hasher);

    format!("{:x}", hasher.finish())
}
