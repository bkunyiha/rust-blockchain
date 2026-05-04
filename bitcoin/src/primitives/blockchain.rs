// SPDX-License-Identifier: MIT OR Apache-2.0
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;

#[derive(Clone, Debug)]
pub struct Blockchain<T> {
    pub tip_hash: Arc<TokioRwLock<String>>,
    pub db: T,
    pub is_empty: bool,
}
