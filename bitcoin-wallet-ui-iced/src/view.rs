use crate::app::WalletApp;
use crate::types::{Menu, Message};
use iced::Element;
use iced::widget::{
    button, column, container, mouse_area, row, scrollable, text, text_editor, text_input,
};
use serde_json::Value;

pub fn view<'a>(app: &'a WalletApp) -> Element<'a, Message> {
    // Top toolbar with menu and title
    let menu_buttons: Element<Message> = {
        let mut menu_row = row![].spacing(10);

        for &menu_item in Menu::ALL.iter() {
            if menu_item == Menu::Wallet {
                // Check if wallets exist (for submenu logic)
                let has_wallets = !app.saved_wallets.is_empty();

                // Wallet menu with submenu support - submenu appears below on hover
                // Main button is always enabled
                // Use same styling as other menu buttons, but highlight when submenu is open
                let wallet_button = if app.wallet_submenu_open {
                    // Highlighted when submenu is open
                    button(text(menu_item.to_string()))
                        .on_press(Message::MenuChanged(menu_item))
                        .style(
                            |_theme: &iced::Theme, _status| iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color {
                                    r: 0.3,
                                    g: 0.5,
                                    b: 0.7,
                                    a: 1.0,
                                })),
                                text_color: iced::Color::WHITE,
                                border: iced::Border {
                                    radius: 4.0.into(),
                                    width: 1.0,
                                    color: iced::Color {
                                        r: 0.4,
                                        g: 0.4,
                                        b: 0.4,
                                        a: 1.0,
                                    },
                                },
                                ..iced::widget::button::Style::default()
                            },
                        )
                        .padding([8, 16])
                } else {
                    // Default styling when submenu is closed (matches Settings and other buttons)
                    button(text(menu_item.to_string()))
                        .on_press(Message::MenuChanged(menu_item))
                        .padding([8, 16])
                };

                // Create column for Wallet button and submenu
                let mut wallet_column = column![wallet_button].spacing(4);

                // Show submenu below if open (always show, even if no wallets, since Create Wallet is always available)
                if app.wallet_submenu_open {
                    let submenu_items: Vec<Element<Message>> = Menu::Wallet
                        .submenu_items()
                        .iter()
                        .map(|&submenu_item| {
                            // Check if this item should be disabled
                            let is_disabled = submenu_item == Menu::WalletInfo && !has_wallets;

                            if is_disabled {
                                // Greyed out button (no on_press handler)
                                button(text(submenu_item.to_string()))
                                    .style(|_theme: &iced::Theme, _status| {
                                        iced::widget::button::Style {
                                            background: Some(iced::Background::Color(
                                                iced::Color {
                                                    r: 0.15,
                                                    g: 0.15,
                                                    b: 0.15,
                                                    a: 1.0,
                                                },
                                            )),
                                            text_color: iced::Color {
                                                r: 0.4,
                                                g: 0.4,
                                                b: 0.4,
                                                a: 1.0,
                                            },
                                            border: iced::Border {
                                                radius: 4.0.into(),
                                                width: 1.0,
                                                color: iced::Color {
                                                    r: 0.3,
                                                    g: 0.3,
                                                    b: 0.3,
                                                    a: 1.0,
                                                },
                                            },
                                            ..iced::widget::button::Style::default()
                                        }
                                    })
                                    .padding([6, 12])
                                    .width(iced::Length::Fixed(150.0))
                                    .into()
                            } else {
                                // Normal button
                                // Use dynamic text for Create Wallet based on whether wallets exist
                                let button_text: String = if submenu_item == Menu::WalletCreate {
                                    if app.saved_wallets.is_empty() {
                                        "Create New Wallet".to_string()
                                    } else {
                                        "Create Additional Wallet".to_string()
                                    }
                                } else {
                                    submenu_item.to_string()
                                };
                                button(text(button_text))
                                    .on_press(Message::MenuChanged(submenu_item))
                                    .style(|_theme: &iced::Theme, _status| {
                                        iced::widget::button::Style {
                                            background: Some(iced::Background::Color(
                                                iced::Color {
                                                    r: 0.25,
                                                    g: 0.25,
                                                    b: 0.25,
                                                    a: 1.0,
                                                },
                                            )),
                                            text_color: iced::Color::WHITE,
                                            border: iced::Border {
                                                radius: 4.0.into(),
                                                width: 1.0,
                                                color: iced::Color {
                                                    r: 0.4,
                                                    g: 0.4,
                                                    b: 0.4,
                                                    a: 1.0,
                                                },
                                            },
                                            ..iced::widget::button::Style::default()
                                        }
                                    })
                                    .padding([6, 12])
                                    .width(iced::Length::Fixed(150.0))
                                    .into()
                            }
                        })
                        .collect();

                    let submenu = column(submenu_items).spacing(4).padding(4);

                    let submenu_container: Element<Message> = container(submenu)
                        .style(|_theme| container::Style {
                            background: Some(iced::Background::Color(iced::Color {
                                r: 0.15,
                                g: 0.15,
                                b: 0.15,
                                a: 1.0,
                            })),
                            border: iced::Border {
                                radius: 4.0.into(),
                                width: 1.0,
                                color: iced::Color {
                                    r: 0.4,
                                    g: 0.4,
                                    b: 0.4,
                                    a: 1.0,
                                },
                            },
                            ..container::Style::default()
                        })
                        .into();

                    wallet_column = wallet_column.push(submenu_container);
                }

                // Wrap the entire wallet column (button + submenu) in mouse_area for hover detection
                // Use container to ensure proper mouse area detection
                let wallet_column_element: Element<Message> = mouse_area(
                    container(wallet_column).width(iced::Length::Shrink), // Container shrinks to fit content
                )
                .on_enter(Message::WalletSubmenuMouseEnter)
                .on_exit(Message::WalletSubmenuMouseExit)
                .into();

                menu_row = menu_row.push(wallet_column_element);
            } else {
                // Regular menu item
                let menu_label = menu_item.to_string();

                // Check if this menu requires an active wallet
                let requires_wallet =
                    matches!(menu_item, Menu::GetBalance | Menu::Send | Menu::History);
                let has_active_wallet = app.active_wallet_address.is_some();
                let is_disabled = requires_wallet && !has_active_wallet;

                let menu_button: Element<Message> = if is_disabled {
                    // Greyed out button when no wallet is selected
                    button(text(menu_label))
                        .style(
                            |_theme: &iced::Theme, _status| iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color {
                                    r: 0.15,
                                    g: 0.15,
                                    b: 0.15,
                                    a: 1.0,
                                })),
                                text_color: iced::Color {
                                    r: 0.4,
                                    g: 0.4,
                                    b: 0.4,
                                    a: 1.0,
                                },
                                border: iced::Border {
                                    radius: 4.0.into(),
                                    width: 1.0,
                                    color: iced::Color {
                                        r: 0.3,
                                        g: 0.3,
                                        b: 0.3,
                                        a: 1.0,
                                    },
                                },
                                ..iced::widget::button::Style::default()
                            },
                        )
                        .padding([8, 16])
                        .into()
                } else {
                    // Normal button
                    button(text(menu_label))
                        .on_press(Message::MenuChanged(menu_item))
                        .padding([8, 16])
                        .into()
                };

                menu_row = menu_row.push(menu_button);
            }
        }

        menu_row.into()
    };

    // Configuration toolbar
    let config_toolbar = row![
        text_input("Base URL", &app.base_url)
            .on_input(Message::BaseUrlChanged)
            .width(250),
        text_input("Wallet API Key", &app.api_key)
            .on_input(Message::ApiKeyChanged)
            .width(250),
    ]
    .spacing(10);

    // Main content section
    let section: Element<Message> = match app.menu {
        Menu::Wallet => wallet_list_section(app),
        Menu::WalletCreate => create_wallet_section(app),
        Menu::WalletInfo => wallet_info_section(app),
        Menu::GetBalance => get_balance_section(app),
        Menu::Send => send_section(app),
        Menu::History => history_section(app),
        Menu::Settings => settings_section(app),
    };

    // Status bar
    let status_bar: Element<Message> = if !app.status.is_empty() {
        container(text(&app.status).size(14))
            .padding(12)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.2,
                    g: 0.2,
                    b: 0.2,
                    a: 1.0,
                })),
                border: iced::Border {
                    radius: 4.0.into(),
                    width: 1.0,
                    color: iced::Color {
                        r: 0.4,
                        g: 0.4,
                        b: 0.4,
                        a: 1.0,
                    },
                },
                ..container::Style::default()
            })
            .width(iced::Length::Fill)
            .into()
    } else {
        container(text("")).height(iced::Length::Fixed(0.0)).into()
    };

    column![config_toolbar, menu_buttons, section, status_bar]
        .spacing(15)
        .padding(20)
        .into()
}

