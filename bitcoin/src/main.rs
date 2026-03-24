use blockchain::node::NodeContext;
use blockchain::web::server::create_web_server;
use blockchain::{
    BlockchainService, BtcError, ConnectNode, GLOBAL_CONFIG, Result, Server, UTXOSet,
    WalletAddress, WalletService, convert_address, hash_pub_key,
};
use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::str::FromStr;

use tracing::{error, info};
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    fmt,
    prelude::*,
};

#[derive(Debug, Clone)]
enum IsMiner {
    Yes,
    No,
}

impl FromStr for IsMiner {
    type Err = BtcError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "yes" => Ok(IsMiner::Yes),
            "no" => Ok(IsMiner::No),
            _ => Err(BtcError::InvalidValueForMiner(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
enum IsWebServer {
    Yes,
    No,
}

impl FromStr for IsWebServer {
    type Err = BtcError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "yes" => Ok(IsWebServer::Yes),
            "no" => Ok(IsWebServer::No),
            _ => Err(BtcError::InvalidValueForWebServer(s.to_string())),
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "blockchain")]
struct Opt {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(name = "createwallet", about = "Create a new wallet")]
    Createwallet,
    // #[command(
    //     name = "getbalance",
    //     about = "Get the wallet balance of the target address"
    // )]
    #[command(name = "listaddresses", about = "Print local wallet addres")]
    ListAddresses,
    #[command(name = "printchain", about = "Print blockchain all block")]
    Printchain,
    #[command(name = "startnode", about = "Start a node")]
    StartNode {
        #[arg(name = "is_miner", help = "Is Node a Miner?")]
        is_miner: IsMiner,
        #[arg(name = "is_web_server", help = "Is Node a Web Server?")]
        is_web_server: IsWebServer,
        #[arg(name = "connect_nodes", required(true), help = "Connect to a node")]
        connect_nodes: Vec<ConnectNode>,
        #[arg(
            name = "wlt_mining_addr",
            required(true),
            help = "Wallet Address (required)",
            last(true)
        )]
        wlt_mining_addr: String,
    },
}

/// Initialize logging with functional configuration
fn initialize_logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(filter))
        .init();
}

/// Create a new wallet and return the address
fn create_wallet() -> Result<WalletAddress> {
    WalletService::new()
        .and_then(|mut wallets| wallets.create_wallet())
        .inspect(|address| info!("Your new address: {}", address.as_str()))
}

/// List all wallet addresses
fn list_addresses() -> Result<()> {
    WalletService::new().map(|wallets| {
        wallets
            .get_addresses()
            .iter()
            .for_each(|address| info!("{}", address.as_str()));
    })
}

