use crate::types::*;
use reqwest::Client as HttpClient;
use serde_json::{Value, json};
use thiserror::Error;
use url::Url;
#[cfg(feature = "client")]
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("invalid base url: {0}")]
    InvalidBaseUrl(String),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Clone, Debug)]
pub struct ApiConfig {
    pub base_url: String,
    pub api_key: Option<String>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:8080".to_string(),
            api_key: None,
        }
    }
}

#[derive(Clone)]
pub struct BaseClient {
    http: HttpClient,
    base: Url,
    api_key: Option<String>,
}

impl BaseClient {
    pub fn new(config: ApiConfig) -> Result<Self, ApiError> {
        let base = Url::parse(&config.base_url)
            .map_err(|_| ApiError::InvalidBaseUrl(config.base_url.clone()))?;
        Ok(Self {
            http: HttpClient::new(),
            base,
            api_key: config.api_key,
        })
    }

    fn url(&self, path: &str) -> Result<Url, ApiError> {
        self.base
            .join(path)
            .map_err(|_| ApiError::InvalidBaseUrl(self.base.to_string()))
    }

    fn with_auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(ref key) = self.api_key {
            req.header("X-API-Key", key)
        } else {
            req
        }
    }
}

#[cfg(feature = "wallet")]
#[derive(Clone)]
pub struct WalletClient {
    base: BaseClient,
}

#[cfg(feature = "wallet")]
impl WalletClient {
    pub fn new(config: ApiConfig) -> Result<Self, ApiError> {
        Ok(Self {
            base: BaseClient::new(config)?,
        })
    }

    pub async fn create_wallet(
        &self,
        req: &CreateWalletRequest,
    ) -> Result<ApiResponse<CreateWalletResponse>, ApiError> {
        let url = self.base.url("/api/v1/wallet")?;
        let rb = self.base.http.post(url).json(req);
        let rb = self.base.with_auth(rb);
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_wallet_info(&self, address: &str) -> Result<ApiResponse<Value>, ApiError> {
        let url = self.base.url(&format!("/api/v1/wallet/{}", address))?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        let json = resp.json().await?;
        tracing::debug!(
            "[get_wallet_info_admin]: {}",
            serde_json::to_string_pretty(&json).unwrap_or_else(|_| "Error formatting".into())
        );
        Ok(json)
    }

    pub async fn get_balance(&self, address: &str) -> Result<ApiResponse<Value>, ApiError> {
        let url = self
            .base
            .url(&format!("/api/v1/wallet/{}/balance", address))?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn send_transaction(
        &self,
        req: &SendTransactionRequest,
    ) -> Result<ApiResponse<SendTransactionResponse>, ApiError> {
        let url = self.base.url("/api/v1/transactions")?;
        let rb = self.base.http.post(url).json(req);
        let rb = self.base.with_auth(rb);
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_address_transactions(
        &self,
        address: &str,
    ) -> Result<ApiResponse<Value>, ApiError> {
        let url = self
            .base
            .url(&format!("/api/v1/transactions/address/{}", address))?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }
}

#[cfg(feature = "admin")]
#[derive(Clone)]
pub struct AdminClient {
    base: BaseClient,
}

#[cfg(feature = "admin")]
impl AdminClient {
    pub fn new(config: ApiConfig) -> Result<Self, ApiError> {
        Ok(Self {
            base: BaseClient::new(config)?,
        })
    }

    pub async fn get_blockchain_info(&self) -> Result<ApiResponse<BlockchainInfo>, ApiError> {
        let url = self.base.url("/api/admin/blockchain")?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_latest_blocks(&self) -> Result<ApiResponse<Vec<BlockSummary>>, ApiError> {
        let url = self.base.url("/api/admin/blockchain/blocks/latest")?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_blocks(&self) -> Result<ApiResponse<Value>, ApiError> {
        let url = self.base.url("/api/admin/blockchain/blocks")?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_block_by_hash(&self, hash: &str) -> Result<ApiResponse<Value>, ApiError> {
        let url = self
            .base
            .url(&format!("/api/admin/blockchain/blocks/{}", hash))?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_mining_info(&self) -> Result<ApiResponse<Value>, ApiError> {
        let url = self.base.url("/api/admin/mining/info")?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn generate_to_address(
        &self,
        address: &str,
        nblocks: u32,
        maxtries: Option<u32>,
    ) -> Result<ApiResponse<Value>, ApiError> {
        let url = self.base.url("/api/admin/mining/generatetoaddress")?;
        let body = json!({ "address": address, "nblocks": nblocks, "maxtries": maxtries });
        let rb = self.base.with_auth(self.base.http.post(url).json(&body));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn health(&self) -> Result<ApiResponse<Value>, ApiError> {
        let url = self.base.url("/api/admin/health")?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn liveness(&self) -> Result<ApiResponse<Value>, ApiError> {
        let url = self.base.url("/api/admin/health/live")?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn readiness(&self) -> Result<ApiResponse<Value>, ApiError> {
        let url = self.base.url("/api/admin/health/ready")?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    // -----------------
    // Wallet (admin)
    // -----------------
    pub async fn create_wallet_admin(
        &self,
        req: &CreateWalletRequest,
    ) -> Result<ApiResponse<CreateWalletResponse>, ApiError> {
        let url = self.base.url("/api/admin/wallet")?;
        let rb = self.base.with_auth(self.base.http.post(url).json(req));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_addresses_admin(&self) -> Result<ApiResponse<Value>, ApiError> {
        let url = self.base.url("/api/admin/wallet/addresses")?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_wallet_info_admin(
        &self,
        address: &str,
    ) -> Result<ApiResponse<Value>, ApiError> {
        let url = self.base.url(&format!("/api/admin/wallet/{}", address))?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        let json = resp.json().await?;
        tracing::debug!(
            "[get_wallet_info_admin]: {}",
            serde_json::to_string_pretty(&json).unwrap_or_else(|_| "Error formatting".into())
        );
        Ok(json)
    }

    pub async fn get_balance_admin(&self, address: &str) -> Result<ApiResponse<Value>, ApiError> {
        let url = self
            .base
            .url(&format!("/api/admin/wallet/{}/balance", address))?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_mempool(&self) -> Result<ApiResponse<Value>, ApiError> {
        let url = self.base.url("/api/admin/transactions/mempool")?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_mempool_transaction(
        &self,
        txid: &str,
    ) -> Result<ApiResponse<Value>, ApiError> {
        let url = self
            .base
            .url(&format!("/api/admin/transactions/mempool/{}", txid))?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_transactions(&self) -> Result<ApiResponse<Value>, ApiError> {
        let url = self.base.url("/api/admin/transactions")?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_address_transactions_admin(
        &self,
        address: &str,
    ) -> Result<ApiResponse<Value>, ApiError> {
        let url = self
            .base
            .url(&format!("/api/admin/transactions/address/{}", address))?;
        let rb = self.base.with_auth(self.base.http.get(url));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn send_transaction_admin(
        &self,
        req: &SendTransactionRequest,
    ) -> Result<ApiResponse<SendTransactionResponse>, ApiError> {
        let url = self.base.url("/api/admin/transactions")?;
        let rb = self.base.with_auth(self.base.http.post(url).json(req));
        let resp = rb.send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }
}
