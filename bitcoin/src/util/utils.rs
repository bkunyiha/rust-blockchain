use std::time::{SystemTime, UNIX_EPOCH};

///
/// The `current_timestamp` function returns the current Unix timestamp in milliseconds.
/// It uses the system time to generate a timestamp that represents the current moment
/// in time since the Unix epoch (January 1, 1970, 00:00:00 UTC).
///
/// # Usage Examples
///
/// - **Block creation**: Used in `Block::new_block()` to timestamp new blocks
/// - **Blockchain operations**: Used to record when blocks are created and added to the chain
/// - **Mining**: Used to timestamp blocks during the mining process
/// - **Transaction ordering**: Used to establish temporal ordering of blockchain events
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/core/block.rs`**: Used in `Block::new_block()` to set the block timestamp
///
/// # Returns
///
/// A 64-bit integer representing the current Unix timestamp in milliseconds.
///
/// # Error Handling
///
/// This function will panic if the system time is set to a date before the Unix epoch,
/// which should never happen in normal operation.
pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}
