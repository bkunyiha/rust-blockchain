use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAddress {
    pub address: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSummary {
    pub hash: String,
    pub height: u64,
    pub timestamp: u64,
    pub transaction_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionRequest {
    pub from_address: String,
    pub to_address: String,
    pub amount_satoshis: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWalletResponse {
    pub address: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub txid: String,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolInfo {
    pub size: u32,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningInfo {
    pub blocks: u64,
    pub difficulty: f64,
    pub mining_address: Option<String>,
}
