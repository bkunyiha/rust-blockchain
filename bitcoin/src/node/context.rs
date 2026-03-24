//! Node Context - Central Coordination Point for Node Operations
//!
//! This module provides the `NodeContext` struct, which serves as the primary
//! interface for coordinating all blockchain node operations. Following Bitcoin Core's
//! architecture, it orchestrates interactions between blockchain state, transaction
//! mempool, network operations, and validation.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │         Web/RPC Layer                   │
//! └──────────────┬──────────────────────────┘
//!                │
//!         ┌──────▼──────┐
//!         │ NodeContext │  ← Central coordination
//!         └──────┬──────┘
//!                │
//!    ┌───────────┼───────────┬───────────┐
//!    │           │           │           │
//! ┌──▼──┐   ┌───▼────┐  ┌───▼───┐  ┌───▼──────┐
//! │Chain│   │Mempool │  │Network│  │Validation│
//! └─────┘   └────────┘  └───────┘  └──────────┘
//! ```
//!
//! # Design Principles
//!
//! - **Single Responsibility**: Each method has one clear purpose
//! - **Bitcoin Core Alignment**: Mirrors Bitcoin's validation.cpp and node context patterns
//! - **Clean Separation**: Blockchain, mempool, network, and validation concerns are distinct
//! - **Thread Safety**: All operations are thread-safe through Arc/RwLock patterns
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use blockchain::node::NodeContext;
//! use blockchain::BlockchainService;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize blockchain
//! let blockchain = BlockchainService::default().await?;
//!
//! // Create node context
//! let node = NodeContext::new(blockchain);
//!
//! // Query blockchain state
//! let height = node.get_blockchain_height().await?;
//! println!("Current height: {}", height);
//!
//! // Get mempool info
//! let mempool_size = node.get_mempool_size()?;
//! println!("Mempool transactions: {}", mempool_size);
//! # Ok(())
//! # }
//! ```

use crate::GLOBAL_CONFIG;
use crate::chain::{BlockchainService, UTXOSet};
use crate::error::{BtcError, Result};
use crate::net::net_processing::send_inv;
use crate::node::miner;
use crate::node::miner::{
    cleanup_invalid_transactions, prepare_mining_utxo, process_mine_block, should_trigger_mining,
};
use crate::node::txmempool::{
    add_to_memory_pool, remove_from_memory_pool, transaction_exists_in_pool,
};
use crate::node::{CENTERAL_NODE, GLOBAL_NODES, Node, OpType};
use crate::transaction::TxSummary;
use crate::{Block, Transaction, WalletAddress, WalletTransaction};
use std::collections::HashMap;
use std::net::SocketAddr;
use tracing::{error, info, warn};

/// Node context - central coordination point for all node operations
///
/// `NodeContext` is the primary interface for coordinating blockchain node operations.
/// It provides a clean abstraction over blockchain state, transaction mempool,
/// network operations, and validation logic.
///
/// Following Bitcoin Core's architecture, this struct serves a similar role to
/// Bitcoin's validation context and node state manager.
///
/// # Thread Safety
///
/// This struct is `Clone` + `Send` + `Sync`, allowing safe sharing across
/// async tasks and thread boundaries. All internal state uses appropriate
/// synchronization primitives.
#[derive(Clone, Debug)]
pub struct NodeContext {
    /// Blockchain service - manages chain state and block storage
    blockchain: BlockchainService,
}

impl NodeContext {
    //=============================================================================
    // Initialization
    //=============================================================================

