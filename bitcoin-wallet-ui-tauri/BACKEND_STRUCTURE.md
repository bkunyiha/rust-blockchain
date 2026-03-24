# Bitcoin Wallet UI Tauri - Rust Backend

Complete Rust backend for the Tauri-based Bitcoin Wallet application.

## Project Structure

```
bitcoin-wallet-ui-tauri/
└── src-tauri/
    ├── Cargo.toml                 # Project manifest with dependencies
    ├── build.rs                   # Tauri build script
    ├── tauri.conf.json            # Tauri configuration
    ├── lib.rs                      # Library entry point
    ├── capabilities/
    │   └── main.json              # Window capabilities and permissions
    └── src/
        ├── main.rs                # Binary entry point
        ├── lib.rs                 # Library exports
        ├── database/
        │   └── mod.rs             # SQLCipher database module
        ├── models/
        │   └── mod.rs             # Data models (SendTxResult, ConnectionStatus)
        ├── config/
        │   └── mod.rs             # AppState and configuration
        ├── commands/
        │   ├── mod.rs             # Commands module
        │   ├── wallet.rs          # Wallet operations commands
        │   ├── settings.rs        # Settings commands
        │   └── health.rs          # Health check commands
        └── services/
            ├── mod.rs             # Services module
            └── bitcoin_api.rs     # Bitcoin API client service
```

## Key Features

### Database (src/database/mod.rs)
- SQLCipher encryption with secure password generation
- Tables: settings, wallet_addresses, users, schema_version
- Schema versioning and migrations
- CRUD operations for wallets and settings

### Models (src/models/mod.rs)
- `SendTxResult`: Transaction response with TXID
- `ConnectionStatus`: API connection status

### Configuration (src/config/mod.rs)
- `AppState`: Manages active wallet, API endpoint, and credentials
- Loads settings from database on startup

### Services (src/services/bitcoin_api.rs)
- `BitcoinApiService`: Bitcoin API client wrapper
- Methods: create_wallet, get_wallet_info, get_balance, send_transaction, etc.
- Async operations with error handling

### Commands (src/commands/)

#### Wallet Commands
- `create_wallet(label)`: Create new Bitcoin wallet
- `get_saved_wallets()`: List all saved wallets
- `set_active_wallet(address)`: Set active wallet
- `delete_wallet_address(address)`: Delete wallet
- `update_wallet_label(address, label)`: Update wallet label
- `get_wallet_info(address)`: Get wallet details
- `get_balance(address)`: Get wallet balance
- `send_transaction(from, to, amount)`: Send Bitcoin transaction
- `get_tx_history(address)`: Get transaction history

#### Settings Commands
- `get_settings()`: Load API settings
- `save_settings(base_url, api_key)`: Save API configuration
- `check_connection()`: Test API connectivity

#### Health Commands
- `health_check()`: Check backend health status

## Security Features

1. **Database Encryption**: SQLCipher with deterministic password generation
   - Uses username, home directory, and app name for password generation
   - Same password generation as bitcoin-wallet-ui-iced

2. **API Key Management**: Environment variable or database storage
   - Configurable via `BITCOIN_API_WALLET_KEY` environment variable
   - Persisted in encrypted database

3. **Tauri Security**:
   - Content Security Policy (CSP)
   - Capability-based permissions
   - Plugin access controls

## Dependencies

- **tauri 2**: Desktop framework
- **tokio**: Async runtime
- **rusqlite + sqlcipher**: Database and encryption
- **serde/serde_json**: Serialization
- **bitcoin-api**: Bitcoin API client
- **dirs**: Platform-specific directory handling
- **tracing**: Logging and diagnostics

## Environment Variables

- `USER` or `USERNAME`: For database password generation
- `BITCOIN_API_WALLET_KEY`: API key (default: "wallet-secret")

## Database Password Generation

The database password is generated using a hash of:
1. System username (USER or USERNAME env var)
2. Home directory path
3. Application name ("bitcoin-wallet-ui")

This ensures consistent password across sessions for the same user.

## Building

```bash
cd src-tauri
cargo build --release
```

## Frontend Integration

The Tauri app expects the frontend to be built to `../dist/` directory.
Configure via `tauri.conf.json`:
- Development: serves from `http://localhost:5173`
- Production: uses built files from `../dist/`
