# Blockchain Implementation

A blockchain implementation following Bitcoin Core's architecture, written in Rust with modern UI clients.

---

## Table of Contents

- [Quick Start](#quick-start)
- [Deployment Options](#deployment-options)
- [Project Structure](#project-structure)
- [API Clients and Authentication](#api-clients-and-authentication)
- [Development](#development)
- [Documentation](#documentation)

---

## Quick Start

### Prerequisites

- **Rust 1.70+** (`rustup install stable`)
- **Cargo** (comes with Rust)
- **Docker & Docker Compose** (for containerized deployment - optional)

### Building the Workspace

```bash
# Build all workspace members
cargo build --release

# Build specific component
cargo build --release -p bitcoin
cargo build --release -p bitcoin-desktop-ui
cargo build --release -p bitcoin-wallet-ui

# Build the Tauri desktop admin app
cd bitcoin-desktop-ui-tauri
npm install
cargo tauri build

# Build the Tauri wallet app
cd bitcoin-wallet-ui-tauri
npm install
cargo tauri build
```

### Running the Blockchain Node

See the [Bitcoin Implementation README](bitcoin/README.md) for detailed instructions on:
- Creating wallets
- Starting nodes (mining and web server modes)
- Running P2P networks
- Accessing the web API

### Accessing the Web UI

The blockchain node includes a modern React-based web interface:

1. **Build the web UI** (first time only):
   ```bash
   cd bitcoin-web-ui
   npm install
   npm run build
   ```

2. **Start the blockchain server**:
   ```bash
   cargo run --release -p bitcoin
   ```

3. **Access the web UI**:
   - Production: `http://localhost:8080` (served by Rust server)
   - Development: `http://localhost:3000` (Vite dev server, requires running `npm run dev` in `bitcoin-web-ui/`)

4. **Configure API Key** (development mode):
   - Click "Configure API" in the navbar
   - Enter API key: `admin-secret` (default) or your `BITCOIN_API_ADMIN_KEY` value
   - Base URL: `http://127.0.0.1:8080` (default)

For detailed web UI documentation, see [bitcoin-web-ui/README.md](bitcoin-web-ui/README.md).

### Running the Iced Desktop Admin UI

The Iced admin UI is a native Rust desktop application for blockchain administration, mining controls, and system monitoring:

1. **Build and run**:
   ```bash
   cargo run --release -p bitcoin-desktop-ui
   ```

2. **Configure API connection** (in the app):
   - Navigate to the Settings section
   - Enter Base URL: `http://127.0.0.1:8080` (default)
   - Enter Admin API Key: `admin-secret` (default) or your `BITCOIN_API_ADMIN_KEY` value

3. **Features**:
   - Blockchain info, block explorer, and block-by-hash lookup
   - Wallet management (create, view info, check balance, send transactions)
   - Transaction browsing (mempool, all transactions, address transactions)
   - Mining controls (mining info, block generation)
   - Health monitoring (health, liveness, readiness checks)

### Running the Iced Wallet UI

The Iced wallet UI is a native Rust desktop application for personal wallet management with encrypted local storage:

1. **Build and run**:
   ```bash
   cargo run --release -p bitcoin-wallet-ui
   ```

2. **Configure API connection** (in the app):
   - Navigate to the Settings section
   - Enter Base URL: `http://127.0.0.1:8080` (default)
   - Enter Wallet API Key: `wallet-secret` (default) or your `BITCOIN_API_WALLET_KEY` value
   - Settings are saved to the encrypted local database

3. **Features**:
   - Create and manage wallets with optional labels
   - Wallet list with select, copy address, and active wallet tracking
   - Check balances and view transaction history for active wallet
   - Send Bitcoin from active wallet to any address
   - SQLCipher encrypted local database for wallet and settings persistence
   - If only one wallet exists, it is automatically set as active on startup

### Running the Tauri Desktop Admin UI

The Tauri admin UI is a standalone desktop application with a Rust core and React/TypeScript interface:

1. **Install dependencies** (first time only):
   ```bash
   cd bitcoin-desktop-ui-tauri
   npm install
   ```

2. **Run in development mode**:
   ```bash
   cargo tauri dev
   ```

3. **Build for production**:
   ```bash
   cargo tauri build
   ```

4. **Configure API Key**:
   - Click the gear icon in the top bar to open Settings
   - Enter API key: `admin-secret` (default) or your `BITCOIN_API_ADMIN_KEY` value
   - Base URL: `http://127.0.0.1:8080` (default)

### Running the Tauri Wallet UI

The Tauri wallet UI is a standalone desktop application for end-user wallet management, with an encrypted local database for wallet persistence:

1. **Install dependencies** (first time only):
   ```bash
   cd bitcoin-wallet-ui-tauri
   npm install
   ```

2. **Run in development mode**:
   ```bash
   cargo tauri dev
   ```

3. **Build for production**:
   ```bash
   cargo tauri build
   ```

4. **Configure API Key**:
   - Navigate to Settings in the sidebar
   - Enter API key: `wallet-secret` (default) or your `BITCOIN_API_WALLET_KEY` value
   - Base URL: `http://127.0.0.1:8080` (default)
   - Click "Test Connection" to verify, then "Save Settings"

5. **Key Features**:
   - Create and manage multiple wallets with encrypted local storage (SQLCipher)
   - Send Bitcoin with confirmation dialog
   - View balances and transaction history
   - Dark/light theme support
   - Wallet data persists between sessions in an AES-256 encrypted database

---

## Deployment Options

This project supports two deployment methods. All deployment files are organized in the `ci/` directory.

### Docker Compose (Recommended for Development)

**Location**: [`ci/docker-compose/`](ci/docker-compose/)

Ideal for local development, single-host deployments, and quick testing.

**Features:**
- **Multi-instance scaling**: Run multiple miners and webservers with automatic configuration
- **Sequential startup**: Nodes wait for previous nodes to be ready before starting
- **Isolated data**: Each instance maintains its own blockchain data directory
- **Automatic port assignment**: Unique ports for each instance (miners: 2001+, webservers: 8080+/2101+)
- **Health checks**: Built-in health monitoring for reliable startup

**Quick Start:**
```bash
cd ci/docker-compose/configs

# Default: 1 miner + 1 webserver
docker compose up -d

# Scale to multiple instances (all ports accessible)
./docker-compose.scale.sh 3 2  # 3 miners, 2 webservers

# Incremental scaling
./scale-up.sh miner 2
./scale-down.sh webserver 3
```

**Documentation:**
- **Quick Start**: [`ci/docker-compose/README.md`](ci/docker-compose/README.md) - Quick reference guide
### Kubernetes (Recommended for Production)

**Location**: [`ci/kubernetes/`](ci/kubernetes/)

Ideal for production deployments, multi-node clusters, and automatic scaling.

**Features:**
- **Native autoscaling**: HPA (Horizontal Pod Autoscaler) for automatic scaling
- **Service discovery**: DNS-based service discovery
- **Rolling updates**: Zero-downtime deployments
- **Self-healing**: Automatic pod restart on failure
- **Resource management**: CPU/Memory limits and requests
- **Multi-node support**: Distribute across cluster nodes

**Quick Start:**
```bash
cd ci/kubernetes/manifests
kubectl apply -f .
```

**Documentation:**
- **Quick Start**: [`ci/kubernetes/README.md`](ci/kubernetes/README.md) - Quick reference guide

---

## Project Structure

This workspace contains eight main components:

| Component | Description | Documentation |
|-----------|-------------|---------------|
| **`bitcoin/`** | Core blockchain implementation with P2P networking, consensus, and web API | [bitcoin/README.md](bitcoin/README.md) |
| **`bitcoin-api/`** | Shared typed HTTP client library for consuming the blockchain API | See API Clients section |
| **`bitcoin-desktop-ui-iced/`** | Admin UI built with Iced (blockchain management, mining, etc.) | - |
| **`bitcoin-desktop-ui-tauri/`** | Admin UI built with Tauri (Rust core + React/TypeScript UI) | - |
| **`bitcoin-wallet-ui-iced/`** | Wallet UI built with Iced (wallet operations, transactions) | - |
| **`bitcoin-wallet-ui-tauri/`** | Wallet UI built with Tauri (Rust core + React/TypeScript UI, SQLCipher encrypted DB) | - |
| **`bitcoin-web-ui/`** | Modern React-based web admin interface | [bitcoin-web-ui/README.md](bitcoin-web-ui/README.md) |

---

## API Clients and Authentication

The blockchain node exposes a RESTful API that can be consumed by UI clients or other applications.

### Architecture

```
┌─────────────────────────┐     ┌──────────────────────────┐
│ bitcoin-desktop-ui-iced │     │ bitcoin-desktop-ui-tauri │
│       (Admin UI)        │     │       (Admin UI)         │                               Desktop UI's
│       (Iced/Rust)       │     │  (Tauri: Rust + React)   │
└──────────┬──────────────┘     └──────────┬───────────────┘
           │                               │
           │  ┌────────────────────────┐   │  ┌─────────────────────────┐
           │  │ bitcoin-wallet-ui-iced │   │  │ bitcoin-wallet-ui-tauri │
           │  │      (Wallet UI)       │   │  │       (Wallet UI)       │                  Wallet UI's
           │  │      (Iced/Rust)       │   │  │  (Tauri: Rust + React)  │
           │  └──────────┬─────────────┘   │  └──────────┬──────────────┘
           │             │                 │             │
           │             │                 │             │   ┌────────────────────┐     
           │             │                 │             │   │   bitcoin-web-ui   │
           │             │                 │             │   │   (Web Admin UI)   │        Web UI
           │             │                 │             │   │   (React/TS)       │
           │             │                 │             │   └──────────┬─────────┘
           │             │                 │             │              │
           └─────────────┴─────────────────┬─────────────┘──────────────┘
                                           │
                            ┌──────────────▼──────────────┐
                            │      bitcoin-api            │
                            │   (Shared Client Library)   │
                            │   (Rust HTTP Client)        │
                            └──────────────┬──────────────┘
                                           │
                            ┌──────────────▼──────────────┐
                            │   bitcoin (Blockchain Node) │
                            │   http://localhost:8080     │
                            │   REST API + Web UI         │
                            └─────────────────────────────┘
```

**Note**: The `bitcoin-web-ui` uses Axios directly (not `bitcoin-api`) and communicates with the Rust server's REST API endpoints. The Tauri app uses `bitcoin-api` from its Rust core, with a React/TypeScript UI rendered in the OS-native WebView.

### Client Feature Flags

The `bitcoin-api` crate uses feature flags to control which client surfaces are compiled:

- **`client`**: Enables HTTP client support (reqwest + tokio)
- **`wallet`**: Enables `WalletClient` APIs (create wallet, send transaction)
- **`admin`**: Enables `AdminClient` APIs (blockchain, mining, etc.)
- **`ws`**: Reserved for future websocket client support

**Default features**: `client`, `wallet`, `admin`

### UI Dependencies

- **`bitcoin-desktop-ui-iced`**: Requires `bitcoin-api` with features `client, wallet, admin` (Iced/Rust)
- **`bitcoin-desktop-ui-tauri`**: Requires `bitcoin-api` with features `client, wallet, admin` (Tauri: Rust core + React/TypeScript UI)
- **`bitcoin-wallet-ui-iced`**: Requires `bitcoin-api` with features `client, wallet` (Iced/Rust)
- **`bitcoin-wallet-ui-tauri`**: Requires `bitcoin-api` with features `client, wallet, admin` (Tauri: Rust core + React/TypeScript UI + SQLCipher)
- **`bitcoin-web-ui`**: Uses Axios directly, no Rust dependencies (React/TypeScript)

### Server Authentication

The web server enforces role-based access using an `X-API-Key` header:

| Role | Endpoints | Environment Variable | Default Value |
|------|-----------|---------------------|---------------|
| **Wallet** | `/api/wallet/*` | `BITCOIN_API_WALLET_KEY` | `wallet-secret` |
| **Admin** | `/api/admin/*` (also has wallet access) | `BITCOIN_API_ADMIN_KEY` | `admin-secret` |

Configure keys via environment variables before starting the node:

```bash
export BITCOIN_API_WALLET_KEY=your-wallet-key-here
export BITCOIN_API_ADMIN_KEY=your-admin-key-here
```

### Client Usage Examples

#### Admin Client (bitcoin-desktop-ui-iced)

```rust
use bitcoin_api::{AdminClient, ApiConfig};

let admin = AdminClient::new(ApiConfig {
    base_url: "http://127.0.0.1:8080".into(),
    api_key: Some("your-admin-key".into()),
})?;

// Admin operations
let blockchain_info = admin.get_blockchain_info().await?;
admin.start_mining().await?;
```

#### Wallet Client (bitcoin-wallet-ui-iced)

```rust
use bitcoin_api::{WalletClient, ApiConfig};

let wallet = WalletClient::new(ApiConfig {
    base_url: "http://127.0.0.1:8080".into(),
    api_key: Some("your-wallet-key".into()),
})?;

// Wallet operations
let addresses = wallet.list_addresses().await?;
let balance = wallet.get_balance(&address).await?;
wallet.send_transaction(&tx_request).await?;
```

#### Tauri Wallet Client (bitcoin-wallet-ui-tauri)

The Tauri wallet app uses the `bitcoin-api` crate from its Rust core, with wallet data persisted in an encrypted SQLCipher database:

```rust
// src-tauri/src/commands/wallet.rs
#[tauri::command]
async fn create_wallet(
    label: Option<String>,
    state: State<'_, RwLock<AppState>>,
) -> Result<WalletAddress, String> {
    let cfg = state.read().unwrap().api_config();
    let response = BitcoinApiService::create_wallet(cfg, label.clone()).await?;
    let wallet = WalletAddress::new(response.data.unwrap().address, label);
    database::save_wallet_address(&wallet).map_err(|e| e.to_string())
}
```

```typescript
// React frontend calls Rust commands via IPC
import { invoke } from "@tauri-apps/api/core";
const wallet = await invoke("create_wallet", { label: "My Wallet" });
const balance = await invoke("get_balance", { address: wallet.address });
```

#### Tauri Admin Client (bitcoin-desktop-ui-tauri)

The Tauri app uses the same `bitcoin-api` crate from its Rust core, with commands exposed to the React UI via Tauri's IPC system:

```rust
// src-tauri/src/commands/blockchain.rs
#[tauri::command]
async fn get_blockchain_info(
    config: tauri::State<'_, RwLock<ApiConfig>>,
) -> Result<serde_json::Value, String> {
    let cfg = config.read().unwrap();
    let client = AdminClient::new(cfg.to_api_config());
    // ... call bitcoin-api and return result
}
```

```typescript
// React frontend calls Rust commands via IPC
import { invoke } from "@tauri-apps/api/core";
const info = await invoke("get_blockchain_info");
```

#### Web UI (bitcoin-web-ui)

The web UI is a React application that provides a browser-based interface:

**Features:**
- Dashboard with real-time blockchain statistics
- Blockchain management (view blocks, search by hash)
- Wallet operations (create, view info, check balance, send transactions)
- Transaction management (mempool, transaction history)
- Mining controls (view info, generate blocks)
- Health monitoring

**Access:**
- After building (`npm run build`), the web UI is served automatically by the Rust server at `http://localhost:8080`
- For development, run `npm run dev` in `bitcoin-web-ui/` to access at `http://localhost:3000`

**API Configuration:**
- Configure API key via the UI's "Configure API" button in the navbar
- Default admin key: `admin-secret` (or `BITCOIN_API_ADMIN_KEY` env var)
- API key is stored in browser localStorage

See [bitcoin-web-ui/README.md](bitcoin-web-ui/README.md) for detailed setup instructions.

---

## Development

### Running Tests

```bash
# Run all Rust tests
cargo test

# Run tests for specific component
cargo test -p bitcoin
cargo test -p bitcoin-api
cargo test -p bitcoin-wallet-ui-tauri

# Run Tauri wallet UI frontend tests
cd bitcoin-wallet-ui-tauri
npm test
```

### Workspace Commands

```bash
# Format all code
cargo fmt --all

# Lint all code
cargo clippy --all -- -D warnings

# Check all components
cargo check --all
```

---

## License


## Contributing

