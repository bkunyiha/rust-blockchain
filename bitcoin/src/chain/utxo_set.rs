use crate::chain::chainstate::BlockchainService;
use crate::error::{BtcError, Result};
use crate::primitives::block::Block;
use crate::primitives::transaction::{TXOutput, Transaction};
use crate::wallet::WalletAddress;
use crate::wallet::get_pub_key_hash;
use data_encoding::HEXLOWER;
use std::collections::HashMap;
use tracing::{debug, info, trace};

const UTXO_TREE: &str = "chainstate";

pub struct UTXOSet {
    blockchain: BlockchainService,
}

impl UTXOSet {
    pub fn new(blockchain: BlockchainService) -> UTXOSet {
        UTXOSet { blockchain }
    }

    pub fn get_blockchain(&self) -> &BlockchainService {
        &self.blockchain
    }

    ///
    /// The `find_spendable_outputs` function finds the spendable outputs for a given public key hash and amount.
    /// It iterates through UTXOs, checks ownership, accumulates values,
    /// and forms a HashMap of transaction IDs to output indices for spendable outputs.
    ///
    /// # Arguments
    ///
    /// * `pub_key_hash` - A reference to the public key hash.
    /// * `amount` - The required amount.
    ///
    /// # Returns
    ///
    /// A tuple containing the accumulated amount and a HashMap of transaction IDs to output indices for spendable outputs.
    pub async fn find_spendable_outputs(
        &self,
        from_pub_key_hash: &[u8],
        amount: i32,
    ) -> Result<(i32, HashMap<String, Vec<usize>>)> {
        debug!("Finding spendable outputs for amount: {}", amount);
        let mut unspent_outputs_indexes: HashMap<String, Vec<usize>> = HashMap::new();
        let mut accmulated = 0;
        let db = self.blockchain.get_db().await?;
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
        let mut total_checked = 0;
        for item in utxo_tree.iter() {
            let (k, v) = item.map_err(|e| BtcError::GettingUTXOError(e.to_string()))?;
            let txid_hex = HEXLOWER.encode(k.to_vec().as_slice());
            let (tx_out, _): (Vec<TXOutput>, usize) = bincode::serde::decode_from_slice(
                v.to_vec().as_slice(),
                bincode::config::standard(),
            )
            .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))?;
            for (current_out_index, out) in tx_out.iter().enumerate() {
                total_checked += 1;
                debug!(
                    "Checking output {} in tx {}: value={}, in_mempool={}, locked_with_key={}",
                    current_out_index,
                    txid_hex,
                    out.get_value(),
                    out.is_in_global_mem_pool(),
                    out.is_locked_with_key(from_pub_key_hash)
                );
                if out.not_in_global_mem_pool()
                    && out.get_value() > 0
                    && out.is_locked_with_key(from_pub_key_hash)
                    && accmulated < amount
                {
                    accmulated += out.get_value();
                    debug!(
                        "Adding spendable output: tx={}, idx={}, value={}, accumulated={}",
                        txid_hex,
                        current_out_index,
                        out.get_value(),
                        accmulated
                    );
                    if unspent_outputs_indexes.contains_key(txid_hex.as_str()) {
                        unspent_outputs_indexes
                            .get_mut(txid_hex.as_str())
                            .ok_or(BtcError::UTXONotFoundError(format!(
                                "(find_spendable_outputs) UTXO {} not found",
                                txid_hex
                            )))?
                            .push(current_out_index);
                    } else {
                        unspent_outputs_indexes.insert(txid_hex.clone(), vec![current_out_index]);
                    }
                }
            }
        }
        debug!(
            "find_spendable_outputs completed: checked {} outputs, accumulated={}, found {} spendable transactions",
            total_checked,
            accmulated,
            unspent_outputs_indexes.len()
        );
        Ok((accmulated, unspent_outputs_indexes))
    }

    pub async fn find_utxo(&self, pub_key_hash: &[u8]) -> Result<Vec<TXOutput>> {
        let db = self.blockchain.get_db().await?;
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
        let mut utxos = vec![];
        let mut total_items = 0;

        for item in utxo_tree.iter() {
            let (k, v) = item.map_err(|e| BtcError::GettingUTXOError(e.to_string()))?;
            total_items += 1;
            let txid_hex = HEXLOWER.encode(&k);
            debug!("Checking UTXO tree item: {}", txid_hex);

            let outs: Vec<TXOutput> = bincode::serde::decode_from_slice(
                v.to_vec().as_slice(),
                bincode::config::standard(),
            )
            .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))?
            .0;

            debug!("Transaction {} has {} outputs", txid_hex, outs.len());
            for (idx, out) in outs.iter().enumerate() {
                debug!(
                    "Output {}: value = {}, checking if locked with key",
                    idx,
                    out.get_value()
                );
                if out.is_locked_with_key(pub_key_hash) {
                    debug!("Found matching UTXO: value = {}", out.get_value());
                    utxos.push(out.clone())
                }
            }
        }
        debug!(
            "UTXO tree has {} total items, found {} matching UTXOs",
            total_items,
            utxos.len()
        );
        Ok(utxos)
    }

    pub async fn count_transactions(&self) -> Result<i32> {
        let db = self.blockchain.get_db().await?;
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
        let mut counter = 0;
        for _ in utxo_tree.iter() {
            counter += 1;
        }
        Ok(counter)
    }

    /// The `reindex` function reindexes the UTXO set by clearing the existing UTXO tree and rebuilding it from the blockchain.
    /// It iterates through the blockchain, finds all UTXOs, and inserts them into the UTXO tree.
    ///
    /// # Arguments
    ///
    /// * `blockchain` - A reference to the blockchain.
    ///
    pub async fn reindex(&self) -> Result<()> {
        debug!("Starting UTXOSet reindex...");
        let db = self.blockchain.get_db().await?;
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
        utxo_tree
            .clear()
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;

        let utxo_map = self.blockchain.find_utxo().await?;
        debug!("Found {} transactions with UTXOs", utxo_map.len());

        for (txid_hex, outs) in &utxo_map {
            debug!(
                "Processing transaction {} with {} outputs",
                txid_hex,
                outs.len()
            );
            let txid = HEXLOWER
                .decode(txid_hex.as_bytes())
                .map_err(|e| BtcError::TransactionIdHexDecodingError(e.to_string()))?;
            let value = bincode::serde::encode_to_vec(outs, bincode::config::standard())
                .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
            let _ = utxo_tree
                .insert(txid.as_slice(), value)
                .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
        }
        debug!("UTXOSet reindex completed");
        Ok(())
    }

    pub async fn update(&self, block: &Block) -> Result<()> {
        let db = self.blockchain.get_db().await?;
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
        for curr_block_tx in block.get_transactions().await? {
            // Coinbase transactions dont have inputs
            if !curr_block_tx.is_coinbase() {
                for curr_blc_tx_inpt in curr_block_tx.get_vin() {
                    let mut updated_outs = vec![];
                    let curr_blc_tx_inpt_utxo_ivec = utxo_tree
                        .get(curr_blc_tx_inpt.get_txid())
                        .map_err(|e| BtcError::GettingUTXOError(e.to_string()))?
                        .ok_or(BtcError::UTXONotFoundError(format!(
                            "(update) UTXO {} not found",
                            curr_blc_tx_inpt.get_input_tx_id_hex()
                        )))?;
                    let curr_blc_tx_inpt_utxo_list: Vec<TXOutput> =
                        bincode::serde::decode_from_slice(
                            curr_blc_tx_inpt_utxo_ivec.as_ref(),
                            bincode::config::standard(),
                        )
                        .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))?
                        .0;
                    for (utxo_curr_utxo_idx, db_curr_utxo) in
                        curr_blc_tx_inpt_utxo_list.iter().enumerate()
                    {
                        if utxo_curr_utxo_idx != curr_blc_tx_inpt.get_vout() {
                            updated_outs.push(db_curr_utxo.clone())
                        }
                    }
                    if updated_outs.is_empty() {
                        utxo_tree
                            .remove(curr_blc_tx_inpt.get_txid())
                            .map_err(|e| BtcError::RemovingUTXOError(e.to_string()))?;
                    } else {
                        let outs_bytes = bincode::serde::encode_to_vec(
                            &updated_outs,
                            bincode::config::standard(),
                        )
                        .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
                        utxo_tree
                            .insert(curr_blc_tx_inpt.get_txid(), outs_bytes)
                            .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
                    }
                }
            }
            let mut new_outputs = vec![];
            for curr_tx_out in curr_block_tx.get_vout() {
                new_outputs.push(curr_tx_out.clone())
            }
            let outs_bytes =
                bincode::serde::encode_to_vec(&new_outputs, bincode::config::standard())
                    .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
            let _ = utxo_tree
                .insert(curr_block_tx.get_id(), outs_bytes)
                .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
        }
        Ok(())
    }

    /// Rollback UTXO set by removing transactions from a block (for chain reorganization)
    ///
    /// This method reverses the effects of a block on the UTXO set:
    /// 1. Removes all outputs created by transactions in the block
    /// 2. Restores all inputs that were spent by those transactions
    /// 3. Processes transactions in reverse order to maintain consistency
    pub async fn rollback_block(&self, block: &Block) -> Result<()> {
        let db = self.blockchain.get_db().await?;
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;

        // Fix 5: Process transactions in REVERSE order (newest first) to correctly
        // handle intra-block dependencies where a later tx spends an earlier tx's output
        let transactions = block.get_transactions().await?;
        let reversed: Vec<_> = transactions.iter().rev().collect();

        for curr_block_tx in reversed {
            // Step 1: Remove this transaction's outputs from UTXO set
            utxo_tree
                .remove(curr_block_tx.get_id())
                .map_err(|e| BtcError::RemovingUTXOError(e.to_string()))?;

            // Step 2: Restore the inputs that this transaction spent (unless coinbase)
            if !curr_block_tx.is_coinbase() {
                for curr_blc_tx_inpt in curr_block_tx.get_vin() {
                    // Get the transaction that this input references
                    if let Some(input_tx) = self
                        .blockchain
                        .find_transaction(curr_blc_tx_inpt.get_txid())
                        .await?
                    {
                        // Find the specific output that was spent and restore it
                        if let Some(output) = input_tx.get_vout().get(curr_blc_tx_inpt.get_vout()) {
                            // Fix 1: Load existing outputs OR start with empty vec
                            // Previously, if the txid was fully removed from the UTXO tree
                            // (all outputs spent), outs_to_restore stayed empty and the
                            // output was lost. Now we always restore the output.
                            let mut outs_to_restore = if let Some(existing_outs_bytes) = utxo_tree
                                .get(curr_blc_tx_inpt.get_txid())
                                .map_err(|e| BtcError::GettingUTXOError(e.to_string()))?
                            {
                                // Deserialize existing outputs for this transaction
                                bincode::serde::decode_from_slice(
                                    existing_outs_bytes.as_ref(),
                                    bincode::config::standard(),
                                )
                                .map_err(|e| {
                                    BtcError::TransactionDeserializationError(e.to_string())
                                })?
                                .0
                            } else {
                                // Transaction was fully spent — no entry in UTXO tree
                                // Start with empty vec; we'll insert the restored output below
                                info!(
                                    "Restoring fully-spent UTXO for txid: {}",
                                    HEXLOWER.encode(curr_blc_tx_inpt.get_txid())
                                );
                                vec![]
                            };

                            // Insert the restored output at the correct vout position
                            let vout_idx = curr_blc_tx_inpt.get_vout();
                            if vout_idx <= outs_to_restore.len() {
                                outs_to_restore.insert(vout_idx, output.clone());
                            } else {
                                // Pad with clones of the output up to the required position
                                while outs_to_restore.len() < vout_idx {
                                    outs_to_restore.push(output.clone());
                                }
                                outs_to_restore.push(output.clone());
                            }

                            // Save the restored UTXOs back to the database
                            let outs_bytes = bincode::serde::encode_to_vec(
                                &outs_to_restore,
                                bincode::config::standard(),
                            )
                            .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;

                            utxo_tree
                                .insert(curr_blc_tx_inpt.get_txid(), outs_bytes)
                                .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn set_global_mem_pool_flag(&self, tx: &Transaction, flag: bool) -> Result<()> {
        let db = self.blockchain.get_db().await?;
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;

        if !tx.is_coinbase() {
            // Coinbase transactions dont have inputs
            for curr_tx_inpt in tx.get_vin() {
                if let Some(curr_tx_inpt_utxo_ivec) = utxo_tree
                    .get(curr_tx_inpt.get_txid())
                    .map_err(|e| BtcError::GettingUTXOError(e.to_string()))?
                {
                    let mut curr_tx_inpt_utxo_list: Vec<TXOutput> =
                        bincode::serde::decode_from_slice(
                            curr_tx_inpt_utxo_ivec.as_ref(),
                            bincode::config::standard(),
                        )
                        .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))?
                        .0;
                    for (utxo_curr_utxo_idx, db_curr_utxo) in
                        curr_tx_inpt_utxo_list.iter_mut().enumerate()
                    {
                        if utxo_curr_utxo_idx == curr_tx_inpt.get_vout() {
                            // Flag the TXOutput as in global mem pool
                            db_curr_utxo.set_in_global_mem_pool(flag);
                            trace!("\n");
                            trace!("------------------------------------------------------");
                            debug!("Set TXOUT to Intransit");
                            trace!("utxo_curr_utxo_idx: {:?}", utxo_curr_utxo_idx);
                            trace!("db_curr_utxo.get_value(): {:?}", db_curr_utxo.get_value());
                            for tx_out in tx.get_vout() {
                                trace!("tx_out.get_value(): {:?}", tx_out.get_value());
                            }
                            trace!("------------------------------------------------------");
                        }
                    }
                    trace!("Update UTXO in DB");
                    let outs_bytes = bincode::serde::encode_to_vec(
                        &curr_tx_inpt_utxo_list,
                        bincode::config::standard(),
                    )
                    .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
                    utxo_tree
                        .insert(curr_tx_inpt.get_txid(), outs_bytes)
                        .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
                } else {
                    debug!("TXOUT not found in DB");
                }
            }
        }
        Ok(())
    }

    pub async fn get_balance(&self, wlt_address: &WalletAddress) -> Result<i32> {
        let pub_key_hash = get_pub_key_hash(wlt_address)?;
        debug!("Getting balance for address: {}", wlt_address.as_str());
        debug!("Public key hash: {:?}", pub_key_hash);

        let utxos = self.find_utxo(pub_key_hash.as_slice()).await?;
        debug!(
            "Found {} UTXOs for address {}",
            utxos.len(),
            wlt_address.as_str()
        );

        let mut balance = 0;
        for (idx, utxo) in utxos.iter().enumerate() {
            debug!("UTXO {}: value = {}", idx, utxo.get_value());
            balance += utxo.get_value();
        }
        debug!("Total balance for {}: {}", wlt_address.as_str(), balance);
        Ok(balance)
    }

    pub async fn utxo_count(&self, wlt_address: &WalletAddress) -> Result<usize> {
        let pub_key_hash = get_pub_key_hash(wlt_address)?;
        debug!("Getting balance for address: {}", wlt_address.as_str());
        debug!("Public key hash: {:?}", pub_key_hash);

        let count = self.find_utxo(pub_key_hash.as_slice()).await?.len();

        debug!("Total count for {}: {}", wlt_address.as_str(), count);
        Ok(count)
    }
}
