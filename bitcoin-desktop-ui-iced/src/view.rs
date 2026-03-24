use crate::app::AdminApp;
use crate::types::{
    BlockchainSection, HealthSection, Menu, Message, MiningSection, TransactionSection,
    WalletSection,
};
use iced::Element;
use iced::Font;
use iced::widget::button::Style as ButtonStyle;
use iced::widget::{
    button, column, container, mouse_area, row, scrollable, text, text_editor, text_input,
};

pub fn view<'a>(app: &'a AdminApp) -> Element<'a, Message> {
    // Helper function to calculate popup width based on longest text
    // Approximate: 6 pixels per character + padding (4px left + 4px right) + container padding (2px * 2)
    let calculate_popup_width = |texts: &[&str]| -> f32 {
        let max_len = texts.iter().map(|s| s.len()).max().unwrap_or(0);
        (max_len as f32 * 6.0) + 8.0 + 4.0 // text width + button padding + container padding
    };

    // Helper function to create blockchain popup items with professional styling
    let create_blockchain_popup_items = || {
        BlockchainSection::ALL
            .iter()
            .map(|&section| {
                let section_label = match section {
                    BlockchainSection::Info => "Get Block Info",
                    BlockchainSection::LatestBlocks => "Latest Blocks",
                    BlockchainSection::AllBlocks => "All Blocks",
                    BlockchainSection::BlockByHash => "Block by Hash",
                };
                // Use regular buttons - they handle clicks directly
                // Button style requires 2 arguments: theme and status
                button(text(section_label).size(13).font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Font::DEFAULT
                })) // Bold text
                .on_press(Message::BlockchainSectionChanged(section))
                .width(iced::Length::Fill)
                .padding([4, 8]) // Reduced padding
                .style(
                    |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                        // Use default button style with transparent background
                        ButtonStyle::default().with_background(iced::Background::Color(
                            iced::Color::WHITE.scale_alpha(0.5),
                        ))
                    },
                )
                .into()
            })
            .collect::<Vec<_>>()
    };

    // Helper function to create wallet popup items with professional styling
    let create_wallet_popup_items = || {
        WalletSection::ALL
            .iter()
            .map(|&section| {
                let section_label = match section {
                    WalletSection::GetWalletInfo => "Get Wallet Info",
                    WalletSection::GetBalance => "Get Balance",
                    WalletSection::Create => "Create Wallet",
                    WalletSection::Send => "Send Bitcoin",
                    WalletSection::History => "Transaction History",
                    WalletSection::Addresses => "All Addresses",
                };
                // Use regular buttons - they handle clicks directly
                button(text(section_label).size(13).font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Font::DEFAULT
                })) // Bold text
                .on_press(Message::WalletSectionChanged(section))
                .width(iced::Length::Fill)
                .padding([4, 8]) // Reduced padding
                .style(
                    |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                        // Use default button style with transparent background
                        ButtonStyle::default().with_background(iced::Background::Color(
                            iced::Color::WHITE.scale_alpha(0.5),
                        ))
                    },
                )
                .into()
            })
            .collect::<Vec<_>>()
    };

    // Helper function to create transaction popup items with professional styling
    let create_transaction_popup_items = || {
        TransactionSection::ALL
            .iter()
            .map(|&section| {
                let section_label = match section {
                    TransactionSection::Mempool => "Mempool",
                    TransactionSection::MempoolTx => "Mempool Transaction",
                    TransactionSection::AllTransactions => "All Transactions",
                    TransactionSection::AddressTransactions => "Address Transactions",
                };
                // Use regular buttons - they handle clicks directly
                button(text(section_label).size(13).font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Font::DEFAULT
                })) // Bold text
                .on_press(Message::TransactionSectionChanged(section))
                .width(iced::Length::Fill)
                .padding([4, 8]) // Reduced padding
                .style(
                    |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                        // Use default button style with transparent background
                        ButtonStyle::default().with_background(iced::Background::Color(
                            iced::Color::WHITE.scale_alpha(0.5),
                        ))
                    },
                )
                .into()
            })
            .collect::<Vec<_>>()
    };

    // Helper function to create mining popup items with professional styling
    let create_mining_popup_items = || {
        MiningSection::ALL
            .iter()
            .map(|&section| {
                let section_label = match section {
                    MiningSection::Info => "Mining Info",
                    MiningSection::Generate => "Generate Blocks",
                };
                // Use regular buttons - they handle clicks directly
                button(text(section_label).size(13).font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Font::DEFAULT
                })) // Bold text
                .on_press(Message::MiningSectionChanged(section))
                .width(iced::Length::Fill)
                .padding([4, 8]) // Reduced padding
                .style(
                    |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                        // Use default button style with transparent background
                        ButtonStyle::default().with_background(iced::Background::Color(
                            iced::Color::WHITE.scale_alpha(0.5),
                        ))
                    },
                )
                .into()
            })
            .collect::<Vec<_>>()
    };

    // Helper function to create health popup items with professional styling
    let create_health_popup_items = || {
        HealthSection::ALL
            .iter()
            .map(|&section| {
                let section_label = match section {
                    HealthSection::Health => "Health Check",
                    HealthSection::Liveness => "Liveness Check",
                    HealthSection::Readiness => "Readiness Check",
                };
                // Use regular buttons - they handle clicks directly
                button(text(section_label).size(13).font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Font::DEFAULT
                })) // Bold text
                .on_press(Message::HealthSectionChanged(section))
                .width(iced::Length::Fill)
                .padding([4, 8]) // Reduced padding
                .style(
                    |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                        // Use default button style with transparent background
                        ButtonStyle::default().with_background(iced::Background::Color(
                            iced::Color::WHITE.scale_alpha(0.5),
                        ))
                    },
                )
                .into()
            })
            .collect::<Vec<_>>()
    };

    // Create menu buttons with Blockchain button having attached popup
    let menu_buttons = row(Menu::ALL
        .iter()
        .map(|&menu_item| {
            let menu_label = match menu_item {
                Menu::Blockchain => "Blockchain",
                Menu::Wallet => "Wallet",
                Menu::Transactions => "Transactions",
                Menu::Mining => "Mining",
                Menu::Health => "Health",
            };

            if menu_item == Menu::Blockchain {
                // Blockchain button with attached popup menu
                let blockchain_button =
                    button(menu_label).on_press(Message::MenuChanged(Menu::Blockchain));

                // Create popup that appears directly below the button
                let popup_content: Element<Message> = if app.blockchain_menu_hovered {
                    // Calculate width based on longest menu text
                    let blockchain_texts = [
                        "Get Block Info",
                        "Latest Blocks",
                        "All Blocks",
                        "Block By Hash",
                    ];
                    let popup_width = calculate_popup_width(&blockchain_texts);
                    // Professional popup menu with dark background, border, and shadow
                    container(
                        column(create_blockchain_popup_items()).spacing(0), // No spacing - buttons handle their own padding
                    )
                    .width(iced::Length::Fixed(popup_width)) // Dynamic width based on longest text
                    .padding(2) // Reduced padding around menu items
                    .style(|_theme| {
                        container::Style {
                            background: None, // No background color - transparent
                            border: iced::Border {
                                radius: 6.0.into(), // Rounded corners
                                width: 1.0,
                                color: iced::Color {
                                    r: 0.35,
                                    g: 0.35,
                                    b: 0.35,
                                    a: 1.0, // Subtle border
                                },
                            },
                            shadow: iced::Shadow {
                                color: iced::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.6, // Dark shadow for depth
                                },
                                offset: iced::Vector::new(0.0, 4.0),
                                blur_radius: 12.0,
                            },
                            ..container::Style {
                                background: None, // Transparent container background
                                ..container::Style::default()
                            }
                        }
                    })
                    .into()
                } else {
                    container(text(""))
                        .height(iced::Length::Fixed(0.0))
                        .width(iced::Length::Fixed(0.0))
                        .into()
                };

                // Wrap button and popup together in a column
                // Use mouse_area to wrap the entire area to maintain hover
                // But ensure it doesn't block button clicks by not using on_press on the wrapper
                mouse_area(
                    container(column![blockchain_button, popup_content,].spacing(0))
                        .width(iced::Length::Shrink), // Container shrinks to fit content
                )
                .on_enter(Message::BlockchainMenuHovered(true))
                .on_exit(Message::BlockchainMenuHovered(false))
                .into()
            } else if menu_item == Menu::Wallet {
                // Wallet button with attached popup menu
                let wallet_button = button(menu_label).on_press(Message::MenuChanged(Menu::Wallet));

                // Create popup that appears directly below the button
                let popup_content: Element<Message> = if app.wallet_menu_hovered {
                    // Calculate width based on longest menu text
                    let wallet_texts = [
                        "Create Wallet",
                        "Send Bitcoin",
                        "Transaction History",
                        "All Addresses",
                        "Query Wallet",
                    ];
                    let popup_width = calculate_popup_width(&wallet_texts);
                    // Professional popup menu with dark background, border, and shadow
                    container(
                        column(create_wallet_popup_items()).spacing(0), // No spacing - buttons handle their own padding
                    )
                    .width(iced::Length::Fixed(popup_width)) // Dynamic width based on longest text
                    .padding(2) // Reduced padding around menu items
                    .style(|_theme| {
                        container::Style {
                            background: None, // No background color - transparent
                            border: iced::Border {
                                radius: 6.0.into(), // Rounded corners
                                width: 1.0,
                                color: iced::Color {
                                    r: 0.35,
                                    g: 0.35,
                                    b: 0.35,
                                    a: 1.0, // Subtle border
                                },
                            },
                            shadow: iced::Shadow {
                                color: iced::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.6, // Dark shadow for depth
                                },
                                offset: iced::Vector::new(0.0, 4.0),
                                blur_radius: 12.0,
                            },
                            ..container::Style {
                                background: None, // Transparent container background
                                ..container::Style::default()
                            }
                        }
                    })
                    .into()
                } else {
                    container(text(""))
                        .height(iced::Length::Fixed(0.0))
                        .width(iced::Length::Fixed(0.0))
                        .into()
                };

                // Wrap button and popup together in a column
                // Use mouse_area to wrap the entire area to maintain hover
                // But ensure it doesn't block button clicks by not using on_press on the wrapper
                mouse_area(
                    container(column![wallet_button, popup_content,].spacing(0))
                        .width(iced::Length::Shrink), // Container shrinks to fit content
                )
                .on_enter(Message::WalletMenuHovered(true))
                .on_exit(Message::WalletMenuHovered(false))
                .into()
            } else if menu_item == Menu::Transactions {
                // Transactions button with attached popup menu
                let transactions_button =
                    button(menu_label).on_press(Message::MenuChanged(Menu::Transactions));

                // Create popup that appears directly below the button
                let popup_content: Element<Message> = if app.transaction_menu_hovered {
                    // Calculate width based on longest menu text
                    let transaction_texts = [
                        "Mempool",
                        "Mempool Transaction",
                        "All Transactions",
                        "Address Transactions",
                    ];
                    let popup_width = calculate_popup_width(&transaction_texts);
                    // Professional popup menu with dark background, border, and shadow
                    container(
                        column(create_transaction_popup_items()).spacing(0), // No spacing - buttons handle their own padding
                    )
                    .width(iced::Length::Fixed(popup_width)) // Dynamic width based on longest text
                    .padding(2) // Reduced padding around menu items
                    .style(|_theme| {
                        container::Style {
                            background: None, // No background color - transparent
                            border: iced::Border {
                                radius: 6.0.into(), // Rounded corners
                                width: 1.0,
                                color: iced::Color {
                                    r: 0.35,
                                    g: 0.35,
                                    b: 0.35,
                                    a: 1.0, // Subtle border
                                },
                            },
                            shadow: iced::Shadow {
                                color: iced::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.6, // Dark shadow for depth
                                },
                                offset: iced::Vector::new(0.0, 4.0),
                                blur_radius: 12.0,
                            },
                            ..container::Style {
                                background: None, // Transparent container background
                                ..container::Style::default()
                            }
                        }
                    })
                    .into()
                } else {
                    container(text(""))
                        .height(iced::Length::Fixed(0.0))
                        .width(iced::Length::Fixed(0.0))
                        .into()
                };

                // Wrap button and popup together in a column
                // Use mouse_area to wrap the entire area to maintain hover
                // But ensure it doesn't block button clicks by not using on_press on the wrapper
                mouse_area(
                    container(column![transactions_button, popup_content,].spacing(0))
                        .width(iced::Length::Shrink), // Container shrinks to fit content
                )
                .on_enter(Message::TransactionMenuHovered(true))
                .on_exit(Message::TransactionMenuHovered(false))
                .into()
            } else if menu_item == Menu::Mining {
                // Mining button with attached popup menu
                let mining_button = button(menu_label).on_press(Message::MenuChanged(Menu::Mining));

                // Create popup that appears directly below the button
                let popup_content: Element<Message> = if app.mining_menu_hovered {
                    // Calculate width based on longest menu text
                    let mining_texts = ["Mining Info", "Generate Blocks"];
                    let popup_width = calculate_popup_width(&mining_texts);
                    // Professional popup menu with dark background, border, and shadow
                    container(
                        column(create_mining_popup_items()).spacing(0), // No spacing - buttons handle their own padding
                    )
                    .width(iced::Length::Fixed(popup_width)) // Dynamic width based on longest text
                    .padding(2) // Reduced padding around menu items
                    .style(|_theme| {
                        container::Style {
                            background: None, // No background color - transparent
                            border: iced::Border {
                                radius: 6.0.into(), // Rounded corners
                                width: 1.0,
                                color: iced::Color {
                                    r: 0.35,
                                    g: 0.35,
                                    b: 0.35,
                                    a: 1.0, // Subtle border
                                },
                            },
                            shadow: iced::Shadow {
                                color: iced::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.6, // Dark shadow for depth
                                },
                                offset: iced::Vector::new(0.0, 4.0),
                                blur_radius: 12.0,
                            },
                            ..container::Style {
                                background: None, // Transparent container background
                                ..container::Style::default()
                            }
                        }
                    })
                    .into()
                } else {
                    container(text(""))
                        .height(iced::Length::Fixed(0.0))
                        .width(iced::Length::Fixed(0.0))
                        .into()
                };

                // Wrap button and popup together in a column
                // Use mouse_area to wrap the entire area to maintain hover
                // But ensure it doesn't block button clicks by not using on_press on the wrapper
                mouse_area(
                    container(column![mining_button, popup_content,].spacing(0))
                        .width(iced::Length::Shrink), // Container shrinks to fit content
                )
                .on_enter(Message::MiningMenuHovered(true))
                .on_exit(Message::MiningMenuHovered(false))
                .into()
            } else if menu_item == Menu::Health {
                // Health button with attached popup menu
                let health_button = button(menu_label).on_press(Message::MenuChanged(Menu::Health));

                // Create popup that appears directly below the button
                let popup_content: Element<Message> = if app.health_menu_hovered {
                    // Calculate width based on longest menu text
                    let health_texts = ["Health Check", "Liveness Check", "Readiness Check"];
                    let popup_width = calculate_popup_width(&health_texts);
                    // Professional popup menu with dark background, border, and shadow
                    container(
                        column(create_health_popup_items()).spacing(0), // No spacing - buttons handle their own padding
                    )
                    .width(iced::Length::Fixed(popup_width)) // Dynamic width based on longest text
                    .padding(2) // Reduced padding around menu items
                    .style(|_theme| {
                        container::Style {
                            background: None, // No background color - transparent
                            border: iced::Border {
                                radius: 6.0.into(), // Rounded corners
                                width: 1.0,
                                color: iced::Color {
                                    r: 0.35,
                                    g: 0.35,
                                    b: 0.35,
                                    a: 1.0, // Subtle border
                                },
                            },
                            shadow: iced::Shadow {
                                color: iced::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.6, // Dark shadow for depth
                                },
                                offset: iced::Vector::new(0.0, 4.0),
                                blur_radius: 12.0,
                            },
                            ..container::Style {
                                background: None, // Transparent container background
                                ..container::Style::default()
                            }
                        }
                    })
                    .into()
                } else {
                    container(text(""))
                        .height(iced::Length::Fixed(0.0))
                        .width(iced::Length::Fixed(0.0))
                        .into()
                };

                // Wrap button and popup together in a column
                // Use mouse_area to wrap the entire area to maintain hover
                // But ensure it doesn't block button clicks by not using on_press on the wrapper
                mouse_area(
                    container(column![health_button, popup_content,].spacing(0))
                        .width(iced::Length::Shrink), // Container shrinks to fit content
                )
                .on_enter(Message::HealthMenuHovered(true))
                .on_exit(Message::HealthMenuHovered(false))
                .into()
            } else {
                button(menu_label)
                    .on_press(Message::MenuChanged(menu_item))
                    .into()
            }
        })
        .collect::<Vec<_>>())
    .spacing(10);

    // Top row: Base URL and Admin API Key inputs
    let top_toolbar = row![
        text_input("Base URL", &app.base_url)
            .on_input(Message::BaseUrlChanged)
            .width(250),
        text_input("Admin API Key", &app.api_key)
            .on_input(Message::ApiKeyChanged)
            .width(250),
    ]
    .spacing(10);

    let section: Element<Message> = match app.menu {
        Menu::Blockchain => view_blockchain(app),
        Menu::Wallet => view_wallet(app),
        Menu::Transactions => view_transactions(app),
        Menu::Mining => view_mining(app),
        Menu::Health => view_health(app),
    };

    column![top_toolbar, menu_buttons, text(&app.status), section,]
        .spacing(12)
        .into()
}

