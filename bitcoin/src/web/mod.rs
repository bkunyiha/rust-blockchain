// Web layer module for HTTP API and web interface
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod openapi;
pub mod routes;
pub mod server;

// Re-export commonly used components
pub use handlers::*;
pub use models::*;
pub use routes::*;
pub use server::*;