// Helper function for JSON data display
fn json_data_display<'a, F>(
    data: &'a Option<Value>,
    editor: &'a iced::widget::text_editor::Content,
    on_action: F,
    height: f32,
) -> Element<'a, Message>
where
    F: Fn(iced::widget::text_editor::Action) -> Message + 'a,
{
    if let Some(data) = data {
        let json_string =
            serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".to_string());
        column![
            row![
                button("📋 Copy")
                    .on_press(Message::CopyToClipboard(json_string.clone()))
                    .style(|_theme: &iced::Theme, _status| {
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(iced::Color {
                                r: 0.3,
                                g: 0.3,
                                b: 0.3,
                                a: 1.0,
                            })),
                            text_color: iced::Color {
                                r: 0.9,
                                g: 0.9,
                                b: 0.9,
                                a: 1.0,
                            },
                            border: iced::Border {
                                radius: 4.0.into(),
                                width: 1.0,
                                color: iced::Color {
                                    r: 0.5,
                                    g: 0.5,
                                    b: 0.5,
                                    a: 1.0,
                                },
                            },
                            ..iced::widget::button::Style::default()
                        }
                    })
                    .padding([6, 12]),
            ]
            .spacing(8),
            scrollable(
                container(
                    text_editor(editor)
                        .on_action(on_action)
                        .height(iced::Length::Fixed(height))
                )
                .padding(8)
                .style(|_theme| container::Style {
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
                .width(iced::Length::Fill)
            )
            .height(iced::Length::Fixed(height))
            .width(iced::Length::Fill)
        ]
        .spacing(8)
        .width(iced::Length::Fill)
        .into()
    } else {
        column![text("No data loaded. Click the button above to fetch data.").size(14)]
            .width(iced::Length::Fill)
            .into()
    }
}

