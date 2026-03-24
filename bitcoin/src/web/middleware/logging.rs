use tower_http::trace::TraceLayer;

/// Create logging middleware for the web server
pub fn create_logging_layer() -> impl tower::Layer<axum::Router> + Clone {
    TraceLayer::new_for_http()
}