fn view_blockchain<'a>(app: &'a AdminApp) -> Element<'a, Message> {
    // Content section based on selected blockchain section
    let content: Element<Message> = match app.blockchain_section {
        BlockchainSection::Info => column![
            row![button("Refresh Info").on_press(Message::FetchInfo),].spacing(10),
            {
                let info_display: Element<Message> = if let Some(_i) = &app.info {
                    scrollable(
                        container(
                            text_editor(&app.blockchain_info_editor)
                                .on_action(Message::BlockchainInfoEditorAction)
                                .height(iced::Length::Fixed(200.0)),
                        )
                        .padding(8)
                        .style(|_theme: &iced::Theme| container::Style {
                            background: Some(iced::Background::Color(iced::Color {
                                r: 0.95,
                                g: 0.95,
                                b: 0.95,
                                a: 1.0,
                            })),
                            border: iced::Border {
                                radius: 4.0.into(),
                                width: 1.0,
                                color: iced::Color {
                                    r: 0.8,
                                    g: 0.8,
                                    b: 0.8,
                                    a: 1.0,
                                },
                            },
                            ..container::Style::default()
                        })
                        .width(iced::Length::Fill),
                    )
                    .height(iced::Length::Fixed(200.0))
                    .width(iced::Length::Fill)
                    .into()
                } else {
                    text("No info loaded. Click 'Refresh Info' to load.").into()
                };
                info_display
            },
        ]
        .spacing(8)
        .into(),
        BlockchainSection::LatestBlocks => column![
            row![button("Refresh Latest Blocks").on_press(Message::FetchBlocks),].spacing(10),
            {
                let blocks_display: Element<Message> = if app.blocks.is_empty() {
                    text("No blocks loaded. Click 'Refresh Latest Blocks' to load.").into()
                } else {
                    scrollable(
                        container(
                            text_editor(&app.latest_blocks_editor)
                                .on_action(Message::LatestBlocksEditorAction)
                                .height(iced::Length::Fixed(400.0)),
                        )
                        .padding(8)
                        .style(|_theme: &iced::Theme| container::Style {
                            background: Some(iced::Background::Color(iced::Color {
                                r: 0.95,
                                g: 0.95,
                                b: 0.95,
                                a: 1.0,
                            })),
                            border: iced::Border {
                                radius: 4.0.into(),
                                width: 1.0,
                                color: iced::Color {
                                    r: 0.8,
                                    g: 0.8,
                                    b: 0.8,
                                    a: 1.0,
                                },
                            },
                            ..container::Style::default()
                        })
                        .width(iced::Length::Fill),
                    )
                    .height(iced::Length::Fixed(400.0))
                    .width(iced::Length::Fill)
                    .into()
                };
                blocks_display
            },
        ]
        .spacing(8)
        .into(),
        BlockchainSection::AllBlocks => column![
            row![button("Load All Blocks").on_press(Message::FetchBlocksAll),].spacing(10),
            json_data_display(
                &app.blocks_all_data,
                &app.blocks_all_editor,
                Message::BlocksAllEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
        BlockchainSection::BlockByHash => column![
            row![
                text_input("Block Hash", &app.block_hash_input)
                    .on_input(Message::BlockHashChanged)
                    .width(400),
                button("Find by Hash")
                    .on_press(Message::FetchBlockByHash(app.block_hash_input.clone())),
            ]
            .spacing(10),
            json_data_display(
                &app.block_by_hash_data,
                &app.block_by_hash_editor,
                Message::BlockByHashEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
    };

    // Main layout: content display only (selection via popup menu)
    container(content)
        .width(iced::Length::Fill)
        .padding(15)
        .into()
}

fn view_wallet<'a>(app: &'a AdminApp) -> Element<'a, Message> {
    // Content section based on selected wallet section (selection via popup menu)
    let content: Element<Message> = match app.wallet_section {
        WalletSection::GetWalletInfo => column![
            row![
                text_input("Wallet Address", &app.wallet_label_input)
                    .on_input(Message::WalletLabelChanged)
                    .width(250),
                button("Refresh Wallet Info").on_press(Message::FetchWalletInfoAdmin(
                    app.wallet_label_input.clone()
                )),
            ]
            .spacing(10),
            text("Wallet Info").size(14),
            json_data_display(
                &app.wallet_info,
                &app.wallet_info_editor,
                Message::WalletInfoEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
        WalletSection::GetBalance => column![
            row![
                text_input("Wallet Address", &app.wallet_label_input)
                    .on_input(Message::WalletLabelChanged)
                    .width(250),
                button("Refresh Balance")
                    .on_press(Message::FetchBalanceAdmin(app.wallet_label_input.clone())),
            ]
            .spacing(10),
            text("Balance").size(14),
            json_data_display(
                &app.wallet_balance,
                &app.wallet_balance_editor,
                Message::WalletBalanceEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
        WalletSection::Create => column![
            row![
                text_input("Label/Name (optional)", &app.wallet_label_input)
                    .on_input(Message::WalletLabelChanged)
                    .width(250),
                button("Create Wallet").on_press(Message::CreateWalletAdmin),
            ]
            .spacing(10),
            if let Some(created_wallet_address) = &app.created_wallet_address {
                column![
                    row![
                        text("Created Wallet Address:").size(12),
                        button("📋 Copy")
                            .on_press(Message::CopyToClipboard(created_wallet_address.clone())),
                    ]
                    .spacing(8),
                    scrollable(
                        container(
                            text_editor(&app.created_wallet_address_editor)
                                .on_action(Message::CreatedWalletAddressEditorAction)
                                .height(iced::Length::Fixed(60.0))
                        )
                        .padding(8)
                        .style(|_theme| {
                            container::Style {
                                background: Some(iced::Background::Color(iced::Color {
                                    r: 0.95,
                                    g: 0.95,
                                    b: 0.95,
                                    a: 1.0,
                                })),
                                border: iced::Border {
                                    radius: 4.0.into(),
                                    width: 1.0,
                                    color: iced::Color {
                                        r: 0.8,
                                        g: 0.8,
                                        b: 0.8,
                                        a: 1.0,
                                    },
                                },
                                ..container::Style::default()
                            }
                        })
                        .width(iced::Length::Fill)
                    )
                    .height(iced::Length::Fixed(60.0))
                    .width(iced::Length::Fill)
                ]
                .spacing(4)
                .width(iced::Length::Fill)
            } else {
                column![text("")].width(iced::Length::Fill)
            },
        ]
        .spacing(8)
        .into(),
        WalletSection::Send => column![
            text_input("From Address", &app.send_from_address)
                .on_input(Message::SendFromChanged)
                .width(400),
            text_input("To Address", &app.send_to_address)
                .on_input(Message::SendToChanged)
                .width(400),
            row![
                text_input("Amount (satoshis)", &app.send_amount)
                    .on_input(Message::SendAmountChanged)
                    .width(200),
                button("Send Transaction").on_press(Message::SendTx),
            ]
            .spacing(10),
            if let Some(ref txid) = app.last_txid {
                row![
                    text(format!("Last TX: {}", txid)).size(12),
                    button("📋 Copy").on_press(Message::CopyToClipboard(txid.clone())),
                ]
                .spacing(5)
            } else {
                row![text("")]
            },
        ]
        .spacing(8)
        .into(),
        WalletSection::History => column![
            row![
                text_input("Address", &app.history_address)
                    .on_input(Message::HistoryAddressChanged)
                    .width(350),
                button("Load History").on_press(Message::FetchTransactionHistory(
                    app.history_address.clone()
                )),
            ]
            .spacing(10),
            json_data_display(
                &app.transaction_history,
                &app.transaction_history_editor,
                Message::TransactionHistoryEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
        WalletSection::Addresses => {
            column![
                row![
                    text("All Addresses").size(16),
                    button("Fetch All Addresses").on_press(Message::FetchAddressesAdmin),
                ]
                .spacing(10),
                {
                    let addresses_list: Element<Message> = if app.addresses.is_empty() {
                        text("No addresses loaded").into()
                    } else {
                        // Header row
                        let header = container(
                            row![
                                container(text("Address").size(12).font(Font {
                                    weight: iced::font::Weight::Bold,
                                    ..Font::DEFAULT
                                }))
                                .width(iced::Length::Fill)
                                .padding([8, 12])
                                .align_x(iced::Alignment::Start),
                                container(text("Info").size(12).font(Font {
                                    weight: iced::font::Weight::Bold,
                                    ..Font::DEFAULT
                                }))
                                .width(iced::Length::Fixed(80.0))
                                .padding([8, 12])
                                .align_x(iced::Alignment::Center),
                                container(text("Balance").size(12).font(Font {
                                    weight: iced::font::Weight::Bold,
                                    ..Font::DEFAULT
                                }))
                                .width(iced::Length::Fixed(100.0))
                                .padding([8, 12])
                                .align_x(iced::Alignment::Center),
                                container(text("History").size(12).font(Font {
                                    weight: iced::font::Weight::Bold,
                                    ..Font::DEFAULT
                                }))
                                .width(iced::Length::Fixed(100.0))
                                .padding([8, 12])
                                .align_x(iced::Alignment::Center),
                            ]
                            .spacing(0),
                        )
                        .style(|_theme| container::Style {
                            background: Some(iced::Background::Color(iced::Color {
                                r: 0.9,
                                g: 0.9,
                                b: 0.9,
                                a: 1.0,
                            })),
                            border: iced::Border {
                                radius: 0.0.into(),
                                width: 1.0,
                                color: iced::Color {
                                    r: 0.7,
                                    g: 0.7,
                                    b: 0.7,
                                    a: 1.0,
                                },
                            },
                            ..container::Style::default()
                        });

                        // Data rows
                        let rows: Vec<Element<Message>> = app
                            .addresses
                            .iter()
                            .enumerate()
                            .map(|(idx, addr)| -> Element<Message> {
                                let bg_color = if idx % 2 == 0 {
                                    iced::Color {
                                        r: 1.0,
                                        g: 1.0,
                                        b: 1.0,
                                        a: 1.0,
                                    }
                                } else {
                                    iced::Color {
                                        r: 0.98,
                                        g: 0.98,
                                        b: 0.98,
                                        a: 1.0,
                                    }
                                };

                                container(
                                    row![
                                        container(
                                            text(addr)
                                                .size(13)
                                                .font(Font {
                                                    weight: iced::font::Weight::Bold,
                                                    ..Font::DEFAULT
                                                })
                                                .style(|_theme| {
                                                    iced::widget::text::Style {
                                                        color: Some(iced::Color {
                                                            r: 0.1,
                                                            g: 0.1,
                                                            b: 0.1,
                                                            a: 1.0,
                                                        }),
                                                    }
                                                })
                                        )
                                        .width(iced::Length::Fill)
                                        .padding([8, 12])
                                        .align_x(iced::Alignment::Start),
                                        container(
                                            button("Info")
                                                .on_press(Message::FetchWalletInfoAdmin(
                                                    addr.clone()
                                                ))
                                                .padding([4, 8])
                                        )
                                        .width(iced::Length::Fixed(80.0))
                                        .align_x(iced::Alignment::Center),
                                        container(
                                            button("Balance")
                                                .on_press(Message::FetchBalanceAdmin(addr.clone()))
                                                .padding([4, 8])
                                        )
                                        .width(iced::Length::Fixed(100.0))
                                        .align_x(iced::Alignment::Center),
                                        container(
                                            button("History")
                                                .on_press(Message::FetchTransactionHistory(
                                                    addr.clone()
                                                ))
                                                .padding([4, 8])
                                        )
                                        .width(iced::Length::Fixed(100.0))
                                        .align_x(iced::Alignment::Center),
                                    ]
                                    .spacing(0),
                                )
                                .style(move |_theme| container::Style {
                                    background: Some(iced::Background::Color(bg_color)),
                                    border: iced::Border {
                                        radius: 0.0.into(),
                                        width: 0.5,
                                        color: iced::Color {
                                            r: 0.9,
                                            g: 0.9,
                                            b: 0.9,
                                            a: 1.0,
                                        },
                                    },
                                    ..container::Style::default()
                                })
                                .into()
                            })
                            .collect();

                        scrollable(column![header, column(rows).spacing(0)].spacing(0))
                            .height(iced::Length::Fixed(200.0))
                            .into()
                    };
                    addresses_list
                },
                // Display wallet info, balance, and history in a SINGLE scrollable window
                {
                    // Build all sections first, then wrap them ALL in ONE scrollable
                    let mut all_sections: Vec<Element<Message>> = Vec::new();

                    // Add Wallet Info section if available
                    if let Some(info) = &app.wallet_info {
                        let info_str = serde_json::to_string_pretty(info)
                            .unwrap_or_else(|_| "Error formatting".to_string());
                        let info_str_clone = info_str.clone();
                        all_sections.push(
                            column![
                                row![
                                    text("Wallet Info").size(14).font(Font {
                                        weight: iced::font::Weight::Bold,
                                        ..Font::DEFAULT
                                    }),
                                    button("📋 Copy Wallet Info")
                                        .on_press(Message::CopyToClipboard(info_str)),
                                ]
                                .spacing(8),
                                container(
                                    scrollable(
                                        text(info_str_clone)
                                            .size(10)
                                            .font(Font {
                                                weight: iced::font::Weight::Bold,
                                                ..Font::DEFAULT
                                            })
                                            .style(|_theme| {
                                                iced::widget::text::Style {
                                                    color: Some(iced::Color {
                                                        r: 0.1,
                                                        g: 0.1,
                                                        b: 0.1,
                                                        a: 1.0,
                                                    }),
                                                }
                                            })
                                            .width(iced::Length::Fill)
                                    )
                                    .height(iced::Length::Fixed(200.0))
                                    .width(iced::Length::Fill)
                                )
                                .padding(8)
                                .style(|_theme| {
                                    container::Style {
                                        background: Some(iced::Background::Color(iced::Color {
                                            r: 0.95,
                                            g: 0.95,
                                            b: 0.95,
                                            a: 1.0,
                                        })),
                                        border: iced::Border {
                                            radius: 4.0.into(),
                                            width: 1.0,
                                            color: iced::Color {
                                                r: 0.8,
                                                g: 0.8,
                                                b: 0.8,
                                                a: 1.0,
                                            },
                                        },
                                        ..container::Style::default()
                                    }
                                }),
                            ]
                            .spacing(8)
                            .into(),
                        );
                    }

                    // Add Balance section if available
                    if let Some(balance) = &app.wallet_balance {
                        let balance_str = serde_json::to_string_pretty(balance)
                            .unwrap_or_else(|_| "Error formatting".to_string());
                        let balance_str_clone = balance_str.clone();
                        all_sections.push(
                            column![
                                row![
                                    text("Balance").size(14).font(Font {
                                        weight: iced::font::Weight::Bold,
                                        ..Font::DEFAULT
                                    }),
                                    button("📋 Copy Balance")
                                        .on_press(Message::CopyToClipboard(balance_str)),
                                ]
                                .spacing(8),
                                container(
                                    scrollable(
                                        text(balance_str_clone)
                                            .size(10)
                                            .font(Font {
                                                weight: iced::font::Weight::Bold,
                                                ..Font::DEFAULT
                                            })
                                            .style(|_theme| {
                                                iced::widget::text::Style {
                                                    color: Some(iced::Color {
                                                        r: 0.1,
                                                        g: 0.1,
                                                        b: 0.1,
                                                        a: 1.0,
                                                    }),
                                                }
                                            })
                                            .width(iced::Length::Fill)
                                    )
                                    .height(iced::Length::Fixed(200.0))
                                    .width(iced::Length::Fill)
                                )
                                .padding(8)
                                .style(|_theme| {
                                    container::Style {
                                        background: Some(iced::Background::Color(iced::Color {
                                            r: 0.95,
                                            g: 0.95,
                                            b: 0.95,
                                            a: 1.0,
                                        })),
                                        border: iced::Border {
                                            radius: 4.0.into(),
                                            width: 1.0,
                                            color: iced::Color {
                                                r: 0.8,
                                                g: 0.8,
                                                b: 0.8,
                                                a: 1.0,
                                            },
                                        },
                                        ..container::Style::default()
                                    }
                                }),
                            ]
                            .spacing(8)
                            .into(),
                        );
                    }

                    // Add Transaction History section if available - MUST be in same scrollable as Info and Balance
                    if let Some(history) = &app.transaction_history {
                        let history_str = serde_json::to_string_pretty(history)
                            .unwrap_or_else(|_| "Error formatting".to_string());
                        let history_str_clone = history_str.clone();
                        all_sections.push(
                            column![
                                row![
                                    text("Transaction History").size(14).font(Font {
                                        weight: iced::font::Weight::Bold,
                                        ..Font::DEFAULT
                                    }),
                                    button("📋 Copy History")
                                        .on_press(Message::CopyToClipboard(history_str)),
                                ]
                                .spacing(8),
                                container(
                                    scrollable(
                                        text(history_str_clone)
                                            .size(10)
                                            .font(Font {
                                                weight: iced::font::Weight::Bold,
                                                ..Font::DEFAULT
                                            })
                                            .style(|_theme| {
                                                iced::widget::text::Style {
                                                    color: Some(iced::Color {
                                                        r: 0.1,
                                                        g: 0.1,
                                                        b: 0.1,
                                                        a: 1.0,
                                                    }),
                                                }
                                            })
                                            .width(iced::Length::Fill)
                                    )
                                    .height(iced::Length::Fixed(200.0))
                                    .width(iced::Length::Fill)
                                )
                                .padding(8)
                                .style(|_theme| {
                                    container::Style {
                                        background: Some(iced::Background::Color(iced::Color {
                                            r: 0.95,
                                            g: 0.95,
                                            b: 0.95,
                                            a: 1.0,
                                        })),
                                        border: iced::Border {
                                            radius: 4.0.into(),
                                            width: 1.0,
                                            color: iced::Color {
                                                r: 0.8,
                                                g: 0.8,
                                                b: 0.8,
                                                a: 1.0,
                                            },
                                        },
                                        ..container::Style::default()
                                    }
                                }),
                            ]
                            .spacing(8)
                            .into(),
                        );
                    }

                    // Wrap ALL sections in ONE scrollable window - NO nested scrollables
                    // All three sections (Info, Balance, History) MUST be in the SAME scrollable
                    if all_sections.is_empty() {
                        let empty: Element<Message> = text("").into();
                        empty
                    } else {
                        // Create ONE scrollable containing ALL sections together
                        // This ensures Info, Balance, and History all scroll together in one window
                        scrollable(column(all_sections).spacing(12))
                            .height(iced::Length::Fixed(500.0))
                            .width(iced::Length::Fill)
                            .into()
                    }
                },
            ]
            .spacing(8)
            .into()
        }
    };

    // Main layout: content display only (selection via popup menu)
    container(content)
        .width(iced::Length::Fill)
        .padding(15)
        .into()
}

fn view_transactions<'a>(app: &'a AdminApp) -> Element<'a, Message> {
    // Content section based on selected transaction section (selection via popup menu)
    let content: Element<Message> = match app.transaction_section {
        TransactionSection::Mempool => column![
            row![button("Refresh Mempool").on_press(Message::FetchMempool),].spacing(10),
            json_data_display(
                &app.mempool_data,
                &app.mempool_editor,
                Message::MempoolEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
        TransactionSection::MempoolTx => column![
            row![
                text_input("Transaction ID", &app.txid_input)
                    .on_input(Message::TxidChanged)
                    .width(400),
                button("Get Transaction").on_press(Message::FetchMempoolTx(app.txid_input.clone())),
            ]
            .spacing(10),
            json_data_display(
                &app.mempool_tx_data,
                &app.mempool_tx_editor,
                Message::MempoolTxEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
        TransactionSection::AllTransactions => column![
            row![button("Load All Transactions").on_press(Message::FetchTransactions),].spacing(10),
            json_data_display(
                &app.transactions_data,
                &app.transactions_editor,
                Message::TransactionsEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
        TransactionSection::AddressTransactions => column![
            row![
                text_input("Address", &app.addr_tx_input)
                    .on_input(Message::AddrTxChanged)
                    .width(400),
                button("Get Address Transactions")
                    .on_press(Message::FetchAddressTransactions(app.addr_tx_input.clone())),
            ]
            .spacing(10),
            json_data_display(
                &app.address_transactions_data,
                &app.address_transactions_editor,
                Message::AddressTransactionsEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
    };

    // Main layout: content display only (selection via popup menu)
    container(content)
        .width(iced::Length::Fill)
        .padding(15)
        .into()
}

fn view_mining<'a>(app: &'a AdminApp) -> Element<'a, Message> {
    // Content section based on selected mining section (selection via popup menu)
    let content: Element<Message> = match app.mining_section {
        MiningSection::Info => column![
            row![button("Refresh Mining Info").on_press(Message::FetchMiningInfo),].spacing(10),
            text("Mining Info").size(14),
            json_data_display(
                &app.mining_info_data,
                &app.wallet_info_editor,
                Message::WalletInfoEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
        MiningSection::Generate => {
            let addr = text_input("Mining reward address", &app.mining_address_input)
                .on_input(Message::MiningAddressChanged)
                .width(320);
            let nblocks = text_input("Blocks", &app.mining_nblocks_input)
                .on_input(Message::MiningNBlocksChanged)
                .width(100);
            let maxtries = text_input("Max tries (opt)", &app.mining_maxtries_input)
                .on_input(Message::MiningMaxTriesChanged)
                .width(140);
            let parsed_nblocks = app.mining_nblocks_input.trim().parse::<u32>().unwrap_or(1);
            let parsed_maxtries = match app.mining_maxtries_input.trim() {
                "" => None,
                s => s.parse::<u32>().ok(),
            };

            column![
                row![
                    addr,
                    nblocks,
                    maxtries,
                    button("Generate").on_press(Message::GenerateToAddress {
                        address: app.mining_address_input.clone(),
                        nblocks: parsed_nblocks,
                        maxtries: parsed_maxtries
                    }),
                ]
                .spacing(10),
                text("Generate Result").size(14),
                json_data_display(
                    &app.generate_result,
                    &app.wallet_info_editor,
                    Message::WalletInfoEditorAction,
                    400.0
                ),
            ]
            .spacing(8)
            .into()
        }
    };

    // Main layout: content display only (selection via popup menu)
    container(content)
        .width(iced::Length::Fill)
        .padding(15)
        .into()
}

fn view_health<'a>(app: &'a AdminApp) -> Element<'a, Message> {
    // Content section based on selected health section (selection via popup menu)
    let content: Element<Message> = match app.health_section {
        HealthSection::Health => column![
            row![button("Refresh Health Check").on_press(Message::FetchHealth),].spacing(10),
            text("Health Check").size(14),
            json_data_display(
                &app.health_data,
                &app.wallet_info_editor,
                Message::WalletInfoEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
        HealthSection::Liveness => column![
            row![button("Refresh Liveness Check").on_press(Message::FetchLiveness),].spacing(10),
            text("Liveness Check").size(14),
            json_data_display(
                &app.liveness_data,
                &app.wallet_info_editor,
                Message::WalletInfoEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
        HealthSection::Readiness => column![
            row![button("Refresh Readiness Check").on_press(Message::FetchReadiness),].spacing(10),
            text("Readiness Check").size(14),
            json_data_display(
                &app.readiness_data,
                &app.wallet_info_editor,
                Message::WalletInfoEditorAction,
                400.0
            ),
        ]
        .spacing(8)
        .into(),
    };

    // Main layout: content display only (selection via popup menu)
    container(content)
        .width(iced::Length::Fill)
        .padding(15)
        .into()
}

// Helper function to create a scrollable JSON data display with copy functionality
// Uses text_editor for selectable text
fn json_data_display<'a, F>(
    data: &'a Option<serde_json::Value>,
    editor: &'a iced::widget::text_editor::Content,
    on_action: F,
    height: f32,
) -> Element<'a, Message>
where
    F: Fn(iced::widget::text_editor::Action) -> Message + 'a,
{
    if let Some(data) = data {
        let json_string =
            serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into());
        column![
            row![button("📋 Copy").on_press(Message::CopyToClipboard(json_string)),].spacing(8),
            scrollable(
                container(
                    text_editor(editor)
                        .on_action(on_action)
                        .height(iced::Length::Fixed(height))
                )
                .padding(8)
                .style(|_theme| {
                    container::Style {
                        background: Some(iced::Background::Color(iced::Color {
                            r: 0.95,
                            g: 0.95,
                            b: 0.95,
                            a: 1.0,
                        })),
                        border: iced::Border {
                            radius: 4.0.into(),
                            width: 1.0,
                            color: iced::Color {
                                r: 0.8,
                                g: 0.8,
                                b: 0.8,
                                a: 1.0,
                            },
                        },
                        ..container::Style::default()
                    }
                })
                .width(iced::Length::Fill)
            )
            .height(iced::Length::Fixed(height))
            .width(iced::Length::Fill)
        ]
        .spacing(4)
        .width(iced::Length::Fill)
        .into()
    } else {
        text("").into()
    }
}
