// SPDX-License-Identifier: MIT OR Apache-2.0
// Network layer (Bitcoin Core: src/net/)
// P2P networking and protocol operations
pub mod net_processing;

// Re-export the modules
pub use net_processing::*;
