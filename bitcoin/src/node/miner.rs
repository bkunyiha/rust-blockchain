//! Mining operations (Bitcoin Core: miner.cpp)
//!
//! This module handles block creation and mining operations, similar to
//! Bitcoin Core's miner.cpp (BlockAssembler, CreateNewBlock)

use super::txmempool::remove_from_memory_pool;
use crate::error::{BtcError, Result};
use crate::net::net_processing::send_inv;
use crate::node::{GLOBAL_MEMORY_POOL, GLOBAL_NODES, OpType};
use crate::primitives::TXOutput;
use crate::{Block, BlockchainService, GLOBAL_CONFIG, Transaction, WalletAddress};
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{info, warn};

const TRANSACTION_THRESHOLD: usize = 3;

// Fix 3: Global flags for mining cancellation and concurrency control
/// Global flag to signal mining cancellation when a competing block arrives
pub static MINING_CANCELLED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
/// Global flag to prevent concurrent mining
static MINING_IN_PROGRESS: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

/// Signal that current mining should be cancelled (called when a new block arrives)
pub fn cancel_current_mining() {
    MINING_CANCELLED.store(true, Ordering::SeqCst);
    info!("Mining cancellation signal sent");
}

/// Reset cancellation flag (called before starting a new mining attempt)
fn reset_mining_cancellation() {
    MINING_CANCELLED.store(false, Ordering::SeqCst);
}

/// Check if mining has been cancelled
pub fn is_mining_cancelled() -> bool {
    MINING_CANCELLED.load(Ordering::SeqCst)
}

/// Create coinbase transaction for mining
fn create_mining_coinbase_transaction(to: &WalletAddress) -> Result<Transaction> {
    Transaction::new_coinbase_tx(to)
}

/// Check if mining should be triggered
pub fn should_trigger_mining() -> bool {
    let pool_size = GLOBAL_MEMORY_POOL.len().expect("Memory pool length error");
    let is_miner = GLOBAL_CONFIG.is_miner();
    pool_size >= TRANSACTION_THRESHOLD && is_miner
}

/// Fix 2: Prepare UTXO for mining — validates inputs against current UTXO set
/// Prepare transactions for mining by validating inputs against the UTXO set.
///
/// This function snapshots the memory pool and filters out any transactions whose
/// inputs have already been spent (e.g., by a competing block that was accepted
/// while transactions were waiting in the mempool). Stale transactions are removed
/// from the mempool. A coinbase transaction is appended as the mining reward.
///
/// This is the first layer of stale-mining protection. A second validation pass
/// runs inside `chainstate.rs::mine_block()` under the write lock to catch any
/// remaining race conditions between this read and the actual mining.
///
/// # Arguments
/// * `mining_address` - Wallet address to receive the coinbase mining reward
/// * `blockchain` - Blockchain service for UTXO set access
///
/// # Returns
/// * `Ok(transactions)` - Valid transactions + coinbase, ready for mining
/// * `Err` - No valid transactions remain (all inputs already spent)
pub async fn prepare_mining_utxo(
    mining_address: &WalletAddress,
    blockchain: &BlockchainService,
) -> Result<Vec<Transaction>> {
    let txs = GLOBAL_MEMORY_POOL.get_all()?;

    // Validate each transaction's inputs are still unspent in the UTXO set.
    // When multiple miners compete, a competing block may have already confirmed
    // these transactions, spending their inputs. Mining with spent inputs would
    // create an invalid block with a duplicate coinbase subsidy.
    let db = blockchain.get_db().await?;
    let utxo_tree = db
        .open_tree("chainstate")
        .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;

    let mut valid_txs = Vec::new();
    for tx in txs {
        if tx.is_coinbase() {
            continue;
        }
        let mut inputs_valid = true;
        for input in tx.get_vin() {
            match utxo_tree.get(input.get_txid()) {
                Ok(Some(outs_bytes)) => {
                    let outputs: Vec<TXOutput> = bincode::serde::decode_from_slice(
                        outs_bytes.as_ref(),
                        bincode::config::standard(),
                    )
                    .map(|(v, _)| v)
                    .unwrap_or_default();
                    if input.get_vout() >= outputs.len() {
                        inputs_valid = false;
                        break;
                    }
                }
                _ => {
                    inputs_valid = false;
                    break;
                }
            }
        }
        if inputs_valid {
            valid_txs.push(tx);
        } else {
            info!(
                "Skipping transaction with already-spent inputs: {}",
                tx.get_tx_id_hex()
            );
            remove_from_memory_pool(tx, blockchain).await;
        }
    }

    if valid_txs.is_empty() {
        return Err(BtcError::InvalidValueForMiner(
            "No valid transactions to mine (all inputs already spent)".to_string(),
        ));
    }

    info!(
        "Preparing to mine with {} valid transactions",
        valid_txs.len()
    );
    let coinbase_tx = create_mining_coinbase_transaction(mining_address)?;
    let mut final_txs = valid_txs;
    final_txs.push(coinbase_tx);

    Ok(final_txs)
}

