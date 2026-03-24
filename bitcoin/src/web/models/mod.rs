// Web-specific data models for requests and responses
pub mod errors;
pub mod requests;
pub mod responses;

// Re-export models
pub use errors::*;
pub use requests::*;
pub use responses::*;
