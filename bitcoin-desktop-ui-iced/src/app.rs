use crate::types::{
    BlockchainSection, DataSection, HealthSection, Menu, Message, MiningSection,
    TransactionSection, WalletSection,
};
use bitcoin_api::{BlockSummary, BlockchainInfo};
use serde_json::Value;

#[derive(Debug)]
pub struct AdminApp {
    pub menu: Menu,
    pub base_url: String,
    pub api_key: String,
    pub status: String,
    pub info: Option<BlockchainInfo>,
    pub blocks: Vec<BlockSummary>,
    // Inputs for actions
    pub block_hash_input: String,
    pub mining_address_input: String,
    pub mining_nblocks_input: String,
    pub mining_maxtries_input: String,
    pub txid_input: String,
    pub addr_tx_input: String,
    // Wallet admin state
    pub wallet_label_input: String,
    pub addresses: Vec<String>,
    pub wallet_info: Option<Value>,
    pub wallet_balance: Option<Value>,
    pub created_wallet_address: Option<String>,
    // Send transaction state
    pub send_from_address: String,
    pub send_to_address: String,
    pub send_amount: String,
    pub last_txid: Option<String>,
    // Transaction history state
    pub history_address: String,
    pub transaction_history: Option<Value>,
    // Wallet section navigation
    pub wallet_section: WalletSection,
    // Transaction section navigation
    pub transaction_section: TransactionSection,
    // Blockchain section navigation
    pub blockchain_section: BlockchainSection,
    // Mining section navigation
    pub mining_section: MiningSection,
    // Health section navigation
    pub health_section: HealthSection,
    // Blockchain menu hover state
    pub blockchain_menu_hovered: bool,
    pub wallet_menu_hovered: bool,
    pub transaction_menu_hovered: bool,
    pub mining_menu_hovered: bool,
    pub health_menu_hovered: bool,
    // Response data storage
    pub blocks_all_data: Option<Value>,
    pub block_by_hash_data: Option<Value>,
    pub mining_info_data: Option<Value>,
    pub generate_result: Option<Value>,
    pub health_data: Option<Value>,
    pub liveness_data: Option<Value>,
    pub readiness_data: Option<Value>,
    pub mempool_data: Option<Value>,
    pub mempool_tx_data: Option<Value>,
    pub transactions_data: Option<Value>,
    pub address_transactions_data: Option<Value>,
    // Text editor states for selectable JSON display
    pub transactions_editor: iced::widget::text_editor::Content,
    pub mempool_editor: iced::widget::text_editor::Content,
    pub mempool_tx_editor: iced::widget::text_editor::Content,
    pub address_transactions_editor: iced::widget::text_editor::Content,
    pub wallet_info_editor: iced::widget::text_editor::Content,
    pub wallet_balance_editor: iced::widget::text_editor::Content,
    pub transaction_history_editor: iced::widget::text_editor::Content,
    pub blocks_all_editor: iced::widget::text_editor::Content,
    pub block_by_hash_editor: iced::widget::text_editor::Content,
    pub blockchain_info_editor: iced::widget::text_editor::Content,
    pub latest_blocks_editor: iced::widget::text_editor::Content,
    pub created_wallet_address_editor: iced::widget::text_editor::Content,
}

impl Default for AdminApp {
    fn default() -> Self {
        Self {
            menu: Menu::Blockchain,
            base_url: String::new(),
            api_key: String::new(),
            status: String::new(),
            info: None,
            blocks: Vec::new(),
            block_hash_input: String::new(),
            mining_address_input: String::new(),
            mining_nblocks_input: String::new(),
            mining_maxtries_input: String::new(),
            txid_input: String::new(),
            addr_tx_input: String::new(),
            wallet_label_input: String::new(),
            addresses: Vec::new(),
            wallet_info: None,
            wallet_balance: None,
            created_wallet_address: None,
            send_from_address: String::new(),
            send_to_address: String::new(),
            send_amount: String::new(),
            last_txid: None,
            history_address: String::new(),
            transaction_history: None,
            wallet_section: WalletSection::Create,
            transaction_section: TransactionSection::Mempool,
            blockchain_section: BlockchainSection::Info,
            mining_section: MiningSection::Info,
            health_section: HealthSection::Health,
            blockchain_menu_hovered: false,
            wallet_menu_hovered: false,
            transaction_menu_hovered: false,
            mining_menu_hovered: false,
            health_menu_hovered: false,
            blocks_all_data: None,
            block_by_hash_data: None,
            mining_info_data: None,
            generate_result: None,
            health_data: None,
            liveness_data: None,
            readiness_data: None,
            mempool_data: None,
            mempool_tx_data: None,
            transactions_data: None,
            address_transactions_data: None,
            transactions_editor: iced::widget::text_editor::Content::new(),
            mempool_editor: iced::widget::text_editor::Content::new(),
            mempool_tx_editor: iced::widget::text_editor::Content::new(),
            address_transactions_editor: iced::widget::text_editor::Content::new(),
            wallet_info_editor: iced::widget::text_editor::Content::new(),
            wallet_balance_editor: iced::widget::text_editor::Content::new(),
            transaction_history_editor: iced::widget::text_editor::Content::new(),
            blocks_all_editor: iced::widget::text_editor::Content::new(),
            block_by_hash_editor: iced::widget::text_editor::Content::new(),
            blockchain_info_editor: iced::widget::text_editor::Content::new(),
            latest_blocks_editor: iced::widget::text_editor::Content::new(),
            created_wallet_address_editor: iced::widget::text_editor::Content::new(),
        }
    }
}

