//! Network P2P operations (Bitcoin Core: net_processing)
//!
//! This module handles peer-to-peer communication operations,
//! similar to Bitcoin Core's net_processing.cpp

use crate::node::{GLOBAL_NODES, MessageType, NODE_VERSION, OpType, Package, TCP_WRITE_TIMEOUT};
use crate::{Block, GLOBAL_CONFIG, Transaction, UTXOSet, WalletAddress};

use crate::node::NodeContext;
use std::collections::HashSet;
use std::io::Write;
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::Duration;

use crate::error::BtcError;
use crate::node::{AdminNodeQueryType, GLOBAL_BLOCKS_IN_TRANSIT, GLOBAL_MEMORY_POOL};

use data_encoding::HEXLOWER;
use serde_json::Deserializer;
use std::error::Error;
use std::io::BufReader;
use tracing::{debug, error, info, instrument, trace, warn};

#[instrument(skip(node_context, stream))]
pub async fn process_stream(
    node_context: NodeContext,
    stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    // peer_addr is the address of the peer that is sending the request.
    let peer_addr = stream.peer_addr()?;
    let reader = BufReader::new(&stream);
    let pkg_reader = Deserializer::from_reader(reader).into_iter::<Package>();

    // The `serve` function processes incoming network requests from a TCP stream.
    // It handles different types of packages, including blocks, transactions, and version information.
    // The function processes each package based on its type and performs the appropriate actions.
    // It also manages the block in transit set and the memory pool to ensure proper synchronization
    // and validation of the blockchain.
    // The function returns an error if the stream cannot be read or if the package cannot be deserialized.
    // It also shuts down the stream after processing the package.
    // Iterate over the deserialized packages from the stream.
    for pkg in pkg_reader {
        let pkg = pkg?;
        info!("Receive request from {}: {:?}", peer_addr, pkg);

        match pkg {
            // When a node receives a block, it adds it to the blockchain and sends a request for the next block.
            // It deserializes the block and adds it to the blockchain.
            // If there are blocks in transit, it sends a get_data request for the next block.
            // If there are no more blocks in transit, it reindexes the UTXO set of the blockchain.
            Package::Block { addr_from, block } => {
                // Fix 3: Cancel any in-progress mining before processing the received block
                crate::node::miner::cancel_current_mining();

                let block =
                    Block::deserialize(block.as_slice()).expect("Block deserialization error");

                // Check if this block is NEW to us (not already in our database)
                // This prevents infinite relay loops: A→B→C→B→A→...
                let block_is_new = node_context
                    .get_block(block.get_hash_bytes().as_slice())
                    .await
                    .unwrap_or(None)
                    .is_none();

                // If the block is not the best block, do nothing
                // `add_block` will not add the block if its height is less than current tip height in the block chain.
                node_context
                    .add_block(&block)
                    .await
                    .expect("Blockchain write error");
                let added_block_hash = block.get_hash_bytes();
                info!("Added block {:?}", added_block_hash.as_slice());

                // Remove transactions in block from memory pool functionally, since they have already been mined by other nodes
                for tx in block.get_transactions().await? {
                    node_context.remove_from_memory_pool(tx.clone()).await;
                }

                // BLOCK RELAY: Forward NEW blocks to all peers except the sender.
                // Only relay if the block was new to us — prevents infinite relay loops.
                // Without relay, blocks only travel one hop from the miner.
                // In a linear topology (1→2→3→4→5→6→7), a block mined by Node 4
                // would only reach Nodes 3 and 5 without relay.
                if block_is_new {
                    let my_node_addr = GLOBAL_CONFIG.get_node_addr();
                    let nodes = GLOBAL_NODES.get_nodes().expect("Global nodes get error");
                    let block_hash_for_relay = block.get_hash_bytes();
                    for node in nodes.iter() {
                        let node_addr = node.get_addr();
                        // Don't relay back to sender or to ourselves
                        if node_addr != addr_from && node_addr != my_node_addr {
                            let hash_clone = block_hash_for_relay.clone();
                            tokio::spawn(async move {
                                send_inv(&node_addr, OpType::Block, &[hash_clone]).await;
                            });
                        }
                    }

                    // Note: blocks on different branches at lower heights are stored
                    // in the DB (by add_block's Sled transaction) and available for
                    // future reorganizations when higher blocks on that branch arrive.
                }

                // The add_block() method already handles UTXO updates internally through the reorganization process.
                // Calling update_utxo_set() here would cause double UTXO updates, leading to multiple SUBSIDY rewards.
                // This was the root cause of the consensus mechanism allowing all nodes to keep their SUBSIDY.

                let removed_block_hash = GLOBAL_BLOCKS_IN_TRANSIT
                    .remove(added_block_hash.as_ref())
                    .expect("Block removal error");
                if let Some(removed_block_hash) = removed_block_hash {
                    info!(
                        "Removed block {:?} FROM GLOBAL_BLOCKS_IN_TRANSIT",
                        removed_block_hash.as_slice()
                    );
                }

                // If there are blocks in transit, it sends a get_data request for the next block.
                // It removes the block from the blocks in transit set when it is added to the blockchain when
                // it is receives Package::Inv message{OpType::Block, items: [block_hash]}
                // If there are no more blocks in transit, it reindexes the UTXO set of the blockchain.
                if GLOBAL_BLOCKS_IN_TRANSIT
                    .is_not_empty()
                    .expect("Blocks in transit error")
                {
                    let block_hash = GLOBAL_BLOCKS_IN_TRANSIT
                        .first()
                        .expect("Blocks in transit error")
                        .expect("Blocks in transit error");
                    send_get_data(&addr_from, OpType::Block, &block_hash).await;

                    //GLOBAL_BLOCKS_IN_TRANSIT.remove(block_hash.as_slice());
                }
            }
            // Retrieves all block hashes from the blockchain and sends an
            // inv message with a list of hashes to the requesting peer.
            Package::GetBlocks { addr_from } => {
                let blocks = node_context
                    .get_block_hashes()
                    .await
                    .expect("Blockchain read error");
                // Send an inv message with a list of hashes to the requesting peer.
                send_inv(&addr_from, OpType::Block, &blocks).await;
            }
            // Retrieves the requested block or transaction from the blockchain
            // or the global memory pool and sends it back to the requesting peer.
            Package::GetData {
                addr_from,
                op_type,
                id,
            } => match op_type {
                // When a node receives a block, it adds it to the blockchain and sends a request for the next block.
                OpType::Block => {
                    if let Some(block) = node_context
                        .get_block(id.as_slice())
                        .await
                        .expect("Blockchain read error")
                    {
                        send_block(&addr_from, &block).await;
                    }
                }
                OpType::Tx => {
                    let txid_hex = HEXLOWER.encode(id.as_slice());
                    if let Some(tx) = GLOBAL_MEMORY_POOL
                        .get(txid_hex.as_str())
                        .expect("Memory pool get error")
                    {
                        send_tx(&addr_from, &tx).await;
                    } else {
                        info!("Received request to forward a Transaction that is not found in memory pool. 
                        Most likely it has been mined!!!: {:?}", txid_hex);
                    }
                }
            },
            // Adds the received blocks or transactions to the global blocks in transit
            // or the memory pool and requests missing blocks or transactions via get_data if necessary.
            Package::Inv {
                addr_from,
                op_type,
                items,
            } => match op_type {
                // When a node receives a block, it adds it to the blocks in transit set and sends a request for the first block.
                OpType::Block => {
                    GLOBAL_BLOCKS_IN_TRANSIT
                        .add_blocks(items.as_slice())
                        .expect("Blocks in transit add error");

                    let block_hash = items.first().expect("Blocks in transit add error");
                    send_get_data(&addr_from, OpType::Block, block_hash).await;

                    //GLOBAL_BLOCKS_IN_TRANSIT.remove(block_hash.as_slice());
                }
                // When a node receives a transaction, it adds it to the memory pool and sends a request for the transaction.
                OpType::Tx => {
                    let txid = items.first().expect("Blocks in transit add error");
                    let txid_hex = HEXLOWER.encode(txid);

                    if !GLOBAL_MEMORY_POOL
                        .contains(txid_hex.as_str())
                        .expect("Memory pool contains error")
                    {
                        send_get_data(&addr_from, OpType::Tx, txid).await;
                    }
                }
            },
            // deserializes the transaction and adds it to the global memory pool.
            // If the node is a miner and the memory pool has reached a certain threshold,
            // it creates a new block containing transactions from the memory pool, mines it,
            // and broadcasts the new block to other nodes via inv.
            Package::Tx {
                addr_from,
                transaction,
            } => {
                let tx = Transaction::deserialize(transaction.as_slice())
                    .expect("Transaction deserialization error");
                // CPU intensive operation.
                // It will create a new transaction and add it to the memory pool.
                // It will also broadcast the transaction to all other nodes.
                // It will also mine a new block if the memory pool has reached a certain threshold.
                match node_context.process_transaction(&addr_from, tx).await {
                    Ok(_) => (),
                    Err(BtcError::TransactionAlreadyExistsInMemoryPool(txid)) => {
                        send_message(
                            &addr_from,
                            MessageType::Error,
                            format!("Transaction: {} already exists", txid),
                        )
                        .await;
                    }
                    Err(e) => Err(e)?,
                }
            }

            // CPU intensive operation.
            // It will create a new transaction and add it to the memory pool.
            // It will also broadcast the transaction to all other nodes.
            // It will also mine a new block if the memory pool has reached a certain threshold.
            Package::SendBitCoin {
                addr_from,
                wlt_frm_addr,
                wlt_to_addr,
                amount,
            } => {
                let validated_wlt_frm_addr = WalletAddress::validate(wlt_frm_addr);
                let validated_wlt_to_addr = WalletAddress::validate(wlt_to_addr);

                match (validated_wlt_frm_addr, validated_wlt_to_addr) {
                    (Ok(_), Err(_)) => {
                        send_message(
                            &addr_from,
                            MessageType::Error,
                            "Invalid addr_to: ${wlt_to_addr}".to_string(),
                        )
                        .await;
                    }
                    (Err(_), Ok(_)) => {
                        send_message(
                            &addr_from,
                            MessageType::Error,
                            "Invalid addr_from: ${wlt_frm_addr}".to_string(),
                        )
                        .await;
                    }
                    (Err(_), Err(_)) => {
                        let send_message_invalid_to = send_message(
                            &addr_from,
                            MessageType::Error,
                            "Invalid addr_to: ${wlt_to_addr}".to_string(),
                        );
                        let send_message_invalid_from = send_message(
                            &addr_from,
                            MessageType::Error,
                            "Invalid addr_from: ${wlt_frm_addr}".to_string(),
                        );
                        // Run both in parallel
                        tokio::join!(send_message_invalid_to, send_message_invalid_from);
                    }
                    (Ok(from), Ok(to)) => {
                        let utxo_set = UTXOSet::new(node_context.get_blockchain().clone());

                        match node_context.btc_transaction(&from, &to, amount).await {
                            Ok(_) => (),
                            Err(BtcError::TransactionAlreadyExistsInMemoryPool(txid)) => {
                                send_message(
                                    &addr_from,
                                    MessageType::Error,
                                    format!("Transaction: {} already exists", txid),
                                )
                                .await;
                            }
                            Err(BtcError::NotEnoughFunds) => {
                                // Get current balance for detailed error message
                                let current_balance =
                                    utxo_set.get_balance(&from).await.unwrap_or(0);

                                send_message(
                                    &addr_from,
                                    MessageType::Error,
                                    format!(
                                        "Insufficient funds: cannot send {} bitcoin. Current balance: {} bitcoin",
                                        amount, current_balance
                                    ),
                                )
                                .await;

                                // Log the error for debugging
                                error!(
                                    "Transaction rejected: insufficient funds. From: {}, To: {}, Amount: {}, Balance: {}",
                                    from.as_str(),
                                    to.as_str(),
                                    amount,
                                    current_balance
                                );
                            }
                            Err(e) => {
                                send_message(
                                    &addr_from,
                                    MessageType::Error,
                                    format!("Transaction creation failed: {}", e),
                                )
                                .await;

                                error!("Transaction creation failed: {}", e);
                            }
                        }
                    }
                }
            }
            Package::Version {
                addr_from,
                version,
                best_height,
            } => {
                debug!("version = {}, best_height = {}", version, best_height);
                let local_best_height = node_context
                    .get_blockchain_height()
                    .await
                    .expect("Blockchain read error");
                if local_best_height < best_height {
                    send_get_blocks(&addr_from).await;
                }
                if local_best_height > best_height {
                    send_version(
                        &addr_from,
                        node_context
                            .get_blockchain_height()
                            .await
                            .expect("Blockchain read error"),
                    )
                    .await;
                }

                // If height is the same then get the first and last block hashes for comparison

                if !GLOBAL_NODES
                    .node_is_known(&addr_from)
                    .expect("Node is known error")
                {
                    GLOBAL_NODES.add_node(addr_from).expect("Node add error");
                }
            }
            Package::KnownNodes { addr_from, nodes } => {
                process_known_nodes(node_context.clone(), &addr_from, nodes).await;
            }
            Package::Message {
                addr_from,
                message_type,
                message,
            } => match message_type {
                MessageType::Error => {
                    error!("{} sent error: {}", addr_from, message);
                }
                MessageType::Warning => {
                    warn!("{} sent warning: {}", addr_from, message);
                }
                MessageType::Info => {
                    debug!("{} sent info: {}", addr_from, message);
                }
                MessageType::Success => {
                    debug!("{} sent success: {}", addr_from, message);
                }
                MessageType::Ack => {
                    debug!("{} sent ack: {}", addr_from, message);
                }
            },
            Package::AdminNodeQuery {
                addr_from,
                query_type,
            } => match query_type {
                AdminNodeQueryType::GetBalance { wlt_address } => {
                    let address_valid = WalletAddress::validate(wlt_address)?;

                    let utxo_set = UTXOSet::new(node_context.get_blockchain().clone());
                    let balance = utxo_set
                        .get_balance(&address_valid)
                        .await
                        .expect("UTXO set get balance error");
                    info!("Balance of {}: {}", addr_from, balance);
                }
                AdminNodeQueryType::GetAllTransactions => {
                    let transactions_summary = node_context
                        .find_all_transactions()
                        .await
                        .expect("Blockchain find all transactions error");

                    info!("═══════════════════════════════════════════════════════════════");
                    info!("                    BLOCKCHAIN TRANSACTIONS");
                    info!("═══════════════════════════════════════════════════════════════");

                    for (idx, (cur_txid_hex, tx_summary)) in transactions_summary.iter().enumerate()
                    {
                        let mut tx_summary_input = tx_summary.clone();
                        let mut tx_summary_output = tx_summary.clone();
                        let tx_summary_inputs = tx_summary_input.get_inputs();
                        let tx_summary_outputs = tx_summary_output.get_outputs();
                        info!("");
                        info!("┌─ Transaction #{}", idx + 1);
                        info!("│  ID: {}", cur_txid_hex);
                        info!(
                            "│  Type: {}",
                            if tx_summary_inputs.is_empty() {
                                "Coinbase"
                            } else {
                                "Regular"
                            }
                        );

                        if !tx_summary_inputs.is_empty() {
                            info!("│  ┌─ Inputs ({}):", tx_summary_inputs.len());
                            for (input_idx, input_summary) in tx_summary_inputs.iter().enumerate() {
                                info!(
                                    "│  │  {} └─ From: {} (txid: {}, vout: {})",
                                    if input_idx == tx_summary_inputs.len() - 1 {
                                        "└"
                                    } else {
                                        "├"
                                    },
                                    input_summary.get_wlt_addr().as_str(),
                                    input_summary.get_txid_hex(),
                                    input_summary.get_output_idx()
                                );
                            }
                        }

                        info!("│  ┌─ Outputs ({}):", tx_summary_outputs.len());
                        for (output_idx, output_summary) in tx_summary_outputs.iter().enumerate() {
                            info!(
                                "│  │  {} └─ To: {} (value: {} BTC)",
                                if output_idx == tx_summary_outputs.len() - 1 {
                                    "└"
                                } else {
                                    "├"
                                },
                                output_summary.get_wlt_addr().as_str(),
                                output_summary.get_value()
                            );
                        }
                        info!("└─────────────────────────────────────────────────────────────");
                    }

                    info!("");
                    info!("═══════════════════════════════════════════════════════════════");
                    info!("Total Transactions: {}", transactions_summary.len());
                    info!("═══════════════════════════════════════════════════════════════");
                }
                AdminNodeQueryType::GetBlockHeight => {
                    let height = node_context
                        .get_blockchain_height()
                        .await
                        .expect("Blockchain read error");
                    info!("Block height: {}", height);
                }
                AdminNodeQueryType::MineEmptyBlock => {
                    if GLOBAL_CONFIG.is_miner() {
                        // Get mining address from config
                        let mining_address =
                            GLOBAL_CONFIG.get_mining_addr().ok_or(BtcError::NotAMiner)?;
                        node_context
                            .mine_empty_block(&mining_address)
                            .await
                            .map(|_| ())?
                    } else {
                        trace!("Not a miner");
                    }
                    trace!("Mining empty block");
                }
                AdminNodeQueryType::ReindexUtxo => {
                    let utxo_set = UTXOSet::new(node_context.get_blockchain().clone());
                    utxo_set.reindex().await.expect("UTXO set reindex error");
                    let count = utxo_set
                        .count_transactions()
                        .await
                        .expect("UTXO set count error");
                    trace!(
                        "Reindexed UTXO set. There are {} transactions in the UTXO set.",
                        count
                    );
                }
            },
        }
    }
    let _ = stream.shutdown(Shutdown::Both);
    Ok(())
}

/// The `send_get_data` function sends a get_data request to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `op_type` - A reference to the operation type.
/// * `id` - A reference to the id.
pub async fn send_get_data(addr_to: &SocketAddr, op_type: OpType, id: &[u8]) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::GetData {
            addr_from: node_addr,
            op_type,
            id: id.to_vec(),
        },
    )
    .await;
}

