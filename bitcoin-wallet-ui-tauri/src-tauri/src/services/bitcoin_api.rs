use bitcoin_api::{
    ApiConfig, ApiResponse, CreateWalletRequest, CreateWalletResponse,
    SendTransactionRequest, SendTransactionResponse, WalletClient,
};
use serde_json::Value;

pub struct BitcoinApiService;

impl BitcoinApiService {
    pub async fn create_wallet(
        cfg: ApiConfig,
        label: Option<String>,
    ) -> Result<ApiResponse<CreateWalletResponse>, String> {
        let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
        let req = CreateWalletRequest { label };
        client.create_wallet(&req).await.map_err(|e| e.to_string())
    }

    pub async fn get_wallet_info(
        cfg: ApiConfig,
        address: &str,
    ) -> Result<ApiResponse<Value>, String> {
        let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
        client.get_wallet_info(address).await.map_err(|e| e.to_string())
    }

    pub async fn get_balance(
        cfg: ApiConfig,
        address: &str,
    ) -> Result<ApiResponse<Value>, String> {
        let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
        client.get_balance(address).await.map_err(|e| e.to_string())
    }

    pub async fn send_transaction(
        cfg: ApiConfig,
        from: &str,
        to: &str,
        amount: u64,
    ) -> Result<ApiResponse<SendTransactionResponse>, String> {
        let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
        let req = SendTransactionRequest {
            from_address: from.to_string(),
            to_address: to.to_string(),
            amount,
        };
        client.send_transaction(&req).await.map_err(|e| e.to_string())
    }

    pub async fn get_address_transactions(
        cfg: ApiConfig,
        address: &str,
    ) -> Result<ApiResponse<Value>, String> {
        let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
        client.get_address_transactions(address).await.map_err(|e| e.to_string())
    }

    pub async fn health_check(cfg: ApiConfig) -> Result<bool, String> {
        let client = bitcoin_api::AdminClient::new(cfg).map_err(|e| e.to_string())?;
        match client.health().await {
            Ok(resp) => Ok(resp.success),
            Err(_) => Ok(false),
        }
    }
}
