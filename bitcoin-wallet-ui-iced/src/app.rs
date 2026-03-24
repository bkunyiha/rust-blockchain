use crate::types::Menu;
use serde_json::Value;

#[derive(Debug)]
pub struct WalletApp {
    pub menu: Menu,
    pub wallet_submenu_open: bool, // Track if Wallet submenu is visible
    pub base_url: String,
    pub api_key: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub status: String,
    pub wallet_label: String, // Label/name for new wallet
    pub new_address: Option<String>,
    pub last_txid: Option<String>,
    // Saved wallet addresses from database
    pub saved_wallets: Vec<crate::database::WalletAddress>,
    // Active/selected wallet address (for sending transactions)
    pub active_wallet_address: Option<String>,
    // Wallet query data
    pub wallet_info_data: Option<Value>,
    pub wallet_balance_data: Option<Value>,
    pub transaction_history_data: Option<Value>,
    // Text editor states for selectable displays
    pub wallet_address_editor: iced::widget::text_editor::Content,
    pub transaction_id_editor: iced::widget::text_editor::Content,
    pub wallet_info_editor: iced::widget::text_editor::Content,
    pub wallet_balance_editor: iced::widget::text_editor::Content,
    pub transaction_history_editor: iced::widget::text_editor::Content,
}

impl Default for WalletApp {
    fn default() -> Self {
        Self {
            menu: Menu::Wallet,
            wallet_submenu_open: false,
            base_url: String::new(),
            api_key: String::new(),
            from: String::new(),
            to: String::new(),
            amount: String::new(),
            status: String::new(),
            wallet_label: String::new(),
            new_address: None,
            last_txid: None,
            saved_wallets: Vec::new(),
            active_wallet_address: None,
            wallet_info_data: None,
            wallet_balance_data: None,
            transaction_history_data: None,
            wallet_address_editor: iced::widget::text_editor::Content::new(),
            transaction_id_editor: iced::widget::text_editor::Content::new(),
            wallet_info_editor: iced::widget::text_editor::Content::new(),
            wallet_balance_editor: iced::widget::text_editor::Content::new(),
            transaction_history_editor: iced::widget::text_editor::Content::new(),
        }
    }
}

impl WalletApp {
    pub fn new() -> (Self, iced::Task<crate::types::Message>) {
        // Load settings from database
        let (base_url, api_key) = match crate::database::load_settings() {
            Ok(settings) => (settings.base_url, settings.api_key),
            Err(_) => {
                // Use defaults if database load fails
                (
                    "http://127.0.0.1:8080".into(),
                    std::env::var("BITCOIN_API_WALLET_KEY")
                        .unwrap_or_else(|_| "wallet-secret".into()),
                )
            }
        };

        // Load saved wallet addresses from database
        let saved_wallets = crate::database::load_wallet_addresses().unwrap_or_else(|e| {
            eprintln!("Failed to load wallet addresses: {}", e);
            Vec::new()
        });

        // Determine active wallet and initial state
        // If exactly one wallet exists, set it as active and populate 'from' field
        // If multiple wallets exist, don't set active (user must select)
        let (active_wallet_address, from_field) = if saved_wallets.len() == 1 {
            let addr = saved_wallets[0].address.clone();
            (Some(addr.clone()), addr)
        } else {
            (None, String::new())
        };

        // Status message
        let status = if saved_wallets.is_empty() {
            String::new()
        } else {
            format!("Loaded {} saved wallet(s)", saved_wallets.len())
        };

        (
            Self {
                menu: Menu::Wallet, // Start with Wallet menu
                wallet_submenu_open: false,
                base_url,
                api_key,
                from: from_field,
                to: String::new(),
                amount: String::new(),
                status,
                wallet_label: String::new(),
                new_address: None,
                last_txid: None,
                saved_wallets,
                active_wallet_address,
                wallet_info_data: None,
                wallet_balance_data: None,
                transaction_history_data: None,
                wallet_address_editor: iced::widget::text_editor::Content::new(),
                transaction_id_editor: iced::widget::text_editor::Content::new(),
                wallet_info_editor: iced::widget::text_editor::Content::new(),
                wallet_balance_editor: iced::widget::text_editor::Content::new(),
                transaction_history_editor: iced::widget::text_editor::Content::new(),
            },
            iced::Task::none(),
        )
    }
}
