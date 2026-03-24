use tower_http::cors::{Any, CorsLayer};

/// Create CORS middleware for the web server
pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers(Any)
        .max_age(std::time::Duration::from_secs(86400)) // 24 hours
}

/// Create CORS middleware with specific origins
pub fn create_cors_layer_with_origins(origins: Vec<String>) -> CorsLayer {
    let mut cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers(Any)
        .max_age(std::time::Duration::from_secs(86400));

    // Add specific origins
    for origin in origins {
        if let Ok(parsed_origin) = origin.parse::<axum::http::HeaderValue>() {
            cors = cors.allow_origin(parsed_origin);
        }
    }

    cors
}