/// The `send_inv` function abstracts the process of sending inventory information (Inv) to a specified address
/// using a standardized package format, including source address, operation type, and a collection of
/// byte vector items, which in this case represent blocks. This function will help broadcast inventory
/// notifications for specific data items to the indicated network address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `op_type` - A reference to the operation type.
/// * `blocks` - A reference to the blocks.
pub async fn send_inv(addr_to: &SocketAddr, op_type: OpType, blocks: &[Vec<u8>]) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Inv {
            addr_from: node_addr,
            op_type,
            items: blocks.to_vec(),
        },
    )
    .await;
}

/// The `send_block` function sends a block to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `block` - A reference to the block.
pub async fn send_block(addr_to: &SocketAddr, block: &Block) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Block {
            addr_from: node_addr,
            block: block.serialize().expect("Block serialization error"),
        },
    )
    .await;
}

/// The `send_tx` function sends a transaction to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `tx` - A reference to the transaction.
pub async fn send_tx(addr_to: &SocketAddr, tx: &Transaction) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Tx {
            addr_from: node_addr,
            transaction: tx.serialize().expect("Transaction serialization error"),
        },
    )
    .await;
}

/// The `send_known_nodes` function sends known nodes to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `nodes` - A vector of socket addresses.
pub async fn send_known_nodes(addr_to: &SocketAddr, nodes: Vec<SocketAddr>) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::KnownNodes {
            addr_from: node_addr,
            nodes,
        },
    )
    .await;
}

