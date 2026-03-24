//! Memory pool management (Bitcoin Core: txmempool.cpp)
//!
//! This module handles the transaction memory pool data structure,
//! similar to Bitcoin Core's txmempool.cpp (CTxMemPool class)

use crate::error::Result;
use crate::node::GLOBAL_MEMORY_POOL;
use crate::{BlockchainService, Transaction, UTXOSet};
use tracing::debug;

/// Add transaction to memory pool
///
/// This is the core mempool operation that adds a transaction to the pool
/// and updates UTXO set flags.
pub async fn add_to_memory_pool(
    tx: Transaction,
    blockchain_service: &BlockchainService,
) -> Result<()> {
    debug!("\n");
    debug!(
        "******************************************************************************************************"
    );
    debug!(
        "Adding transaction to memory pool: {:?}",
        tx.get_tx_id_hex()
    );
    debug!(
        "******************************************************************************************************\n"
    );
    GLOBAL_MEMORY_POOL
        .add(tx.clone())
        .expect("Memory pool add error");

    let utxo_set = UTXOSet::new(blockchain_service.clone());
    utxo_set.set_global_mem_pool_flag(&tx.clone(), true).await?;

    Ok(())
}

/// Remove transaction from memory pool
///
/// This is the core mempool operation that removes a transaction from the pool
/// and updates UTXO set flags.
pub async fn remove_from_memory_pool(tx: Transaction, blockchain: &BlockchainService) {
    GLOBAL_MEMORY_POOL
        .remove(tx.clone())
        .expect("Memory pool remove error");

    let utxo_set = UTXOSet::new(blockchain.clone());
    utxo_set
        .set_global_mem_pool_flag(&tx.clone(), false)
        .await
        .expect("Failed to get blockchain");
}

/// Check if transaction exists in memory pool
///
/// Quick lookup to check if a transaction is already in the mempool.
pub fn transaction_exists_in_pool(tx: &Transaction) -> bool {
    GLOBAL_MEMORY_POOL.contains_transaction(tx).unwrap_or(false)
}