// Wallet list section - shows saved wallets
fn wallet_list_section<'a>(app: &'a WalletApp) -> Element<'a, Message> {
    column![
        // Section title
        text("Saved Wallets").size(20),
        if app.saved_wallets.is_empty() {
            column![
                text("No wallet created yet. Use the Wallet menu to create a new wallet.").size(14)
            ]
            .width(iced::Length::Fill)
        } else {
            column![
                text(format!("Saved Wallets ({})", app.saved_wallets.len())).size(16),
                scrollable(
                    column(
                        app.saved_wallets
                            .iter()
                            .map(|wallet| {
                                  let is_active = app.active_wallet_address.as_ref()
                                      .map(|active| active == &wallet.address)
                                      .unwrap_or(false);
                                  let is_active_clone = is_active;

                                  // Wallet card
                                  container(
                                    column![
                                          // Header row with name/label, address and select button
                                          row![
                                              column![
                                                  // Wallet name/label (prominently displayed)
                                                  if let Some(label) = &wallet.label {
                                                      if !label.trim().is_empty() {
                                                          text(label).size(16).style(|_theme| iced::widget::text::Style {
                                                              color: Some(iced::Color {
                                                                  r: 0.9,
                                                                  g: 0.9,
                                                                  b: 1.0,
                                                                  a: 1.0,
                                                              }),
                                                          })
                                                      } else {
                                                          text("Unnamed Wallet").size(16).style(|_theme| iced::widget::text::Style {
                                                              color: Some(iced::Color {
                                                                  r: 0.6,
                                                                  g: 0.6,
                                                                  b: 0.6,
                                                                  a: 1.0,
                                                              }),
                                                          })
                                                      }
                                                  } else {
                                                      text("Unnamed Wallet").size(16).style(|_theme| iced::widget::text::Style {
                                                          color: Some(iced::Color {
                                                              r: 0.6,
                                                              g: 0.6,
                                                              b: 0.6,
                                                              a: 1.0,
                                                          }),
                                                      })
                                                  },
                                                  // Wallet address
                                                  text(&wallet.address).size(12).style(|_theme| iced::widget::text::Style {
                                                      color: Some(iced::Color {
                                                          r: 0.6,
                                                          g: 0.6,
                                                          b: 0.6,
                                                          a: 1.0,
                                                      }),
                                                  }),
                                                  // Creation date
                                                  text(format!("Created: {}", wallet.created_at)).size(11).style(|_theme| iced::widget::text::Style {
                                                      color: Some(iced::Color {
                                                          r: 0.5,
                                                          g: 0.5,
                                                          b: 0.5,
                                                          a: 1.0,
                                                      }),
                                                  }),
                                              ]
                                              .spacing(6)
                                              .width(iced::Length::Fill),
                                            {
                                                let select_button: Element<Message> = button("Select")
                                                    .on_press(Message::SelectWallet(wallet.address.clone()))
                                                    .style(|_theme: &iced::Theme, _status| {
                                                        iced::widget::button::Style {
                                                            background: Some(iced::Background::Color(iced::Color {
                                                                r: 0.3,
                                                                g: 0.5,
                                                                b: 0.8,
                                                                a: 1.0,
                                                            })),
                                                            text_color: iced::Color::WHITE,
                                                            border: iced::Border {
                                                                radius: 4.0.into(),
                                                                width: 1.0,
                                                                color: iced::Color {
                                                                    r: 0.4,
                                                                    g: 0.6,
                                                                    b: 0.9,
                                                                    a: 1.0,
                                                                },
                                                            },
                                                            ..iced::widget::button::Style::default()
                                                        }
                                                    })
                                                    .padding([6, 12])
                                                    .into();

                                                let active_text: Element<Message> = text("✓ Active").size(12).into();

                                                row![
                                                    if is_active { active_text } else { select_button },
                                                button("📋 Copy")
                                                    .on_press(Message::CopyToClipboard(wallet.address.clone()))
                                                    .style(|_theme: &iced::Theme, _status| {
                                                        iced::widget::button::Style {
                                                            background: Some(iced::Background::Color(iced::Color {
                                                                r: 0.3,
                                                                g: 0.3,
                                                                b: 0.3,
                                                                a: 1.0,
                                                            })),
                                                            text_color: iced::Color {
                                                                r: 0.9,
                                                                g: 0.9,
                                                                b: 0.9,
                                                                a: 1.0,
                                                            },
                                                            border: iced::Border {
                                                                radius: 4.0.into(),
                                                                width: 1.0,
                                                                color: iced::Color {
                                                                    r: 0.5,
                                                                    g: 0.5,
                                                                    b: 0.5,
                                                                    a: 1.0,
                                                                },
                                                            },
                                                            ..iced::widget::button::Style::default()
                                                        }
                                                    })
                                                    .padding([6, 12]),
                                                ]
                                                .spacing(8)
                                            }
                                        ]
                                        .spacing(8)
                                        .width(iced::Length::Fill),
                                    ]
                                    .spacing(8)
                                    .width(iced::Length::Fill)
                                )
                                .padding(12)
                                .style(move |_theme: &iced::Theme| container::Style {
                                    background: Some(iced::Background::Color(if is_active_clone {
                                        iced::Color {
                                            r: 0.15,
                                            g: 0.25,
                                            b: 0.4,
                                            a: 1.0,
                                        }
                                    } else {
                                        iced::Color {
                                            r: 0.2,
                                            g: 0.2,
                                            b: 0.2,
                                            a: 1.0,
                                        }
                                    })),
                                      border: iced::Border {
                                          radius: 6.0.into(),
                                          width: if is_active_clone { 2.0 } else { 1.0 },
                                          color: if is_active_clone {
                                            iced::Color {
                                                r: 0.2,
                                                g: 0.6,
                                                b: 0.9,
                                                a: 1.0,
                                            }
                                        } else {
                                            iced::Color {
                                                r: 0.4,
                                                g: 0.4,
                                                b: 0.4,
                                                a: 1.0,
                                            }
                                        },
                                    },
                                    ..container::Style::default()
                                })
                                .width(iced::Length::Fill)
                                .into()
                            })
                            .collect::<Vec<_>>(),
                    )
                    .spacing(10)
                    .width(iced::Length::Fill)
                )
                .height(iced::Length::Fixed(400.0))
                .width(iced::Length::Fill),
            ]
            .spacing(10)
            .width(iced::Length::Fill)
        },
    ]
    .spacing(15)
    .width(iced::Length::Fill)
    .into()
}

