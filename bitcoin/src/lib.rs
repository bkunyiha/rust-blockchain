pub mod primitives;
pub use primitives::*;

pub mod crypto;
pub use crypto::*;

pub mod error;
pub use error::*;

pub mod chain;
pub use chain::{BlockchainService, UTXOSet};

pub mod net;
pub use net::*;

pub mod node;
pub use node::{ConnectNode, Node, NodeContext, Nodes, Server};

mod config;
pub use config::Config;
pub use config::GLOBAL_CONFIG;

pub mod util;
pub use util::*;

pub mod store;
pub use store::*;

pub mod wallet;
pub use wallet::{Wallet, WalletAddress, WalletService, convert_address, hash_pub_key};

pub mod web;
// Don't re-export web to avoid naming conflicts.
// Use Explicit Dependencies ie When you use web types, it's clear they're from the web layer

// Moved from primitives/ to root level (Bitcoin Core alignment)
pub mod txmempool;
pub use txmempool::{BlockInTransit, MemoryPool};

pub mod pow;
pub use pow::ProofOfWork;

#[cfg(test)]
mod test_utils {
    use std::sync::Once;
    use tracing::info;

    static INIT: Once = Once::new();

    /// Global test setup - runs once before any tests
    pub fn setup_test_environment() {
        INIT.call_once(|| {
            // Set environment variable to force single-threaded tests
            unsafe {
                std::env::set_var("RUST_TEST_THREADS", "1");
            }

            // Clean up any existing test directories from previous runs
            cleanup_existing_test_directories();
        });

        // Also clean up on every call (not just once) to be more aggressive
        cleanup_existing_test_directories();
    }

    /// Global test teardown - runs after tests complete
    pub fn teardown_test_environment() {
        cleanup_existing_test_directories();
    }

    /// Clean up any existing test directories
    fn cleanup_existing_test_directories() {
        use std::path::Path;

        let current_dir = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());

        if let Ok(entries) = std::fs::read_dir(current_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy();
                    if (name_str.starts_with("test_") && name_str.contains("db_"))
                        || name_str.starts_with("test_persistence_db_")
                    {
                        info!("Cleaning up test directory: {}", name_str);
                        let _ = cleanup_test_directory_with_retry(&path.to_string_lossy());
                    }
                }
            }
        }
    }

    /// Clean up test directory with aggressive retry logic
    fn cleanup_test_directory_with_retry(db_path: &str) -> std::io::Result<()> {
        for attempt in 1..=10 {
            match std::fs::remove_dir_all(db_path) {
                Ok(_) => return Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if attempt < 10 {
                        let delay = std::time::Duration::from_millis(500 * attempt);
                        std::thread::sleep(delay);
                        continue;
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    return Ok(());
                }
                Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                    if attempt < 10 {
                        std::thread::sleep(std::time::Duration::from_millis(1000 * attempt));
                        continue;
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Global cleanup attempt {} failed for {}: {}",
                        attempt, db_path, e
                    );
                    if attempt < 10 {
                        std::thread::sleep(std::time::Duration::from_millis(800 * attempt));
                        continue;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
use test_utils::{setup_test_environment, teardown_test_environment};
