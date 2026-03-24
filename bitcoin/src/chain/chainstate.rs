use crate::WalletAddress;
use crate::chain::utxo_set::UTXOSet;
use crate::error::{BtcError, Result};
use crate::primitives::block::Block;
use crate::primitives::transaction::{TXOutput, Transaction, TxSummary, WalletTransaction};

use sled::Db;
use std::collections::HashMap;
#[allow(unused_imports)]
use std::fs;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;

pub use crate::store::file_system_db_chain::*;

#[derive(Debug)]
pub struct BlockchainService(Arc<TokioRwLock<BlockchainFileSystem>>);

impl Clone for BlockchainService {
    fn clone(&self) -> Self {
        BlockchainService(self.0.clone())
    }
}
impl BlockchainService {
    pub async fn initialize(genesis_address: &WalletAddress) -> Result<BlockchainService> {
        let blockchain = BlockchainFileSystem::create_blockchain(genesis_address).await?;
        Ok(BlockchainService(Arc::new(TokioRwLock::new(blockchain))))
    }
    pub async fn default() -> Result<BlockchainService> {
        let blockchain = BlockchainFileSystem::open_blockchain().await?;
        Ok(BlockchainService(Arc::new(TokioRwLock::new(blockchain))))
    }
    pub async fn empty() -> Result<BlockchainService> {
        let blockchain = BlockchainFileSystem::open_blockchain_empty().await?;
        Ok(BlockchainService(Arc::new(TokioRwLock::new(blockchain))))
    }

    /// Create a BlockchainService from an existing BlockchainFileSystem (for testing)
    pub fn from_blockchain_file_system(blockchain: BlockchainFileSystem) -> BlockchainService {
        BlockchainService(Arc::new(TokioRwLock::new(blockchain)))
    }

    /// Apply a readfunction to a blockchain and return the result
    async fn read<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(BlockchainFileSystem) -> Fut + Send,
        Fut: Future<Output = Result<T>> + Send,
        T: Send + 'static,
    {
        let blockchain_guard = self.0.read().await;
        f(blockchain_guard.clone()).await
    }

    // /// Apply a write function to a blockchain and return the result
    // async fn write<F, T>(&self, mut f: F) -> Result<T>
    // where
    //     F: FnMut(&mut BlockchainFileSystem) -> Result<T> + Send,
    //     T: Send + 'static,
    // {
    //     let mut blockchain_guard = self.0.write().await;
    //     f(&mut blockchain_guard)
    // }

    pub async fn get_db(&self) -> Result<Db> {
        self.read(|blockchain: BlockchainFileSystem| async move { Ok(blockchain.get_db().clone()) })
            .await
    }

    /// Get the best height of the blockchain
    pub async fn get_best_height(&self) -> Result<usize> {
        self.read(
            |blockchain: BlockchainFileSystem| async move { blockchain.get_best_height().await },
        )
        .await
    }

    /// Get the tip hash of the blockchain
    pub async fn get_tip_hash(&self) -> Result<String> {
        self.read(|blockchain: BlockchainFileSystem| async move { blockchain.get_tip_hash().await })
            .await
    }

    /// Get the block hashes of the blockchain
    pub async fn get_block_hashes(&self) -> Result<Vec<Vec<u8>>> {
        self.read(
            |blockchain: BlockchainFileSystem| async move { blockchain.get_block_hashes().await },
        )
        .await
    }

    /// Get the block of the blockchain
    pub async fn get_block(&self, block_hash: &[u8]) -> Result<Option<Block>> {
        self.read(|blockchain: BlockchainFileSystem| async move {
            blockchain.get_block(block_hash).await
        })
        .await
    }

    /// Get blocks by its height in the blockchain
    pub async fn get_blocks_by_height(
        &self,
        initial_height: usize,
        height: usize,
    ) -> Result<Vec<Block>> {
        let mut blocks = Vec::new();
        let iterator = self.iterator().await?;
        for block in iterator {
            let h = block.get_height();
            if h > height || h < initial_height {
                break;
            }
            blocks.push(block);
        }
        Ok(blocks)
    }

    /// Add a block to the blockchain
    pub async fn add_block(&self, block: &Block) -> Result<()> {
        let mut blockchain_guard = self.0.write().await;
        blockchain_guard.add_block(block).await
    }

