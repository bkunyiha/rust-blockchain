// Validation service for web layer
use crate::web::models::requests::*;
use validator::Validate;

/// Validation service for web requests
pub struct ValidationService;

impl ValidationService {
    /// Validate a create wallet request
    pub fn validate_create_wallet_request(request: &CreateWalletRequest) -> Result<(), String> {
        request.validate().map_err(|e| e.to_string())
    }

    /// Validate a send transaction request
    pub fn validate_send_transaction_request(
        request: &SendTransactionRequest,
    ) -> Result<(), String> {
        request.validate().map_err(|e| e.to_string())
    }

    /// Validate a mining request
    pub fn validate_mining_request(request: &MiningRequest) -> Result<(), String> {
        request.validate().map_err(|e| e.to_string())
    }

    /// Validate a block query
    pub fn validate_block_query(query: &BlockQuery) -> Result<(), String> {
        query.validate().map_err(|e| e.to_string())
    }

    /// Validate a transaction query
    pub fn validate_transaction_query(query: &TransactionQuery) -> Result<(), String> {
        query.validate().map_err(|e| e.to_string())
    }
}
