use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Generic API response wrapper
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
        }
    }
}

/// Blockchain information response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BlockchainInfoResponse {
    pub height: usize,
    pub difficulty: u32,
    pub total_blocks: usize,
    pub total_transactions: usize,
    pub mempool_size: usize,
    pub last_block_hash: String,
    pub last_block_timestamp: DateTime<Utc>,
}

/// Block response model
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BlockResponse {
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: DateTime<Utc>,
    pub height: usize,
    pub nonce: u64,
    pub difficulty: u32,
    pub transaction_count: usize,
    pub merkle_root: String,
    pub size_bytes: usize,
}

/// Transaction response model
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SendBitCoinResponse {
    pub txid: String,
    pub timestamp: DateTime<Utc>,
}

/// Transaction response model
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponse {
    pub txid: String,
    pub is_coinbase: bool,
    pub input_count: usize,
    pub output_count: usize,
    pub total_input_value: i32,
    pub total_output_value: i32,
    pub fee: i32,
    pub timestamp: DateTime<Utc>,
    pub size_bytes: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TxInputSummaryResponse {
    pub txid_hex: String,
    pub output_idx: usize,
    pub wlt_addr: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TxOutputSummaryResponse {
    pub wlt_addr: String,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TxSummaryResponse {
    pub transaction_id: String,
    pub inputs: Vec<TxInputSummaryResponse>,
    pub outputs: Vec<TxOutputSummaryResponse>,
}

/// Transaction response model
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WalletTransactionRespose {
    pub tx_id: Vec<u8>,
    pub from_wlt_addr: Option<String>,
    pub to_wlt_addr: String,
    pub value: i32,
    pub transaction_type: String,
    pub status: String,
    pub vout: usize,
    pub is_coinbase: bool,
    pub input_count: usize,
    pub output_count: usize,
    pub total_output_value: i32,
    pub fee: i32,
    pub timestamp: i64,
    pub size_bytes: usize,
}

/// Wallet response model
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WalletResponse {
    pub address: String,
    pub public_key: String,
    pub created_at: DateTime<Utc>,
}

/// Balance response model
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BalanceResponse {
    pub address: String,
    pub balance: i32,
    pub unconfirmed_balance: i32,
    pub utxo_count: usize,
    pub last_updated: DateTime<Utc>,
}

/// Mining status response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MiningStatusResponse {
    pub is_mining: bool,
    pub mining_address: Option<String>,
    pub hash_rate: f64,
    pub blocks_mined: usize,
    pub last_block_time: Option<DateTime<Utc>>,
}

/// Health check response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub blockchain_height: usize,
    pub connected_peers: usize,
    pub memory_usage_mb: f64,
}

/// Paginated response wrapper
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub limit: u32,
    pub total: u32,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

/// Response for getmininginfo RPC command
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MiningInfoResponse {
    /// Current block height
    pub blocks: u64,
    /// Size of current block in bytes
    pub currentblocksize: u64,
    /// Number of transactions in current block
    pub currentblocktx: u32,
    /// Current difficulty
    pub difficulty: f64,
    /// Network hash rate per second
    pub networkhashps: f64,
    /// Number of pooled transactions
    pub pooledtx: u32,
    /// Chain name (main, test, regtest)
    pub chain: String,
    /// Warnings
    pub warnings: String,
}

/// Response for generatetoaddress RPC command
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GenerateToAddressResponse {
    /// Array of generated block hashes
    pub block_hashes: Vec<String>,
    /// Status message
    pub message: String,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, page: u32, limit: u32, total: u32) -> Self {
        let total_pages = if total == 0 {
            0
        } else {
            ((total as f64 / limit as f64).ceil() as u32).max(1)
        };
        // Page is 1-indexed (page 1 is first page)
        // has_next: current page < total pages
        // has_prev: current page > 1
        Self {
            has_next: page < total_pages,
            has_prev: page > 1,
            items,
            page,
            limit,
            total,
            total_pages,
        }
    }
}
