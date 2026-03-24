use bitcoin_api::{
    AdminClient, ApiConfig, ApiResponse, BlockSummary, BlockchainInfo, CreateWalletRequest,
    CreateWalletResponse, SendTransactionRequest, SendTransactionResponse,
};
use serde_json::Value;

pub async fn fetch_info(cfg: ApiConfig) -> Result<ApiResponse<BlockchainInfo>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_blockchain_info()
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_blocks(cfg: ApiConfig) -> Result<ApiResponse<Vec<BlockSummary>>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_latest_blocks().await.map_err(|e| e.to_string())
}

// Additional admin endpoints
pub async fn fetch_blocks_all(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_blocks().await.map_err(|e| e.to_string())
}

pub async fn fetch_block_by_hash(
    cfg: ApiConfig,
    hash: String,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_block_by_hash(&hash)
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_mining_info(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_mining_info().await.map_err(|e| e.to_string())
}

pub async fn generate_to_address(
    cfg: ApiConfig,
    address: String,
    nblocks: u32,
    maxtries: Option<u32>,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .generate_to_address(&address, nblocks, maxtries)
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_health(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.health().await.map_err(|e| e.to_string())
}

pub async fn fetch_liveness(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.liveness().await.map_err(|e| e.to_string())
}

pub async fn fetch_readiness(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.readiness().await.map_err(|e| e.to_string())
}

pub async fn fetch_mempool(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_mempool().await.map_err(|e| e.to_string())
}

pub async fn fetch_mempool_tx(cfg: ApiConfig, txid: String) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_mempool_transaction(&txid)
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_transactions(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_transactions().await.map_err(|e| e.to_string())
}

pub async fn fetch_address_transactions(
    cfg: ApiConfig,
    address: String,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_address_transactions_admin(&address)
        .await
        .map_err(|e| e.to_string())
}

// Wallet admin functions
pub async fn create_wallet_admin(
    cfg: ApiConfig,
    req: CreateWalletRequest,
) -> Result<ApiResponse<CreateWalletResponse>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .create_wallet_admin(&req)
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_addresses_admin(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_addresses_admin()
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_wallet_info_admin(
    cfg: ApiConfig,
    address: String,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_wallet_info_admin(&address)
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_balance_admin(
    cfg: ApiConfig,
    address: String,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_balance_admin(&address)
        .await
        .map_err(|e| e.to_string())
}

// Send transaction using admin client
pub async fn send_transaction(
    cfg: ApiConfig,
    req: SendTransactionRequest,
) -> Result<ApiResponse<SendTransactionResponse>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .send_transaction_admin(&req)
        .await
        .map_err(|e| e.to_string())
}
