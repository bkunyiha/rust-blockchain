//! Node orchestration module (Bitcoin Core style)
//!
//! This module provides a clean interface for the web/RPC layer to interact
//! with the blockchain node, following Bitcoin Core's architecture.

pub mod context;
pub mod miner;
pub mod peers;
pub mod server;
pub mod txmempool;

pub use context::NodeContext;
pub use miner::{mine_empty_block, prepare_mining_utxo, process_mine_block, should_trigger_mining};
pub use peers::{Node, Nodes};
pub use server::*;
pub use txmempool::{add_to_memory_pool, remove_from_memory_pool, transaction_exists_in_pool};