// Create wallet section - shows create wallet form
fn create_wallet_section<'a>(app: &'a WalletApp) -> Element<'a, Message> {
    // Determine button text based on whether wallets exist
    let create_button_text = if app.saved_wallets.is_empty() {
        "Create New Wallet"
    } else {
        "Create Additional Wallet"
    };

    container(
        column![
            // Section title
            text("Create Wallet")
                .size(24)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(iced::Color::WHITE),
                }),
            // Form container
            container(
                column![
                    // Wallet name/label input
                    column![
                        text("Wallet Name (Optional)").size(14).style(|_theme| {
                            iced::widget::text::Style {
                                color: Some(iced::Color {
                                    r: 0.8,
                                    g: 0.8,
                                    b: 0.8,
                                    a: 1.0,
                                }),
                            }
                        }),
                        text_input(
                            "Enter a name for this wallet (e.g., 'My Savings', 'Trading Wallet')",
                            &app.wallet_label
                        )
                        .on_input(Message::WalletLabelChanged)
                        .width(400)
                        .padding(12)
                        .size(14),
                    ]
                    .spacing(8),
                    // Create wallet button
                    row![
                        button(create_button_text)
                            .on_press(Message::CreateWallet)
                            .style(|_theme: &iced::Theme, _status| {
                                iced::widget::button::Style {
                                    background: Some(iced::Background::Color(iced::Color {
                                        r: 0.2,
                                        g: 0.6,
                                        b: 0.9,
                                        a: 1.0,
                                    })),
                                    text_color: iced::Color::WHITE,
                                    border: iced::Border {
                                        radius: 6.0.into(),
                                        width: 1.0,
                                        color: iced::Color {
                                            r: 0.3,
                                            g: 0.7,
                                            b: 1.0,
                                            a: 1.0,
                                        },
                                    },
                                    ..iced::widget::button::Style::default()
                                }
                            })
                            .padding([12, 24])
                            .width(iced::Length::Fixed(200.0)),
                    ]
                    .spacing(10),
                ]
                .spacing(20)
                .width(iced::Length::Fill)
            )
            .padding(24)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.15,
                    g: 0.15,
                    b: 0.15,
                    a: 1.0,
                })),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: iced::Color {
                        r: 0.3,
                        g: 0.3,
                        b: 0.3,
                        a: 1.0,
                    },
                },
                ..container::Style::default()
            })
            .width(iced::Length::Fill),
            // Show created wallet address if one was just created
            if let Some(new_address) = &app.new_address {
                let address_display: Element<Message> = container(
                    column![
                        row![
                            text("Created Wallet Address:").size(14).style(|_theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color::WHITE),
                                }
                            }),
                            button("📋 Copy")
                                .on_press(Message::CopyToClipboard(new_address.clone()))
                                .style(|_theme: &iced::Theme, _status| {
                                    iced::widget::button::Style {
                                        background: Some(iced::Background::Color(iced::Color {
                                            r: 0.3,
                                            g: 0.3,
                                            b: 0.3,
                                            a: 1.0,
                                        })),
                                        text_color: iced::Color {
                                            r: 0.9,
                                            g: 0.9,
                                            b: 0.9,
                                            a: 1.0,
                                        },
                                        border: iced::Border {
                                            radius: 4.0.into(),
                                            width: 1.0,
                                            color: iced::Color {
                                                r: 0.5,
                                                g: 0.5,
                                                b: 0.5,
                                                a: 1.0,
                                            },
                                        },
                                        ..iced::widget::button::Style::default()
                                    }
                                })
                                .padding([6, 12]),
                        ]
                        .spacing(10),
                        scrollable(
                            container(
                                text_editor(&app.wallet_address_editor)
                                    .on_action(Message::WalletAddressEditorAction)
                                    .height(iced::Length::Fixed(80.0))
                            )
                            .padding(12)
                            .style(|_theme: &iced::Theme| container::Style {
                                background: Some(iced::Background::Color(iced::Color {
                                    r: 0.95,
                                    g: 0.95,
                                    b: 0.95,
                                    a: 1.0,
                                })),
                                border: iced::Border {
                                    radius: 6.0.into(),
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
                            .width(iced::Length::Fill)
                        )
                        .height(iced::Length::Fixed(80.0))
                        .width(iced::Length::Fill)
                    ]
                    .spacing(8)
                    .width(iced::Length::Fill),
                )
                .padding(16)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(iced::Color {
                        r: 0.12,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    })),
                    border: iced::Border {
                        radius: 8.0.into(),
                        width: 1.0,
                        color: iced::Color {
                            r: 0.3,
                            g: 0.5,
                            b: 0.7,
                            a: 1.0,
                        },
                    },
                    ..container::Style::default()
                })
                .width(iced::Length::Fill)
                .into();
                address_display
            } else {
                container(text("")).width(iced::Length::Fill).into()
            },
        ]
        .spacing(20)
        .width(iced::Length::Fill),
    )
    .padding(20)
    .width(iced::Length::Fill)
    .into()
}

