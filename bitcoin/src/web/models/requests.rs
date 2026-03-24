use crate::WalletAddress;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Request model for creating a new wallet
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateWalletRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Wallet name must be between 1 and 100 characters"
    ))]
    pub name: Option<String>,
}

/// Request model for sending a transaction
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct SendTransactionRequest {
    /// Already validated via WalletAddress::validate()
    pub from_address: WalletAddress,

    /// Already validated via WalletAddress::validate()
    pub to_address: WalletAddress,

    #[validate(range(min = 1, message = "Amount must be greater than 0"))]
    pub amount: i32,
}

/// Request model for mining operations
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct MiningRequest {
    #[validate(length(min = 26, max = 35, message = "Invalid mining address format"))]
    pub mining_address: String,

    #[validate(range(min = 1, max = 10, message = "Thread count must be between 1 and 10"))]
    pub thread_count: Option<u8>,
}

/// Request model for querying blocks
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct BlockQuery {
    #[validate(range(min = 0, message = "Page must be 0 or greater"))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    pub limit: Option<u32>,

    pub hash: Option<String>,
}

/// Request model for querying transactions
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct TransactionQuery {
    #[validate(range(min = 0, message = "Page must be 0 or greater"))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    pub limit: Option<u32>,

    pub txid: Option<String>,
}

/// Request model for balance queries
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct BalanceQuery {
    #[validate(length(min = 26, max = 35, message = "Invalid address format"))]
    pub address: String,
}

/// Request model for generatetoaddress RPC command
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct GenerateToAddressRequest {
    /// Number of blocks to generate
    #[validate(range(
        min = 1,
        max = 1000,
        message = "Block count must be between 1 and 1000"
    ))]
    pub nblocks: u32,
    /// Address to receive block rewards
    #[validate(length(min = 26, max = 35, message = "Invalid address format"))]
    pub address: String,
    /// Maximum iterations to try (optional)
    #[validate(range(
        min = 1,
        max = 1000000,
        message = "Max tries must be between 1 and 1000000"
    ))]
    pub maxtries: Option<u32>,
}
