mod api;
mod app;
mod runtime;
mod types;
mod update;
mod view;

use app::AdminApp;
use iced::{Theme, application};
use runtime::init_runtime;
use update::update;
use view::view;

fn title(_: &AdminApp) -> String {
    "Bitcoin Admin UI".to_string()
}

fn theme(_: &AdminApp) -> Theme {
    Theme::Dark
}

fn main() -> iced::Result {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .init();

    // Initialize Tokio runtime for async operations
    init_runtime();

    // Run the application
    application(AdminApp::new, update, view)
        .title(title)
        .theme(theme)
        .run()
}