/// The `send_version` function sends a version request to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `height` - A reference to the height.
pub async fn send_version(addr_to: &SocketAddr, height: usize) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Version {
            addr_from: node_addr,
            version: NODE_VERSION,
            best_height: height,
        },
    )
    .await;
}

/// The `send_get_blocks` function sends a get_blocks request to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
pub async fn send_get_blocks(addr_to: &SocketAddr) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::GetBlocks {
            addr_from: node_addr,
        },
    )
    .await;
}

/// The `send_message` function sends a message to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
pub async fn send_message(addr_to: &SocketAddr, message_type: MessageType, message: String) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Message {
            addr_from: node_addr,
            message_type,
            message,
        },
    )
    .await;
}

///
/// The `send_data` function abstracts the process of sending data to a specified address
/// using a standardized package format. It includes source address, operation type, and a collection of
/// byte vector items, which in this case represent blocks. This function will help broadcast inventory
/// notifications for specific data items to the indicated network address.
///
async fn send_data(addr_to: &SocketAddr, pkg: Package) {
    info!("send package: {:?}", &pkg);
    let stream = TcpStream::connect(addr_to);
    if stream.is_err() {
        error!("The {} is not valid", addr_to);

        GLOBAL_NODES
            .evict_node(addr_to)
            .expect("Node eviction error");
        return;
    }

    let mut stream = stream.expect("Stream connect error");
    let _ = stream.set_write_timeout(Option::from(Duration::from_millis(TCP_WRITE_TIMEOUT)));
    let _ = serde_json::to_writer(&stream, &pkg);
    let _ = stream.flush();
}

