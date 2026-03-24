//! Blockchain state management (Bitcoin Core: chain/)
//!
//! This module manages the active blockchain state, similar to
//! Bitcoin Core's chain/ directory which contains:
//! - CChainState: Active blockchain state
//! - CCoinsView: UTXO set management
//! - CBlockIndex: Block indexing

pub mod chainstate;
pub mod utxo_set;

// Re-export main types for convenience
pub use chainstate::BlockchainService;
pub use utxo_set::UTXOSet;