fn wallet_info_section<'a>(app: &'a WalletApp) -> Element<'a, Message> {
    // Get active wallet info
    let (active_address, active_label) = if let Some(addr) = &app.active_wallet_address {
        let wallet = app.saved_wallets.iter().find(|w| &w.address == addr);
        let label = wallet.and_then(|w| w.label.as_ref());
        (Some(addr.clone()), label.cloned())
    } else {
        (None, None)
    };

    column![
        // Section title
        text("Get Wallet Information").size(20),
        // Active Wallet Display Section
        if let Some(address) = &active_address {
            let address_clone = address.clone();
            container(
                column![
                    // Wallet label if available (prominently displayed)
                    if let Some(label) = &active_label {
                        if !label.trim().is_empty() {
                            text(label.clone()).size(20).style(|_theme: &iced::Theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color {
                                        r: 0.9,
                                        g: 0.9,
                                        b: 1.0,
                                        a: 1.0,
                                    }),
                                }
                            })
                        } else {
                            text("Unnamed Wallet")
                                .size(18)
                                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                                    color: Some(iced::Color {
                                        r: 0.6,
                                        g: 0.6,
                                        b: 0.6,
                                        a: 1.0,
                                    }),
                                })
                        }
                    } else {
                        text("Unnamed Wallet")
                            .size(18)
                            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                                color: Some(iced::Color {
                                    r: 0.6,
                                    g: 0.6,
                                    b: 0.6,
                                    a: 1.0,
                                }),
                            })
                    },
                    // Wallet address (read-only)
                    column![
                        text("Wallet Address:").size(12),
                        container(text(address_clone).size(12))
                            .padding(12)
                            .style(|_theme: &iced::Theme| container::Style {
                                background: Some(iced::Background::Color(iced::Color {
                                    r: 0.2,
                                    g: 0.2,
                                    b: 0.2,
                                    a: 1.0,
                                })),
                                border: iced::Border {
                                    radius: 6.0.into(),
                                    width: 1.0,
                                    color: iced::Color {
                                        r: 0.4,
                                        g: 0.4,
                                        b: 0.4,
                                        a: 1.0,
                                    },
                                },
                                ..container::Style::default()
                            })
                            .width(iced::Length::Fill)
                    ]
                    .spacing(6),
                ]
                .spacing(10)
                .width(iced::Length::Fill),
            )
            .padding(16)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.15,
                    g: 0.15,
                    b: 0.15,
                    a: 1.0,
                })),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: iced::Color {
                        r: 0.3,
                        g: 0.3,
                        b: 0.3,
                        a: 1.0,
                    },
                },
                ..container::Style::default()
            })
            .width(iced::Length::Fill)
            .into()
        } else {
            let no_wallet_msg: Element<Message> = container(
                text("No wallet selected. Please select a wallet from the Wallet menu.")
                    .size(14)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color {
                            r: 0.8,
                            g: 0.6,
                            b: 0.2,
                            a: 1.0,
                        }),
                    }),
            )
            .padding(16)
            .width(iced::Length::Fill)
            .into();
            no_wallet_msg
        },
        // Wallet Info Display
        if app.wallet_info_data.is_some() {
            column![
                text("Wallet Info:").size(14),
                json_data_display(
                    &app.wallet_info_data,
                    &app.wallet_info_editor,
                    Message::WalletInfoEditorAction,
                    300.0
                )
            ]
            .spacing(8)
            .width(iced::Length::Fill)
            .into()
        } else {
            let no_info: Element<Message> = column![text("Loading wallet information...").size(14)]
                .width(iced::Length::Fill)
                .into();
            no_info
        },
    ]
    .spacing(15)
    .width(iced::Length::Fill)
    .into()
}