impl AdminApp {
    pub fn new() -> (Self, iced::Task<Message>) {
        (
            Self {
                menu: Menu::Blockchain,
                base_url: "http://127.0.0.1:8080".into(),
                api_key: std::env::var("BITCOIN_API_ADMIN_KEY")
                    .unwrap_or_else(|_| "admin-secret".into()),
                status: String::new(),
                info: None,
                blocks: Vec::new(),
                block_hash_input: String::new(),
                mining_address_input: String::new(),
                mining_nblocks_input: String::new(),
                mining_maxtries_input: String::new(),
                txid_input: String::new(),
                addr_tx_input: String::new(),
                wallet_label_input: String::new(),
                addresses: Vec::new(),
                wallet_info: None,
                wallet_balance: None,
                created_wallet_address: None,
                send_from_address: String::new(),
                send_to_address: String::new(),
                send_amount: String::new(),
                last_txid: None,
                history_address: String::new(),
                transaction_history: None,
                wallet_section: WalletSection::Create,
                transaction_section: TransactionSection::Mempool,
                blockchain_section: BlockchainSection::Info,
                mining_section: MiningSection::Info,
                health_section: HealthSection::Health,
                blockchain_menu_hovered: false,
                wallet_menu_hovered: false,
                transaction_menu_hovered: false,
                mining_menu_hovered: false,
                health_menu_hovered: false,
                blocks_all_data: None,
                block_by_hash_data: None,
                mining_info_data: None,
                generate_result: None,
                health_data: None,
                liveness_data: None,
                readiness_data: None,
                mempool_data: None,
                mempool_tx_data: None,
                transactions_data: None,
                address_transactions_data: None,
                transactions_editor: iced::widget::text_editor::Content::new(),
                mempool_editor: iced::widget::text_editor::Content::new(),
                mempool_tx_editor: iced::widget::text_editor::Content::new(),
                address_transactions_editor: iced::widget::text_editor::Content::new(),
                wallet_info_editor: iced::widget::text_editor::Content::new(),
                wallet_balance_editor: iced::widget::text_editor::Content::new(),
                transaction_history_editor: iced::widget::text_editor::Content::new(),
                blocks_all_editor: iced::widget::text_editor::Content::new(),
                block_by_hash_editor: iced::widget::text_editor::Content::new(),
                blockchain_info_editor: iced::widget::text_editor::Content::new(),
                latest_blocks_editor: iced::widget::text_editor::Content::new(),
                created_wallet_address_editor: iced::widget::text_editor::Content::new(),
            },
            iced::Task::none(),
        )
    }

    /// Clear previously loaded data for a specific section
    pub fn clear_related_data(&mut self, section: DataSection) {
        match section {
            DataSection::BlockchainInfo => {
                self.blocks_all_data = None;
                self.block_by_hash_data = None;
            }
            DataSection::Blocks => {
                self.blocks_all_data = None;
                self.block_by_hash_data = None;
            }
            DataSection::BlocksAll => {
                self.block_by_hash_data = None;
            }
            DataSection::BlockByHash => {
                self.blocks_all_data = None;
            }
            DataSection::MiningInfo => {
                self.generate_result = None;
            }
            DataSection::Generate => {
                self.mining_info_data = None;
            }
            DataSection::Health => {
                self.liveness_data = None;
                self.readiness_data = None;
            }
            DataSection::Liveness => {
                self.health_data = None;
                self.readiness_data = None;
            }
            DataSection::Readiness => {
                self.health_data = None;
                self.liveness_data = None;
            }
            DataSection::Mempool => {
                self.mempool_tx_data = None;
                self.transactions_data = None;
                self.address_transactions_data = None;
            }
            DataSection::MempoolTx => {
                self.mempool_data = None;
                self.transactions_data = None;
                self.address_transactions_data = None;
            }
            DataSection::Transactions => {
                self.mempool_data = None;
                self.mempool_tx_data = None;
                self.address_transactions_data = None;
            }
            DataSection::AddressTransactions => {
                self.mempool_data = None;
                self.mempool_tx_data = None;
                self.transactions_data = None;
            }
            DataSection::WalletInfo => {
                self.wallet_balance = None;
            }
            DataSection::WalletBalance => {
                self.wallet_info = None;
            }
        }
    }
}
