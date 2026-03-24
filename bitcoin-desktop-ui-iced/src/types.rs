use bitcoin_api::{
    ApiResponse, BlockSummary, BlockchainInfo, CreateWalletResponse, SendTransactionResponse,
};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Blockchain,
    Wallet,
    Transactions,
    Mining,
    Health,
}

impl Menu {
    pub const ALL: [Menu; 5] = [
        Menu::Blockchain,
        Menu::Wallet,
        Menu::Transactions,
        Menu::Mining,
        Menu::Health,
    ];
}

impl core::fmt::Display for Menu {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Menu::Blockchain => "Blockchain",
            Menu::Wallet => "Wallet",
            Menu::Transactions => "Transactions",
            Menu::Mining => "Mining",
            Menu::Health => "Health",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletSection {
    GetWalletInfo,
    GetBalance,
    Create,
    Send,
    History,
    Addresses,
}

impl WalletSection {
    pub const ALL: [WalletSection; 6] = [
        WalletSection::GetWalletInfo,
        WalletSection::GetBalance,
        WalletSection::Create,
        WalletSection::Send,
        WalletSection::History,
        WalletSection::Addresses,
    ];
}

impl core::fmt::Display for WalletSection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            WalletSection::GetWalletInfo => "Get Wallet Info",
            WalletSection::GetBalance => "Get Balance",
            WalletSection::Create => "Create Wallet",
            WalletSection::Send => "Send Bitcoin",
            WalletSection::History => "Transaction History",
            WalletSection::Addresses => "All Addresses",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionSection {
    Mempool,
    MempoolTx,
    AllTransactions,
    AddressTransactions,
}

impl TransactionSection {
    pub const ALL: [TransactionSection; 4] = [
        TransactionSection::Mempool,
        TransactionSection::MempoolTx,
        TransactionSection::AllTransactions,
        TransactionSection::AddressTransactions,
    ];
}

impl core::fmt::Display for TransactionSection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            TransactionSection::Mempool => "Mempool",
            TransactionSection::MempoolTx => "Mempool Transaction",
            TransactionSection::AllTransactions => "All Transactions",
            TransactionSection::AddressTransactions => "Address Transactions",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiningSection {
    Info,
    Generate,
}

impl MiningSection {
    pub const ALL: [MiningSection; 2] = [MiningSection::Info, MiningSection::Generate];
}

impl core::fmt::Display for MiningSection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            MiningSection::Info => "Mining Info",
            MiningSection::Generate => "Generate Blocks",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthSection {
    Health,
    Liveness,
    Readiness,
}

impl HealthSection {
    pub const ALL: [HealthSection; 3] = [
        HealthSection::Health,
        HealthSection::Liveness,
        HealthSection::Readiness,
    ];
}

impl core::fmt::Display for HealthSection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            HealthSection::Health => "Health Check",
            HealthSection::Liveness => "Liveness Check",
            HealthSection::Readiness => "Readiness Check",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockchainSection {
    Info,
    LatestBlocks,
    AllBlocks,
    BlockByHash,
}

impl BlockchainSection {
    pub const ALL: [BlockchainSection; 4] = [
        BlockchainSection::Info,
        BlockchainSection::LatestBlocks,
        BlockchainSection::AllBlocks,
        BlockchainSection::BlockByHash,
    ];
}

impl core::fmt::Display for BlockchainSection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            BlockchainSection::Info => "Get Block Info",
            BlockchainSection::LatestBlocks => "Latest Blocks",
            BlockchainSection::AllBlocks => "All Blocks",
            BlockchainSection::BlockByHash => "Block by Hash",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DataSection {
    BlockchainInfo,
    Blocks,
    BlocksAll,
    BlockByHash,
    MiningInfo,
    Generate,
    Health,
    Liveness,
    Readiness,
    Mempool,
    MempoolTx,
    Transactions,
    AddressTransactions,
    WalletInfo,
    WalletBalance,
}

#[derive(Debug, Clone)]
pub enum Message {
    MenuChanged(Menu),
    BaseUrlChanged(String),
    ApiKeyChanged(String),
    // Inputs
    BlockHashChanged(String),
    MiningAddressChanged(String),
    MiningNBlocksChanged(String),
    MiningMaxTriesChanged(String),
    TxidChanged(String),
    AddrTxChanged(String),
    FetchInfo,
    FetchBlocks,
    InfoLoaded(Result<ApiResponse<BlockchainInfo>, String>),
    BlocksLoaded(Result<ApiResponse<Vec<BlockSummary>>, String>),
    // Extra blockchain
    FetchBlocksAll,
    BlocksAllLoaded(Result<ApiResponse<Value>, String>),
    FetchBlockByHash(String),
    BlockByHashLoaded(Result<ApiResponse<Value>, String>),
    // Mining
    FetchMiningInfo,
    MiningInfoLoaded(Result<ApiResponse<Value>, String>),
    GenerateToAddress {
        address: String,
        nblocks: u32,
        maxtries: Option<u32>,
    },
    GenerateToAddressDone(Result<ApiResponse<Value>, String>),
    // Health
    FetchHealth,
    HealthLoaded(Result<ApiResponse<Value>, String>),
    FetchLiveness,
    LivenessLoaded(Result<ApiResponse<Value>, String>),
    FetchReadiness,
    ReadinessLoaded(Result<ApiResponse<Value>, String>),
    // Transactions
    FetchMempool,
    MempoolLoaded(Result<ApiResponse<Value>, String>),
    FetchMempoolTx(String),
    MempoolTxLoaded(Result<ApiResponse<Value>, String>),
    FetchTransactions,
    TransactionsLoaded(Result<ApiResponse<Value>, String>),
    FetchAddressTransactions(String),
    AddressTransactionsLoaded(Result<ApiResponse<Value>, String>),
    // Wallet admin
    WalletLabelChanged(String),
    CreateWalletAdmin,
    CreateWalletAdminDone(Result<ApiResponse<CreateWalletResponse>, String>),
    FetchAddressesAdmin,
    AddressesAdminLoaded(Result<ApiResponse<Value>, String>),
    FetchWalletInfoAdmin(String),
    WalletInfoAdminLoaded(Result<ApiResponse<Value>, String>),
    FetchBalanceAdmin(String),
    BalanceAdminLoaded(Result<ApiResponse<Value>, String>),
    // Send transaction
    SendFromChanged(String),
    SendToChanged(String),
    SendAmountChanged(String),
    SendTx,
    TxSent(Result<ApiResponse<SendTransactionResponse>, String>),
    // Transaction history
    HistoryAddressChanged(String),
    FetchTransactionHistory(String),
    TransactionHistoryLoaded(Result<ApiResponse<Value>, String>),
    // Wallet section navigation
    WalletSectionChanged(WalletSection),
    // Transaction section navigation
    TransactionSectionChanged(TransactionSection),
    // Blockchain section navigation
    BlockchainSectionChanged(BlockchainSection),
    // Blockchain menu hover
    BlockchainMenuHovered(bool),
    WalletMenuHovered(bool),
    TransactionMenuHovered(bool),
    MiningMenuHovered(bool),
    HealthMenuHovered(bool),
    // Mining section navigation
    MiningSectionChanged(MiningSection),
    // Health section navigation
    HealthSectionChanged(HealthSection),
    // Clipboard
    CopyToClipboard(String),
    ClipboardCopied(bool), // true = success, false = failed
    // Text editor edit handlers for JSON displays
    TransactionsEditorAction(iced::widget::text_editor::Action),
    MempoolEditorAction(iced::widget::text_editor::Action),
    MempoolTxEditorAction(iced::widget::text_editor::Action),
    AddressTransactionsEditorAction(iced::widget::text_editor::Action),
    WalletInfoEditorAction(iced::widget::text_editor::Action),
    WalletBalanceEditorAction(iced::widget::text_editor::Action),
    TransactionHistoryEditorAction(iced::widget::text_editor::Action),
    BlocksAllEditorAction(iced::widget::text_editor::Action),
    BlockByHashEditorAction(iced::widget::text_editor::Action),
    BlockchainInfoEditorAction(iced::widget::text_editor::Action),
    LatestBlocksEditorAction(iced::widget::text_editor::Action),
    CreatedWalletAddressEditorAction(iced::widget::text_editor::Action),
}