fn get_balance_section<'a>(app: &'a WalletApp) -> Element<'a, Message> {
    // Get active wallet info
    let (active_address, active_label) = if let Some(addr) = &app.active_wallet_address {
        let wallet = app.saved_wallets.iter().find(|w| &w.address == addr);
        let label = wallet.and_then(|w| w.label.as_ref());
        (Some(addr.clone()), label.cloned())
    } else {
        (None, None)
    };

    column![
        // Section title
        text("Get Balance").size(20),
        // Active Wallet Display Section
        if let Some(address) = &active_address {
            let address_clone = address.clone();
            container(
                column![
                    // Wallet label if available (prominently displayed)
                    if let Some(label) = &active_label {
                        if !label.trim().is_empty() {
                            text(label.clone()).size(20).style(|_theme: &iced::Theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color {
                                        r: 0.9,
                                        g: 0.9,
                                        b: 1.0,
                                        a: 1.0,
                                    }),
                                }
                            })
                        } else {
                            text("Unnamed Wallet")
                                .size(18)
                                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                                    color: Some(iced::Color {
                                        r: 0.6,
                                        g: 0.6,
                                        b: 0.6,
                                        a: 1.0,
                                    }),
                                })
                        }
                    } else {
                        text("Unnamed Wallet")
                            .size(18)
                            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                                color: Some(iced::Color {
                                    r: 0.6,
                                    g: 0.6,
                                    b: 0.6,
                                    a: 1.0,
                                }),
                            })
                    },
                    // Wallet address (read-only)
                    column![
                        text("Wallet Address:").size(12),
                        container(text(address_clone).size(12))
                            .padding(12)
                            .style(|_theme: &iced::Theme| container::Style {
                                background: Some(iced::Background::Color(iced::Color {
                                    r: 0.2,
                                    g: 0.2,
                                    b: 0.2,
                                    a: 1.0,
                                })),
                                border: iced::Border {
                                    radius: 6.0.into(),
                                    width: 1.0,
                                    color: iced::Color {
                                        r: 0.4,
                                        g: 0.4,
                                        b: 0.4,
                                        a: 1.0,
                                    },
                                },
                                ..container::Style::default()
                            })
                            .width(iced::Length::Fill)
                    ]
                    .spacing(6),
                ]
                .spacing(10)
                .width(iced::Length::Fill),
            )
            .padding(16)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.15,
                    g: 0.15,
                    b: 0.15,
                    a: 1.0,
                })),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: iced::Color {
                        r: 0.3,
                        g: 0.3,
                        b: 0.3,
                        a: 1.0,
                    },
                },
                ..container::Style::default()
            })
            .width(iced::Length::Fill)
            .into()
        } else {
            let no_wallet_msg: Element<Message> = container(
                text("No wallet selected. Please select a wallet from the Wallet menu.")
                    .size(14)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color {
                            r: 0.8,
                            g: 0.6,
                            b: 0.2,
                            a: 1.0,
                        }),
                    }),
            )
            .padding(16)
            .width(iced::Length::Fill)
            .into();
            no_wallet_msg
        },
        // Balance Display
        if app.wallet_balance_data.is_some() {
            column![
                text("Balance:").size(14),
                json_data_display(
                    &app.wallet_balance_data,
                    &app.wallet_balance_editor,
                    Message::WalletBalanceEditorAction,
                    300.0
                )
            ]
            .spacing(8)
            .width(iced::Length::Fill)
            .into()
        } else {
            let no_balance: Element<Message> = column![text("Loading balance...").size(14)]
                .width(iced::Length::Fill)
                .into();
            no_balance
        },
    ]
    .spacing(15)
    .width(iced::Length::Fill)
    .into()
}

