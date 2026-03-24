use blockchain::{
    BlockchainService, ConnectNode, GLOBAL_CONFIG, Transaction, UTXOSet, WalletService,
};
use std::str::FromStr;

mod test_helpers;

/// Generate a unique genesis address for testing
fn generate_test_genesis_address() -> blockchain::WalletAddress {
    blockchain::Wallet::new()
        .and_then(|wallet| wallet.get_address())
        .expect("Failed to create test wallet address")
}

/// Create a unique database path with timestamp and UUID
fn create_unique_db_path() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("test_integration_db_{}_{}", timestamp, uuid::Uuid::new_v4())
}

/// Set environment variables for blockchain database
fn set_blockchain_env_vars(db_path: &str) {
    unsafe {
        std::env::set_var("TREE_DIR", db_path);
        std::env::set_var("BLOCKS_TREE", db_path);
    }
}

/// Create a blockchain with given genesis address and database path
async fn create_blockchain_with_config(
    genesis_address: &blockchain::WalletAddress,
    db_path: &str,
) -> BlockchainService {
    set_blockchain_env_vars(db_path);
    BlockchainService::initialize(genesis_address)
        .await
        .expect("Failed to create test blockchain")
}

/// Create a blockchain with given genesis address and database path (clears existing data)
async fn create_blockchain_with_config_clean(
    genesis_address: &blockchain::WalletAddress,
    db_path: &str,
) -> BlockchainService {
    let _ = std::fs::remove_dir_all(db_path);
    set_blockchain_env_vars(db_path);
    BlockchainService::initialize(genesis_address)
        .await
        .expect("Failed to create test blockchain")
}

/// Create a test blockchain with automatic cleanup
async fn create_test_blockchain() -> (BlockchainService, String) {
    let db_path = create_unique_db_path();
    let genesis_address = generate_test_genesis_address();
    let blockchain = create_blockchain_with_config_clean(&genesis_address, &db_path).await;
    (blockchain, db_path)
}

/// Clean up test blockchain database
fn cleanup_test_blockchain(db_path: &str) {
    let _ = std::fs::remove_dir_all(db_path);
}

/// Database guard with automatic cleanup via Drop trait
struct TestDatabaseGuard {
    db_path: String,
}

impl TestDatabaseGuard {
    fn new_with_cleanup() -> Self {
        let db_path = create_unique_db_path();
        set_blockchain_env_vars(&db_path);
        TestDatabaseGuard { db_path }
    }

    fn get_path(&self) -> &str {
        &self.db_path
    }
}

impl Drop for TestDatabaseGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.db_path);
    }
}

/// Create a coinbase transaction for given address
fn create_coinbase_transaction(address: &blockchain::WalletAddress) -> Transaction {
    Transaction::new_coinbase_tx(address).expect("Failed to create coinbase transaction")
}

/// Mine a block with given transactions
async fn mine_block(
    blockchain: &BlockchainService,
    transactions: &[Transaction],
) -> blockchain::Block {
    blockchain
        .mine_block(transactions)
        .await
        .expect("Failed to mine block")
}

/// Add a block to the blockchain
async fn add_block(blockchain: &BlockchainService, block: &blockchain::Block) {
    blockchain
        .add_block(block)
        .await
        .expect("Failed to add block");
}

/// Create and add a single block with coinbase transaction
async fn create_and_add_block(
    blockchain: &BlockchainService,
    address: &blockchain::WalletAddress,
) -> blockchain::Block {
    let coinbase_tx = create_coinbase_transaction(address);
    let transactions = vec![coinbase_tx];
    let block = mine_block(blockchain, &transactions).await;
    add_block(blockchain, &block).await;
    block
}

/// Create a UTXO set and reindex it
async fn create_and_reindex_utxo_set(blockchain: BlockchainService) -> UTXOSet {
    let utxo_set = UTXOSet::new(blockchain);
    utxo_set
        .reindex()
        .await
        .expect("Failed to reindex UTXO set");
    utxo_set
}

/// Validate blockchain height
async fn validate_blockchain_height(
    blockchain: &BlockchainService,
    expected_height: usize,
) -> bool {
    blockchain
        .get_best_height()
        .await
        .expect("Failed to get height")
        == expected_height
}

/// Create a wallet with unique path
fn create_wallet_with_temp_path() -> (WalletService, tempfile::TempDir) {
    let temp_dir = tempfile::TempDir::new().expect("Failed to create temp directory");
    let wallet_path = temp_dir.path().join("test_wallets.dat");

    unsafe {
        std::env::set_var("WALLET_FILE", wallet_path.to_str().unwrap());
    }

    let wallets = WalletService::new().expect("Failed to create wallets");
    (wallets, temp_dir)
}

#[tokio::test]
async fn test_blockchain_integration() {
    let (blockchain, db_path) = create_test_blockchain().await;
    let genesis_address = generate_test_genesis_address();

    // Test creating a new blockchain
    assert!(validate_blockchain_height(&blockchain, 1).await);

    // Test mining a block with the same blockchain instance
    let _new_block = create_and_add_block(&blockchain, &genesis_address).await;
    assert!(validate_blockchain_height(&blockchain, 2).await);

    // Cleanup
    cleanup_test_blockchain(&db_path);
}

#[tokio::test]
async fn test_wallet_integration() {
    let (mut wallets, _temp_dir) = create_wallet_with_temp_path();
    let address = wallets.create_wallet().expect("Failed to create wallet");
    assert!(!address.as_str().is_empty());

    // Test getting wallet
    let wallet = wallets.get_wallet(&address).expect("Failed to get wallet");
    assert_eq!(
        wallet.get_address().expect("Failed to get address"),
        address
    );
}

