use serde::{Deserialize, Serialize};

// Basic shared DTOs matching server responses/requests (subset to start)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    // Server sends DateTime<Utc> which serializes to ISO 8601 string
    // We deserialize it as a String since we don't want chrono dependency
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWalletResponse {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionRequest {
    pub from_address: String,
    pub to_address: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionResponse {
    pub txid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub address: String,
    pub confirmed: u64,
    pub unconfirmed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSummary {
    pub hash: String,
    pub previous_hash: String,
    #[serde(deserialize_with = "deserialize_datetime_string")]
    pub timestamp: String, // DateTime<Utc> serializes to ISO 8601 string
    pub height: usize,
    pub nonce: u64,
    pub difficulty: u32,
    pub transaction_count: usize,
    pub merkle_root: String,
    pub size_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub height: usize,
    pub difficulty: u32,
    pub total_blocks: usize,
    pub total_transactions: usize,
    pub mempool_size: usize,
    pub last_block_hash: String,
    #[serde(deserialize_with = "deserialize_datetime_string")]
    pub last_block_timestamp: String, // DateTime<Utc> serializes to ISO 8601 string
}

fn deserialize_datetime_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    String::deserialize(deserializer)
}