    /// Get the last block in the blockchain
    pub async fn get_last_block(&self) -> Result<Option<Block>> {
        self.read(
            |blockchain: BlockchainFileSystem| async move { blockchain.get_last_block().await },
        )
        .await
    }

    /// Get a block by its hash
    pub async fn get_block_by_hash(&self, block_hash: &[u8]) -> Result<Option<Block>> {
        self.read(|blockchain: BlockchainFileSystem| async move {
            blockchain.get_block_by_hash(block_hash).await
        })
        .await
    }

    /// Mine a block with the transactions in the memory pool.
    ///
    /// This method verifies all transactions, acquires the write lock, then
    /// re-validates transaction inputs against the UTXO set before calling the
    /// inner `mine_block` on `BlockchainFileSystem`.
    ///
    /// The re-validation under the write lock is critical: between
    /// `prepare_mining_utxo` (which validates with a read lock) and here,
    /// a competing block may have been accepted via `add_block`, spending
    /// the same transaction inputs. Without this double-check, `mine_block`
    /// would create a block with already-spent inputs, and `update_utxo_set`
    /// would silently add the coinbase — creating money from nothing.
    ///
    /// This two-phase validation (read lock + write lock) is analogous to
    /// Bitcoin Core's `TestBlockValidity` which validates the block template
    /// before committing to mining.
    pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
        for trasaction in transactions {
            let is_valid = trasaction.verify(self).await?;
            if !is_valid {
                return Err(BtcError::InvalidTransaction);
            }
        }
        let blockchain_guard = self.0.write().await;

        // Re-validate transaction inputs under the write lock.
        // Between prepare_mining_utxo (read lock) and here (write lock),
        // a competing block may have been accepted, spending the same inputs.
        // Without this check, mine_block creates a block with already-spent inputs,
        // and update_utxo_set silently adds the coinbase — creating money from nothing.
        let db = blockchain_guard.get_db();
        let utxo_tree = db
            .open_tree("chainstate")
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;

        for tx in transactions {
            if tx.is_coinbase() {
                continue;
            }
            for input in tx.get_vin() {
                match utxo_tree.get(input.get_txid()) {
                    Ok(Some(outs_bytes)) => {
                        let outputs: Vec<crate::primitives::TXOutput> =
                            bincode::serde::decode_from_slice(
                                outs_bytes.as_ref(),
                                bincode::config::standard(),
                            )
                            .map(|(v, _)| v)
                            .unwrap_or_default();
                        if input.get_vout() >= outputs.len() {
                            return Err(BtcError::InvalidValueForMiner(
                                "Transaction input already spent (stale mining)".to_string(),
                            ));
                        }
                    }
                    _ => {
                        return Err(BtcError::InvalidValueForMiner(
                            "Transaction input UTXO not found (stale mining)".to_string(),
                        ));
                    }
                }
            }
        }

