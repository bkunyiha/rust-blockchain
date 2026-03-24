use crate::WalletAddress;
use crate::chain::BlockchainService;
use crate::chain::UTXOSet;
use crate::crypto::hash::sha256_digest;
use crate::crypto::signature::{schnorr_sign_digest, schnorr_sign_verify};
use crate::error::{BtcError, Result};
use crate::wallet::{WalletService, convert_address, get_pub_key_hash, hash_pub_key};
use data_encoding::HEXLOWER;
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

const SUBSIDY: i32 = 10;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

impl TXInput {
    pub fn new(txid: &[u8], vout: usize) -> TXInput {
        TXInput {
            txid: txid.to_vec(),
            vout,
            signature: vec![],
            pub_key: vec![],
        }
    }

    pub fn get_txid(&self) -> &[u8] {
        self.txid.as_slice()
    }

    pub fn get_input_tx_id_hex(&self) -> String {
        HEXLOWER.encode(self.txid.as_slice())
    }

    pub fn get_vout(&self) -> usize {
        self.vout
    }

    pub fn get_pub_key(&self) -> &[u8] {
        self.pub_key.as_slice()
    }

    pub fn get_signature(&self) -> &[u8] {
        self.signature.as_slice()
    }

    pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
        let locking_hash = hash_pub_key(self.pub_key.as_slice());
        locking_hash.eq(pub_key_hash)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TXOutput {
    value: i32,
    in_global_mem_pool: bool,
    pub_key_hash: Vec<u8>,
}

impl TXOutput {
    pub fn new(value: i32, address: &WalletAddress) -> Result<TXOutput> {
        let mut output = TXOutput {
            value,
            in_global_mem_pool: false,
            pub_key_hash: vec![],
        };
        output.lock(address)?;
        Ok(output)
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_pub_key_hash(&self) -> &[u8] {
        self.pub_key_hash.as_slice()
    }

    // The `lock` function locks the output to the address.
    // It uses the `base58_decode` function to decode the address.
    // It uses the `ADDRESS_CHECK_SUM_LEN` constant to get the length of the address check sum.
    // It uses the `pub_key_hash` field to store the public key hash.
    // It returns the new output.
    fn lock(&mut self, address: &WalletAddress) -> Result<()> {
        let pub_key_hash = get_pub_key_hash(address)?;
        self.pub_key_hash = pub_key_hash;
        Ok(())
    }

    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash.eq(pub_key_hash)
    }

    pub fn not_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash.ne(pub_key_hash)
    }

    pub fn set_in_global_mem_pool(&mut self, value: bool) {
        self.in_global_mem_pool = value;
    }

