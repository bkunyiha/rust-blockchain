//! Primitives module (Bitcoin Core alignment)
//!
//! This module contains ONLY pure data structures, following Bitcoin Core's pattern.
//! Business logic belongs in other modules (consensus, node, chain, etc.)
//!
//! Bitcoin Core's primitives/:
//! - primitives/block.h - Block data structure
//! - primitives/transaction.h - Transaction data structure

pub mod block;
pub mod blockchain;
pub mod transaction;

// Re-export the core types
pub use block::{Block, GENESIS_BLOCK_PRE_BLOCK_HASH};
pub use blockchain::Blockchain;
pub use transaction::{
    TXInput, TXOutput, Transaction, WalletTransaction, WalletTransactionStatus,
    WalletTransactionType,
};

// Re-exports for moved modules (backward compatibility)
// These have moved to root level to match Bitcoin Core structure
pub use crate::pow::ProofOfWork;
pub use crate::txmempool::{BlockInTransit, MemoryPool};
