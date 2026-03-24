use std::sync::OnceLock;

// Store the Tokio runtime handle globally so tasks can access it
static TOKIO_HANDLE: OnceLock<tokio::runtime::Handle> = OnceLock::new();

pub fn init_runtime() {
    // Create a Tokio runtime for async operations
    // This must outlive the application to keep the reactor running
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    // Store the handle globally so it can be accessed from any thread
    TOKIO_HANDLE
        .set(rt.handle().clone())
        .expect("Failed to set Tokio handle");

    // Keep the runtime alive in a background thread
    std::thread::spawn(move || {
        rt.block_on(async {
            // Keep the runtime alive indefinitely
            std::future::pending::<()>().await;
        });
    });
}

// Helper function to wrap a future to ensure it runs on Tokio runtime
pub fn spawn_on_tokio<F>(fut: F) -> impl std::future::Future<Output = F::Output> + Send
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    let handle = TOKIO_HANDLE
        .get()
        .expect("Tokio runtime not initialized")
        .clone();
    async move { handle.spawn(fut).await.unwrap() }
}
