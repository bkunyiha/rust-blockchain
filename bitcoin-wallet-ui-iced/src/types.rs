#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Wallet,
    WalletCreate,
    WalletInfo,
    GetBalance,
    Send,
    History,
    Settings,
}

impl Menu {
    pub const ALL: [Menu; 5] = [
        Menu::Wallet,
        Menu::GetBalance,
        Menu::Send,
        Menu::History,
        Menu::Settings,
    ];

    pub fn submenu_items(&self) -> Vec<Menu> {
        match self {
            Menu::Wallet => vec![Menu::WalletCreate, Menu::WalletInfo],
            _ => vec![],
        }
    }
}

impl core::fmt::Display for Menu {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Menu::Wallet => "Wallet",
            Menu::WalletCreate => "Create Wallet",
            Menu::WalletInfo => "Get Wallet Info",
            Menu::GetBalance => "Get Balance",
            Menu::Send => "Send",
            Menu::History => "History",
            Menu::Settings => "Settings",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    MenuChanged(Menu),
    WalletSubmenuMouseEnter, // Mouse entered Wallet button area
    WalletSubmenuMouseExit,  // Mouse exited Wallet button/submenu area
    BaseUrlChanged(String),
    ApiKeyChanged(String),
    SaveSettings, // Save settings to database
    ToChanged(String),
    AmountChanged(String),
    WalletLabelChanged(String), // Label/name for new wallet
    CreateWallet,
    SelectWallet(String), // Wallet address to select as active
    SendTx,
    WalletCreated(Result<bitcoin_api::ApiResponse<bitcoin_api::CreateWalletResponse>, String>),
    TxSent(Result<bitcoin_api::ApiResponse<bitcoin_api::SendTransactionResponse>, String>),
    // Wallet query messages
    WalletInfoLoaded(Result<bitcoin_api::ApiResponse<serde_json::Value>, String>),
    // Balance query messages
    BalanceLoaded(Result<bitcoin_api::ApiResponse<serde_json::Value>, String>),
    // Transaction history messages
    TransactionHistoryLoaded(Result<bitcoin_api::ApiResponse<serde_json::Value>, String>),
    // Clipboard messages
    CopyToClipboard(String),
    ClipboardCopied(bool), // true = success, false = failed
    // Text editor actions for selectable displays
    WalletAddressEditorAction(iced::widget::text_editor::Action),
    TransactionIdEditorAction(iced::widget::text_editor::Action),
    WalletInfoEditorAction(iced::widget::text_editor::Action),
    WalletBalanceEditorAction(iced::widget::text_editor::Action),
    TransactionHistoryEditorAction(iced::widget::text_editor::Action),
}
