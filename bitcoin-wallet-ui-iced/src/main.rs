mod api;
mod app;
mod database;
mod runtime;
mod types;
mod update;
mod view;

use app::WalletApp;
use iced::{Theme, application};
use runtime::init_runtime;
use update::update;
use view::view;

fn title(_: &WalletApp) -> String {
    "Bitcoin Wallet UI".to_string()
}

fn theme(_: &WalletApp) -> Theme {
    Theme::Dark
}

fn main() -> iced::Result {
    // Initialize Tokio runtime for async operations
    init_runtime();

    // Initialize database with SQLCipher encryption
    // Generate a secure password based on machine/user-specific data
    let db_password = generate_database_password();
    if let Err(e) = database::init_database(&db_password) {
        eprintln!("Failed to initialize database: {}", e);
        // Continue anyway - settings will use defaults
    }

    // Run the application
    application(WalletApp::new, update, view)
        .title(title)
        .theme(theme)
        .run()
}

/// Generate a secure database password
/// Uses a combination of machine-specific and user-specific data
/// This ensures the database is encrypted but doesn't require user input
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

    // Convert to hex string
    format!("{:x}", hasher.finish())
}
