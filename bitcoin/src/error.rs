use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum BtcError {
    // Recoverable errors
    #[error("Blockchain not found error: {0}")]
    BlockchainNotFoundError(String),

    #[error("Invalid value:{0} for miner. Must be yes or no.")]
    InvalidValueForMiner(String),

    #[error("Invalid value:{0} for web server. Must be yes or no.")]
    InvalidValueForWebServer(String),

    #[error("Node is not a miner")]
    NotAMiner,

    // Unrecoverable errors
    #[error("Invalid transaction")]
    InvalidTransaction,
    #[error("Invalid block")]
    InvalidBlock,
    #[error("Invalid block header")]
    InvalidBlockHeader,
    #[error("Invalid transaction input")]
    InvalidTransactionInput,
    #[error("Invalid transaction output")]
    InvalidTransactionOutput,
    #[error("Invalid merkle root")]
    InvalidMerkleRoot,
    #[error("Invalid hash")]
    InvalidHash,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Invalid private key")]
    InvalidPrivateKey,
    #[error("Block deserialization error: {0}")]
    BlockDeserializationError(String),
    #[error("Block serialization error: {0}")]
    BlockSerializationError(String),

    #[error("Not enough funds")]
    NotEnoughFunds,

    #[error("Blockchain tip hash error: {0}")]
    BlockChainTipHashError(String),

    #[error("Transaction deserialization error: {0}")]
    TransactionDeserializationError(String),
    #[error("Transaction serialization error: {0}")]
    TransactionSerializationError(String),
    #[error("TransactionSignatureError lock error: {0}")]
    TransactionSignatureError(String),
    #[error("Transaction Id Hex encoding error: {0}")]
    TransactionIdHexEncodingError(String),
    #[error("Transaction Id Hex  decoding  error: {0}")]
    TransactionIdHexDecodingError(String),
    #[error("Transaction not found error: {0}")]
    TransactionNotFoundError(String),
    #[error("Transaction Already Exists In Memory Pool: {0}")]
    TransactionAlreadyExistsInMemoryPool(String),

    #[error("Address encoding error: {0}")]
    AddressEncodingError(String),
    #[error("Address decoding  error: {0}")]
    AddressDecodingError(String),
    #[error("Blockchain tip hash poisoned lock error: {0}")]
    BlockchainTipHashPoisonedLockError(String),
    #[error("Nodes inner poisoned lock error: {0}")]
    NodesInnerPoisonedLockError(String),
    #[error("Memory pool inner poisoned lock error: {0}")]
    MemoryPoolInnerPoisonedLockError(String),
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    // IO errors
    #[error("Saving wallets error: {0}")]
    SavingWalletsError(String),
    #[error("Wallets file path error: {0}")]
    WalletsFilePathError(String),
    #[error("Wallets file open error: {0}")]
    WalletsFileOpenError(String),
    #[error("Wallets file read error: {0}")]
    WalletsFileReadError(String),
    #[error("Wallets file metadata error: {0}")]
    WalletsFileMetadataError(String),

    #[error("Wallet key error: {0}")]
    WalletKeyError(String),
    #[error("Wallet key pair error: {0}")]
    WalletKeyPairError(String),

    #[error("Wallets serialization error: {0}")]
    WalletsSerializationError(String),
    #[error("Wallets deserialization error: {0}")]
    WalletsDeserializationError(String),
    #[error("Wallet not found error: {0}")]
    WalletNotFoundError(String),

    // Sled errors
    #[error("UTXO DB connection error: {0}")]
    UTXODBconnection(String),
    #[error("Saving UTXO error: {0}")]
    SavingUTXOError(String),
    #[error("Getting UTXO error: {0}")]
    GettingUTXOError(String),
    #[error("UTXO not found error: {0}")]
    UTXONotFoundError(String),
    #[error("Removing UTXO error: {0}")]
    RemovingUTXOError(String),

    #[error("Blockchain DB connection error: {0}")]
    BlockchainDBconnection(String),
    #[error("Saving Blockchain error: {0}")]
    SavingBlockchainError(String),
    #[error("Getting Blockchain error: {0}")]
    GetBlockchainError(String),
    #[error("Open Blockchain tree error: {0}")]
    OpenBlockchainTreeError(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

pub type Result<T> = std::result::Result<T, BtcError>;
