use crate::net::net_processing;
use crate::net::net_processing::{send_known_nodes, send_version};
use crate::node::NodeContext;
use crate::{BlockInTransit, MemoryPool, Nodes};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::TcpListener;
use tracing::{error, info, instrument};

pub const NODE_VERSION: usize = 1;

pub static CENTERAL_NODE: Lazy<SocketAddr> = Lazy::new(|| {
    let central_node_str =
        env::var("CENTERAL_NODE").unwrap_or_else(|_| "127.0.0.1:2001".to_string());

    // Handle empty string case (when CENTERAL_NODE is set but empty)
    if central_node_str.is_empty() {
        "127.0.0.1:2001"
            .parse()
            .expect("Failed to parse default CENTERAL_NODE address")
    } else {
        central_node_str
            .parse()
            .expect("CENTERAL_NODE environment variable is not a valid socket address")
    }
});

pub const TRANSACTION_THRESHOLD: usize = 3;

pub static GLOBAL_NODES: Lazy<Nodes> = Lazy::new(|| {
    let nodes = Nodes::new();

    nodes.add_node(*CENTERAL_NODE).expect("Node add error");
    nodes
});

/// The `GLOBAL_MEMORY_POOL` is a lazy static variable that holds a `MemoryPool` instance.
/// It is used to store transactions that are in the memory pool.
///
/// # Returns
///
/// A `MemoryPool` instance.
///
pub static GLOBAL_MEMORY_POOL: Lazy<MemoryPool> = Lazy::new(MemoryPool::new);

/// The `GLOBAL_BLOCKS_IN_TRANSIT` is a lazy static variable that holds a `BlockInTransit` instance.
/// It is used to store blocks that are in transit between nodes.
///
/// # Returns
///
/// A `BlockInTransit` instance.
///
pub static GLOBAL_BLOCKS_IN_TRANSIT: Lazy<BlockInTransit> = Lazy::new(BlockInTransit::new);

pub const TCP_WRITE_TIMEOUT: u64 = 1000;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum ConnectNode {
    Local,
    Remote(SocketAddr),
}

impl ConnectNode {
    pub fn is_remote(&self) -> bool {
        matches!(self, ConnectNode::Remote(_))
    }

    pub fn get_addr(&self) -> SocketAddr {
        match self {
            ConnectNode::Remote(addr) => *addr,
            ConnectNode::Local => *CENTERAL_NODE,
        }
    }
}

impl FromStr for ConnectNode {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        if s == "local" {
            Ok(ConnectNode::Local)
        } else {
            let ip_addr = s.parse()?;
            Ok(ConnectNode::Remote(ip_addr))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Server {
    node_context: NodeContext,
}

impl Server {
    pub fn new(blockchain: NodeContext) -> Server {
        Server {
            node_context: blockchain,
        }
    }

    #[instrument(skip(self, addrs, connect_nodes, shutdown))]
    pub async fn run_with_shutdown(
        &self,
        addrs: &SocketAddr,
        connect_nodes: HashSet<ConnectNode>,
        mut shutdown: tokio::sync::broadcast::Receiver<()>,
    ) {
        let listener = TcpListener::bind(addrs)
            .await
            .expect("TcpListener bind error");
        info!(
            "Server listening on {:?}",
            listener.local_addr().expect("TcpListener local_addr error")
        );

        // If the node is not the central node, send the version message to the central node.
        if !addrs.eq(&CENTERAL_NODE) {
            let best_height = self
                .node_context
                .get_blockchain_height()
                .await
                .expect("Blockchain read error");
            send_version(&CENTERAL_NODE, best_height).await;
        } else {
            info!("Register with node {:?}", connect_nodes);
            // Add the connect node to the global nodes set.

            let remote_nodes: HashSet<SocketAddr> = connect_nodes
                .iter()
                .filter(|node| node.is_remote())
                .map(|node| node.get_addr())
                .collect();

            GLOBAL_NODES
                .add_nodes(remote_nodes.clone())
                .expect("Global nodes add error");

            for remote_node in remote_nodes {
                send_known_nodes(
                    &remote_node,
                    GLOBAL_NODES
                        .get_nodes()
                        .expect("Global nodes get error")
                        .iter()
                        .map(|node| node.get_addr())
                        .collect(),
                )
                .await;
            }
        }

        // Serve incoming connections with graceful shutdown.
        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Network server shutdown signal received");
                    break;
                }
                accept_res = listener.accept() => {
                    match accept_res {
                        Ok((stream, _peer)) => {
                            // Cheap clone: `NodeContext` is an `Arc`-backed handle to shared state
                            // (not a deep copy of the blockchain). We clone it so we can move a
                            // thread-safe reference into the per-connection task without borrowing `self`.
                            let blockchain = self.node_context.clone();
                            tokio::spawn(async move {
                                // Convert tokio stream to std stream for existing processing code
                                match stream.into_std() {
                                    Ok(std_stream) => {
                                        // `net_processing::process_stream` is implemented on top of blocking `std::io`
                                        // (e.g., `BufReader` + `serde_json::Deserializer::from_reader`). If this socket
                                        // stayed non-blocking, reads can return `WouldBlock` / partial data and break
                                        // the synchronous parsing logic—so we force a blocking std socket here.
                                        let _ = std_stream.set_nonblocking(false);
                                        if let Err(e) = net_processing::process_stream(blockchain, std_stream).await {
                                            error!("Serve error: {}", e);
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to convert stream: {}", e);
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            error!("accept error: {}", e);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OpType {
    Tx,
    Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Error,
    Success,
    Info,
    Warning,
    Ack,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AdminNodeQueryType {
    GetBalance { wlt_address: String },
    GetAllTransactions,
    GetBlockHeight,
    MineEmptyBlock,
    ReindexUtxo,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Package {
    Block {
        addr_from: SocketAddr,
        block: Vec<u8>,
    },
    GetBlocks {
        addr_from: SocketAddr,
    },
    GetData {
        addr_from: SocketAddr,
        op_type: OpType,
        id: Vec<u8>,
    },
    Inv {
        addr_from: SocketAddr,
        op_type: OpType,
        items: Vec<Vec<u8>>,
    },
    Tx {
        addr_from: SocketAddr,
        transaction: Vec<u8>,
    },
    SendBitCoin {
        addr_from: SocketAddr,
        wlt_frm_addr: String,
        wlt_to_addr: String,
        amount: i32,
    },
    KnownNodes {
        addr_from: SocketAddr,
        nodes: Vec<SocketAddr>,
    },
    Version {
        addr_from: SocketAddr,
        version: usize,
        best_height: usize,
    },
    Message {
        addr_from: SocketAddr,
        message_type: MessageType,
        message: String,
    },
    AdminNodeQuery {
        addr_from: SocketAddr,
        query_type: AdminNodeQueryType,
    },
}
