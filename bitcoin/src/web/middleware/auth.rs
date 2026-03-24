use axum::http::StatusCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Wallet,
    Admin,
}

// Simple header-based API key auth -> role mapping.
// For production, replace with a secure key store.
pub async fn require_role(
    mut req: axum::http::Request<axum::body::Body>,
    required: Role,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let key = req.headers().get("X-API-Key").and_then(|h| h.to_str().ok());

    let caller_role = match key {
        Some(k) if is_admin_key(k) => Role::Admin,
        Some(k) if is_wallet_key(k) => Role::Wallet,
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    // Admin can access wallet routes too
    let allowed =
        caller_role == required || (caller_role == Role::Admin && required == Role::Wallet);
    if !allowed {
        return Err(StatusCode::FORBIDDEN);
    }

    // Attach role to extensions if needed by handlers
    req.extensions_mut().insert(caller_role);

    Ok(next.run(req).await)
}

pub async fn require_admin(
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    require_role(req, Role::Admin, next).await
}

pub async fn require_wallet(
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    require_role(req, Role::Wallet, next).await
}

fn is_admin_key(k: &str) -> bool {
    // Read from env vars; fallback to defaults
    let expected =
        std::env::var("BITCOIN_API_ADMIN_KEY").unwrap_or_else(|_| "admin-secret".to_string());
    k == expected
}

fn is_wallet_key(k: &str) -> bool {
    let expected =
        std::env::var("BITCOIN_API_WALLET_KEY").unwrap_or_else(|_| "wallet-secret".to_string());
    k == expected
}