    /// Create a new node context
    ///
    /// # Arguments
    ///
    /// * `blockchain` - The blockchain service to coordinate with
    ///
    /// # Returns
    ///
    /// A new `NodeContext` instance ready for operation
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::{BlockchainService, node::NodeContext};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let blockchain = BlockchainService::default().await?;
    /// let node = NodeContext::new(blockchain);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(blockchain: BlockchainService) -> Self {
        Self { blockchain }
    }

    /// Get reference to underlying blockchain service
    ///
    /// # Returns
    ///
    /// Immutable reference to the `BlockchainService`
    ///
    /// # Note
    ///
    /// This is provided for cases where direct blockchain access is needed,
    /// but prefer using the high-level methods on `NodeContext` when possible.
    pub fn get_blockchain(&self) -> &BlockchainService {
        &self.blockchain
    }

    /// Get reference to blockchain service (alias for compatibility)
    ///
    /// # Returns
    ///
    /// Immutable reference to the `BlockchainService`
    pub fn blockchain(&self) -> &BlockchainService {
        &self.blockchain
    }

    //=============================================================================
    // Blockchain State Methods
    //=============================================================================

    /// Add a block to the blockchain
    ///
    /// Adds a validated block to the blockchain and updates the chain state.
    /// This operation updates the chain tip and persists the block to storage.
    ///
    /// # Arguments
    ///
    /// * `block` - The block to add to the chain
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Block successfully added
    /// * `Err(_)` - Block validation failed or storage error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::{Block, node::NodeContext};
    /// # async fn example(node: &NodeContext, block: &Block) -> Result<(), Box<dyn std::error::Error>> {
    /// node.add_block(block).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_block(&self, block: &Block) -> Result<()> {
        self.blockchain.add_block(block).await
    }

    /// Get current blockchain height
    ///
    /// Returns the height of the best (longest) chain. Genesis block is height 1.
    ///
    /// # Returns
    ///
    /// * `Ok(height)` - Current blockchain height
    /// * `Err(_)` - Database error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::node::NodeContext;
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let height = node.get_blockchain_height().await?;
    /// println!("Current height: {}", height);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_blockchain_height(&self) -> Result<usize> {
        self.blockchain.get_best_height().await
    }

    /// Get all block hashes in the blockchain
    ///
    /// Returns a vector of all block hashes in the current best chain,
    /// ordered from genesis to tip.
    ///
    /// # Returns
    ///
    /// * `Ok(hashes)` - Vector of block hashes (32-byte binary)
    /// * `Err(_)` - Database error
    ///
    /// # Note
    ///
    /// Block hashes are returned as `Vec<u8>` for efficiency. Use
    /// `hex::encode()` to convert to human-readable format.
    pub async fn get_block_hashes(&self) -> Result<Vec<Vec<u8>>> {
        self.blockchain.get_block_hashes().await
    }

    /// Get a block by its hash (binary format)
    ///
    /// # Arguments
    ///
    /// * `block_hash` - The block hash as a byte slice (32 bytes)
    ///
    /// # Returns
    ///
    /// * `Ok(Some(block))` - Block found
    /// * `Ok(None)` - Block not found
    /// * `Err(_)` - Database error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::node::NodeContext;
    /// # async fn example(node: &NodeContext, hash: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    /// if let Some(block) = node.get_block(hash).await? {
    ///     println!("Found block at height: {}", block.get_height());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_block(&self, block_hash: &[u8]) -> Result<Option<Block>> {
        self.blockchain.get_block(block_hash).await
    }

    /// Get a block by its hash (hex string format)
    ///
    /// Convenience method that accepts hex-encoded block hash strings.
    ///
    /// # Arguments
    ///
    /// * `hash` - The block hash as a hex string
    ///
    /// # Returns
    ///
    /// * `Ok(Some(block))` - Block found
    /// * `Ok(None)` - Block not found
    /// * `Err(_)` - Database error or invalid hex
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::node::NodeContext;
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let hash = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";
    /// if let Some(block) = node.get_block_by_hash(hash).await? {
    ///     println!("Block found!");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_block_by_hash(&self, hash: &str) -> Result<Option<Block>> {
        self.blockchain.get_block_by_hash(hash.as_bytes()).await
    }

    /// Get the latest N blocks from the blockchain
    ///
    /// Returns the most recent blocks up to the specified count.
    /// Useful for displaying recent blockchain activity.
    ///
    /// # Arguments
    ///
    /// * `count` - Maximum number of blocks to return
    ///
    /// # Returns
    ///
    /// * `Ok(blocks)` - Vector of blocks (newest first)
    /// * `Err(_)` - Database error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::node::NodeContext;
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let latest_blocks = node.get_latest_blocks(10).await?;
    /// for block in latest_blocks {
    ///     println!("Block hash: {}", block.get_hash());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_latest_blocks(&self, count: usize) -> Result<Vec<Block>> {
        let height = self.blockchain.get_best_height().await?;
        let start_height = height.saturating_sub(count);

        self.blockchain
            .get_blocks_by_height(start_height, height)
            .await
    }

    /// Mine a new block with the given transactions
    ///
    /// Creates a new block containing the specified transactions and mines it
    /// by finding a valid proof-of-work nonce.
    ///
    /// # Arguments
    ///
    /// * `transactions` - Slice of transactions to include in the block
    ///
    /// # Returns
    ///
    /// * `Ok(block)` - The newly mined block
    /// * `Err(_)` - Mining or validation error
    ///
    /// # Note
    ///
    /// This is a blocking operation that may take significant time depending
    /// on the proof-of-work difficulty.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::{Transaction, node::NodeContext};
    /// # async fn example(node: &NodeContext, txs: &[Transaction]) -> Result<(), Box<dyn std::error::Error>> {
    /// let block = node.mine_block(txs).await?;
    /// println!("Mined block: {}", block.get_hash());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
        self.blockchain.mine_block(transactions).await
    }

    /// Mine an empty block (only coinbase transaction)
    ///
    /// Creates and mines a block with no user transactions, only the
    /// mandatory coinbase transaction for miner rewards.
    ///
    /// # Returns
    ///
    /// * `Ok(block)` - The newly mined empty block
    /// * `Err(_)` - Mining error
    ///
    /// # Use Cases
    ///
    /// - Testing blockchain progression
    /// - Keeping the chain moving when mempool is empty
    /// - Network synchronization testing
    pub async fn mine_empty_block(&self, wallet_address: &WalletAddress) -> Result<Block> {
        miner::mine_empty_block(&self.blockchain, wallet_address).await
    }

    /// Find all transactions across the entire blockchain
    ///
    /// Scans all blocks and returns a summary of every transaction in the chain.
    /// This is an expensive operation on large blockchains.
    ///
    /// # Returns
    ///
    /// * `Ok(map)` - HashMap of transaction ID (hex) to transaction summary
    /// * `Err(_)` - Database or scanning error
    ///
    /// # Performance
    ///
    /// O(n) where n is the total number of transactions in the blockchain.
    /// Use sparingly on production systems.
    pub async fn find_all_transactions(&self) -> Result<HashMap<String, TxSummary>> {
        self.blockchain.find_all_transactions().await
    }

    /// Find all transactions for a given wallet address
    ///
    /// Scans all blocks and returns a vector of every transaction for the given wallet address.
    /// This is an expensive operation on large blockchains.
    ///
    /// # Returns
    ///
    /// * `Ok(transactions)` - Vector of wallet transactions
    /// * `Err(_)` - Database or scanning error
    ///
    pub async fn find_user_transaction(
        &self,
        address: &WalletAddress,
    ) -> Result<Vec<WalletTransaction>> {
        self.blockchain.find_user_transaction(address).await
    }

    //=============================================================================
    // Transaction Mempool Methods
    //=============================================================================

    /// Create and submit a Bitcoin transaction
    ///
    /// Creates a UTXO transaction transferring funds from one address to another,
    /// then submits it to the mempool and broadcasts it to the network.
    ///
    /// # Arguments
    ///
    /// * `wlt_frm_addr` - Source wallet address
    /// * `wlt_to_addr` - Destination wallet address
    /// * `amount` - Amount to transfer (in satoshis)
    ///
    /// # Returns
    ///
    /// * `Ok(txid)` - Transaction ID (hex) if successful
    /// * `Err(_)` - Insufficient funds, invalid address, or validation error
    ///
    /// # Process Flow
    ///
    /// 1. Create UTXO set
    /// 2. Build and sign transaction
    /// 3. Validate transaction
    /// 4. Add to mempool
    /// 5. Broadcast to network
    /// 6. Trigger mining if threshold met
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::{WalletAddress, node::NodeContext};
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let from = WalletAddress::validate("1A1zP1eP...".to_string())?;
    /// let to = WalletAddress::validate("1BvBMSEY...".to_string())?;
    ///
    /// let txid = node.btc_transaction(&from, &to, 50).await?;
    /// println!("Transaction submitted: {}", txid);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn btc_transaction(
        &self,
        wlt_frm_addr: &WalletAddress,
        wlt_to_addr: &WalletAddress,
        amount: i32,
    ) -> Result<String> {
        // Create UTXO set for transaction building
        let utxo_set = UTXOSet::new(self.blockchain.clone());

        // Create and sign the transaction
        let utxo =
            Transaction::new_utxo_transaction(wlt_frm_addr, wlt_to_addr, amount, &utxo_set).await?;

        // Process through mempool and network
        let addr_from = crate::GLOBAL_CONFIG.get_node_addr();
        self.process_transaction(&addr_from, utxo).await
    }

    /// Submit a pre-built transaction to the mempool
    ///
    /// Similar to Bitcoin Core's `BroadcastTransaction`. Accepts an already-created
    /// transaction, validates it, adds to mempool, and broadcasts to peers.
    ///
    /// # Arguments
    ///
    /// * `addr_from` - Source address (for network tracking)
    /// * `utxo` - The transaction to submit
    ///
    /// # Returns
    ///
    /// * `Ok(txid)` - Transaction ID (hex) if accepted
    /// * `Err(_)` - Validation failed or already in mempool
    ///
    /// # Validation Checks
    ///
    /// - Transaction not already in mempool
    /// - Valid transaction structure
    /// - Sufficient input funds
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::{Transaction, node::NodeContext};
    /// # use std::net::SocketAddr;
    /// # async fn example(node: &NodeContext, tx: Transaction) -> Result<(), Box<dyn std::error::Error>> {
    /// let addr = "127.0.0.1:8080".parse()?;
    /// let txid = node.submit_transaction(&addr, tx).await?;
    /// println!("Submitted: {}", txid);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn submit_transaction(
        &self,
        addr_from: &std::net::SocketAddr,
        utxo: Transaction,
    ) -> Result<String> {
        self.process_transaction(addr_from, utxo).await
    }

    /// Process transaction - core mempool acceptance logic
    ///
    /// This is the main entry point for transaction processing, similar to
    /// Bitcoin Core's `AcceptToMemoryPool` and `ProcessNewTransaction`.
    ///
    /// # Arguments
    ///
    /// * `addr_from` - Source peer address (for network coordination)
    /// * `utxo` - The transaction to process
    ///
    /// # Returns
    ///
    /// * `Ok(txid)` - Transaction accepted, returns transaction ID (hex)
    /// * `Err(TransactionAlreadyExistsInMemoryPool)` - Duplicate transaction
    /// * `Err(_)` - Other validation or processing error
    ///
    /// # Process Flow
    ///
    /// 1. **Check for duplicates** - Reject if already in mempool
    /// 2. **Add to mempool** - Store transaction for mining consideration
    /// 3. **Broadcast** - If central node, relay to other peers (background)
    /// 4. **Trigger mining** - If threshold met, start mining (background)
    /// 5. **Return txid** - Immediately return to caller
    ///
    /// # Background Operations
    ///
    /// Steps 3-4 run asynchronously to prevent blocking the caller.
    /// This follows Bitcoin's pattern of immediate acceptance with async propagation.
    ///
    /// # Bitcoin Core Equivalent
    ///
    /// ```cpp
    /// // Bitcoin Core: validation.cpp
    /// bool AcceptToMemoryPool(CTxMemPool& pool, CValidationState& state,
    ///                         const CTransactionRef& ptx, ...)
    /// ```
    pub async fn process_transaction(
        &self,
        addr_from: &std::net::SocketAddr,
        utxo: Transaction,
    ) -> Result<String> {
        // Check if transaction already exists in mempool
        if transaction_exists_in_pool(&utxo) {
            info!("Transaction: {:?} already exists", utxo.get_id());
            return Err(BtcError::TransactionAlreadyExistsInMemoryPool(
                utxo.get_tx_id_hex(),
            ));
        }

        // Add to memory pool
        add_to_memory_pool(utxo.clone(), &self.blockchain).await?;

        // Submit transaction for mining and broadcast in background
        // This prevents blocking the API response
        let context = self.clone();
        let addr_copy = *addr_from;
        let tx = utxo.clone();
        tokio::spawn(async move {
            let _ = context.submit_transaction_for_mining(&addr_copy, tx).await;
        });

        // Return transaction ID immediately
        Ok(utxo.get_tx_id_hex())
    }

    /// Get a transaction from the mempool by ID
    ///
    /// Looks up a transaction in the mempool using its transaction ID.
    ///
    /// # Arguments
    ///
    /// * `txid` - Transaction ID as hex string
    ///
    /// # Returns
    ///
    /// * `Ok(Some(tx))` - Transaction found in mempool
    /// * `Ok(None)` - Transaction not in mempool
    /// * `Err(_)` - Mempool access error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::node::NodeContext;
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let txid = "9a2f3c4d5e6f...";
    /// if let Some(tx) = node.get_mempool_transaction(txid)? {
    ///     println!("Found transaction in mempool");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_mempool_transaction(&self, txid: &str) -> Result<Option<Transaction>> {
        use crate::node::GLOBAL_MEMORY_POOL;
        GLOBAL_MEMORY_POOL.get(txid)
    }

    /// Get all transactions currently in the mempool
    ///
    /// Returns a snapshot of all unconfirmed transactions waiting to be mined.
    ///
    /// # Returns
    ///
    /// * `Ok(transactions)` - Vector of all mempool transactions
    /// * `Err(_)` - Mempool access error
    ///
    /// # Note
    ///
    /// This creates a copy of all transactions. For large mempools,
    /// consider using `get_mempool_size()` or pagination.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::node::NodeContext;
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let pending = node.get_mempool_transactions()?;
    /// println!("Pending transactions: {}", pending.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_mempool_transactions(&self) -> Result<Vec<Transaction>> {
        use crate::node::GLOBAL_MEMORY_POOL;
        GLOBAL_MEMORY_POOL.get_all()
    }

    /// Get the current size of the mempool
    ///
    /// Returns the number of transactions currently waiting in the mempool.
    ///
    /// # Returns
    ///
    /// * `Ok(count)` - Number of transactions in mempool
    /// * `Err(_)` - Mempool access error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::node::NodeContext;
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let size = node.get_mempool_size()?;
    /// println!("Mempool size: {} transactions", size);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_mempool_size(&self) -> Result<usize> {
        use crate::node::GLOBAL_MEMORY_POOL;
        GLOBAL_MEMORY_POOL.len()
    }

    /// Remove a transaction from the mempool
    ///
    /// Removes a transaction from the mempool and updates UTXO flags.
    /// Typically called after a transaction is confirmed in a block.
    ///
    /// # Arguments
    ///
    /// * `tx` - The transaction to remove
    ///
    /// # Note
    ///
    /// This also clears the "in mempool" flag on the transaction's outputs
    /// in the UTXO set.
    pub async fn remove_from_memory_pool(&self, tx: Transaction) {
        remove_from_memory_pool(tx, &self.blockchain).await;
    }

    //=============================================================================
    // Wallet Operations
    //=============================================================================

    /// Get balance for a wallet address
    ///
    /// Calculates the total balance (sum of unspent outputs) for the given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The wallet address to check
    ///
    /// # Returns
    ///
    /// * `Ok(balance)` - Total balance in satoshis
    /// * `Err(_)` - Invalid address or UTXO set error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::{WalletAddress, node::NodeContext};
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let addr = WalletAddress::validate("1A1zP1eP...".to_string())?;
    /// let balance = node.get_balance(&addr).await?;
    /// println!("Balance: {} satoshis", balance);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_balance(&self, address: &WalletAddress) -> Result<i32> {
        let utxo_set = UTXOSet::new(self.blockchain.clone());
        utxo_set.get_balance(address).await
    }

    /// Create a new wallet
    ///
    /// Generates a new wallet with a fresh key pair and address.
    ///
    /// # Returns
    ///
    /// * `Ok(wallet)` - Newly created wallet
    /// * `Err(_)` - Cryptographic error or storage failure
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::node::NodeContext;
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let wallet = node.create_wallet()?;
    /// let address = wallet.get_address()?;
    /// println!("New wallet address: {}", address.as_str());
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_wallet(&self) -> Result<crate::Wallet> {
        crate::Wallet::new()
    }

    /// Get a wallet by its address
    ///
    /// Retrieves a wallet from the wallet service by its address.
    ///
    /// # Arguments
    ///
    /// * `address` - The wallet address to look up
    ///
    /// # Returns
    ///
    /// * `Ok(Some(wallet))` - Wallet found
    /// * `Ok(None)` - Wallet not found
    /// * `Err(_)` - Wallet service error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::{WalletAddress, node::NodeContext};
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let addr = WalletAddress::validate("1A1zP1eP...".to_string())?;
    /// if let Some(wallet) = node.get_wallet(&addr)? {
    ///     println!("Found wallet");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_wallet(&self, address: &WalletAddress) -> Result<Option<crate::Wallet>> {
        use crate::wallet::WalletService;
        let wallets = WalletService::new()?;
        Ok(wallets.get_wallet(address).cloned())
    }

    /// List all wallet addresses
    ///
    /// Returns all wallet addresses currently managed by this node.
    ///
    /// # Returns
    ///
    /// * `Ok(addresses)` - Vector of wallet addresses
    /// * `Err(_)` - Wallet service error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::node::NodeContext;
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let addresses = node.list_wallet_addresses()?;
    /// for addr in addresses {
    ///     println!("Wallet: {}", addr.as_str());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_wallet_addresses(&self) -> Result<Vec<WalletAddress>> {
        use crate::wallet::WalletService;
        let wallets = WalletService::new()?;
        Ok(wallets.get_addresses())
    }

    //=============================================================================
    // Network Operations Methods
    //=============================================================================

    /// Get all connected peers
    ///
    /// Returns the socket addresses of all currently connected peer nodes.
    ///
    /// # Returns
    ///
    /// * `Ok(peers)` - Vector of peer socket addresses
    /// * `Err(_)` - Network state access error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::node::NodeContext;
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let peers = node.get_peers()?;
    /// for peer in peers {
    ///     println!("Connected to: {}", peer);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_peers(&self) -> Result<Vec<SocketAddr>> {
        use crate::node::GLOBAL_NODES;
        let nodes = GLOBAL_NODES.get_nodes()?;
        Ok(nodes.into_iter().map(|n| n.get_addr()).collect())
    }

    /// Get the number of connected peers
    ///
    /// Returns a count of active peer connections.
    ///
    /// # Returns
    ///
    /// * `Ok(count)` - Number of connected peers
    /// * `Err(_)` - Network state access error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::node::NodeContext;
    /// # async fn example(node: &NodeContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let peer_count = node.get_peer_count()?;
    /// println!("Connected peers: {}", peer_count);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_peer_count(&self) -> Result<usize> {
        use crate::node::GLOBAL_NODES;
        let nodes = GLOBAL_NODES.get_nodes()?;
        Ok(nodes.len())
    }

    //=============================================================================
    // Validation Methods
    //=============================================================================

    /// Validate a transaction according to consensus rules
    ///
    /// Performs basic validation checks on a transaction before accepting
    /// it to the mempool. This is a simplified version of Bitcoin Core's
    /// transaction validation.
    ///
    /// # Arguments
    ///
    /// * `tx` - The transaction to validate
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Transaction is valid
    /// * `Ok(false)` - Transaction is invalid
    /// * `Err(_)` - Validation error
    ///
    /// # Validation Rules
    ///
    /// 1. **Coinbase transactions** - Always valid (mined by network)
    /// 2. **Input validation** - Must have at least one input
    /// 3. **Output validation** - Must have at least one output
    /// 4. **Mempool check** - Outputs must not be already spent in mempool
    ///
    /// # Missing Validations (TODO)
    ///
    /// - Signature verification
    /// - Input value verification (sufficient funds)
    /// - Fee calculation
    /// - Script execution
    /// - Double-spend detection via UTXO set
    ///
    /// # Bitcoin Core Equivalent
    ///
    /// ```cpp
    /// // Bitcoin Core: validation.cpp
    /// bool CheckTransaction(const CTransaction& tx, CValidationState& state)
    /// ```
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::{Transaction, node::NodeContext};
    /// # async fn example(node: &NodeContext, tx: &Transaction) -> Result<(), Box<dyn std::error::Error>> {
    /// if node.validate_transaction(tx).await? {
    ///     println!("Transaction is valid");
    /// } else {
    ///     println!("Transaction is invalid");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn validate_transaction(&self, tx: &Transaction) -> Result<bool> {
        let _utxo_set = UTXOSet::new(self.blockchain.clone());

        // Coinbase transactions are always valid (created by miners)
        if tx.is_coinbase() {
            return Ok(true);
        }

        // Verify transaction has valid inputs/outputs
        if tx.get_vin().is_empty() || tx.get_vout().is_empty() {
            return Ok(false);
        }

        // Verify outputs are not already in mempool (prevent double-spend)
        for output in tx.get_vout() {
            if output.is_in_global_mem_pool() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Validate a block according to consensus rules
    ///
    /// Performs structural validation on a block before accepting it to the chain.
    ///
    /// # Arguments
    ///
    /// * `block` - The block to validate
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Block is valid
    /// * `Ok(false)` - Block is invalid
    /// * `Err(_)` - Validation error
    ///
    /// # Validation Rules
    ///
    /// 1. **Has transactions** - Block must contain at least coinbase
    /// 2. **First is coinbase** - First transaction must be coinbase
    /// 3. **Only one coinbase** - No other transaction can be coinbase
    ///
    /// # Missing Validations (TODO)
    ///
    /// - Merkle root verification
    /// - Proof-of-work verification
    /// - Timestamp validation
    /// - Block size/weight limits
    /// - Transaction validation (signatures, scripts)
    /// - Coinbase maturity
    ///
    /// # Bitcoin Core Equivalent
    ///
    /// ```cpp
    /// // Bitcoin Core: validation.cpp
    /// bool CheckBlock(const CBlock& block, CValidationState& state, ...)
    /// ```
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use blockchain::{Block, node::NodeContext};
    /// # async fn example(node: &NodeContext, block: &Block) -> Result<(), Box<dyn std::error::Error>> {
    /// if node.validate_block(block).await? {
    ///     println!("Block is valid");
    /// } else {
    ///     println!("Block is invalid");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn validate_block(&self, block: &Block) -> Result<bool> {
        let transactions = block.get_transactions().await?;

        // Check block has transactions
        if transactions.is_empty() {
            return Ok(false);
        }

        // Check first transaction is coinbase
        if !transactions[0].is_coinbase() {
            return Ok(false);
        }

        // Check only first transaction is coinbase
        for tx in &transactions[1..] {
            if tx.is_coinbase() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    //=============================================================================
    // Internal Helper Methods
    //=============================================================================

    /// Submit transaction for mining and network broadcast (internal)
    ///
    /// This is called asynchronously after a transaction is added to mempool.
    /// It handles:
    /// 1. Broadcasting transaction to network peers
    /// 2. Triggering mining if threshold is met
    /// 3. Cleaning up invalid transactions
    ///
    /// # Arguments
    ///
    /// * `addr_from` - Source peer address (to avoid echoing back)
    /// * `utxo` - The transaction to broadcast and potentially mine
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Processing completed successfully
    /// * `Err(_)` - Network or mining error
    ///
    /// # Note
    ///
    /// This method is called in a background task and errors are logged
    /// rather than propagated to the caller.
    async fn submit_transaction_for_mining(
        &self,
        addr_from: &std::net::SocketAddr,
        utxo: Transaction,
    ) -> Result<()> {
        let my_node_addr = GLOBAL_CONFIG.get_node_addr();

        // Broadcast to network if this is the central node
        if my_node_addr.eq(&CENTERAL_NODE) {
            let nodes = self.get_nodes_excluding_sender(addr_from).await?;
            self.broadcast_transaction_to_nodes(&nodes, utxo.get_id_bytes())
                .await;
        }

        // Trigger mining if threshold is met
        if should_trigger_mining() {
            // Get mining address from config
            if let Some(mining_address) = GLOBAL_CONFIG.get_mining_addr() {
                match prepare_mining_utxo(&mining_address, &self.blockchain).await {
                    Ok(txs) => {
                        if !txs.is_empty() {
                            process_mine_block(txs, &self.blockchain).await.map(|_| ())
                        } else {
                            warn!("Mining triggered but no valid transactions to mine");
                            Ok(())
                        }
                    }
                    Err(e) => {
                        error!("Failed to prepare mining transactions: {}", e);
                        cleanup_invalid_transactions().await
                    }
                }
            } else {
                warn!("Mining triggered but no mining address configured");
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    /// Get nodes excluding the sender (internal)
    ///
    /// Filters the global node list to exclude the sender and this node,
    /// preventing message echo and loops.
    ///
    /// # Arguments
    ///
    /// * `addr_from` - Address to exclude from the result
    ///
    /// # Returns
    ///
    /// * `Ok(nodes)` - Vector of nodes to broadcast to
    /// * `Err(_)` - Network state access error
    async fn get_nodes_excluding_sender(
        &self,
        addr_from: &std::net::SocketAddr,
    ) -> Result<Vec<Node>> {
        let nodes = GLOBAL_NODES
            .get_nodes()
            .expect("Global nodes get error")
            .into_iter()
            .filter(|node| {
                let node_addr = node.get_addr();
                let my_addr = GLOBAL_CONFIG.get_node_addr();
                node_addr != *addr_from && node_addr != my_addr
            })
            .collect();
        Ok(nodes)
    }

    /// Broadcast transaction inventory to nodes (internal)
    ///
    /// Sends INV messages to notify peers about a new transaction.
    /// Uses async tasks to parallelize network operations.
    ///
    /// # Arguments
    ///
    /// * `nodes` - Peers to notify
    /// * `txid` - Transaction ID (binary format)
    ///
    /// # Note
    ///
    /// Each peer notification is spawned as a separate task for concurrency.
    async fn broadcast_transaction_to_nodes(&self, nodes: &[Node], txid: Vec<u8>) {
        let txid_clone = txid.clone();
        nodes.iter().for_each(|node| {
            let node_addr = node.get_addr();
            let txid = txid_clone.clone();
            tokio::spawn(async move {
                send_inv(&node_addr, OpType::Tx, &[txid]).await;
            });
        });
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Generate a test wallet address for testing
    fn generate_test_address() -> crate::WalletAddress {
        let wallet = crate::Wallet::new().expect("Failed to create wallet");
        wallet.get_address().expect("Failed to get wallet address")
    }

    /// Create unique database path for each test
    fn create_unique_db_path() -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("test_node_context_{}_{}", timestamp, uuid::Uuid::new_v4())
    }

    /// Setup test environment with unique database
    async fn setup_test_blockchain() -> (BlockchainService, String) {
        let db_path = create_unique_db_path();
        unsafe {
            std::env::set_var("TREE_DIR", &db_path);
            std::env::set_var("BLOCKS_TREE", &db_path);
        }

        let genesis_address = generate_test_address();
        let blockchain = BlockchainService::initialize(&genesis_address)
            .await
            .expect("Failed to create blockchain");
        (blockchain, db_path)
    }

    /// Cleanup test blockchain directory
    fn cleanup_test_blockchain(db_path: &str) {
        use std::fs;
        if let Err(e) = fs::remove_dir_all(db_path) {
            eprintln!(
                "Warning: Failed to clean up test directory {}: {}",
                db_path, e
            );
        }
    }

    #[tokio::test]
    async fn test_node_context_creation() {
        let (blockchain, db_path) = setup_test_blockchain().await;
        let node = NodeContext::new(blockchain);

        // Should be able to get height
        let height = node.get_blockchain_height().await;
        assert!(height.is_ok());

        cleanup_test_blockchain(&db_path);
    }

    #[tokio::test]
    async fn test_get_balance() {
        let genesis_address = generate_test_address();
        let (blockchain, db_path) = setup_test_blockchain().await;
        let node = NodeContext::new(blockchain);

        // Should be able to get balance
        let balance = node.get_balance(&genesis_address).await;
        assert!(balance.is_ok());

        cleanup_test_blockchain(&db_path);
    }

    #[tokio::test]
    async fn test_create_wallet() {
        let (blockchain, db_path) = setup_test_blockchain().await;
        let node = NodeContext::new(blockchain);

        // Should be able to create wallet
        let wallet = node.create_wallet();
        assert!(wallet.is_ok());

        cleanup_test_blockchain(&db_path);
    }

    #[tokio::test]
    async fn test_list_wallet_addresses() {
        let (blockchain, db_path) = setup_test_blockchain().await;
        let node = NodeContext::new(blockchain);

        // Should be able to list addresses
        let addresses = node.list_wallet_addresses();
        assert!(addresses.is_ok());

        cleanup_test_blockchain(&db_path);
    }
}