    pub fn is_in_global_mem_pool(&self) -> bool {
        self.in_global_mem_pool
    }
    pub fn not_in_global_mem_pool(&self) -> bool {
        !self.in_global_mem_pool
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

impl Transaction {
    // The `new_coinbase_tx` function creates a new coinbase transaction.
    // It uses the `SUBSIDY` constant to set the value of the transaction.
    // It uses the `to` parameter to set the address of the recipient.
    // It returns the new transaction.
    pub fn new_coinbase_tx(to: &WalletAddress) -> Result<Transaction> {
        let txout = TXOutput::new(SUBSIDY, to)?;
        let tx_input = TXInput {
            signature: Uuid::new_v4().as_bytes().to_vec(),
            ..Default::default()
        };

        let mut tx = Transaction {
            id: vec![],
            vin: vec![tx_input],
            vout: vec![txout],
        };

        tx.id = tx.hash()?;
        Ok(tx)
    }

    ///
    /// This function constructs a new UTXO-based transaction
    /// by selecting spendable outputs and creating inputs for the transaction.
    /// It calculates the inputs required based on available outputs,
    /// manages outputs for the recipient and change, signs the transaction,
    /// and computes its ID through hashing:
    ///
    /// # Arguments
    ///
    /// * `from` - The address of the sender.
    /// * `to` - The address of the recipient.
    pub async fn new_utxo_transaction(
        from_wlt_addr: &WalletAddress,
        to_wlt_addr: &WalletAddress,
        tx_amount: i32,
        utxo_set: &UTXOSet,
    ) -> Result<Transaction> {
        let wallets = WalletService::new()?;
        let from_wallet = wallets
            .get_wallet(from_wlt_addr)
            .ok_or_else(|| BtcError::UTXONotFoundError(from_wlt_addr.as_string()))?;
        let from_public_key_hash = hash_pub_key(from_wallet.get_public_key());

        let (available_funds, valid_outputs) = utxo_set
            .find_spendable_outputs(from_public_key_hash.as_slice(), tx_amount)
            .await?;

        debug!(
            "Transaction creation: from={}, to={}, amount={}",
            from_wlt_addr.as_str(),
            to_wlt_addr.as_str(),
            tx_amount
        );
        debug!(
            "Found spendable outputs: accumulated={}, valid_outputs={:?}",
            available_funds, valid_outputs
        );

        if available_funds < tx_amount {
            return Err(BtcError::NotEnoughFunds);
        }

        let mut inputs = vec![];
        for (txid_hex, out_indexes) in valid_outputs {
            let txid = HEXLOWER
                .decode(txid_hex.as_bytes())
                .map_err(|e| BtcError::TransactionIdHexDecodingError(e.to_string()))?;
            for current_out_index in out_indexes {
                let input = TXInput {
                    txid: txid.clone(), // txid is the hash of the previous transaction or transaction that contains the output that is being spent
                    vout: current_out_index, // vout is the index of the output that is being spent in the previous transaction or transaction that contains the output that is being spent
                    signature: vec![],
                    pub_key: from_wallet.get_public_key().to_vec(),
                };
                inputs.push(input);
            }
        }

        let mut outputs = vec![TXOutput::new(tx_amount, to_wlt_addr)?];

        if available_funds > tx_amount {
            let change = available_funds - tx_amount;
            debug!(
                "Creating change output: {} to {}",
                change,
                from_wlt_addr.as_str()
            );
            outputs.push(TXOutput::new(change, from_wlt_addr)?); // to: Return change to the sender
        }

        // Create a new transaction with the spent inputs and unspent outputs
        let mut tx = Transaction {
            id: vec![],
            vin: inputs,
            vout: outputs,
        };
        tx.id = tx.hash()?;
        debug!(
            "Created transaction with {} inputs and {} outputs",
            tx.get_vin().len(),
            tx.get_vout().len()
        );
        tx.sign(utxo_set.get_blockchain(), from_wallet.get_pkcs8())
            .await?;
        Ok(tx)
    }

    ///
    /// `trimmed_copy` is an internal function that creates a trimmed copy of the transaction,
    /// excluding signatures, enabling signature verification without modifying
    /// the original transaction
    ///
    /// # Returns
    ///
    /// A trimmed copy of the transaction.
    fn trimmed_copy(&self) -> Transaction {
        let mut inputs = vec![];
        let mut outputs = vec![];
        for input in &self.vin {
            let txinput = TXInput::new(input.get_txid(), input.get_vout());
            inputs.push(txinput);
        }
        for output in &self.vout {
            outputs.push(output.clone());
        }
        Transaction {
            id: self.id.clone(),
            vin: inputs,
            vout: outputs,
        }
    }

    ///
    /// The `sign` function signs the transaction inputs using Schnorr signatures with secp256k1.
    /// This is the signature scheme used by P2TR (Pay-to-Taproot) addresses.
    /// It retrieves previous transactions, prepares a copy for signature verification,
    /// signs inputs with the corresponding private keys, and updates the transaction with signatures.
    ///
    /// # Arguments
    ///
    /// * `blockchain` - A reference to the blockchain.
    /// * `private_key` - A reference to the private key.
    ///
    /// # Returns
    ///
    /// A signed transaction.
    async fn sign(&mut self, blockchain: &BlockchainService, private_key: &[u8]) -> Result<()> {
        let mut tx_copy = self.trimmed_copy();

        for (idx, vin) in self.vin.iter_mut().enumerate() {
            let prev_tx_option = blockchain.find_transaction(vin.get_txid()).await?;
            let prev_tx = match prev_tx_option {
                Some(tx) => tx,
                None => {
                    return Err(BtcError::TransactionNotFoundError(
                        "(sign) Previous transaction is not correct".to_string(),
                    ));
                }
            };

            tx_copy.vin[idx].signature = vec![];
            tx_copy.vin[idx].pub_key = prev_tx.vout[vin.vout].pub_key_hash.clone();
            tx_copy.id = tx_copy.hash()?;
            tx_copy.vin[idx].pub_key = vec![];

            let signature = schnorr_sign_digest(private_key, tx_copy.get_id())?;
            vin.signature = signature;
        }
        Ok(())
    }

    ///
    /// This function verifies transaction signatures against corresponding public keys using Schnorr signatures.
    /// It checks for Coinbase transactions, prepares a trimmed copy,
    /// validates Schnorr signatures against public keys, and ensures the transaction is valid.
    ///
    /// # Arguments
    ///
    /// * `blockchain` - A reference to the blockchain.
    ///
    /// # Returns
    ///
    pub async fn verify(&self, blockchain: &BlockchainService) -> Result<bool> {
        if self.is_coinbase() {
            return Ok(true);
        }
        let mut trimmed_self_copy = self.trimmed_copy();
        for (idx, vin) in self.vin.iter().enumerate() {
            let current_vin_tx_option = blockchain.find_transaction(vin.get_txid()).await?;
            let current_vin_tx = match current_vin_tx_option {
                Some(tx) => tx,
                None => {
                    return Err(BtcError::TransactionNotFoundError(
                        "(verify) Previous transaction is not correct".to_string(),
                    ));
                }
            };

            trimmed_self_copy.vin[idx].signature = vec![];
            trimmed_self_copy.vin[idx].pub_key = current_vin_tx.vout[vin.vout].pub_key_hash.clone();
            trimmed_self_copy.id = trimmed_self_copy.hash()?;
            trimmed_self_copy.vin[idx].pub_key = vec![];

            let verify = schnorr_sign_verify(
                vin.get_pub_key(),
                vin.get_signature(),
                trimmed_self_copy.get_id(),
            );
            if !verify {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1
            && self
                .vin
                .first()
                .iter()
                .any(|tx_in| tx_in.get_pub_key().is_empty())
    }

    pub fn not_coinbase(&self) -> bool {
        !self.is_coinbase()
    }

    ///
    /// The `hash` function generates the transaction's hash by creating a copy without the ID,
    /// serializing it, and computing its SHA-256 digest
    ///
    /// # Returns
    ///
    /// The transaction's hash.
    fn hash(&mut self) -> Result<Vec<u8>> {
        let tx_copy = Transaction {
            id: vec![],
            vin: self.vin.clone(),
            vout: self.vout.clone(),
        };
        Ok(sha256_digest(tx_copy.serialize()?.as_slice()))
    }

    // get the transaction id as a bytes vector
    // transaction.id is an owned vector, so we need to return a reference to the id bytes
    pub fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }

    // get the transaction id as a hex string
    // Use Cases: APIs/JSON responses, Logging/debugging, User interfaces, Block explorers
    pub fn get_tx_id_hex(&self) -> String {
        HEXLOWER.encode(self.get_id())
    }

    // get the transaction id as a bytes vector
    // Use Cases: Network protocol messages, Binary storage/database keys, Cryptographic operations, Memory efficiency
    pub fn get_id_bytes(&self) -> Vec<u8> {
        self.id.clone()
    }

    pub fn get_vin(&self) -> &[TXInput] {
        self.vin.as_slice()
    }

    pub fn get_vout(&self) -> &[TXOutput] {
        self.vout.as_slice()
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serde::encode_to_vec(self, bincode::config::standard())
            .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Transaction> {
        bincode::serde::decode_from_slice(bytes, bincode::config::standard())
            .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))
            .map(|(transaction, _)| transaction)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TxInputSummary {
    txid_hex: String,
    output_idx: usize,
    wlt_addr: WalletAddress,
}
impl TxInputSummary {
    pub fn new(txid_hex: String, output_idx: usize, wlt_addr: WalletAddress) -> TxInputSummary {
        TxInputSummary {
            txid_hex,
            output_idx,
            wlt_addr,
        }
    }
    pub fn get_txid_hex(&self) -> &str {
        &self.txid_hex
    }
    pub fn get_output_idx(&self) -> usize {
        self.output_idx
    }
    pub fn get_wlt_addr(&self) -> &WalletAddress {
        &self.wlt_addr
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct TxOutputSummary {
    wlt_addr: WalletAddress,
    value: i32,
}
impl TxOutputSummary {
    pub fn new(wlt_addr: WalletAddress, value: i32) -> TxOutputSummary {
        TxOutputSummary { wlt_addr, value }
    }
    pub fn get_wlt_addr(&self) -> &WalletAddress {
        &self.wlt_addr
    }
    pub fn get_value(&self) -> i32 {
        self.value
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct TxSummary {
    transaction_id: String,
    inputs: Vec<TxInputSummary>,
    outputs: Vec<TxOutputSummary>,
}
impl TxSummary {
    pub fn new(transaction_id: String) -> TxSummary {
        TxSummary {
            transaction_id,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
    pub fn add_input(&mut self, input: TxInputSummary) {
        self.inputs.push(input);
    }
    pub fn add_output(&mut self, output: TxOutputSummary) {
        self.outputs.push(output);
    }
    pub fn get_transaction_id(&self) -> &str {
        &self.transaction_id
    }
    pub fn get_inputs(&mut self) -> &[TxInputSummary] {
        let _ = &self.inputs.reverse();
        &self.inputs
    }
    pub fn get_outputs(&mut self) -> &[TxOutputSummary] {
        let _ = &self.outputs.reverse();
        &self.outputs
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WalletTransactionType {
    Debit,
    Credit,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WalletTransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WalletTransaction {
    tx_id: Vec<u8>,
    from_wlt_addr: Option<WalletAddress>,
    to_wlt_addr: WalletAddress,
    value: i32,
    transaction_type: WalletTransactionType,
    status: WalletTransactionStatus,
    vout: usize,
    is_coinbase: bool,
    input_count: usize,
    output_count: usize,
    total_output_value: i32,
    fee: i32,
    timestamp: i64,
    size_bytes: usize,
}
impl WalletTransaction {
    pub fn new(
        tx: Transaction,
        tx_output: &TXOutput,
        from_wlt_addr: Option<WalletAddress>,
        transaction_type: WalletTransactionType,
        vout_index: usize,
        fee: i32,
        timestamp: i64,
    ) -> Result<WalletTransaction> {
        let status = if tx_output.is_in_global_mem_pool() {
            WalletTransactionStatus::Pending
        } else {
            WalletTransactionStatus::Confirmed
        };
        Ok(WalletTransaction {
            tx_id: tx.get_id().to_vec(),
            from_wlt_addr,
            to_wlt_addr: convert_address(tx_output.get_pub_key_hash())?,
            value: tx_output.get_value(),
            transaction_type,
            status,
            vout: vout_index,
            is_coinbase: tx.is_coinbase(),
            input_count: tx.get_vin().len(),
            output_count: tx.get_vout().len(),
            total_output_value: tx.get_vout().iter().map(|v| v.get_value()).sum(),
            fee,
            timestamp,
            size_bytes: tx.serialize().unwrap_or_default().len(),
        })
    }
    pub fn get_tx_id(&self) -> &[u8] {
        &self.tx_id
    }
    pub fn get_from_wlt_addr(&self) -> &Option<WalletAddress> {
        &self.from_wlt_addr
    }
    pub fn get_to_wlt_addr(&self) -> &WalletAddress {
        &self.to_wlt_addr
    }
    pub fn get_value(&self) -> i32 {
        self.value
    }
    pub fn get_transaction_type(&self) -> &WalletTransactionType {
        &self.transaction_type
    }
    pub fn get_status(&self) -> &WalletTransactionStatus {
        &self.status
    }
    pub fn get_vout(&self) -> usize {
        self.vout
    }
    pub fn is_coinbase(&self) -> bool {
        self.is_coinbase
    }
    pub fn get_input_count(&self) -> usize {
        self.input_count
    }
    pub fn get_output_count(&self) -> usize {
        self.output_count
    }
    pub fn get_total_output_value(&self) -> i32 {
        self.total_output_value
    }
    pub fn get_fee(&self) -> i32 {
        self.fee
    }
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
    pub fn get_size_bytes(&self) -> usize {
        self.size_bytes
    }
    pub fn get_tx_id_hex(&self) -> String {
        data_encoding::HEXLOWER.encode(&self.tx_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_test_genesis_address() -> crate::WalletAddress {
        // Create a wallet to get a valid Bitcoin address
        let wallet = crate::wallet::Wallet::new().expect("Failed to create test wallet");
        wallet.get_address().expect("Failed to get wallet address")
    }

    #[test]
    fn test_coinbase_transaction_creation() {
        let address = generate_test_genesis_address(); // Valid Bitcoin address
        let tx = Transaction::new_coinbase_tx(&address.clone())
            .expect("Failed to create coinbase transaction");

        assert!(tx.is_coinbase());
        assert_eq!(tx.get_vout().len(), 1);
        assert_eq!(tx.get_vin().len(), 1);

        let output = &tx.get_vout()[0];
        assert_eq!(output.get_value(), 10); // Default coinbase reward
    }

    #[test]
    fn test_transaction_serialization_deserialization() {
        let genesis_address = generate_test_genesis_address();
        let tx = Transaction::new_coinbase_tx(&genesis_address)
            .expect("Failed to create coinbase transaction");

        let serialized = tx.serialize().expect("Serialization failed");
        let deserialized = Transaction::deserialize(&serialized).expect("Deserialization failed");

        assert_eq!(tx.get_id(), deserialized.get_id());
        assert_eq!(tx.get_vin().len(), deserialized.get_vin().len());
        assert_eq!(tx.get_vout().len(), deserialized.get_vout().len());
    }

    #[test]
    fn test_transaction_id() {
        let genesis_address = generate_test_genesis_address();
        let tx = Transaction::new_coinbase_tx(&genesis_address)
            .expect("Failed to create coinbase transaction");
        let tx_id = tx.get_id();

        assert!(!tx_id.is_empty());
        assert_eq!(tx_id.len(), 32); // SHA256 hash is 32 bytes
    }

    #[test]
    fn test_transaction_id_bytes() {
        let genesis_address = generate_test_genesis_address();
        let tx = Transaction::new_coinbase_tx(&genesis_address)
            .expect("Failed to create coinbase transaction");
        let tx_id_bytes = tx.get_id_bytes();

        assert_eq!(tx_id_bytes.len(), 32);
        assert_eq!(tx_id_bytes, tx.get_id());
    }

    #[test]
    fn test_coinbase_transaction_validation() {
        let genesis_address = generate_test_genesis_address();
        let tx = Transaction::new_coinbase_tx(&genesis_address)
            .expect("Failed to create coinbase transaction");

        // Coinbase transactions should have empty signature and pub_key
        let vin = &tx.get_vin()[0];
        assert!(vin.get_pub_key().is_empty());
    }

    #[test]
    fn test_transaction_output_value() {
        let genesis_address = generate_test_genesis_address();
        let tx = Transaction::new_coinbase_tx(&genesis_address)
            .expect("Failed to create coinbase transaction");
        let output = &tx.get_vout()[0];

        assert_eq!(output.get_value(), 10);
        assert!(!output.get_pub_key_hash().is_empty());
    }

    #[test]
    fn test_transaction_input_creation() {
        let tx_id = vec![1, 2, 3, 4];
        let vout = 0;

        let tx_input = TXInput::new(&tx_id, vout);

        assert_eq!(tx_input.get_txid(), tx_id.as_slice());
        assert_eq!(tx_input.get_vout(), vout);
        assert!(tx_input.get_pub_key().is_empty());
    }

    #[test]
    fn test_transaction_output_creation() {
        let value = 100;
        let address = generate_test_genesis_address();

        let tx_output = TXOutput::new(value, &address.clone()).expect("Failed to create output");

        assert_eq!(tx_output.get_value(), value);
        assert!(!tx_output.get_pub_key_hash().is_empty());
    }

    #[test]
    fn test_transaction_output_lock_unlock() {
        let value = 100;
        let address = generate_test_genesis_address();
        let tx_output = TXOutput::new(value, &address.clone()).expect("Failed to create output");

        // Test locking with pub_key_hash
        let pub_key_hash = tx_output.get_pub_key_hash();
        assert!(tx_output.is_locked_with_key(pub_key_hash));

        // Test with wrong pub_key_hash
        let wrong_pub_key_hash = vec![5, 6, 7, 8];
        assert!(!tx_output.is_locked_with_key(&wrong_pub_key_hash));
    }

    #[test]
    fn test_transaction_input_can_unlock() {
        let tx_id = vec![1, 2, 3, 4];
        let vout = 0;

        let tx_input = TXInput::new(&tx_id, vout);

        // Test with empty pub_key (should return false)
        let pub_key_hash = vec![1, 2, 3, 4];
        assert!(!tx_input.uses_key(&pub_key_hash));
    }
}
