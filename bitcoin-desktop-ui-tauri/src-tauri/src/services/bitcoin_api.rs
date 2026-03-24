use crate::config::ApiConfig;
use bitcoin_api::client::AdminClient;
use bitcoin_api::types::{CreateWalletRequest, SendTransactionRequest};
use serde_json::{json, Value};

pub struct BitcoinApiService;

impl BitcoinApiService {
    // Blockchain operations
    pub async fn get_blockchain_info(config: &ApiConfig) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_blockchain_info().await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn get_latest_blocks(config: &ApiConfig) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_latest_blocks().await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn get_all_blocks(config: &ApiConfig) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_blocks().await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn get_block_by_hash(config: &ApiConfig, hash: &str) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_block_by_hash(hash).await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    // Wallet operations
    pub async fn create_wallet(
        config: &ApiConfig,
        label: Option<String>,
    ) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        let request = CreateWalletRequest { label };
        match client.create_wallet_admin(request).await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn get_wallet_info(config: &ApiConfig, address: &str) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_wallet_info_admin(address).await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn get_balance(config: &ApiConfig, address: &str) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_balance_admin(address).await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn send_transaction(
        config: &ApiConfig,
        from_address: &str,
        to_address: &str,
        amount_satoshis: u64,
    ) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        let request = SendTransactionRequest {
            from_address: from_address.to_string(),
            to_address: to_address.to_string(),
            amount_satoshis,
        };
        match client.send_transaction_admin(request).await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn get_tx_history(config: &ApiConfig, address: &str) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_address_transactions_admin(address).await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn get_all_addresses(config: &ApiConfig) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_addresses_admin().await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    // Transaction operations
    pub async fn get_mempool(config: &ApiConfig) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_mempool().await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn get_mempool_transaction(
        config: &ApiConfig,
        txid: &str,
    ) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_mempool_transaction(txid).await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn get_all_transactions(config: &ApiConfig) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_transactions().await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn get_address_transactions(
        config: &ApiConfig,
        address: &str,
    ) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_address_transactions_admin(address).await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    // Mining operations
    pub async fn get_mining_info(config: &ApiConfig) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.get_mining_info().await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn generate_blocks(
        config: &ApiConfig,
        address: &str,
        nblocks: u32,
        maxtries: Option<u32>,
    ) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.generate_to_address(address, nblocks, maxtries).await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    // Health check operations
    pub async fn health_check(config: &ApiConfig) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.health().await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn liveness_check(config: &ApiConfig) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.liveness().await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }

    pub async fn readiness_check(config: &ApiConfig) -> Result<Value, String> {
        let client = AdminClient::new(&config.base_url, &config.api_key);
        match client.readiness().await {
            Ok(response) => {
                if response.success {
                    Ok(json!(response.data))
                } else {
                    Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                }
            }
            Err(e) => Err(format!("API error: {}", e)),
        }
    }
}
