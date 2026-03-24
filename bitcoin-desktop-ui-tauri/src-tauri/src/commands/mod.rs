pub mod blockchain;
pub mod health;
pub mod mining;
pub mod settings;
pub mod transactions;
pub mod wallet;

pub use blockchain::{get_all_blocks, get_block_by_hash, get_blockchain_info, get_latest_blocks};
pub use health::{health_check, liveness_check, readiness_check};
pub use mining::{generate_blocks, get_mining_info};
pub use settings::{check_connection, get_config, update_config};
pub use transactions::{
    get_address_transactions, get_all_transactions, get_mempool, get_mempool_transaction,
};
pub use wallet::{
    create_wallet, get_all_addresses, get_balance, get_tx_history, get_wallet_info,
    send_transaction,
};
