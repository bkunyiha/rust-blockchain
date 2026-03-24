use crate::error::{BtcError, Result};
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::RwLock;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Node {
    addr: SocketAddr,
}

impl Node {
    fn new(addr: SocketAddr) -> Node {
        Node { addr }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }
}

pub struct Nodes {
    inner: RwLock<HashSet<Node>>,
}

impl Nodes {
    pub fn new() -> Nodes {
        Nodes {
            inner: RwLock::new(HashSet::new()),
        }
    }

    pub fn add_node(&self, addr: SocketAddr) -> Result<()> {
        let mut inner = self
            .inner
            .write()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        inner.insert(Node::new(addr));
        Ok(())
    }

    pub fn add_nodes(&self, nodes: HashSet<SocketAddr>) -> Result<()> {
        let mut inner = self
            .inner
            .write()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        for node in nodes {
            inner.insert(Node::new(node));
        }
        Ok(())
    }

    pub fn evict_node(&self, addr: &SocketAddr) -> Result<bool> {
        let mut inner = self
            .inner
            .write()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.remove(&Node::new(*addr)))
    }

    pub fn first(&self) -> Result<Option<Node>> {
        let inner = self
            .inner
            .read()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.iter().next().cloned())
    }

    pub fn get_nodes(&self) -> Result<Vec<Node>> {
        let inner = self
            .inner
            .read()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.iter().cloned().collect())
    }

    pub fn len(&self) -> Result<usize> {
        let inner = self
            .inner
            .read()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.len())
    }

    pub fn is_empty(&self) -> Result<bool> {
        let inner = self
            .inner
            .read()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.is_empty())
    }

    pub fn node_is_known(&self, addr: &SocketAddr) -> Result<bool> {
        let inner = self
            .inner
            .read()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.iter().any(|x| x.get_addr().eq(addr)))
    }
}

/// The `Default` trait is implemented for the `Nodes` struct.
///
/// # Implementation
///
/// The `Default` trait is implemented for the `Nodes` struct.
///
/// This calls the `new` method to create a new `Nodes` instance.
impl Default for Nodes {
    fn default() -> Self {
        Self::new()
    }
}