/// The `process_known_nodes` function processes known nodes.
/// 1) It will add new nodes to the global nodes set and send version to all new nodes plus sender.
/// 2) If I know nodes not known by sender, then i will
///     - Send all known nodes to the sender
///     - Send all known nodes to all new nodes that i received from the sender
/// 3) It will also send version to all new noded that i received from the sender.
///
/// # Arguments
///
/// * `blockchain` - A reference to the blockchain.
/// * `addr_from` - A reference to the address of the sender.
/// * `nodes` - A reference to the nodes.
pub async fn process_known_nodes(
    node_context: NodeContext,
    addr_from: &SocketAddr,
    nodes: Vec<SocketAddr>,
) {
    // Find new nodes functionally
    let new_nodes: HashSet<SocketAddr> = nodes
        .iter()
        .filter(|current_new_node_candidate| {
            !GLOBAL_NODES
                .node_is_known(current_new_node_candidate)
                .expect("Node is known error")
        })
        .cloned()
        .collect();

    info!("new_nodes: {:?}", new_nodes);

    // Add host and new nodes to the global nodes set
    GLOBAL_NODES
        .add_nodes(new_nodes.clone())
        .expect("Global nodes add error");

    let all_known_nodes_addresses: Vec<SocketAddr> = GLOBAL_NODES
        .get_nodes()
        .expect("Global nodes get error")
        .into_iter()
        .map(|node| node.get_addr())
        .collect();

    let mut nodes_to_add: HashSet<SocketAddr> = HashSet::new();
    // Add new nodes to the nodes to add
    nodes_to_add.extend(new_nodes.clone());
    // Add sender to the nodes to add
    nodes_to_add.insert(*addr_from);

    // Empty nodes sent or have sender doesn't know all nodes that i know
    if all_known_nodes_addresses.len() > nodes.len() {
        // Send All known nodes to sender and new nodes
        nodes_to_add.iter().for_each(|node| {
            let node_addr = *node;
            let all_nodes = all_known_nodes_addresses.clone();
            tokio::spawn(async move {
                send_known_nodes(&node_addr, all_nodes).await;
            });
        });
    }

    // Send Version to all new nodes plus sender
    let best_height = node_context
        .get_blockchain_height()
        .await
        .expect("Blockchain get best height error");

    send_version(addr_from, best_height).await;

    nodes_to_add
        .into_iter()
        .filter(|node| node.ne(addr_from))
        .for_each(|node| {
            let node_addr = node;
            let height = best_height;
            tokio::spawn(async move {
                send_version(&node_addr, height).await;
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::BlockchainService;
    use crate::primitives::transaction::Transaction;
    use std::fs;
    use std::net::SocketAddr;
    use std::str::FromStr;

    fn generate_test_genesis_address() -> crate::WalletAddress {
        // Create a wallet to get a valid Bitcoin address
        let wallet = crate::Wallet::new().expect("Failed to create test wallet");
        wallet.get_address().expect("Failed to get wallet address")
    }

    struct TestBlockchain {
        blockchain: BlockchainService,
        db_path: String,
    }

    impl TestBlockchain {
        async fn new() -> Self {
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();

            // Use process ID and random number for better isolation
            let process_id = std::process::id();
            let random_num = rand::random::<u32>();
            let test_db_path = format!(
                "test_blockchain_db_{}_{}_{}_{}",
                timestamp,
                process_id,
                random_num,
                uuid::Uuid::new_v4()
            );

            // Clean up any existing test database with retry logic
            let _ = Self::cleanup_with_retry(&test_db_path);

            // Create a unique subdirectory for this test
            let unique_db_path = format!("{}/db", test_db_path);
            let _ = fs::create_dir_all(&unique_db_path);

            // Set environment variable for unique database path
            unsafe {
                std::env::set_var("TREE_DIR", &unique_db_path);
            }
            unsafe {
                std::env::set_var("BLOCKS_TREE", &unique_db_path);
            }

            let genesis_address = generate_test_genesis_address();
            let blockchain = BlockchainService::initialize(&genesis_address)
                .await
                .expect("Failed to create test blockchain");

            TestBlockchain {
                blockchain,
                db_path: test_db_path,
            }
        }

        /// Clean up test database with retry logic to handle lock issues
        fn cleanup_with_retry(db_path: &str) -> std::io::Result<()> {
            for attempt in 1..=3 {
                match fs::remove_dir_all(db_path) {
                    Ok(_) => return Ok(()),
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        if attempt < 3 {
                            std::thread::sleep(std::time::Duration::from_millis(100 * attempt));
                            continue;
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        return Ok(()); // Directory doesn't exist, that's fine
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        }

        fn blockchain(&self) -> &BlockchainService {
            &self.blockchain
        }
    }

    impl Drop for TestBlockchain {
        fn drop(&mut self) {
            // Ensure cleanup happens even if test panics
            let _ = Self::cleanup_with_retry(&self.db_path);
        }
    }

    #[tokio::test]
    async fn test_send_tx() {
        let genesis_address = generate_test_genesis_address();
        let tx = Transaction::new_coinbase_tx(&genesis_address.clone())
            .expect("Failed to create transaction");
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");

        // This should not panic even if the connection fails
        send_tx(&addr, &tx).await;
    }

    #[tokio::test]
    async fn test_send_block() {
        let test_blockchain = TestBlockchain::new().await;
        let block = test_blockchain
            .blockchain()
            .mine_block(&[])
            .await
            .expect("Failed to mine block");
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");

        // This should not panic even if the connection fails
        send_block(&addr, &block).await;
    }

    #[tokio::test]
    async fn test_send_get_data() {
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");
        let id = vec![1, 2, 3, 4];

        // This should not panic even if the connection fails
        send_get_data(&addr, OpType::Block, &id).await;
        send_get_data(&addr, OpType::Tx, &id).await;
    }

    #[tokio::test]
    async fn test_send_inv() {
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");
        let items = vec![vec![1, 2, 3], vec![4, 5, 6]];

        // This should not panic even if the connection fails
        send_inv(&addr, OpType::Block, &items).await;
        send_inv(&addr, OpType::Tx, &items).await;
    }

    #[tokio::test]
    async fn test_send_message() {
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");
        let message = "Test message".to_string();

        // This should not panic even if the connection fails
        send_message(&addr, MessageType::Info, message.clone()).await;
        send_message(&addr, MessageType::Error, message.clone()).await;
        send_message(&addr, MessageType::Success, message.clone()).await;
        send_message(&addr, MessageType::Warning, message.clone()).await;
        send_message(&addr, MessageType::Ack, message).await;
    }

    #[tokio::test]
    async fn test_send_version() {
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");
        let height = 42;

        // This should not panic even if the connection fails
        send_version(&addr, height).await;
    }

    #[tokio::test]
    async fn test_send_get_blocks() {
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");

        // This should not panic even if the connection fails
        send_get_blocks(&addr).await;
    }

    #[tokio::test]
    async fn test_send_known_nodes() {
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");
        let nodes = vec![
            SocketAddr::from_str("127.0.0.1:8081").expect("Failed to parse address"),
            SocketAddr::from_str("127.0.0.1:8082").expect("Failed to parse address"),
        ];

        // This should not panic even if the connection fails
        send_known_nodes(&addr, nodes).await;
    }

    #[tokio::test]
    async fn test_process_known_nodes() {
        let test_blockchain = TestBlockchain::new().await;
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");
        let nodes = vec![
            SocketAddr::from_str("127.0.0.1:8081").expect("Failed to parse address"),
            SocketAddr::from_str("127.0.0.1:8082").expect("Failed to parse address"),
        ];

        // This should not panic
        let node_context = crate::node::NodeContext::new(test_blockchain.blockchain().clone());
        process_known_nodes(node_context, &addr, nodes).await;
    }

    #[test]
    fn test_op_type_serialization() {
        let op_type_block = OpType::Block;
        let op_type_tx = OpType::Tx;

        // Test that they can be serialized (this would fail at compile time if not)
        let _block_json = serde_json::to_string(&op_type_block).expect("Failed to serialize Block");
        let _tx_json = serde_json::to_string(&op_type_tx).expect("Failed to serialize Tx");
    }

    #[test]
    fn test_message_type_serialization() {
        let message_types = vec![
            MessageType::Error,
            MessageType::Success,
            MessageType::Info,
            MessageType::Warning,
            MessageType::Ack,
        ];

        for msg_type in message_types {
            let _json = serde_json::to_string(&msg_type).expect("Failed to serialize MessageType");
        }
    }
}
