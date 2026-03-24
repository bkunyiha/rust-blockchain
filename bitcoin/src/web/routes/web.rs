use crate::node::NodeContext;
use crate::web::openapi::create_swagger_ui;
use axum::{Router, response::Html, routing::get};
use std::sync::Arc;
use tower_http::services::ServeDir;

/// Create web UI routes (serves React app)
pub fn create_web_routes() -> Router<Arc<NodeContext>> {
    // Try to serve React app from bitcoin-web-ui/dist
    // Check multiple possible paths (workspace root or relative to binary location)
    let possible_paths = [
        "../bitcoin-web-ui/dist",
        "../../bitcoin-web-ui/dist",
        "bitcoin-web-ui/dist",
    ];

    let react_app_path = possible_paths
        .iter()
        .find(|path| std::path::Path::new(path).exists())
        .copied();

    if let Some(path) = react_app_path {
        let assets_path = format!("{}/assets", path);
        let index_path = format!("{}/index.html", path);

        // Serve React app with fallback to index.html for client-side routing
        Router::new()
            .nest_service("/assets", ServeDir::new(&assets_path))
            .route(
                "/",
                get({
                    let index_path = index_path.clone();
                    move || serve_react_app(index_path.clone())
                }),
            )
            .fallback(get({
                let index_path = index_path.clone();
                move || serve_react_app(index_path.clone())
            })) // Catch-all for React Router
            .merge(create_swagger_ui())
    } else {
        // React app not built - show helpful message
        Router::new()
            .route("/", get(react_app_not_built))
            .fallback(get(react_app_not_built))
            .merge(create_swagger_ui())
    }
}

/// Serve React app index.html (for client-side routing)
async fn serve_react_app(index_path: String) -> Html<String> {
    let path = std::path::Path::new(&index_path);
    if path.exists() {
        match std::fs::read_to_string(path) {
            Ok(content) => Html(content),
            Err(_) => {
                Html("<html><body><h1>Error loading React app</h1></body></html>".to_string())
            }
        }
    } else {
        Html("<html><body><h1>React app not built. Run 'npm run build' in bitcoin-web-ui directory.</h1></body></html>".to_string())
    }
}

/// Show message when React app is not built
async fn react_app_not_built() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>React App Not Built</title>
            <style>
                body { font-family: Arial, sans-serif; max-width: 600px; margin: 50px auto; padding: 20px; background: #1a1a1a; color: #fff; }
                h1 { color: #f7931a; }
                code { background: #2a2a2a; padding: 2px 6px; border-radius: 3px; }
            </style>
        </head>
        <body>
            <h1>React Web UI Not Built</h1>
            <p>To build the React web UI, run:</p>
            <pre><code>cd bitcoin-web-ui
npm install
npm run build</code></pre>
            <p>Then restart the blockchain server.</p>
        </body>
        </html>
    "#,
    )
}
