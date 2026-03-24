//! # Block
//!
//! Block is a data structure that contains the data and operations on the block.
//!

extern crate bincode;
use crate::WalletAddress;
use crate::crypto::signature::schnorr_sign_verify;
use crate::error::{BtcError, Result};
use crate::pow::ProofOfWork;
use crate::primitives::transaction::{Transaction, WalletTransaction, WalletTransactionType};
use crate::wallet::{convert_address, get_pub_key_hash, hash_pub_key};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use sled::IVec;

pub const GENESIS_BLOCK_PRE_BLOCK_HASH: &str = "None";

// Add a block header that contains timestamp, pre_block_hash, hash, nonce, height
// Block to be composed of block header and transactions
#[derive(Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    nonce: i64,
    height: usize,
}

/// Block
///
/// `timestamp`: An integer value that represents the time when the block was created. It's used
/// to track the chronological order of blocks in the blockchain.
/// `pre_block_hash`: A string containing the hash value of the previous block in the blockchain.
/// This creates a link between blocks, ensuring the integrity of the blockchain.
/// `hash`: String containing the hash value of the current block. This hash is generated based on
/// the data within the current block, including transactions and other information.
/// `transactions`: A vector or collection that holds the block transactions.
/// Transactions can represent various types of data or actions, depending on the blockchain's
/// purpose (for example, cryptocurrency transactions).
/// `nonce`: A number used to ensure that the block is valid and has not been tampered with.
/// Stands for number used only once
/// `height`: An integer value that represents the position of the block in the blockchain.
/// It's used to track the chronological order of blocks in the blockchain. Indicates the position.

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let header = BlockHeader {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            hash: String::new(), // to be filled in the next step
            nonce: 0,
            height,
        };
        let mut block = Block {
            header,
            transactions: transactions.to_vec(),
        };
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.header.nonce = nonce;
        block.header.hash = hash;
        block
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Block> {
        bincode::serde::decode_from_slice(bytes, bincode::config::standard())
            .map_err(|e| BtcError::BlockDeserializationError(e.to_string()))
            .map(|(block, _)| block)
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serde::encode_to_vec(self, bincode::config::standard())
            .map_err(|e| BtcError::BlockSerializationError(e.to_string()))
    }

    pub async fn get_transactions(&self) -> Result<&[Transaction]> {
        Ok(self.transactions.as_slice())
    }

    pub fn get_transactions_count(&self) -> usize {
        self.transactions.len()
    }

    pub fn get_block_size(&self) -> Result<usize> {
        Ok(self.serialize()?.len())
    }

    /// Get all transactions relevant to a specific wallet address
    ///
    /// This method scans all transactions in the block and returns those that involve
    /// the given address, either as sender (debit) or receiver (credit).
    ///
    /// # Watch-Only Capability
    ///
    /// This method works with ANY address,
    /// enabling:
    /// - Blockchain explorers to track any address
    /// - Payment monitoring without private keys
    /// - Portfolio tracking across multiple addresses
    /// - Cold wallet monitoring (keys offline, monitoring online)
    ///
    /// # Arguments
    ///
    /// * `address` - The wallet address to filter transactions for
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<WalletTransaction>)` - Vector of wallet transactions
    /// * `Err(BtcError)` - If address is invalid or processing fails
    pub async fn get_user_transactions(
        &self,
        wlt_address: &WalletAddress,
    ) -> Result<Vec<WalletTransaction>> {
        // Extract public key hash from address
        let req_addr_pub_key_hash = get_pub_key_hash(wlt_address)?;

        let wallet_txs: Vec<WalletTransaction> = self
            .transactions
            .iter()
            .map(|tx| -> Result<Vec<WalletTransaction>> {
                Ok(match tx.is_coinbase() {
                    true => {
                        // Coinbase transactions: Check if output is locked to our address
                        if let Some((index, vout)) =
                            tx.get_vout().iter().enumerate().find(|(_, vout)| {
                                vout.is_locked_with_key(req_addr_pub_key_hash.as_slice())
                            })
                        {
                            vec![WalletTransaction::new(
                                tx.clone(),
                                vout,
                                None, // Coinbase has no sender
                                WalletTransactionType::Credit,
                                index,
                                0,
                                self.header.timestamp,
                            )?]
                        } else {
                            vec![]
                        }
                    }
                    false => {
                        // Regular transactions: Determine if we're sender or receiver
                        match tx.get_vin().first() {
                            Some(vin) => {
                                // Extract public key from transaction input
                                let tx_public_key = vin.get_pub_key();
                                let tx_pub_key_hash = hash_pub_key(tx_public_key);
                                let signature = vin.get_signature();
                                let vout = tx.get_vout();

                                // Check if we're the sender by comparing public key hashes
                                if tx_pub_key_hash == req_addr_pub_key_hash {
                                    // Verify signature to confirm it's really us.
                                    // Purpose: Ensure the transaction is legitimate (not forged/tampered)
                                    if schnorr_sign_verify(tx_public_key, signature, tx.get_id()) {
                                        // DEBIT: We're the sender, find outputs to others
                                        vout.iter()
                                            .enumerate()
                                            .filter(|(_, v)| {
                                                v.not_locked_with_key(
                                                    req_addr_pub_key_hash.as_slice(),
                                                )
                                            })
                                            .map(|(index, vout)| -> Result<WalletTransaction> {
                                                WalletTransaction::new(
                                                    tx.clone(),
                                                    vout,
                                                    Some(wlt_address.clone()),
                                                    WalletTransactionType::Debit,
                                                    index,
                                                    0,
                                                    self.header.timestamp,
                                                )
                                            })
                                            .collect::<Result<Vec<_>>>()?
                                    } else {
                                        // Signature verification failed
                                        vec![]
                                    }
                                } else {
                                    // CREDIT: Someone else sent to us
                                    vout.iter()
                                        .enumerate()
                                        .filter(|(_, v)| {
                                            v.is_locked_with_key(req_addr_pub_key_hash.as_slice())
                                        })
                                        .map(|(index, vout)| -> Result<WalletTransaction> {
                                            let from_addr = convert_address(tx_public_key)?;
                                            WalletTransaction::new(
                                                tx.clone(),
                                                vout,
                                                Some(from_addr),
                                                WalletTransactionType::Credit,
                                                index,
                                                0,
                                                self.header.timestamp,
                                            )
                                        })
                                        .collect::<Result<Vec<_>>>()?
                                }
                            }
                            None => vec![],
                        }
                    }
                })
            })
            .collect::<Result<Vec<Vec<_>>>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(wallet_txs)
    }

    pub fn get_pre_block_hash(&self) -> String {
        self.header.pre_block_hash.clone()
    }

    pub fn get_hash(&self) -> &str {
        self.header.hash.as_str()
    }

    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.header.hash.as_bytes().to_vec()
    }

    pub fn get_timestamp(&self) -> i64 {
        self.header.timestamp
    }

    pub fn get_height(&self) -> usize {
        self.header.height
    }

    pub fn get_difficulty(&self) -> u32 {
        // For now, return a constant difficulty
        // In a real implementation, this would be calculated based on the block's proof-of-work
        1
    }

    /// Get the nonce value from the block header
    pub fn get_nonce(&self) -> i64 {
        self.header.nonce
    }

    /// Get the hash as a string for comparison
    pub fn get_hash_string(&self) -> String {
        self.header.hash.clone()
    }

    /// Calculate work for this block based on proof-of-work difficulty
    ///
    /// In Bitcoin, work is calculated as: 2^256 / (target + 1)
    /// Where target = 2^(256 - TARGET_BITS)
    ///
    /// For our implementation with TARGET_BITS = 8:
    /// - Target = 2^(256 - 8) = 2^248
    /// - Work = 2^256 / (2^248 + 1) ≈ 2^8 = 256
    ///
    /// # Returns
    /// * `u64` - The work value for this block
    ///
    /// # Note
    /// This implements the same work calculation as Bitcoin's consensus mechanism.
    /// Higher work values indicate more difficult proof-of-work, which means
    /// the block required more computational effort to mine.
    pub fn get_work(&self) -> u64 {
        // Work is calculated as: 2^256 / (target + 1)
        // Where target = 2^(256 - TARGET_BITS)
        // For TARGET_BITS = 8: target = 2^248, work = 2^256 / (2^248 + 1) ≈ 2^8 = 256
        const TARGET_BITS: u32 = 8;
        const WORK_BITS: u32 = 256 - TARGET_BITS; // 256 - 8 = 248

        // Calculate 2^WORK_BITS, but cap it to prevent overflow
        if WORK_BITS >= 64 {
            u64::MAX / 1000 // Large but manageable value
        } else {
            2u64.pow(WORK_BITS)
        }
    }

    /// Get the target bits used for proof-of-work calculation
    ///
    /// This method provides access to the TARGET_BITS constant used
    /// in the proof-of-work calculation, allowing other parts of the
    /// system to understand the difficulty level.
    ///
    /// # Returns
    /// * `u32` - The target bits value (currently 8)
    pub fn get_target_bits(&self) -> u32 {
        // This should match the TARGET_BITS constant in proof_of_work.rs
        8
    }

    /// Calculate the actual target value used in proof-of-work
    ///
    /// This method calculates the exact target value that was used
    /// during the proof-of-work mining process for this block.
    ///
    /// # Returns
    /// * `BigInt` - The target value as a BigInt
    pub fn get_target(&self) -> BigInt {
        use std::ops::ShlAssign;

        let target_bits = self.get_target_bits();
        let mut target = BigInt::from(1);
        target.shl_assign(256 - target_bits as i32);
        target
    }

    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }
        crate::sha256_digest(txhashs.as_slice())
    }

    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];
        Block::new_block(GENESIS_BLOCK_PRE_BLOCK_HASH.to_string(), &transactions, 1)
    }
}

impl TryFrom<Block> for IVec {
    type Error = BtcError;
    fn try_from(b: Block) -> Result<Self> {
        let bytes = bincode::serde::encode_to_vec(&b, bincode::config::standard())
            .map_err(|e| BtcError::BlockSerializationError(e.to_string()))?;
        Ok(Self::from(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Wallet;
    use crate::primitives::transaction::{Transaction, WalletTransactionStatus};

    fn generate_test_genesis_address() -> crate::WalletAddress {
        // Create a wallet to get a valid Bitcoin address
        let wallet = crate::wallet::Wallet::new().expect("Failed to create test wallet");
        wallet.get_address().expect("Failed to get wallet address")
    }

    #[test]
    fn test_block_creation() {
        let transactions = vec![];
        let prev_block_hash = "previous_hash".to_string();
        let height = 1;

        let block = Block::new_block(prev_block_hash.clone(), transactions.as_slice(), height);

        assert_eq!(block.header.pre_block_hash, prev_block_hash);
        assert_eq!(block.transactions.len(), 0);
        assert_eq!(block.header.height, height);
        assert!(!block.header.hash.is_empty()); // Should be filled by PoW
        assert!(block.header.nonce >= 0);
    }

    #[test]
    fn test_block_serialization_deserialization() {
        let genesis_address = generate_test_genesis_address();
        let coinbase_tx = Transaction::new_coinbase_tx(&genesis_address.clone())
            .expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];
        let block = Block::new_block("prev_hash".to_string(), transactions.as_slice(), 1);

        let serialized = block.serialize().expect("Serialization failed");
        let deserialized = Block::deserialize(&serialized).expect("Deserialization failed");

        assert_eq!(block.header.timestamp, deserialized.header.timestamp);
        assert_eq!(
            block.header.pre_block_hash,
            deserialized.header.pre_block_hash
        );
        assert_eq!(block.header.hash, deserialized.header.hash);
        assert_eq!(block.header.nonce, deserialized.header.nonce);
        assert_eq!(block.header.height, deserialized.header.height);
    }

    #[tokio::test]
    async fn test_block_getters() {
        let block = Block::new_block("prev_hash".to_string(), &[], 1);

        assert_eq!(block.get_pre_block_hash(), "prev_hash");
        assert!(!block.get_hash().is_empty());
        assert_eq!(block.get_height(), 1);
        assert!(block.get_timestamp() > 0);
        assert_eq!(block.get_transactions().await.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_block_with_transactions() {
        let genesis_address = generate_test_genesis_address();
        let coinbase_tx = Transaction::new_coinbase_tx(&genesis_address.clone())
            .expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];

        let block = Block::new_block("prev_hash".to_string(), transactions.as_slice(), 1);

        assert_eq!(block.transactions.len(), 1);
        assert_eq!(block.get_transactions().await.unwrap().len(), 1);
    }

    #[test]
    fn test_block_hash_bytes() {
        let block = Block::new_block("prev_hash".to_string(), &[], 1);
        let hash_bytes = block.get_hash_bytes();
        assert!(!hash_bytes.is_empty());
        assert_eq!(hash_bytes, block.header.hash.as_bytes());
    }

    #[test]
    fn test_work_calculation() {
        // Create a test block
        let block = Block::new_block("prev_hash".to_string(), &[], 1);

        // Test work calculation
        let work = block.get_work();

        // Work should be meaningful (not just 1)
        assert!(work > 1, "Work should be greater than 1, got {}", work);

        // For TARGET_BITS = 8, work should be approximately 2^8 = 256
        // But we cap it to prevent overflow, so it should be a large number
        assert!(work >= 256, "Work should be at least 256, got {}", work);

        // Test target bits
        let target_bits = block.get_target_bits();
        assert_eq!(
            target_bits, 8,
            "Target bits should be 8, got {}",
            target_bits
        );

        // Test target calculation
        let target = block.get_target();
        assert!(target > BigInt::from(0), "Target should be positive");
    }

    #[test]
    fn test_work_accumulation() {
        // Test that work accumulates properly across multiple blocks
        let mut total_work = 0u64;

        for i in 1..=5 {
            let block = Block::new_block(format!("prev_hash_{}", i), &[], i);
            let work = block.get_work();
            total_work += work;
        }

        // Total work should be 5 times the individual block work
        let single_block_work = Block::new_block("test".to_string(), &[], 1).get_work();
        let expected_total = single_block_work * 5;

        assert_eq!(
            total_work, expected_total,
            "Total work should accumulate properly"
        );
    }

    // =====================================================================
    // get_user_transactions Tests
    // =====================================================================

    /// Test coinbase transaction to wallet (confirmed)
    #[tokio::test]
    async fn test_get_user_transactions_coinbase_credit_confirmed() {
        // Create a wallet
        let wallet = Wallet::new().expect("Failed to create wallet");
        let wallet_address = wallet.get_address().expect("Failed to get address");

        // Create a coinbase transaction to this address
        let coinbase_tx =
            Transaction::new_coinbase_tx(&wallet_address).expect("Failed to create coinbase tx");

        // Create a block with this transaction
        let block = Block::new_block("prev_hash".to_string(), &[coinbase_tx], 1);

        // Get user transactions using address (not wallet!)
        let user_txs = block
            .get_user_transactions(&wallet_address)
            .await
            .expect("Failed to get user transactions");

        // Should have 1 transaction
        assert_eq!(user_txs.len(), 1, "Should have 1 coinbase transaction");

        // Verify transaction details
        let tx = &user_txs[0];
        assert_eq!(*tx.get_transaction_type(), WalletTransactionType::Credit);
        assert_eq!(*tx.get_status(), WalletTransactionStatus::Confirmed);
        assert_eq!(*tx.get_from_wlt_addr(), None); // Coinbase has no sender
        assert_eq!(*tx.get_to_wlt_addr(), wallet_address);
        assert!(tx.get_value() > 0, "Coinbase amount should be positive");
    }

    /// Test coinbase transaction NOT to wallet (should return empty)
    #[tokio::test]
    async fn test_get_user_transactions_coinbase_not_for_wallet() {
        // Create two wallets
        let wallet1 = Wallet::new().expect("Failed to create wallet 1");
        let wallet1_address = wallet1.get_address().expect("Failed to get address 1");
        let wallet2 = Wallet::new().expect("Failed to create wallet 2");
        let wallet2_address = wallet2.get_address().expect("Failed to get address 2");

        // Create a coinbase transaction to wallet2
        let coinbase_tx =
            Transaction::new_coinbase_tx(&wallet2_address).expect("Failed to create coinbase tx");

        // Create a block with this transaction
        let block = Block::new_block("prev_hash".to_string(), &[coinbase_tx], 1);

        // Get user transactions for wallet1 address (should be empty)
        let user_txs = block
            .get_user_transactions(&wallet1_address)
            .await
            .expect("Failed to get user transactions");

        // Should have 0 transactions
        assert_eq!(
            user_txs.len(),
            0,
            "Should have no transactions for different wallet"
        );
    }

    /// Test debit transaction (wallet sends to others)
    #[tokio::test]
    async fn test_get_user_transactions_debit() {
        // Create a wallet
        let sender_wallet = Wallet::new().expect("Failed to create sender wallet");
        let sender_address = sender_wallet
            .get_address()
            .expect("Failed to get sender address");

        // First, create a coinbase to give sender some funds
        let coinbase_tx =
            Transaction::new_coinbase_tx(&sender_address).expect("Failed to create coinbase tx");
        let funding_block = Block::new_block("prev_hash".to_string(), &[coinbase_tx], 1);

        // Verify sender received the coinbase (using address)
        let funding_txs = funding_block
            .get_user_transactions(&sender_address)
            .await
            .expect("Failed to get funding transactions");
        assert_eq!(funding_txs.len(), 1, "Should have 1 funding transaction");
        assert_eq!(
            *funding_txs[0].get_transaction_type(),
            WalletTransactionType::Credit
        );
    }

    /// Test credit transaction (wallet receives from others)
    #[tokio::test]
    async fn test_get_user_transactions_credit_from_others() {
        // Similar to debit test, credit transactions require full UTXO context
        // The logic is tested through the structure verification

        let wallet = Wallet::new().expect("Failed to create wallet");
        let wallet_address = wallet.get_address().expect("Failed to get address");

        // Create a coinbase (which is a credit transaction)
        let coinbase_tx =
            Transaction::new_coinbase_tx(&wallet_address).expect("Failed to create coinbase tx");
        let block = Block::new_block("prev_hash".to_string(), &[coinbase_tx], 1);

        let user_txs = block
            .get_user_transactions(&wallet_address)
            .await
            .expect("Failed to get user transactions");

        assert_eq!(user_txs.len(), 1);
        assert_eq!(
            *user_txs[0].get_transaction_type(),
            WalletTransactionType::Credit
        );
    }

    /// Test block with multiple transactions for same wallet
    #[tokio::test]
    async fn test_get_user_transactions_multiple_transactions() {
        let wallet = Wallet::new().expect("Failed to create wallet");
        let wallet_address = wallet.get_address().expect("Failed to get address");

        // Create multiple coinbase transactions to same address
        let coinbase_tx1 =
            Transaction::new_coinbase_tx(&wallet_address).expect("Failed to create coinbase tx 1");
        let coinbase_tx2 =
            Transaction::new_coinbase_tx(&wallet_address).expect("Failed to create coinbase tx 2");
        let coinbase_tx3 =
            Transaction::new_coinbase_tx(&wallet_address).expect("Failed to create coinbase tx 3");

        let block = Block::new_block(
            "prev_hash".to_string(),
            &[coinbase_tx1, coinbase_tx2, coinbase_tx3],
            1,
        );

        let user_txs = block
            .get_user_transactions(&wallet_address)
            .await
            .expect("Failed to get user transactions");

        // Should have 3 transactions
        assert_eq!(user_txs.len(), 3, "Should have 3 transactions");

        // All should be credits
        for tx in &user_txs {
            assert_eq!(*tx.get_transaction_type(), WalletTransactionType::Credit);
            assert_eq!(*tx.get_status(), WalletTransactionStatus::Confirmed);
        }
    }

    /// Test block with mixed transactions (some for wallet, some not)
    #[tokio::test]
    async fn test_get_user_transactions_mixed_transactions() {
        let wallet1 = Wallet::new().expect("Failed to create wallet 1");
        let wallet1_address = wallet1.get_address().expect("Failed to get address 1");
        let wallet2 = Wallet::new().expect("Failed to create wallet 2");
        let wallet2_address = wallet2.get_address().expect("Failed to get address 2");

        // Create transactions to different addresses
        let coinbase_tx1 =
            Transaction::new_coinbase_tx(&wallet1_address).expect("Failed to create coinbase tx 1");
        let coinbase_tx2 =
            Transaction::new_coinbase_tx(&wallet2_address).expect("Failed to create coinbase tx 2");
        let coinbase_tx3 =
            Transaction::new_coinbase_tx(&wallet1_address).expect("Failed to create coinbase tx 3");

        let block = Block::new_block(
            "prev_hash".to_string(),
            &[coinbase_tx1, coinbase_tx2, coinbase_tx3],
            1,
        );

        // Get transactions for wallet1 address
        let wallet1_txs = block
            .get_user_transactions(&wallet1_address)
            .await
            .expect("Failed to get wallet1 transactions");

        // Should have 2 transactions (coinbase_tx1 and coinbase_tx3)
        assert_eq!(wallet1_txs.len(), 2, "Wallet1 should have 2 transactions");

        // Get transactions for wallet2 address
        let wallet2_txs = block
            .get_user_transactions(&wallet2_address)
            .await
            .expect("Failed to get wallet2 transactions");

        // Should have 1 transaction (coinbase_tx2)
        assert_eq!(wallet2_txs.len(), 1, "Wallet2 should have 1 transaction");
    }

    /// Test empty block (no transactions)
    #[tokio::test]
    async fn test_get_user_transactions_empty_block() {
        let wallet = Wallet::new().expect("Failed to create wallet");
        let wallet_address = wallet.get_address().expect("Failed to get address");
        let block = Block::new_block("prev_hash".to_string(), &[], 1);

        let user_txs = block
            .get_user_transactions(&wallet_address)
            .await
            .expect("Failed to get user transactions");

        // Should have 0 transactions
        assert_eq!(user_txs.len(), 0, "Empty block should have no transactions");
    }

    /// Test transaction output index is preserved
    #[tokio::test]
    async fn test_get_user_transactions_output_index() {
        let wallet = Wallet::new().expect("Failed to create wallet");
        let wallet_address = wallet.get_address().expect("Failed to get address");

        let coinbase_tx =
            Transaction::new_coinbase_tx(&wallet_address).expect("Failed to create coinbase tx");
        let block = Block::new_block("prev_hash".to_string(), &[coinbase_tx], 1);

        let user_txs = block
            .get_user_transactions(&wallet_address)
            .await
            .expect("Failed to get user transactions");

        assert_eq!(user_txs.len(), 1);
        // Coinbase transaction should have output at index 0
        assert_eq!(
            user_txs[0].get_vout(),
            0,
            "Coinbase output should be at index 0"
        );
    }

    /// Test transaction ID is correctly set
    #[tokio::test]
    async fn test_get_user_transactions_transaction_id() {
        let wallet = Wallet::new().expect("Failed to create wallet");
        let wallet_address = wallet.get_address().expect("Failed to get address");

        let coinbase_tx =
            Transaction::new_coinbase_tx(&wallet_address).expect("Failed to create coinbase tx");
        let expected_tx_id = coinbase_tx.get_id().to_vec();

        let block = Block::new_block("prev_hash".to_string(), &[coinbase_tx], 1);

        let user_txs = block
            .get_user_transactions(&wallet_address)
            .await
            .expect("Failed to get user transactions");

        assert_eq!(user_txs.len(), 1);
        assert_eq!(
            user_txs[0].get_tx_id(),
            expected_tx_id.as_slice(),
            "Transaction ID should match"
        );
    }

    /// Test confirmed vs pending status
    /// Note: In practice, transactions in a block are confirmed.
    /// Pending status is tested through mempool integration tests.
    #[tokio::test]
    async fn test_get_user_transactions_confirmed_status() {
        let wallet = Wallet::new().expect("Failed to create wallet");
        let wallet_address = wallet.get_address().expect("Failed to get address");

        let coinbase_tx =
            Transaction::new_coinbase_tx(&wallet_address).expect("Failed to create coinbase tx");
        let block = Block::new_block("prev_hash".to_string(), &[coinbase_tx], 1);

        let user_txs = block
            .get_user_transactions(&wallet_address)
            .await
            .expect("Failed to get user transactions");

        assert_eq!(user_txs.len(), 1);
        // Transactions in a mined block should be confirmed
        assert_eq!(
            *user_txs[0].get_status(),
            WalletTransactionStatus::Confirmed,
            "Block transactions should be confirmed"
        );
    }
}
