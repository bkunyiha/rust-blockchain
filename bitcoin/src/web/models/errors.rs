use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

/// Web-specific error types
#[derive(Debug, Serialize, Deserialize)]
pub enum WebError {
    ValidationError(String),
    NotFound(String),
    InternalError(String),
    Unauthorized(String),
    RateLimitExceeded,
    InvalidRequest(String),
    ServiceUnavailable(String),
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            WebError::NotFound(msg) => write!(f, "Not found: {}", msg),
            WebError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            WebError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            WebError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            WebError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            WebError::ServiceUnavailable(msg) => write!(f, "Service unavailable: {}", msg),
        }
    }
}

impl std::error::Error for WebError {}

/// HTTP status code mapping for web errors
impl WebError {
    pub fn status_code(&self) -> u16 {
        match self {
            WebError::ValidationError(_) => 400,
            WebError::NotFound(_) => 404,
            WebError::InternalError(_) => 500,
            WebError::Unauthorized(_) => 401,
            WebError::RateLimitExceeded => 429,
            WebError::InvalidRequest(_) => 400,
            WebError::ServiceUnavailable(_) => 503,
        }
    }
}

/// Error response model
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: u16,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl From<WebError> for ErrorResponse {
    fn from(err: WebError) -> Self {
        Self {
            error: format!("{}", err),
            message: format!("{}", err),
            status_code: err.status_code(),
            timestamp: chrono::Utc::now(),
        }
    }
}