fn send_section<'a>(app: &'a WalletApp) -> Element<'a, Message> {
    // Get active wallet info
    let (active_address, active_label) = if let Some(addr) = &app.active_wallet_address {
        let wallet = app.saved_wallets.iter().find(|w| &w.address == addr);
        let label = wallet.and_then(|w| w.label.as_ref());
        (Some(addr.clone()), label.cloned())
    } else {
        (None, None)
    };

    column![
        // Section title
        text("Send Bitcoin").size(20),
        // Active Wallet Display Section (From Address)
        if let Some(address) = active_address {
            container(
                column![
                    // Wallet label if available (prominently displayed)
                    if let Some(label) = &active_label {
                        if !label.trim().is_empty() {
                            text(label.clone()).size(20).style(|_theme: &iced::Theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color {
                                        r: 0.9,
                                        g: 0.9,
                                        b: 1.0,
                                        a: 1.0,
                                    }),
                                }
                            })
                        } else {
                            text("Unnamed Wallet")
                                .size(18)
                                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                                    color: Some(iced::Color {
                                        r: 0.6,
                                        g: 0.6,
                                        b: 0.6,
                                        a: 1.0,
                                    }),
                                })
                        }
                    } else {
                        text("Unnamed Wallet")
                            .size(18)
                            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                                color: Some(iced::Color {
                                    r: 0.6,
                                    g: 0.6,
                                    b: 0.6,
                                    a: 1.0,
                                }),
                            })
                    },
                    // From Address (read-only)
                    column![
                        text("From Address:").size(12),
                        container(text(address.clone()).size(12))
                            .padding(12)
                            .style(|_theme: &iced::Theme| container::Style {
                                background: Some(iced::Background::Color(iced::Color {
                                    r: 0.2,
                                    g: 0.2,
                                    b: 0.2,
                                    a: 1.0,
                                })),
                                border: iced::Border {
                                    radius: 6.0.into(),
                                    width: 1.0,
                                    color: iced::Color {
                                        r: 0.4,
                                        g: 0.4,
                                        b: 0.4,
                                        a: 1.0,
                                    },
                                },
                                ..container::Style::default()
                            })
                            .width(iced::Length::Fill),
                    ]
                    .spacing(6),
                ]
                .spacing(10)
                .width(iced::Length::Fill),
            )
            .padding(16)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.15,
                    g: 0.15,
                    b: 0.15,
                    a: 1.0,
                })),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: iced::Color {
                        r: 0.3,
                        g: 0.3,
                        b: 0.3,
                        a: 1.0,
                    },
                },
                ..container::Style::default()
            })
            .width(iced::Length::Fill)
            .into()
        } else {
            let no_wallet_msg: Element<Message> = container(
                text("No wallet selected. Please select a wallet from the Wallet menu.")
                    .size(14)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color {
                            r: 0.8,
                            g: 0.6,
                            b: 0.2,
                            a: 1.0,
                        }),
                    }),
            )
            .padding(16)
            .width(iced::Length::Fill)
            .into();
            no_wallet_msg
        },
        // Form inputs
        column![
            text_input("To Address", &app.to)
                .on_input(Message::ToChanged)
                .width(400)
                .padding(10),
            row![
                text_input("Amount (satoshis)", &app.amount)
                    .on_input(Message::AmountChanged)
                    .width(200)
                    .padding(10),
                button("Send Transaction")
                    .on_press(Message::SendTx)
                    .style(|_theme: &iced::Theme, _status| {
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(iced::Color {
                                r: 0.2,
                                g: 0.7,
                                b: 0.3,
                                a: 1.0,
                            })),
                            text_color: iced::Color::WHITE,
                            border: iced::Border {
                                radius: 6.0.into(),
                                width: 1.0,
                                color: iced::Color {
                                    r: 0.3,
                                    g: 0.8,
                                    b: 0.4,
                                    a: 1.0,
                                },
                            },
                            ..iced::widget::button::Style::default()
                        }
                    })
                    .padding([10, 20]),
            ]
            .spacing(10),
        ]
        .spacing(12),
        // Transaction ID display
        if let Some(last_txid) = &app.last_txid {
            column![
                row![
                    text("Transaction ID:").size(14),
                    button("📋 Copy")
                        .on_press(Message::CopyToClipboard(last_txid.clone()))
                        .style(|_theme: &iced::Theme, _status| {
                            iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color {
                                    r: 0.3,
                                    g: 0.3,
                                    b: 0.3,
                                    a: 1.0,
                                })),
                                text_color: iced::Color {
                                    r: 0.9,
                                    g: 0.9,
                                    b: 0.9,
                                    a: 1.0,
                                },
                                border: iced::Border {
                                    radius: 4.0.into(),
                                    width: 1.0,
                                    color: iced::Color {
                                        r: 0.5,
                                        g: 0.5,
                                        b: 0.5,
                                        a: 1.0,
                                    },
                                },
                                ..iced::widget::button::Style::default()
                            }
                        })
                        .padding([6, 12]),
                ]
                .spacing(10),
                scrollable(
                    container(
                        text_editor(&app.transaction_id_editor)
                            .on_action(Message::TransactionIdEditorAction)
                            .height(iced::Length::Fixed(80.0))
                    )
                    .padding(12)
                    .style(|_theme| container::Style {
                        background: Some(iced::Background::Color(iced::Color {
                            r: 0.95,
                            g: 0.95,
                            b: 0.95,
                            a: 1.0,
                        })),
                        border: iced::Border {
                            radius: 6.0.into(),
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
                    .width(iced::Length::Fill)
                )
                .height(iced::Length::Fixed(80.0))
                .width(iced::Length::Fill)
            ]
            .spacing(8)
            .width(iced::Length::Fill)
        } else {
            column![text("No transaction sent yet.").size(14)].width(iced::Length::Fill)
        },
    ]
    .spacing(15)
    .width(iced::Length::Fill)
    .into()
}