/// Format transaction input information
fn format_transaction_input(input: &blockchain::TXInput) -> String {
    let txid_hex = input.get_input_tx_id_hex();
    let pub_key_hash = hash_pub_key(input.get_pub_key());
    let address = convert_address(pub_key_hash.as_slice())
        .map(|a| a.as_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    format!(
        "-- Input txid = {}, vout = {}, from = {}",
        txid_hex,
        input.get_vout(),
        address.as_str(),
    )
}

/// Format transaction output information
fn format_transaction_output(output: &blockchain::TXOutput) -> String {
    let pub_key_hash = output.get_pub_key_hash();
    let address = convert_address(pub_key_hash)
        .map(|a| a.as_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    format!("-- Output value = {}, to = {}", output.get_value(), address)
}

/// Process a single transaction and log its details
fn process_transaction(tx: &blockchain::Transaction) {
    let cur_txid_hex = tx.get_tx_id_hex();
    info!("- Transaction txid_hex: {}", cur_txid_hex);

    // Process inputs if not coinbase
    if !tx.is_coinbase() {
        tx.get_vin()
            .iter()
            .map(format_transaction_input)
            .for_each(|input_info| info!("{}", input_info));
    }

    // Process outputs
    tx.get_vout()
        .iter()
        .map(format_transaction_output)
        .for_each(|output_info| info!("{}", output_info));
}

/// Process a single block and log its details
async fn process_block(block: &blockchain::Block) {
    info!("Pre block hash: {}", block.get_pre_block_hash());
    info!("Cur block hash: {}", block.get_hash());
    info!("Cur block Timestamp: {}", block.get_timestamp());

    if let Ok(transactions) = block.get_transactions().await {
        transactions.iter().for_each(process_transaction);
    }
}

/// Print the entire blockchain using functional iteration
async fn print_blockchain() -> Result<()> {
    let blockchain = BlockchainService::default().await?;
    let iterator = blockchain.iterator().await.expect("Failed to get iterator");
    for block in iterator {
        process_block(&block).await;
    }
    Ok(())
}

/// Validate miner configuration
fn validate_miner_config(
    wlt_mining_addr: &WalletAddress,
    is_miner: &IsMiner,
    is_web_server: &IsWebServer,
) -> Result<()> {
    match is_miner {
        IsMiner::Yes => {
            GLOBAL_CONFIG.set_mining_addr(wlt_mining_addr);
            Ok(())
        }
        IsMiner::No => {
            GLOBAL_CONFIG.set_web_server_enabled(matches!(is_web_server, IsWebServer::Yes));
            Ok(())
        }
    }
}

/// Create blockchain for seed node
async fn create_seed_blockchain(wlt_mining_addr: &WalletAddress) -> Result<BlockchainService> {
    info!(
        "Seed Node, Creating BlockChain With Address: {}",
        wlt_mining_addr.as_str()
    );
    let blockchain = BlockchainService::initialize(wlt_mining_addr).await?;
    let utxo_set = UTXOSet::new(blockchain.clone());
    utxo_set.reindex().await?;
    Ok(blockchain)
}

/// Handle blockchain opening with fallback logic
async fn open_or_create_blockchain(
    wlt_mining_addr: &WalletAddress,
    connect_nodes: &[ConnectNode],
) -> Result<BlockchainService> {
    match BlockchainService::default().await {
        Ok(blockchain) => {
            // Check if blockchain is empty - if so and we're a seed node, create genesis block
            let height = blockchain.get_best_height().await?;
            if height == 0 && connect_nodes.contains(&ConnectNode::Local) {
                info!("Blockchain database exists but is empty, creating seed blockchain");
                // Close the empty blockchain and create a new seed blockchain
                drop(blockchain);
                return create_seed_blockchain(wlt_mining_addr).await;
            }
            // Reindex UTXOSet when opening existing blockchain
            let utxo_set = UTXOSet::new(blockchain.clone());
            utxo_set.reindex().await?;
            Ok(blockchain)
        }
        Err(BtcError::BlockchainNotFoundError(_)) => {
            if connect_nodes.contains(&ConnectNode::Local) {
                create_seed_blockchain(wlt_mining_addr).await
            } else {
                BlockchainService::empty().await
            }
        }
        Err(e) => {
            info!("Blockchain error: {}", e);
            Err(e)
        }
    }
}

/// Start the node with functional configuration
async fn start_node(
    is_miner: IsMiner,
    is_web_server: IsWebServer,
    connect_nodes: Vec<ConnectNode>,
    wlt_mining_addr: WalletAddress,
) -> Result<()> {
    // Validate miner configuration
    validate_miner_config(&wlt_mining_addr, &is_miner, &is_web_server)?;

    // Open or create blockchain
    let blockchain = open_or_create_blockchain(&wlt_mining_addr, &connect_nodes).await?;
    let node_context = NodeContext::new(blockchain);

    // Get node configuration
    let socket_addr = GLOBAL_CONFIG.get_node_addr();
    info!("Starting node at address: {}", socket_addr);
    info!("Will try connect to nodes: {:?}", connect_nodes);

    // Convert connect nodes to HashSet
    let connect_nodes_set: HashSet<ConnectNode> = connect_nodes.into_iter().collect();

    // Centralized shutdown handling
    let (shutdown_tx, _) = tokio::sync::broadcast::channel::<()>(1);

    // Start network server concurrently using tokio::spawn
    let network_server = Server::new(node_context.clone());
    info!("Starting network server...");
    // Spawn server as separate tasks
    let net_shutdown_rx = shutdown_tx.subscribe();
    let network_handle = tokio::spawn(async move {
        network_server
            .run_with_shutdown(&socket_addr, connect_nodes_set, net_shutdown_rx)
            .await;
    });

    // Wait for Ctrl+C or any server to stop.
    // Shadow the JoinHandles as mutable because tokio::select! polls branches by &mut,
    // requiring mutable bindings to pass &mut handle into the select arms below.
    let mut network_handle = network_handle;

    let result: Result<()> = match (is_web_server, is_miner) {
        (IsWebServer::Yes, IsMiner::No) => {
            // Start both servers concurrently using tokio::spawn
            let web_server = create_web_server(node_context);
            info!("Starting web server...");
            let web_handle = tokio::spawn(async move {
                match web_server.start_with_shutdown().await {
                    Ok(_) => info!("Web server stopped gracefully"),
                    Err(e) => error!("Web server error: {}", e),
                }
            });

            // requiring mutable bindings to pass &mut handle into the select arms below.
            let mut web_handle = web_handle;

            // Wait for Ctrl+C or any server to stop.
            // Shadow the JoinHandles as mutable because tokio::select! polls branches by &mut,
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    info!("Ctrl-C received, initiating shutdown...");
                    let _ = shutdown_tx.send(());
                }
                _ = &mut web_handle => {
                    info!("Web server task finished");
                }
                _ = &mut network_handle => {
                    info!("Network server task finished");
                }
            }

            // Ensure all tasks are concluded
            let _ = web_handle.await;
            let _ = network_handle.await;
            Ok(())
        }
        (is_web, _) => {
            if matches!(is_web, IsWebServer::Yes) {
                error!(
                    "Web server and miner cannot be enabled at the same time, WILL NOT START WEB SERVER"
                );
                return Err(BtcError::InvalidConfiguration(
                    "Web server and miner cannot be enabled at the same time".to_string(),
                ));
            }
            // Start only network server
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    info!("Ctrl-C received, initiating shutdown...");
                    let _ = shutdown_tx.send(());
                }
                _ = &mut network_handle => {
                    info!("Web server task finished");
                }
            }

            // Ensure both tasks are concluded
            let _ = network_handle.await;
            Ok(())
        }
    };

    result
}

/// Process commands using functional patterns
async fn process_command(command: Command) -> Result<()> {
    match command {
        Command::Createwallet => create_wallet().map(|_| ()),
        Command::ListAddresses => list_addresses(),
        Command::Printchain => print_blockchain().await,
        Command::StartNode {
            is_miner,
            is_web_server,
            connect_nodes,
            wlt_mining_addr,
        } => {
            let validated_addr = WalletAddress::validate(wlt_mining_addr)?;
            start_node(is_miner, is_web_server, connect_nodes, validated_addr).await
        }
    }
}

#[tokio::main]
#[deny(unused_must_use)]
async fn main() {
    // Initialize logging
    initialize_logging();

    // Parse command line arguments
    let opt = Opt::parse();

    // Process command with error handling
    if let Err(e) = process_command(opt.command).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
