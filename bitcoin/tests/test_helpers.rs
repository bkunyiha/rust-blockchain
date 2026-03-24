use blockchain::{BlockchainService, Transaction, WalletService};
use std::path::PathBuf;
use tempfile::TempDir;

/// Generate a unique genesis address for testing
pub fn generate_test_genesis_address() -> blockchain::WalletAddress {
    blockchain::Wallet::new()
        .and_then(|wallet| wallet.get_address())
        .expect("Failed to create test wallet address")
}

/// Create a temporary directory for testing
pub fn create_temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

/// Set environment variables for blockchain database
pub fn set_blockchain_env_vars(db_path: &PathBuf) {
    unsafe {
        std::env::set_var("TREE_DIR", db_path.to_str().unwrap());
        std::env::set_var("BLOCKS_TREE", db_path.to_str().unwrap());
    }
}

/// Create a blockchain with given genesis address
pub async fn create_blockchain_with_address(
    genesis_address: &blockchain::WalletAddress,
    db_path: &PathBuf,
) -> BlockchainService {
    set_blockchain_env_vars(db_path);
    BlockchainService::initialize(genesis_address)
        .await
        .expect("Failed to create test blockchain")
}

/// Create a temporary blockchain for testing
pub async fn create_temp_blockchain() -> (BlockchainService, TempDir) {
    let temp_dir = create_temp_dir();
    let db_path = temp_dir.path().join("test_blockchain");
    let genesis_address = generate_test_genesis_address();
    let blockchain = create_blockchain_with_address(&genesis_address, &db_path).await;
    (blockchain, temp_dir)
}

/// Create a coinbase transaction for given address
pub fn create_coinbase_transaction(address: &blockchain::WalletAddress) -> Transaction {
    Transaction::new_coinbase_tx(address).expect("Failed to create coinbase transaction")
}

/// Mine a block with given transactions
pub async fn mine_block(
    blockchain: &BlockchainService,
    transactions: &[Transaction],
) -> blockchain::Block {
    blockchain
        .mine_block(transactions)
        .await
        .expect("Failed to mine block")
}

/// Add a block to the blockchain
pub async fn add_block(blockchain: &BlockchainService, block: &blockchain::Block) {
    blockchain
        .add_block(block)
        .await
        .expect("Failed to add block");
}

/// Create a single block with coinbase transaction
pub async fn create_single_block(
    blockchain: &BlockchainService,
    address: &blockchain::WalletAddress,
) -> blockchain::Block {
    let coinbase_tx = create_coinbase_transaction(address);
    let transactions = vec![coinbase_tx];
    mine_block(blockchain, &transactions).await
}

/// Helper function to create a blockchain with some initial blocks
pub async fn create_blockchain_with_blocks(num_blocks: usize) -> (BlockchainService, TempDir) {
    let (blockchain, temp_dir) = create_temp_blockchain().await;
    let genesis_address = generate_test_genesis_address();

    for _ in 0..num_blocks {
        let block = create_single_block(&blockchain, &genesis_address).await;
        add_block(&blockchain, &block).await;
    }

    (blockchain, temp_dir)
}

/// Helper function to create test wallets
pub fn create_test_wallets() -> WalletService {
    WalletService::new().expect("Failed to create test wallets")
}

/// Collect blocks from iterator into a sorted vector
pub async fn collect_and_sort_blocks(
    blockchain: &BlockchainService,
) -> Option<Vec<blockchain::Block>> {
    let mut iterator = blockchain.iterator().await.ok()?;
    let mut blocks = Vec::new();
    while let Some(block) = iterator.next() {
        blocks.push(block);
    }
    blocks.sort_by_key(|block: &blockchain::Block| block.get_height());
    Some(blocks)
}

/// Verify a single block's integrity
pub fn verify_block_integrity(
    block: &blockchain::Block,
    expected_height: usize,
    prev_block_hash: Option<&str>,
) -> bool {
    block.get_height() == expected_height
        && prev_block_hash.map_or(true, |hash| block.get_pre_block_hash() == hash)
}

/// Verify blockchain integrity using functional approach
pub async fn verify_blockchain_integrity(blockchain: &BlockchainService) -> bool {
    collect_and_sort_blocks(blockchain)
        .await
        .map(|blocks| {
            blocks.iter().enumerate().all(|(i, block)| {
                let expected_height = i + 1;
                let prev_hash = if i > 0 {
                    Some(blocks[i - 1].get_hash().as_ref())
                } else {
                    None
                };
                verify_block_integrity(block, expected_height, prev_hash)
            })
        })
        .unwrap_or(false)
}

/// Create a single test address
pub fn create_single_test_address(wallets: &mut WalletService) -> blockchain::WalletAddress {
    wallets.create_wallet().expect("Failed to create wallet")
}

/// Create multiple test addresses using functional approach
pub fn create_test_addresses(count: usize) -> Vec<blockchain::WalletAddress> {
    let mut wallets = create_test_wallets();
    (0..count)
        .map(|_| create_single_test_address(&mut wallets))
        .collect()
}

/// Validate that all addresses are non-empty
pub fn validate_addresses(addresses: &[blockchain::WalletAddress]) -> bool {
    addresses.iter().all(|addr| !addr.as_str().is_empty())
}

/// Compose blockchain creation with validation
pub async fn create_validated_blockchain() -> (BlockchainService, TempDir) {
    let (blockchain, temp_dir) = create_temp_blockchain().await;
    assert_eq!(
        blockchain
            .get_best_height()
            .await
            .expect("Failed to get height"),
        1
    );
    (blockchain, temp_dir)
}

/// Compose blockchain creation with blocks and validation
pub async fn create_validated_blockchain_with_blocks(
    num_blocks: usize,
) -> (BlockchainService, TempDir) {
    let (blockchain, temp_dir) = create_blockchain_with_blocks(num_blocks).await;
    let expected_height = num_blocks + 1;
    assert_eq!(
        blockchain
            .get_best_height()
            .await
            .expect("Failed to get height"),
        expected_height
    );
    (blockchain, temp_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_temp_blockchain() {
        let (_blockchain, temp_dir) = create_validated_blockchain().await;
        assert!(temp_dir.path().exists());
    }

    #[tokio::test]
    async fn test_create_blockchain_with_blocks() {
        let (blockchain, _temp_dir) = create_validated_blockchain_with_blocks(3).await;
        assert_eq!(
            blockchain
                .get_best_height()
                .await
                .expect("Failed to get height"),
            4
        );
    }

    #[test]
    fn test_create_test_wallets() {
        let mut wallets = create_test_wallets();
        let address = create_single_test_address(&mut wallets);
        assert!(!address.as_str().is_empty());
    }

    #[tokio::test]
    async fn test_verify_blockchain_integrity() {
        let (blockchain, _temp_dir) = create_validated_blockchain_with_blocks(2).await;
        assert!(verify_blockchain_integrity(&blockchain).await);
    }

    #[test]
    fn test_create_test_addresses() {
        let addresses = create_test_addresses(3);
        assert_eq!(addresses.len(), 3);
        assert!(validate_addresses(&addresses));
    }

    #[tokio::test]
    async fn test_functional_block_creation() {
        let (blockchain, _temp_dir) = create_temp_blockchain().await;
        let genesis_address = generate_test_genesis_address();

        // Test functional block creation
        let block = create_single_block(&blockchain, &genesis_address).await;
        add_block(&blockchain, &block).await;

        assert_eq!(
            blockchain
                .get_best_height()
                .await
                .expect("Failed to get height"),
            2
        );
    }
}