fn history_section<'a>(app: &'a WalletApp) -> Element<'a, Message> {
    // Get active wallet info
    let (active_address, active_label) = if let Some(addr) = &app.active_wallet_address {
        let wallet = app.saved_wallets.iter().find(|w| &w.address == addr);
        let label = wallet.and_then(|w| w.label.as_ref());
        (Some(addr.clone()), label.cloned())
    } else {
        (None, None)
    };

    column![
        // Section title
        text("Transaction History").size(20),
        // Active Wallet Display Section
        if let Some(address) = &active_address {
            let address_clone = address.clone();
            container(
                column![
                    // Wallet label if available (prominently displayed)
                    if let Some(label) = &active_label {
                        if !label.trim().is_empty() {
                            text(label.clone()).size(20).style(|_theme: &iced::Theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color {
                                        r: 0.9,
                                        g: 0.9,
                                        b: 1.0,
                                        a: 1.0,
                                    }),
                                }
                            })
                        } else {
                            text("Unnamed Wallet")
                                .size(18)
                                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                                    color: Some(iced::Color {
                                        r: 0.6,
                                        g: 0.6,
                                        b: 0.6,
                                        a: 1.0,
                                    }),
                                })
                        }
                    } else {
                        text("Unnamed Wallet")
                            .size(18)
                            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                                color: Some(iced::Color {
                                    r: 0.6,
                                    g: 0.6,
                                    b: 0.6,
                                    a: 1.0,
                                }),
                            })
                    },
                    // Wallet address (read-only)
                    column![
                        text("Wallet Address:").size(12),
                        container(text(address_clone).size(12))
                            .padding(12)
                            .style(|_theme: &iced::Theme| container::Style {
                                background: Some(iced::Background::Color(iced::Color {
                                    r: 0.2,
                                    g: 0.2,
                                    b: 0.2,
                                    a: 1.0,
                                })),
                                border: iced::Border {
                                    radius: 6.0.into(),
                                    width: 1.0,
                                    color: iced::Color {
                                        r: 0.4,
                                        g: 0.4,
                                        b: 0.4,
                                        a: 1.0,
                                    },
                                },
                                ..container::Style::default()
                            })
                            .width(iced::Length::Fill)
                    ]
                    .spacing(6),
                ]
                .spacing(10)
                .width(iced::Length::Fill),
            )
            .padding(16)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.15,
                    g: 0.15,
                    b: 0.15,
                    a: 1.0,
                })),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: iced::Color {
                        r: 0.3,
                        g: 0.3,
                        b: 0.3,
                        a: 1.0,
                    },
                },
                ..container::Style::default()
            })
            .width(iced::Length::Fill)
            .into()
        } else {
            let no_wallet_msg: Element<Message> = container(
                text("No wallet selected. Please select a wallet from the Wallet menu.")
                    .size(14)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color {
                            r: 0.8,
                            g: 0.6,
                            b: 0.2,
                            a: 1.0,
                        }),
                    }),
            )
            .padding(16)
            .width(iced::Length::Fill)
            .into();
            no_wallet_msg
        },
        // Transaction History Display
        if app.transaction_history_data.is_some() {
            column![
                text("Transaction History:").size(14),
                json_data_display(
                    &app.transaction_history_data,
                    &app.transaction_history_editor,
                    Message::TransactionHistoryEditorAction,
                    400.0
                )
            ]
            .spacing(8)
            .width(iced::Length::Fill)
            .into()
        } else {
            let no_history: Element<Message> =
                column![text("Loading transaction history...").size(14)]
                    .width(iced::Length::Fill)
                    .into();
            no_history
        },
    ]
    .spacing(15)
    .width(iced::Length::Fill)
    .into()
}

fn settings_section<'a>(app: &'a WalletApp) -> Element<'a, Message> {
    column![
        text("Settings").size(20),
        container(
            column![
                text("API Configuration").size(16),
                text_input("Base URL", &app.base_url)
                    .on_input(Message::BaseUrlChanged)
                    .width(400)
                    .padding(10),
                text_input("Wallet API Key", &app.api_key)
                    .on_input(Message::ApiKeyChanged)
                    .width(400)
                    .padding(10),
                // Save Settings button
                row![
                    button("Save Settings")
                        .on_press(Message::SaveSettings)
                        .style(|_theme: &iced::Theme, _status| {
                            iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color {
                                    r: 0.2,
                                    g: 0.6,
                                    b: 0.9,
                                    a: 1.0,
                                })),
                                text_color: iced::Color::WHITE,
                                border: iced::Border {
                                    radius: 6.0.into(),
                                    width: 1.0,
                                    color: iced::Color {
                                        r: 0.3,
                                        g: 0.7,
                                        b: 1.0,
                                        a: 1.0,
                                    },
                                },
                                ..iced::widget::button::Style::default()
                            }
                        })
                        .padding([10, 20]),
                ]
                .spacing(10),
            ]
            .spacing(15)
        )
        .padding(20)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(iced::Color {
                r: 0.15,
                g: 0.15,
                b: 0.15,
                a: 1.0,
            })),
            border: iced::Border {
                radius: 6.0.into(),
                width: 1.0,
                color: iced::Color {
                    r: 0.3,
                    g: 0.3,
                    b: 0.3,
                    a: 1.0,
                },
            },
            ..container::Style::default()
        })
        .width(iced::Length::Fill),
    ]
    .spacing(15)
    .width(iced::Length::Fill)
    .into()
}
