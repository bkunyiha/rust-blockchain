// Declare and defines a module for the util layer
pub mod functional_operations;
pub mod utils;

// Re-export the utils module
pub use utils::current_timestamp;
// Re-export functional utilities
pub use functional_operations::transaction as functional_transaction;
