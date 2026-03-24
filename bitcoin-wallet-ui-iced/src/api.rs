use bitcoin_api::{
    ApiConfig, ApiResponse, CreateWalletRequest, CreateWalletResponse, SendTransactionRequest,
    SendTransactionResponse, WalletClient,
};
use serde_json::Value;

pub async fn create_wallet(
    cfg: ApiConfig,
    req: CreateWalletRequest,
) -> Result<ApiResponse<CreateWalletResponse>, String> {
    let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
    client.create_wallet(&req).await.map_err(|e| e.to_string())
}

pub async fn send_tx(
    cfg: ApiConfig,
    req: SendTransactionRequest,
) -> Result<ApiResponse<SendTransactionResponse>, String> {
    let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .send_transaction(&req)
        .await
        .map_err(|e| e.to_string())
}

// Wallet info and balance functions using admin client
pub async fn fetch_wallet_info(
    cfg: ApiConfig,
    address: String,
) -> Result<ApiResponse<Value>, String> {
    let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_wallet_info(&address)
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_balance(cfg: ApiConfig, address: String) -> Result<ApiResponse<Value>, String> {
    let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_balance(&address)
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_address_transactions(
    cfg: ApiConfig,
    address: String,
) -> Result<ApiResponse<Value>, String> {
    let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_address_transactions(&address)
        .await
        .map_err(|e| e.to_string())
}
