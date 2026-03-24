use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::web::handlers::{blockchain, health, mining, transaction, wallet};

/// OpenAPI documentation for the Blockchain API
#[derive(OpenApi)]
#[openapi(
    paths(
        // Health endpoints
        health::health_check,
        health::liveness,
        health::readiness,
        // Blockchain endpoints
        blockchain::get_blockchain_info,
        blockchain::get_blocks,
        blockchain::get_latest_blocks,
        blockchain::get_block_by_hash,
        // Wallet endpoints
        wallet::create_wallet,
        wallet::get_addresses,
        wallet::get_wallet_info,
        wallet::get_balance,
        // Transaction endpoints(mempool)
        transaction::send_transaction,
        transaction::get_mempool_transaction,
        transaction::get_mempool,
        // Transaction endpoints(blockchain)
        transaction::get_transactions,
        transaction::get_address_transactions,
        // Mining endpoints
        mining::get_mining_info,
        mining::generate_to_address,
    ),
    components(
        schemas(
            // Response schemas
            crate::web::models::responses::HealthResponse,
            crate::web::models::responses::BlockchainInfoResponse,
            crate::web::models::responses::BlockResponse,
            crate::web::models::responses::SendBitCoinResponse,
            crate::web::models::responses::TransactionResponse,
            crate::web::models::responses::TxInputSummaryResponse,
            crate::web::models::responses::TxOutputSummaryResponse,
            crate::web::models::responses::TxSummaryResponse,
            crate::web::models::responses::WalletTransactionRespose,
            crate::web::models::responses::WalletResponse,
            crate::web::models::responses::BalanceResponse,
            crate::web::models::responses::MiningStatusResponse,
            // Request schemas
            crate::web::models::requests::CreateWalletRequest,
            crate::web::models::requests::SendTransactionRequest,
            crate::web::models::requests::MiningRequest,
            // Error schemas
            crate::web::models::errors::ErrorResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Blockchain", description = "Blockchain data and information"),
        (name = "Wallet", description = "Wallet management and operations"),
        (name = "Transaction", description = "Transaction creation and management"),
        (name = "Mining", description = "Mining operations and status"),
    ),
    info(
        title = "Blockchain API",
        version = "0.1.0",
        description = "A comprehensive blockchain API for managing wallets, transactions, and mining operations"
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development server")
    )
)]
pub struct ApiDoc;

/// Create Swagger UI router
pub fn create_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())
}
