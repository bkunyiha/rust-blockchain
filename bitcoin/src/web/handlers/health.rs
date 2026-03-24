use axum::{extract::State, http::StatusCode, response::Json};
use std::sync::Arc;
use std::time::Instant;

use crate::node::NodeContext;
use crate::web::models::{ApiResponse, HealthResponse};

/// Health check endpoint
///
/// Returns the current health status of the blockchain node including
/// system metrics, blockchain height, and operational status.
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Health check successful", body = ApiResponse<HealthResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn health_check(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<HealthResponse>>, StatusCode> {
    let start_time = Instant::now();

    // Get blockchain information
    let height = node
        .get_blockchain_height()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Calculate uptime (simplified - in real implementation, track start time)
    let uptime_seconds = start_time.elapsed().as_secs();

    // Get memory usage (simplified)
    let memory_usage_mb = get_memory_usage();

    // Get connected peers
    let connected_peers = node.get_peer_count().unwrap_or(0);

    let health_response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds,
        blockchain_height: height,
        connected_peers,
        memory_usage_mb,
    };

    Ok(Json(ApiResponse::success(health_response)))
}

/// Liveness probe endpoint
///
/// Simple endpoint to check if the service is alive.
/// Used by container orchestration systems for health monitoring.
#[utoipa::path(
    get,
    path = "/health/live",
    tag = "Health",
    responses(
        (status = 200, description = "Service is alive", body = ApiResponse<String>)
    )
)]
pub async fn liveness() -> Result<Json<ApiResponse<String>>, StatusCode> {
    Ok(Json(ApiResponse::success("alive".to_string())))
}

/// Readiness probe endpoint
///
/// Checks if the service is ready to accept requests.
/// Verifies blockchain connectivity and system readiness.
#[utoipa::path(
    get,
    path = "/health/ready",
    tag = "Health",
    responses(
        (status = 200, description = "Service is ready", body = ApiResponse<String>),
        (status = 503, description = "Service not ready")
    )
)]
pub async fn readiness(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // Check if blockchain is accessible
    match node.get_blockchain_height().await {
        Ok(_) => Ok(Json(ApiResponse::success("ready".to_string()))),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// Get memory usage in MB (simplified implementation)
fn get_memory_usage() -> f64 {
    // This is a simplified implementation
    // In a real application, you'd use a proper memory monitoring library
    0.0
}
