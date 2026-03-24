use crate::error::{BtcError, Result};
use crate::primitives::transaction::Transaction;
use std::collections::HashMap;
use std::sync::RwLock;

/// The `MemoryPool` struct is used to store transactions that are in the memory pool.
///
/// # Fields
///
/// `inner_tx` - A `RwLock` that holds a `HashMap` of `Transaction`s.
///
pub struct MemoryPool {
    inner_tx: RwLock<HashMap<String, Transaction>>,
}

impl MemoryPool {
    pub fn new() -> MemoryPool {
        MemoryPool {
            inner_tx: RwLock::new(HashMap::new()),
        }
    }

    pub fn contains(&self, txid_hex: &str) -> Result<bool> {
        let inner = self
            .inner_tx
            .read()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.contains_key(txid_hex))
    }

    pub fn contains_transaction(&self, tx: &Transaction) -> Result<bool> {
        let txid_hex = tx.get_tx_id_hex();
        self.contains(&txid_hex)
    }

    pub fn add(&self, tx: Transaction) -> Result<()> {
        let txid_hex = tx.get_tx_id_hex();
        self.inner_tx
            .write()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?
            .insert(txid_hex, tx);
        Ok(())
    }

    pub fn get(&self, txid_hex: &str) -> Result<Option<Transaction>> {
        let inner = self
            .inner_tx
            .read()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.get(txid_hex).cloned())
    }

    pub fn remove(&self, tx: Transaction) -> Result<Option<Transaction>> {
        let txid_hex = tx.get_tx_id_hex();
        let mut inner = self
            .inner_tx
            .write()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?;
        let rem_trx_op = inner.remove(txid_hex.as_str());
        Ok(rem_trx_op)
    }

    pub fn get_all(&self) -> Result<Vec<Transaction>> {
        let inner = self
            .inner_tx
            .read()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?;
        let mut txs = vec![];
        for (_, v) in inner.iter() {
            txs.push(v.clone());
        }
        Ok(txs)
    }

    pub fn len(&self) -> Result<usize> {
        let inner = self
            .inner_tx
            .read()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.len())
    }

    pub fn is_empty(&self) -> Result<bool> {
        let inner = self
            .inner_tx
            .read()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.is_empty())
    }
}

/// The `Default` trait is implemented for the `MemoryPool` struct.
///
/// # Implementation
///
/// The `Default` trait is implemented for the `MemoryPool` struct.
///
/// This calls the `new` method to create a new `MemoryPool` instance.
impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}

/// The `BlockInTransit` struct is used to store blocks that are in transit between nodes.
///
/// # Fields
///
/// `inner` - A `RwLock` that holds a `Vec` of `Vec<u8>`s.
///
pub struct BlockInTransit {
    inner: RwLock<Vec<Vec<u8>>>,
}

impl BlockInTransit {
    pub fn new() -> BlockInTransit {
        BlockInTransit {
            inner: RwLock::new(vec![]),
        }
    }

    pub fn add_blocks(&self, blocks: &[Vec<u8>]) -> Result<()> {
        let mut inner = self
            .inner
            .write()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?;
        for hash in blocks {
            inner.push(hash.to_vec());
        }
        Ok(())
    }

    pub fn first(&self) -> Result<Option<Vec<u8>>> {
        let inner = self
            .inner
            .read()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.first().map(|block_hash| block_hash.to_vec()))
    }

    pub fn remove(&self, block_hash: &[u8]) -> Result<Option<Vec<u8>>> {
        let mut inner = self
            .inner
            .write()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?;
        if let Some(idx) = inner.iter().position(|x| x.eq(block_hash)) {
            inner.remove(idx);
            Ok(Some(block_hash.to_vec()))
        } else {
            Ok(None)
        }
    }

    pub fn clear(&self) -> Result<()> {
        let mut inner = self
            .inner
            .write()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?;
        inner.clear();
        Ok(())
    }

    pub fn len(&self) -> Result<usize> {
        let inner = self
            .inner
            .read()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.len())
    }

    pub fn is_empty(&self) -> Result<bool> {
        let inner = self
            .inner
            .read()
            .map_err(|e| BtcError::MemoryPoolInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.is_empty())
    }

    pub fn is_not_empty(&self) -> Result<bool> {
        let is_empty = self.is_empty()?;
        Ok(!is_empty)
    }
}

/// The `Default` trait is implemented for the `MemoryPool` struct.
///
/// # Implementation
///
/// The `Default` trait is implemented for the `MemoryPool` struct.
///
/// This calls the `new` method to create a new `MemoryPool` instance.
impl Default for BlockInTransit {
    fn default() -> Self {
        Self::new()
    }
}