/// Process mining: create block, persist, clean mempool, and broadcast.
///
/// This is the main mining entry point called by `submit_transaction_for_mining`.
/// It includes two safety mechanisms:
///
/// 1. **Concurrency guard** (`MINING_IN_PROGRESS`): Prevents multiple mining tasks
///    from running simultaneously via `compare_exchange` on an `AtomicBool`.
///
/// 2. **Cancellation check** (`MINING_CANCELLED`): Before mining starts, checks if
///    a competing block arrived (set by `cancel_current_mining()` in the
///    `Package::Block` handler). If cancelled, mining is aborted.
///
/// **CRITICAL**: Cancellation is NOT checked after `mine_block()` completes.
/// Once the block is created and added to the local chain (tip updated, UTXO
/// updated), it MUST be broadcast. Skipping the broadcast would leave an
/// unbroadcast block in the local chain, creating a permanent fork where this
/// node has a block that no other node knows about.
pub async fn process_mine_block(
    txs: Vec<Transaction>,
    blockchain: &BlockchainService,
) -> Result<Block> {
    // Prevent concurrent mining
    if MINING_IN_PROGRESS
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        warn!("Mining already in progress, skipping");
        return Err(BtcError::InvalidValueForMiner(
            "Mining already in progress".to_string(),
        ));
    }

    // Reset cancellation flag before starting
    reset_mining_cancellation();

    let result = async {
        // Check for cancellation before mining
        if is_mining_cancelled() {
            info!("Mining cancelled before block creation");
            return Err(BtcError::InvalidValueForMiner(
                "Mining cancelled".to_string(),
            ));
        }

        let my_node_addr = GLOBAL_CONFIG.get_node_addr();

        // Mine a new block with the transactions in the memory pool
        let new_block = blockchain.mine_block(&txs).await?;

        // CRITICAL: Do NOT cancel after block creation. Once mine_block() completes,
        // the block is already in our local chain (tip updated, UTXO updated).
        // If we skip broadcasting, other nodes never learn about this block,
        // creating a permanent fork where this node has a block nobody else has.
        // The consensus mechanism (add_block on other nodes) will handle the
        // tie-breaking and reorganization when they receive this block.

        info!(
            "New block {} is mined by node {}!",
            new_block.get_hash(),
            my_node_addr
        );

        // Remove transactions from memory pool functionally
        for tx in &txs {
            remove_from_memory_pool(tx.clone(), blockchain).await;
        }

        // Broadcast new block to nodes
        broadcast_new_block(&new_block).await?;
        Ok(new_block)
    }
    .await;

    // Always release the mining lock
    MINING_IN_PROGRESS.store(false, Ordering::SeqCst);
    result
}

pub async fn broadcast_new_block(block: &Block) -> Result<()> {
    let my_node_addr = GLOBAL_CONFIG.get_node_addr();
    let nodes = GLOBAL_NODES.get_nodes().expect("Global nodes get error");
    nodes
        .iter()
        .filter(|node| !my_node_addr.eq(&node.get_addr()))
        .for_each(|node| {
            let node_addr = node.get_addr();
            let block_hash = block.get_hash_bytes();
            tokio::spawn(async move {
                send_inv(&node_addr, OpType::Block, &[block_hash]).await;
            });
        });
    Ok(())
}

/// Bitcoin mining without including user transactions is possible because the core incentive for
/// mining is the block reward (or block subsidy), not solely the transaction fees.
/// Even if there are no transactions waiting in the mempool (the holding area for unconfirmed transactions),
/// miners can still attempt to find a valid block by performing the necessary computational work.
/// The block they mine will then include the coinbase transaction,
/// which generates newly minted bitcoins as a reward to the successful miner.
///
/// Here's why miners can mine without user transactions and why it's sometimes done:
/// - **Block Reward:** This is the primary incentive for mining. Every time a miner successfully adds a block to the blockchain, they receive a fixed amount of newly created Bitcoin. This reward is currently 3.125 BTC and halves approximately every four years.
/// - **Security:** Even empty blocks (those containing only the coinbase transaction) contribute to the security of the Bitcoin network. They add to the cumulative Proof-of-Work, making it more difficult for an attacker to reverse previous transactions.
/// - **Early Mining & Network Activity:** In the early days of Bitcoin, there were few user transactions, so mining was primarily driven by the block reward. Even today, empty blocks can occur, especially if a block is found very quickly after the previous one, not giving mining pools enough time to assemble a full block with transactions.
/// - **Miner Efficiency:** Mining pools sometimes prioritize speed over including every available transaction. To maximize the chances of finding the next block and claiming the block reward, pools may begin hashing an empty block template immediately after a new block is broadcast. A full block template, containing transactions, is then sent shortly after.
///
/// In summary, Bitcoin miners can mine without including user transactions because they are rewarded with the
/// newly minted bitcoins from the coinbase transaction. This process contributes to network security and helps
/// bring new Bitcoin into circulation, even in the absence of user transactions.
///
pub async fn mine_empty_block(
    blockchain: &BlockchainService,
    wallet_address: &WalletAddress,
) -> Result<Block> {
    if GLOBAL_CONFIG.is_miner() {
        // Create only coinbase transaction for empty block
        let coinbase_tx = create_mining_coinbase_transaction(wallet_address)?;
        let txs = vec![coinbase_tx];

        // Mine the block with only coinbase transaction
        process_mine_block(txs, blockchain).await
    } else {
        Err(BtcError::NotAMiner)
    }
}

/// Clean up invalid transactions from memory pool
pub async fn cleanup_invalid_transactions() -> Result<()> {
    info!("Cleaning up invalid transactions from memory pool");
    // For now, this is a placeholder - in a production system,
    // you would validate each transaction and remove invalid ones
    // This ensures the memory pool stays clean and doesn't accumulate invalid transactions
    Ok(())
}
