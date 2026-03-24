use crate::api;
use crate::app::WalletApp;
use crate::runtime::spawn_on_tokio;
use crate::types::{Menu, Message};
use bitcoin_api::{ApiConfig, CreateWalletRequest};
use iced::Task;

pub fn update(app: &mut WalletApp, message: Message) -> Task<Message> {
    match message {
        Message::MenuChanged(m) => {
            // Clear Send Bitcoin data when navigating away from Send menu
            if app.menu == Menu::Send && m != Menu::Send {
                app.to.clear();
                app.amount.clear();
                app.last_txid = None;
                app.transaction_id_editor = iced::widget::text_editor::Content::new();
            }

            // Set menu and close submenu when selecting any menu item
            // Check if menu requires active wallet
            let requires_wallet = matches!(m, Menu::GetBalance | Menu::Send | Menu::History);
            if requires_wallet && app.active_wallet_address.is_none() {
                // Don't allow navigation to these menus without an active wallet
                app.status = "Please select a wallet first".into();
            } else if m == Menu::WalletInfo && app.saved_wallets.is_empty() {
                // Don't allow navigation to Wallet Info without any wallets
                app.status = "No wallets available. Please create a wallet first.".into();
            } else {
                app.menu = m;
                app.wallet_submenu_open = false; // Close submenu when selecting any menu item

                // Auto-fetch data when navigating to these sections if wallet is selected
                if let Some(address) = &app.active_wallet_address {
                    let cfg = ApiConfig {
                        base_url: app.base_url.clone(),
                        api_key: Some(app.api_key.clone()),
                    };
                    let address_clone = address.clone();

                    match m {
                        Menu::WalletInfo => {
                            // Clear previous data and fetch wallet info
                            app.wallet_info_data = None;
                            app.wallet_info_editor = iced::widget::text_editor::Content::new();
                            return Task::perform(
                                spawn_on_tokio(api::fetch_wallet_info(cfg, address_clone)),
                                Message::WalletInfoLoaded,
                            );
                        }
                        Menu::GetBalance => {
                            // Clear previous data and fetch balance
                            app.wallet_balance_data = None;
                            app.wallet_balance_editor = iced::widget::text_editor::Content::new();
                            return Task::perform(
                                spawn_on_tokio(api::fetch_balance(cfg, address_clone)),
                                Message::BalanceLoaded,
                            );
                        }
                        Menu::History => {
                            // Clear previous data and fetch transaction history
                            app.transaction_history_data = None;
                            app.transaction_history_editor =
                                iced::widget::text_editor::Content::new();
                            return Task::perform(
                                spawn_on_tokio(api::fetch_address_transactions(cfg, address_clone)),
                                Message::TransactionHistoryLoaded,
                            );
                        }
                        _ => {}
                    }
                }
            }
            Task::none()
        }
        Message::WalletSubmenuMouseEnter => {
            // Open submenu on mouse enter
            app.wallet_submenu_open = true;
            Task::none()
        }
        Message::WalletSubmenuMouseExit => {
            // Close submenu on mouse exit
            app.wallet_submenu_open = false;
            Task::none()
        }
        Message::BaseUrlChanged(v) => {
            // Only update the app state, don't save to database yet
            app.base_url = v;
            Task::none()
        }
        Message::ApiKeyChanged(v) => {
            // Only update the app state, don't save to database yet
            app.api_key = v;
            Task::none()
        }
        Message::SaveSettings => {
            // Save settings to database when Save button is clicked
            match crate::database::save_settings(&crate::database::Settings {
                base_url: app.base_url.clone(),
                api_key: app.api_key.clone(),
            }) {
                Ok(()) => {
                    app.status = "Settings saved successfully".into();
                }
                Err(e) => {
                    app.status = format!("Failed to save settings: {}", e);
                    eprintln!("Failed to save settings: {}", e);
                }
            }
            Task::none()
        }
        Message::ToChanged(v) => {
            app.to = v;
            Task::none()
        }
        Message::AmountChanged(v) => {
            app.amount = v;
            Task::none()
        }
        Message::WalletLabelChanged(v) => {
            app.wallet_label = v;
            Task::none()
        }
        Message::CreateWallet => {
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            // Use the label from the form, or None if empty
            let label = if app.wallet_label.trim().is_empty() {
                None
            } else {
                Some(app.wallet_label.trim().to_string())
            };
            let req = CreateWalletRequest { label };
            Task::perform(
                spawn_on_tokio(api::create_wallet(cfg, req)),
                Message::WalletCreated,
            )
        }
        Message::SendTx => {
            // Use active wallet address (required)
            let from_address = if let Some(addr) = &app.active_wallet_address {
                addr.clone()
            } else {
                app.status = "Please select a wallet first".into();
                return Task::none();
            };
            let amount_sat = app.amount.trim().parse::<u64>().unwrap_or(0);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            let req = bitcoin_api::SendTransactionRequest {
                from_address,
                to_address: app.to.clone(),
                amount: amount_sat,
            };
            Task::perform(spawn_on_tokio(api::send_tx(cfg, req)), Message::TxSent)
        }
        Message::WalletCreated(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        if let Some(addr) = api.data.as_ref().map(|d| d.address.clone()) {
                            // Save wallet address to database using WalletAddress struct
                            // Use the label from the form, or None if empty
                            let label = if app.wallet_label.trim().is_empty() {
                                None
                            } else {
                                Some(app.wallet_label.trim().to_string())
                            };
                            let wallet = crate::database::WalletAddress::new(addr.clone(), label);
                            match crate::database::save_wallet_address(&wallet) {
                                Ok(_saved_wallet) => {
                                    // Reload wallet list from database
                                    match crate::database::load_wallet_addresses() {
                                        Ok(wallets) => {
                                            app.saved_wallets = wallets;

                                            // If this is now the only wallet, set it as active and populate 'from'
                                            if app.saved_wallets.len() == 1 {
                                                app.active_wallet_address = Some(addr.clone());
                                                app.from = addr.clone();
                                            }

                                            // Set the new address for display
                                            app.new_address = Some(addr.clone());
                                            // Update the wallet address editor with the new address
                                            app.wallet_address_editor =
                                                iced::widget::text_editor::Content::with_text(
                                                    &addr,
                                                );
                                            // Navigate to Wallet list section to show the newly created wallet
                                            app.menu = Menu::Wallet;
                                            app.wallet_label.clear(); // Clear the label field after successful creation
                                            app.status = "Wallet created successfully".into();
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to reload wallet addresses: {}", e);
                                            app.status =
                                                "Wallet created but failed to reload list".into();
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to save wallet address: {}", e);
                                    app.status =
                                        "Wallet created but failed to save to database".into();
                                }
                            }
                        } else {
                            app.status = "Wallet created but no address returned".into();
                        }
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error creating wallet".into());
                    }
                }
                Err(e) => {
                    app.status = format!("Error: {}", e);
                }
            }
            Task::none()
        }
        Message::SelectWallet(address) => {
            // Set the selected wallet as active and populate 'from' field
            app.active_wallet_address = Some(address.clone());
            app.from = address.clone();
            // Update all wallet address editors with the selected address
            app.wallet_info_editor = iced::widget::text_editor::Content::with_text(&address);
            app.wallet_balance_editor = iced::widget::text_editor::Content::with_text(&address);
            app.transaction_history_editor =
                iced::widget::text_editor::Content::with_text(&address);

            // Automatically fetch wallet info, balance, and transaction history
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            let address_clone = address.clone();

            // Clear previous data
            app.wallet_info_data = None;
            app.wallet_balance_data = None;
            app.transaction_history_data = None;
            app.wallet_info_editor = iced::widget::text_editor::Content::new();
            app.wallet_balance_editor = iced::widget::text_editor::Content::new();
            app.transaction_history_editor = iced::widget::text_editor::Content::new();

            // Fetch all three in parallel
            let wallet_info_task = Task::perform(
                spawn_on_tokio(api::fetch_wallet_info(cfg.clone(), address_clone.clone())),
                Message::WalletInfoLoaded,
            );
            let balance_task = Task::perform(
                spawn_on_tokio(api::fetch_balance(cfg.clone(), address_clone.clone())),
                Message::BalanceLoaded,
            );
            let history_task = Task::perform(
                spawn_on_tokio(api::fetch_address_transactions(cfg, address_clone)),
                Message::TransactionHistoryLoaded,
            );

            app.status = "Wallet selected. Loading wallet data...".into();

            // Return a batch of tasks
            Task::batch(vec![wallet_info_task, balance_task, history_task])
        }
        Message::TxSent(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.last_txid = api.data.map(|d| d.txid.clone());
                        if let Some(txid) = &app.last_txid {
                            app.transaction_id_editor =
                                iced::widget::text_editor::Content::with_text(txid);
                        }
                        app.status = "Transaction sent successfully".into();
                    } else {
                        app.status = api
                            .error
                            .unwrap_or_else(|| "Error sending transaction".into());
                        app.transaction_id_editor = iced::widget::text_editor::Content::new();
                    }
                }
                Err(e) => {
                    app.status = format!("Error: {}", e);
                    app.transaction_id_editor = iced::widget::text_editor::Content::new();
                }
            }
            Task::none()
        }
        Message::CopyToClipboard(text) => {
            let text_clone = text.clone();
            Task::perform(
                async move {
                    // Try to copy to clipboard
                    #[cfg(target_os = "macos")]
                    {
                        use std::process::Command;
                        let mut cmd = Command::new("pbcopy");
                        cmd.stdin(std::process::Stdio::piped());
                        if let Ok(mut child) = cmd.spawn() {
                            if let Some(mut stdin) = child.stdin.take() {
                                use std::io::Write;
                                let _ = stdin.write_all(text_clone.as_bytes());
                            }
                            if child.wait().is_ok() {
                                return true;
                            }
                        }
                    }
                    #[cfg(target_os = "linux")]
                    {
                        use std::process::Command;
                        let mut cmd = Command::new("xclip");
                        cmd.arg("-selection").arg("clipboard");
                        cmd.stdin(std::process::Stdio::piped());
                        if let Ok(mut child) = cmd.spawn() {
                            if let Some(mut stdin) = child.stdin.take() {
                                use std::io::Write;
                                let _ = stdin.write_all(text_clone.as_bytes());
                            }
                            if child.wait().is_ok() {
                                return true;
                            }
                        }
                    }
                    #[cfg(target_os = "windows")]
                    {
                        use std::process::Command;
                        let mut cmd = Command::new("clip");
                        cmd.stdin(std::process::Stdio::piped());
                        if let Ok(mut child) = cmd.spawn() {
                            if let Some(mut stdin) = child.stdin.take() {
                                use std::io::Write;
                                let _ = stdin.write_all(text_clone.as_bytes());
                            }
                            if child.wait().is_ok() {
                                return true;
                            }
                        }
                    }
                    false
                },
                Message::ClipboardCopied,
            )
        }
        Message::ClipboardCopied(success) => {
            if success {
                app.status = "Copied to clipboard!".into();
            } else {
                app.status = "Failed to copy to clipboard".into();
            }
            Task::none()
        }
        Message::WalletAddressEditorAction(action) => {
            app.wallet_address_editor.perform(action);
            Task::none()
        }
        Message::TransactionIdEditorAction(action) => {
            app.transaction_id_editor.perform(action);
            Task::none()
        }
        Message::WalletInfoLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.wallet_info_data = api.data.clone();
                        if let Some(ref data) = api.data {
                            let json_str = serde_json::to_string_pretty(data)
                                .unwrap_or_else(|_| "Error formatting".to_string());
                            app.wallet_info_editor =
                                iced::widget::text_editor::Content::with_text(&json_str);
                        }
                        app.status = "Wallet info loaded successfully".into();
                    } else {
                        app.status = api
                            .error
                            .unwrap_or_else(|| "Error loading wallet info".into());
                        app.wallet_info_data = None;
                        app.wallet_info_editor = iced::widget::text_editor::Content::new();
                    }
                }
                Err(e) => {
                    app.status = format!("Error: {}", e);
                    app.wallet_info_data = None;
                    app.wallet_info_editor = iced::widget::text_editor::Content::new();
                }
            }
            Task::none()
        }
        Message::BalanceLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.wallet_balance_data = api.data.clone();
                        if let Some(ref data) = api.data {
                            let json_str = serde_json::to_string_pretty(data)
                                .unwrap_or_else(|_| "Error formatting".to_string());
                            app.wallet_balance_editor =
                                iced::widget::text_editor::Content::with_text(&json_str);
                        }
                        app.status = "Balance loaded successfully".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading balance".into());
                        app.wallet_balance_data = None;
                        app.wallet_balance_editor = iced::widget::text_editor::Content::new();
                    }
                }
                Err(e) => {
                    app.status = format!("Error: {}", e);
                    app.wallet_balance_data = None;
                    app.wallet_balance_editor = iced::widget::text_editor::Content::new();
                }
            }
            Task::none()
        }
        Message::TransactionHistoryLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.transaction_history_data = api.data.clone();
                        if let Some(ref data) = api.data {
                            let json_str = serde_json::to_string_pretty(data)
                                .unwrap_or_else(|_| "Error formatting".to_string());
                            app.transaction_history_editor =
                                iced::widget::text_editor::Content::with_text(&json_str);
                        }
                        app.status = "Transaction history loaded successfully".into();
                    } else {
                        app.status = api
                            .error
                            .unwrap_or_else(|| "Error loading transaction history".into());
                        app.transaction_history_data = None;
                        app.transaction_history_editor = iced::widget::text_editor::Content::new();
                    }
                }
                Err(e) => {
                    app.status = format!("Error: {}", e);
                    app.transaction_history_data = None;
                    app.transaction_history_editor = iced::widget::text_editor::Content::new();
                }
            }
            Task::none()
        }
        // Text editor actions for new displays
        Message::WalletInfoEditorAction(action) => {
            app.wallet_info_editor.perform(action);
            Task::none()
        }
        Message::WalletBalanceEditorAction(action) => {
            app.wallet_balance_editor.perform(action);
            Task::none()
        }
        Message::TransactionHistoryEditorAction(action) => {
            app.transaction_history_editor.perform(action);
            Task::none()
        }
    }
}