        blockchain_guard.mine_block(transactions).await
    }

    pub async fn find_user_transaction(
        &self,
        address: &WalletAddress,
    ) -> Result<Vec<WalletTransaction>> {
        self.read(|blockchain: BlockchainFileSystem| async move {
            blockchain.find_user_transaction(address).await
        })
        .await
    }

    /// Find a transaction in the blockchain
    pub async fn find_transaction(&self, txid: &[u8]) -> Result<Option<Transaction>> {
        self.read(|blockchain: BlockchainFileSystem| async move {
            blockchain.find_transaction(txid).await
        })
        .await
    }

    pub async fn find_all_transactions(&self) -> Result<HashMap<String, TxSummary>> {
        self.read(|blockchain: BlockchainFileSystem| async move {
            blockchain.find_all_transactions().await
        })
        .await
    }

    pub async fn find_utxo(&self) -> Result<HashMap<String, Vec<TXOutput>>> {
        self.read(|blockchain: BlockchainFileSystem| async move { blockchain.find_utxo().await })
            .await
    }

    pub async fn iterator(&self) -> Result<BlockchainIterator> {
        self.read(|blockchain: BlockchainFileSystem| async move { blockchain.iterator().await })
            .await
    }

    /// Update UTXO set incrementally with a new block
    pub async fn update_utxo_set(&self, block: &Block) -> Result<()> {
        let utxo_set = UTXOSet::new(self.clone());
        utxo_set.update(block).await
    }

    /// Rollback UTXO set for chain reorganization
    pub async fn rollback_utxo_set(&self, block: &Block) -> Result<()> {
        let utxo_set = UTXOSet::new(self.clone());
        utxo_set.rollback_block(block).await
    }

    /// Add block with tie-breaking support
    pub async fn add_block_with_tie_breaking(&self, block: &Block) -> Result<()> {
        let mut blockchain_guard = self.0.write().await;
        blockchain_guard.add_block(block).await
    }

    /// Reorganize the blockchain to handle forks
    pub async fn reorganize_chain(&self, new_tip_hash: &str) -> Result<()> {
        let mut blockchain_guard = self.0.write().await;
        blockchain_guard.reorganize_chain(new_tip_hash).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::transaction::Transaction;

    use std::fs;

    fn generate_test_genesis_address() -> WalletAddress {
        // Create a wallet to get a valid Bitcoin address
        let wallet = crate::wallet::Wallet::new().expect("Failed to create test wallet");
        wallet.get_address().expect("Failed to get wallet address")
    }

    /// Test fixture that automatically cleans up the test database
    struct TestBlockchain {
        blockchain: BlockchainService,
        db_path: String,
    }

    impl TestBlockchain {
        async fn new() -> Self {
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();

            // Use process ID and multiple random numbers for better isolation
            let process_id = std::process::id();
            let random_num1 = rand::random::<u32>();
            let random_num2 = rand::random::<u32>();
            let test_db_path = format!(
                "test_blockchain_db_{}_{}_{}_{}_{}",
                timestamp,
                process_id,
                random_num1,
                random_num2,
                uuid::Uuid::new_v4()
            );

            // Clean up any existing test database with retry logic
            let _ = Self::cleanup_with_retry(&test_db_path);

            // Create a unique subdirectory for this test
            let unique_db_path = format!("{}/db", test_db_path);
            let _ = fs::create_dir_all(&unique_db_path);

            // Set environment variable for unique database path
            unsafe {
                std::env::set_var("TREE_DIR", &unique_db_path);
            }
            unsafe {
                std::env::set_var("BLOCKS_TREE", &unique_db_path);
            }

            let genesis_address = generate_test_genesis_address();

            // Try to create blockchain with retry logic
            let blockchain = match Self::create_blockchain_with_retry(&genesis_address).await {
                Ok(bc) => bc,
                Err(_) => {
                    // If creation fails, clean up and retry once more
                    let _ = Self::cleanup_with_retry(&test_db_path);
                    Self::create_blockchain_with_retry(&genesis_address)
                        .await
                        .expect("Failed to create test blockchain after retry")
                }
            };

            TestBlockchain {
                blockchain,
                db_path: test_db_path,
            }
        }

        /// Create blockchain with retry logic to handle database lock issues
        async fn create_blockchain_with_retry(
            genesis_address: &WalletAddress,
        ) -> Result<BlockchainService> {
            for attempt in 1..=3 {
                match BlockchainService::initialize(genesis_address).await {
                    Ok(bc) => return Ok(bc),
                    Err(e) if e.to_string().contains("could not acquire lock") => {
                        if attempt < 3 {
                            std::thread::sleep(std::time::Duration::from_millis(200 * attempt));
                            continue;
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Err(BtcError::BlockchainDBconnection(
                "Failed to create blockchain after retries".to_string(),
            ))
        }

        /// Clean up test database with retry logic to handle lock issues
        fn cleanup_with_retry(db_path: &str) -> std::io::Result<()> {
            for attempt in 1..=5 {
                match fs::remove_dir_all(db_path) {
                    Ok(_) => return Ok(()),
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        if attempt < 5 {
                            // Exponential backoff with longer delays
                            let delay =
                                std::time::Duration::from_millis(200 * (1 << (attempt - 1)));
                            std::thread::sleep(delay);
                            continue;
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        return Ok(()); // Directory doesn't exist, that's fine
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                        if attempt < 5 {
                            // Wait longer for permission issues
                            std::thread::sleep(std::time::Duration::from_millis(500 * attempt));
                            continue;
                        }
                    }
                    Err(e) => {
                        // Log the error but continue trying
                        eprintln!("Cleanup attempt {} failed: {}", attempt, e);
                        if attempt < 5 {
                            std::thread::sleep(std::time::Duration::from_millis(300 * attempt));
                            continue;
                        }
                    }
                }
            }
            Ok(())
        }

        fn blockchain(&self) -> &BlockchainService {
            &self.blockchain
        }
    }

    impl Drop for TestBlockchain {
        fn drop(&mut self) {
            // Ensure cleanup happens even if test panics
            let _ = Self::cleanup_with_retry(&self.db_path);
        }
    }

    #[tokio::test]
    async fn test_blockchain_creation() {
        // Setup test environment
        crate::setup_test_environment();

        let test_blockchain = TestBlockchain::new().await;

        assert_eq!(
            test_blockchain
                .blockchain()
                .get_best_height()
                .await
                .expect("Failed to get height"),
            1
        );

        // Teardown test environment
        crate::teardown_test_environment();
    }

    #[tokio::test]
    async fn test_genesis_block_creation() {
        let test_blockchain = TestBlockchain::new().await;

        // Genesis block should be created automatically
        assert_eq!(
            test_blockchain
                .blockchain()
                .get_best_height()
                .await
                .expect("Failed to get height"),
            1
        );

        // Get the genesis block using the iterator
        let mut iterator = test_blockchain
            .blockchain()
            .iterator()
            .await
            .expect("Failed to create iterator");
        let genesis_block = iterator.next().expect("Genesis block should exist");
        assert_eq!(genesis_block.get_height(), 1);
        assert_eq!(genesis_block.get_pre_block_hash(), "None");
    }

    #[tokio::test]
    async fn test_add_block() {
        let test_blockchain = TestBlockchain::new().await;
        let genesis_address = generate_test_genesis_address();

        // Create a new block
        let coinbase_tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];
        let new_block = test_blockchain
            .blockchain()
            .mine_block(transactions.as_slice())
            .await
            .expect("Failed to mine block");

        // Add the block
        test_blockchain
            .blockchain()
            .add_block(&new_block)
            .await
            .expect("Failed to add block");

        assert_eq!(
            test_blockchain
                .blockchain()
                .get_best_height()
                .await
                .expect("Failed to get height"),
            2
        );
    }

    #[tokio::test]
    async fn test_get_block() {
        let test_blockchain = TestBlockchain::new().await;
        let genesis_address = generate_test_genesis_address();

        // Create and add a block
        let coinbase_tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];
        let new_block = test_blockchain
            .blockchain()
            .mine_block(transactions.as_slice())
            .await
            .expect("Failed to mine block");
        test_blockchain
            .blockchain()
            .add_block(&new_block)
            .await
            .expect("Failed to add block");

        // Get the block by hash
        let retrieved_block = test_blockchain
            .blockchain()
            .get_block(new_block.get_hash_bytes().as_slice())
            .await
            .expect("Failed to get block")
            .expect("Block should exist");

        assert_eq!(retrieved_block.get_hash(), new_block.get_hash());
        assert_eq!(retrieved_block.get_height(), 2);
    }

    #[tokio::test]
    async fn test_get_block_hashes() {
        let test_blockchain = TestBlockchain::new().await;
        let genesis_address = generate_test_genesis_address();

        // Add a few blocks
        for _i in 0..3 {
            let coinbase_tx = Transaction::new_coinbase_tx(&genesis_address)
                .expect("Failed to create coinbase tx");
            let transactions = vec![coinbase_tx];
            let new_block = test_blockchain
                .blockchain()
                .mine_block(transactions.as_slice())
                .await
                .expect("Failed to mine block");
            test_blockchain
                .blockchain()
                .add_block(&new_block)
                .await
                .expect("Failed to add block");
        }

        let block_hashes = test_blockchain
            .blockchain()
            .get_block_hashes()
            .await
            .expect("Failed to get block hashes");

        // Should have genesis block + 3 new blocks = 4 total
        assert_eq!(block_hashes.len(), 4);
    }

    #[tokio::test]
    async fn test_blockchain_iterator() {
        let test_blockchain = TestBlockchain::new().await;

        // Add a block
        let genesis_address = generate_test_genesis_address();
        let coinbase_tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];
        let new_block = test_blockchain
            .blockchain()
            .mine_block(transactions.as_slice())
            .await
            .expect("Failed to mine block");
        test_blockchain
            .blockchain()
            .add_block(&new_block)
            .await
            .expect("Failed to add block");

        let mut iterator = test_blockchain
            .blockchain()
            .iterator()
            .await
            .expect("Failed to create iterator");
        let mut block_count = 0;

        while iterator.next().is_some() {
            block_count += 1;
        }

        // Should have genesis block + 1 new block = 2 total
        assert_eq!(block_count, 2);
    }

    struct TestPersistenceBlockchain {
        db_path: String,
    }

    impl TestPersistenceBlockchain {
        async fn new() -> Self {
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let test_db_path =
                format!("test_persistence_db_{}_{}", timestamp, uuid::Uuid::new_v4());

            // Clean up any existing test database
            let _ = fs::remove_dir_all(&test_db_path);

            // Create a unique subdirectory for this test
            let unique_db_path = format!("{}/db", test_db_path);
            let _ = fs::create_dir_all(&unique_db_path);

            // Set environment variable for unique database path
            unsafe {
                std::env::set_var("TREE_DIR", &unique_db_path);
            }
            unsafe {
                std::env::set_var("BLOCKS_TREE", &unique_db_path);
            }

            TestPersistenceBlockchain {
                db_path: test_db_path,
            }
        }
    }

    impl Drop for TestPersistenceBlockchain {
        fn drop(&mut self) {
            // Ensure cleanup happens even if test panics
            let _ = cleanup_test_directory_with_retry(&self.db_path);
        }
    }

    /// Clean up test directory with retry logic to handle lock issues
    fn cleanup_test_directory_with_retry(db_path: &str) -> std::io::Result<()> {
        for attempt in 1..=5 {
            match fs::remove_dir_all(db_path) {
                Ok(_) => return Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if attempt < 5 {
                        let delay = std::time::Duration::from_millis(200 * attempt);
                        std::thread::sleep(delay);
                        continue;
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    return Ok(()); // Directory doesn't exist, that's fine
                }
                Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                    if attempt < 5 {
                        std::thread::sleep(std::time::Duration::from_millis(500 * attempt));
                        continue;
                    }
                }
                Err(e) => {
                    eprintln!("Cleanup attempt {} failed for {}: {}", attempt, db_path, e);
                    if attempt < 5 {
                        std::thread::sleep(std::time::Duration::from_millis(300 * attempt));
                        continue;
                    }
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_blockchain_persistence() {
        // Setup test environment
        crate::setup_test_environment();

        let _ = TestPersistenceBlockchain::new().await;
        let genesis_address = generate_test_genesis_address();

        {
            let blockchain = BlockchainService::initialize(&genesis_address)
                .await
                .expect("Failed to create blockchain");

            // Add a block
            let coinbase_tx = Transaction::new_coinbase_tx(&genesis_address)
                .expect("Failed to create coinbase tx");
            let transactions = vec![coinbase_tx];
            let new_block = blockchain
                .mine_block(transactions.as_slice())
                .await
                .expect("Failed to mine block");
            blockchain
                .add_block(&new_block)
                .await
                .expect("Failed to add block");
        } // blockchain goes out of scope here

        // Create a new blockchain instance with the same database
        let blockchain = BlockchainService::initialize(&genesis_address)
            .await
            .expect("Failed to create new blockchain");

        // Should still have the block we added
        assert_eq!(
            blockchain
                .get_best_height()
                .await
                .expect("Failed to get height"),
            2
        );

        // Teardown test environment
        crate::teardown_test_environment();
    }

    #[tokio::test]
    async fn test_mine_block() {
        let test_blockchain = TestBlockchain::new().await;

        let genesis_address = generate_test_genesis_address();
        let coinbase_tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];

        let new_block = test_blockchain
            .blockchain()
            .mine_block(transactions.as_slice())
            .await
            .expect("Failed to mine block");

        // Check that the block was mined correctly
        assert_eq!(new_block.get_height(), 2); // Height 2 because genesis block is height 1
        assert!(!new_block.get_hash().is_empty());
        assert!(new_block.get_transactions().await.unwrap().len() > 0);
    }
}
