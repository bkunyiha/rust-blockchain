use axum_rate_limiter::{limiter::RateLimiterManager, settings::Settings};
use std::sync::Arc;

/// Rate limiting configuration.
///
/// The underlying `axum_rate_limiter` crate is configured via a TOML file.
/// By convention, the file path can be provided via the `RL_SETTINGS_PATH`
/// environment variable; otherwise the crate's defaults apply.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Enable/disable rate limiting globally.
    pub enabled: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Build a rate limiter manager from the configured settings.
///
/// Returns `Ok(None)` when rate limiting is disabled.
pub fn build_rate_limiter_manager(
    config: &RateLimitConfig,
) -> Result<Option<Arc<RateLimiterManager>>, Box<dyn std::error::Error + Send + Sync>> {
    if !config.enabled {
        return Ok(None);
    }

    // The crate reads settings from a TOML file at `RL_SETTINGS_PATH` (default: ./Settings.toml).
    // Note: setting env vars at runtime is `unsafe` in Rust 2024, so configure it externally.
    let settings = Settings::new()?;
    let manager = RateLimiterManager::new(settings.rate_limiter_settings)?;
    Ok(Some(Arc::new(manager)))
}