#[tokio::test]
async fn test_utxo_set_integration() {
    let (blockchain, db_path) = create_test_blockchain().await;
    let genesis_address = generate_test_genesis_address();

    // Create blockchain and add a block
    create_and_add_block(&blockchain, &genesis_address).await;

    // Test UTXO set - need to reindex first
    let utxo_set = create_and_reindex_utxo_set(blockchain).await;
    let count = utxo_set
        .count_transactions()
        .await
        .expect("Failed to count transactions");
    assert!(count > 0);

    // Cleanup
    cleanup_test_blockchain(&db_path);
}

#[tokio::test]
async fn test_server_creation() {
    let (blockchain, db_path) = create_test_blockchain().await;

    // Test that we can create a server (the blockchain field is private, so we can't test it directly)
    let node_context = blockchain::node::NodeContext::new(blockchain);
    let _server = blockchain::Server::new(node_context);
    // If we get here without panicking, the server was created successfully

    // Cleanup
    cleanup_test_blockchain(&db_path);
}

#[tokio::test]
async fn test_connect_node_parsing() {
    // Test local node
    let local_node = ConnectNode::from_str("local").expect("Failed to parse local");
    assert!(!local_node.is_remote());

    // Test remote node
    let remote_node = ConnectNode::from_str("127.0.0.1:8080").expect("Failed to parse remote");
    assert!(remote_node.is_remote());

    // Test invalid address
    let invalid_result = ConnectNode::from_str("invalid_address");
    assert!(invalid_result.is_err());
}

#[tokio::test]
async fn test_global_config() {
    // Test that global config can be accessed
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    assert!(node_addr.port() > 0);

    let is_miner = GLOBAL_CONFIG.is_miner();
    // This should be a boolean value
    assert!(is_miner == true || is_miner == false);
}

#[tokio::test]
async fn test_transaction_creation_and_validation() {
    // Test coinbase transaction
    let genesis_address = generate_test_genesis_address();
    let coinbase_tx = create_coinbase_transaction(&genesis_address);
    assert!(coinbase_tx.is_coinbase());
    assert_eq!(coinbase_tx.get_vout().len(), 1);
    assert_eq!(coinbase_tx.get_vin().len(), 1);

    // Test transaction serialization
    let serialized = coinbase_tx.serialize().expect("Failed to serialize");
    let deserialized = Transaction::deserialize(&serialized).expect("Failed to deserialize");
    assert_eq!(coinbase_tx.get_id(), deserialized.get_id());
}

#[tokio::test]
async fn test_blockchain_persistence() {
    let _guard = TestDatabaseGuard::new_with_cleanup();
    let genesis_address = generate_test_genesis_address();

    // Create blockchain and add a block
    {
        let blockchain =
            create_blockchain_with_config_clean(&genesis_address, _guard.get_path()).await;
        create_and_add_block(&blockchain, &genesis_address).await;
    }

    // Create new blockchain instance and verify persistence
    let blockchain = create_blockchain_with_config(&genesis_address, _guard.get_path()).await;
    assert!(validate_blockchain_height(&blockchain, 2).await);
    // Guard will automatically clean up when it goes out of scope
}

#[tokio::test]
async fn test_blockchain_iterator() {
    let (blockchain, db_path) = create_test_blockchain().await;
    let genesis_address = generate_test_genesis_address();

    // Add multiple blocks using functional approach
    for _ in 0..3 {
        create_and_add_block(&blockchain, &genesis_address).await;
    }

    // Test iterator
    let mut iterator = blockchain
        .iterator()
        .await
        .expect("Failed to create iterator");
    let mut block_count = 0;
    while let Some(block) = iterator.next() {
        block_count += 1;
        assert!(block.get_height() > 0); // Fixed: height should be > 0, not >= 0
        assert!(!block.get_hash().is_empty());
    }

    // Should have genesis block + 3 new blocks = 4 total
    assert_eq!(block_count, 4);

    // Cleanup
    cleanup_test_blockchain(&db_path);
}

#[tokio::test]
async fn test_wallet_transaction_creation() {
    let (blockchain, db_path) = create_test_blockchain().await;

    // Create wallets with unique path
    let (mut wallets, _temp_dir) = create_wallet_with_temp_path();
    let address1 = wallets.create_wallet().expect("Failed to create wallet 1");
    let address2 = wallets.create_wallet().expect("Failed to create wallet 2");

    // Create blockchain with some initial balance to address1
    create_and_add_block(&blockchain, &address1).await;

    // Create UTXO set and reindex
    let utxo_set = create_and_reindex_utxo_set(blockchain).await;

    // Test creating a transaction between wallets
    let transaction_result = Transaction::new_utxo_transaction(&address1, &address2, 5, &utxo_set);
    let transaction = transaction_result.await;
    assert!(transaction.is_ok());

    let tx = transaction.expect("Failed to create transaction");
    assert!(!tx.is_coinbase());
    assert_eq!(tx.get_vout().len(), 2); // One output to address2, one change back to address1

    // Cleanup
    cleanup_test_blockchain(&db_path);
}

/// Global cleanup function to remove any remaining test directories
/// This can be called manually or used in test teardown
pub fn cleanup_all_test_directories() {
    use std::fs;
    use std::path::Path;

    let current_dir = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());

    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str.starts_with("test_") && name_str.contains("db_") {
                    let _ = std::fs::remove_dir_all(&path);
                }
            }
        }
    }
}
