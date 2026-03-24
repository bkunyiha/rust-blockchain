use crate::wallet::WalletAddress;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::RwLock;

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(Config::new);

static DEFAULT_NODE_ADDR: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 2001);

const NODE_ADDRESS_KEY: &str = "NODE_ADDRESS";
const MINING_ADDRESS_KEY: &str = "MINING_ADDRESS";

///
/// The `Config` struct manages configuration settings for the blockchain system.
/// It uses a read-write lock to ensure thread-safe access to the configuration data.
/// Stores NODE_ADDRESS and MINING_ADDRESS
///
pub struct Config {
    node_addresses: RwLock<HashMap<String, SocketAddr>>,
    minner_addresses: RwLock<HashMap<String, WalletAddress>>,
    web_server_enabled: RwLock<bool>,
}

impl Config {
    pub fn new() -> Config {
        let node_addr = Config::get_server_addr_port();
        let mut map = HashMap::new();
        map.insert(String::from(NODE_ADDRESS_KEY), node_addr);

        Config {
            node_addresses: RwLock::new(map),
            minner_addresses: RwLock::new(HashMap::new()),
            web_server_enabled: RwLock::new(false),
        }
    }

    pub fn get_node_addr(&self) -> SocketAddr {
        let node_addresses = self.node_addresses.read().unwrap();
        *node_addresses.get(NODE_ADDRESS_KEY).unwrap()
    }

    ///
    /// The `set_mining_addr` function sets the mining address in the configuration.
    ///
    /// # Arguments
    ///
    /// * `addr` - A reference to the mining address.
    pub fn set_mining_addr(&self, addr: &WalletAddress) {
        let mut miners = self.minner_addresses.write().unwrap();
        let _ = miners.insert(String::from(MINING_ADDRESS_KEY), addr.clone());
    }

    pub fn set_web_server_enabled(&self, enabled: bool) {
        let mut web_server_enabled = self.web_server_enabled.write().unwrap();
        *web_server_enabled = enabled;
    }

    pub fn is_web_server_enabled(&self) -> bool {
        let web_server_enabled = self.web_server_enabled.read().unwrap();
        *web_server_enabled
    }

    pub fn get_mining_addr(&self) -> Option<WalletAddress> {
        let miners = self.minner_addresses.read().unwrap();
        miners.get(MINING_ADDRESS_KEY).cloned()
    }

    pub fn is_miner(&self) -> bool {
        let miners = self.minner_addresses.read().unwrap();
        miners.contains_key(MINING_ADDRESS_KEY)
    }

    pub fn get_server_addr_port() -> SocketAddr {
        env::var("NODE_ADDR")
            .ok()
            .and_then(|node| node.parse().ok())
            .unwrap_or(DEFAULT_NODE_ADDR)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
