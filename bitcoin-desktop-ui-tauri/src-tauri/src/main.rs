mod commands;
mod config;
mod models;
mod services;

use config::ApiConfig;
use std::sync::RwLock;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let api_config = ApiConfig::default();

    tauri::Builder::default()
        .manage(RwLock::new(api_config))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Blockchain commands
            commands::get_blockchain_info,
            commands::get_latest_blocks,
            commands::get_all_blocks,
            commands::get_block_by_hash,
            // Wallet commands
            commands::create_wallet,
            commands::get_wallet_info,
            commands::get_balance,
            commands::send_transaction,
            commands::get_tx_history,
            commands::get_all_addresses,
            // Transaction commands
            commands::get_mempool,
            commands::get_mempool_transaction,
            commands::get_all_transactions,
            commands::get_address_transactions,
            // Mining commands
            commands::get_mining_info,
            commands::generate_blocks,
            // Health commands
            commands::health_check,
            commands::liveness_check,
            commands::readiness_check,
            // Settings commands
            commands::get_config,
            commands::update_config,
            commands::check_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
