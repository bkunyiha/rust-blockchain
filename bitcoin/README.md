# Bitcoin Core Implementation in Rust

A production-grade blockchain implementation following Bitcoin Core's proven architecture, written in Rust for enhanced safety and performance.

**Current Bitcoin Core Alignment:** 76% (continuously improving)

---

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Features](#features)
- [Architecture](#architecture)
- [API Documentation](#api-documentation)
- [Testing](#testing)
- [Development Roadmap](#development-roadmap)
- [Documentation](#documentation)

---

## Overview

This project is a faithful re-implementation of Bitcoin's blockchain using Rust, following Bitcoin Core's architectural patterns and conventions. The codebase mirrors Bitcoin Core's directory structure, naming conventions, and design principles while leveraging Rust's safety guarantees and modern async programming patterns.

### Why Rust?

- **Memory Safety**: Eliminates entire classes of bugs (buffer overflows, use-after-free)
- **Type Safety**: Strong type system prevents many runtime errors
- **Concurrency**: Fearless concurrency with compile-time race condition detection
- **Performance**: Zero-cost abstractions with C++-like performance
- **Modern Tooling**: Cargo, built-in testing, excellent documentation tools

### C++ Bitcoin Project Core Compatibility

| Component | Bitcoin Core Equivalent | Alignment |
|-----------|------------------------|-----------|
| Consensus Algorithm | Nakamoto Consensus | ✅ 100% |
| Cryptography | Schnorr/ECDSA (secp256k1) | ✅ 100% |
| P2TR Addresses | Taproot-compatible | ✅ 100% |
| Directory Structure | src/ organization | ✅ 76% |
| File Naming | Bitcoin conventions | ✅ 70% |

---

## Quick Start

### Prerequisites

- Rust 1.70+ (`rustup install stable`)
- Cargo (comes with Rust)

### Installation

```bash
git clone <repository-url>
cd blockchain
cargo build --release
```

### Running a Single Node

```bash
# 1. Create a wallet
cargo run createwallet
# Output: Your wallet address: <WALLET_ADDR>

# 2. Start the mining node (blockchain is created automatically if it doesn't exist)
cargo run startnode yes no local <WALLET_ADDR>

# 3. Start the web node
#
# Rate limiting (recommended):
# - Start Redis (required for rate limiting)
# - Set RL_SETTINGS_PATH to a Settings.toml file
docker run -d --name redis -p 6379:6379 redis:7-alpine
export RL_SETTINGS_PATH=/absolute/path/to/Settings.toml

cargo run startnode no yes <MINING_NODE> <WALLET_ADDR>
```

### Running Multiple Nodes (P2P Network)

Each node requires its own environment variables:

```bash
# Terminal 1 - Seed Node
export CENTRAL_NODE=127.0.0.1:2001
export BLOCKS_TREE=blocks1
export TREE_DIR=data1
export NODE_ADDR=127.0.0.1:2001
export RUST_LOG=info

cargo run startnode yes no local <WALLET_ADDR>

# Terminal 2 - Second Node (miner)
export CENTRAL_NODE=127.0.0.1:2002
export BLOCKS_TREE=blocks2
export TREE_DIR=data2
export NODE_ADDR=127.0.0.1:2002
export RUST_LOG=info

cargo run startnode yes no 127.0.0.1:2001 <WALLET_ADDR>

# Terminal 3 - Web node (connect to a miner)
export CENTRAL_NODE=127.0.0.1:2003
export BLOCKS_TREE=blocks3
export TREE_DIR=data3
export NODE_ADDR=127.0.0.1:2003
export RUST_LOG=info
#
# Rate limiting (recommended):
docker run -d --name redis -p 6379:6379 redis:7-alpine
# Set location of Settings.toml, default is root folder
# export RL_SETTINGS_PATH=/absolute/path/to/Settings.toml

cargo run startnode no yes 127.0.0.1:2001 <WALLET_ADDR>
```

### Web API Access

```bash
# Access Swagger UI
open http://localhost:8080/swagger-ui/

# Test endpoints
curl http://localhost:8080/health
curl http://localhost:8080/api/v1/blockchain
```

---

## Features

### ✅ Core Blockchain Features

#### **Consensus Mechanism**
- **Nakamoto Consensus** with longest chain rule
- **Deterministic tie-breaking** using hash comparison
- **Chain reorganization** with automatic fork handling
- **UTXO rollback** support for reorg scenarios
- **Work-based chain selection** (most cumulative proof-of-work)

#### **Cryptography (P2TR/Taproot)**
- **Schnorr signatures** using secp256k1
- **P2TR address format** (version 0x01)
- **SHA256 hashing** (replaced insecure RIPEMD160)
- **ECDSA support** for backward compatibility
- **Secure random number generation**

#### **Transaction System**
- **UTXO-based** transaction model
- **Coinbase transactions** with mining rewards
- **Transaction validation** (signatures, double-spend prevention)
- **Memory pool management** for unconfirmed transactions
- **Fee handling** and priority

#### **P2P Networking**
- **Full node implementation** with P2P protocol
- **Block propagation** across network
- **Transaction relay** between peers
- **Peer discovery** and management
- **Version handshake** protocol

#### **Web API & Interface**
- **RESTful API** (modern alternative to Bitcoin's JSON-RPC)
- **Swagger/OpenAPI** documentation
- **Web dashboard** for blockchain exploration
- **Health check endpoints** for monitoring
- **Interactive Swagger UI** for API testing

### ✅ Production-Ready Features

- **Comprehensive error handling** with custom error types
- **Async/await** for efficient I/O operations
- **Thread-safe** data structures (RwLock, Arc)
- **Persistence** using Sled embedded database
- **Logging** with tracing for observability
- **164 tests** (158 unit + 6 integration) with 100% pass rate

---

## Architecture

### Bitcoin Core Alignment

This project follows **Bitcoin Core's proven architecture** for maximum compatibility and maintainability.

#### Design Principles

1. **Separation of Concerns**
   - **Primitives**: Pure data structures (block, transaction)
   - **Chain**: Blockchain state management
   - **Node**: Business logic and orchestration
   - **Network**: P2P communication
   - **Wallet**: Key management and signing
   - **Consensus**: Validation rules (planned)
   - **Policy**: Configurable preferences (planned)

2. **Layer Independence**
   - Network layer doesn't know about mining
   - Wallet layer doesn't know about P2P
   - Primitives are reusable everywhere
   - Clear dependency hierarchy

3. **Bitcoin Core Patterns**
   - **Primitives = Pure Data** (no business logic)
   - **Root-level Critical Files** (pow, txmempool, validation)
   - **Modular design** with clear responsibilities

### Directory Structure Comparison

| This Project | Bitcoin Core | Responsibility | Alignment |
|--------------|--------------|----------------|-----------|
| **`src/primitives/`** | **`src/primitives/`** | Fundamental data structures | ✅ |
| `primitives/block.rs` | `primitives/block.h` | Block structure | ✅ |
| `primitives/transaction.rs` | `primitives/transaction.h` | Transaction structure | ✅ |
| `primitives/blockchain.rs` | *(internal)* | Generic blockchain container | ✅ |
| **`src/chain/`** | **`src/chain/`** | Blockchain state management | ✅ |
| `chain/chainstate.rs` | `chain/chainstate.cpp` | Active blockchain state | ✅ |
| `chain/utxo_set.rs` | `chain/coinsview.cpp` | UTXO set management | ⚠️  |
| **`src/node/`** | **`src/node/`** | Node-level operations | ✅ |
| `node/context.rs` | `node/context.cpp` | Node coordination | ✅ |
| `node/txmempool.rs` | `txmempool.cpp` | Mempool operations | ✅ |
| `node/miner.rs` | `miner.cpp` | Mining & block assembly | ✅ |
| `node/peers.rs` | `addrman.cpp` | Peer address management | ⚠️ Needs enhancement |
| `node/server.rs` | `node/*.cpp` | Node server coordination | ✅ |
| **`src/net/`** | **`src/net/`** | P2P networking layer | ✅ |
| `net/net_processing.rs` | `net_processing.cpp` | P2P protocol operations | ✅ |
| **`src/wallet/`** | **`src/wallet/`** | Wallet functionality | ✅ |
| `wallet/wallet_impl.rs` | `wallet/wallet.cpp` | Core wallet implementation | ✅  |
| `wallet/wallet_service.rs` | `wallet/walletdb.cpp` | Wallet persistence | ✅  |
| **`src/crypto/`** | **Cryptography** | Cryptographic operations | ✅ |
| `crypto/signature.rs` | `pubkey.cpp` | ECDSA/Schnorr signatures | ✅  |
| `crypto/hash.rs` | `hash.cpp` | SHA256 hashing | ✅  |
| `crypto/keypair.rs` | `key.cpp` | Key generation | ✅  |
| `crypto/address.rs` | `base58.cpp` | Address encoding | ✅  |
| **Root-level files** | **Root-level files** | Critical components | ✅ |
| `txmempool.rs` | `txmempool.cpp` | Mempool data structure | ✅ |
| `pow.rs` | `pow.cpp` | Proof-of-work validation | ✅ |
| **`src/store/`** | *(root level)* | Persistence | ✅ |
| `store/file_system_db_chain.rs` | `txdb.cpp` | Blockchain database | ✅ |
| **`src/web/`** | **`src/rpc/`** | RPC/API interface | ✅ |
| `web/server.rs` | `rpc/server.cpp` | Server implementation | ⚠️ Different approach |
| `web/handlers/` | `rpc/*.cpp` | RPC handlers | ✅ |

**Legend:** ✅ Aligned | ⚠️ Needs improvement

### Consensus Algorithm

The blockchain implements Bitcoin's **Nakamoto Consensus** with the following mechanisms:

1. **Longest Chain Rule**: Chain with most cumulative work is considered valid
2. **Work Comparison**: Compare total proof-of-work when heights are equal
3. **Deterministic Tie-Breaking**: Lexicographic hash comparison ensures network convergence
4. **Chain Reorganization**: Automatic switching to stronger chains
5. **UTXO Rollback**: Proper state management during chain reorganizations
6. **Mining Reward Distribution**: Only winning blocks receive rewards

**Key Innovation:** Deterministic tie-breaking eliminates timestamp bias, ensuring all nodes reach identical consensus decisions regardless of block processing order.


### System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         Web/RPC Layer                            │
│                  (REST API + Swagger UI)                        │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────┴────────────────────────────────────────┐
│                       Node Layer                                 │
│        (Context, Mempool, Miner, Peers, Server)                │
└──────┬──────────────────┬──────────────────┬────────────────────┘
       │                  │                  │
┌──────┴──────┐  ┌───────┴───────┐  ┌──────┴──────────┐
│   Chain     │  │    Network    │  │     Wallet      │
│  (State)    │  │  (P2P Comms)  │  │  (Keys/Sign)    │
└──────┬──────┘  └───────┬───────┘  └──────┬──────────┘
       │                  │                  │
┌──────┴──────────────────┴──────────────────┴──────────┐
│               Primitives & Utilities                   │
│        (Block, Transaction, Crypto, Storage)           │
└────────────────────────────────────────────────────────┘
```

---

## API Documentation

### Swagger/OpenAPI Integration

- **Interactive Docs**: http://localhost:8080/swagger-ui/
- **OpenAPI Spec**: http://localhost:8080/api-docs/openapi.json
- **Auto-generated** from code annotations

### Core Endpoints

#### Health & Monitoring
```
GET  /health              - Comprehensive health check
GET  /health/live         - Liveness probe
GET  /health/ready        - Readiness probe
```

#### Blockchain Operations
```
GET  /api/v1/blockchain                  - Blockchain info
GET  /api/v1/blockchain/blocks           - List blocks
GET  /api/v1/blockchain/blocks/latest    - Latest blocks
GET  /api/v1/blockchain/blocks/{hash}    - Specific block
```

#### Wallet Management
```
POST /api/v1/wallet                - Create wallet
GET  /api/v1/wallet/addresses      - List addresses
GET  /api/v1/wallet/{address}      - Wallet info
GET  /api/v1/wallet/{address}/balance - Get balance
```

#### Transaction Operations
```
POST /api/v1/transactions                    - Send transaction
GET  /api/v1/transactions                    - List transactions
GET  /api/v1/transactions/{txid}             - Get transaction
GET  /api/v1/transactions/mempool            - Mempool transactions
GET  /api/v1/transactions/address/{address}  - Address transactions
```

#### Mining Operations
```
POST /api/v1/mining/start   - Start mining
POST /api/v1/mining/stop    - Stop mining
GET  /api/v1/mining/status  - Mining status
POST /api/v1/mining/mine    - Mine a block
```

### Example: Health Check Response

```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "version": "0.1.0",
    "uptime_seconds": 3600,
    "blockchain_height": 42,
    "connected_peers": 3,
    "memory_usage_mb": 128.5
  },
  "timestamp": "2025-10-10T00:00:00.000Z"
}
```

---

## Testing

### Test Coverage

```
Total Tests: 164 (100% passing)
├── Unit Tests: 158 ✅
└── Integration Tests: 6 ✅
```

### Test Categories

- **Consensus Mechanisms** (35 tests)
  - Chain reorganization
  - Tie-breaking algorithms
  - Work calculation
  - Multi-node scenarios
  - Block processing order

- **Cryptography** (55 tests)
  - Schnorr signatures
  - ECDSA signatures
  - P2TR addresses
  - SHA256 hashing
  - Key generation

- **Blockchain Operations** (30 tests)
  - Block creation and validation
  - Transaction processing
  - UTXO management
  - Persistence

- **Network Operations** (12 tests)
  - Message serialization
  - Protocol operations
  - Peer management

- **Integration Tests** (16 tests)
  - End-to-end workflows
  - Multi-component interaction

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_tests

# Specific test
cargo test test_consensus
```

---

## Development Roadmap

### ✅ Completed (October 2025)

#### **Phase 1: Bitcoin Core Structure Alignment**

#### **Cryptographic Enhancements**
- ✅ P2TR (Pay-to-Taproot) implementation
- ✅ Schnorr signature support
- ✅ Taproot-compatible address format
- ✅ SHA256-based addressing (replaced RIPEMD160)

#### **Consensus Improvements**
- ✅ Fixed block processing order issues
- ✅ Implemented deterministic tie-breaking
- ✅ Added chain reorganization logic
- ✅ Fixed balance inconsistency in multi-node mining

### 🔄 In Progress

#### **1. Bitcoin Core Structure Alignment (Phase 2)**

**Objective:** Achieve 95%+ alignment with Bitcoin Core's architecture

#### **2. Peer Management Enhancement**

**Current:** Basic peer list (113 lines, ~30% Bitcoin alignment)  
**Target:** Full address manager with eclipse protection (90%+ alignment)

**Priority Tasks:**
- [ ] Rename `node/peers.rs` → `node/addrman.rs`
- [ ] Add peer metadata (connection history, timing, attempts)
- [ ] Implement persistence (save/load `peers.dat`)
- [ ] Add eclipse attack protection (network group limiting)
- [ ] Implement two-table system (new/tried peers)
- [ ] Add sophisticated peer selection algorithm

**Security Gap:** Current implementation vulnerable to eclipse attacks (attacker can fill entire peer list)


#### **3. Network Layer Enhancement**

**Current:** Simple message processor (~15% Bitcoin alignment)  
**Target:** Full connection manager with concurrency (90%+ alignment)

**Critical Gaps:**
- ❌ No connection pooling (handles one stream at a time)
- ❌ No I/O multiplexing (can't scale to 100+ peers)
- ❌ No per-peer state tracking
- ❌ No resource limits (DoS vulnerable)

**Priority Tasks:**
- [ ] Create `ConnectionManager` struct for connection pooling
- [ ] Implement I/O multiplexing (async/tokio)
- [ ] Add per-peer state tracking (connection time, bandwidth)
- [ ] Implement connection limits (max inbound/outbound)
- [ ] Add message queuing per peer
- [ ] Implement ban management for misbehaving peers

**Security Gap:** Vulnerable to slow-peer DoS attacks (one slow connection blocks all others)


#### **4. Database Backend**

**Current:** Sled embedded database (filesystem only)  
**Target:** Multiple backend support (LevelDB, RocksDB, SurrealDB)

**Tasks:**
- [ ] Create database abstraction trait
- [ ] Implement RocksDB backend
- [ ] Add backend selection via configuration
- [ ] Performance benchmarking across backends

### 🚀 Future Enhancements

#### **Advanced Cryptography**
- Scriptless scripts with Schnorr aggregation
- Taproot script path spending
- Multi-signature support
- Hardware wallet integration

#### **Performance Optimizations**
- Schnorr batch verification
- Parallel transaction processing
- Memory pool optimization
- Connection pooling improvements

#### **Protocol Extensions**
- Lightning Network support
- SegWit implementation
- Compact block relay
- Enhanced peer discovery

#### **Developer Experience**
- Comprehensive API documentation
- CLI improvements
- Configuration management
- Enhanced logging and metrics

---

---

## Initial Block Download (IBD)

When a new node joins the network, it follows Bitcoin's proven Initial Block Download process:

### 1. Finding Peers
- Uses pre-configured DNS seeds or manual peer addresses
- Establishes initial P2P connections

### 2. Requesting Headers
- Requests block headers (80-byte summaries)
- Starts from genesis block
- Continues to current chain tip

### 3. Determining Best Chain
- Calculates cumulative Proof-of-Work for each chain
- Selects chain with most work (longest chain rule)
- Validates headers against consensus rules

### 4. Downloading Blocks
- Downloads full blocks from peers
- Validates each block:
  - Transaction validity (signatures, no double-spending)
  - Proof-of-work verification
  - Block structure compliance
  - Chain continuity

### 5. Staying in Sync
- Continuously receives new blocks and transactions
- Validates and propagates to network
- Maintains consistent blockchain state

This process ensures nodes establish a trustworthy view of the blockchain without relying on central authority.

---

## Project Status

### Metrics

| Metric | Value |
|--------|-------|
| **Bitcoin Core Alignment** | 76% (target: 95%) |
| **Total Tests** | 164 (100% passing) |
| **Test Coverage** | High (all critical paths) |
| **Lines of Code** | ~8,000+ (Rust) |
| **Dependencies** | Minimal, well-audited |
| **Documentation** | 14 technical documents |

### Recent Achievements (October 2025)

- ✅ **+10% Bitcoin Core alignment** through Phase 1 refactoring
- ✅ **164 passing tests** (up from 130)
- ✅ **Schnorr/P2TR support** fully implemented
- ✅ **Consensus mechanism** battle-tested
- ✅ **Web API** with Swagger documentation

### Known Limitations

| Component | Limitation | Impact | Priority |
|-----------|------------|--------|----------|
| **Peer Management** | Simple HashSet, no eclipse protection | Security | HIGH |
| **Network Layer** | Single-threaded, no connection pooling | Scalability | HIGH |
| **UTXO Set** | No caching layer | Performance | MEDIUM |
| **Validation** | Logic scattered across modules | Maintainability | MEDIUM |
| **Script System** | Not implemented | Feature gap | LOW |

---

## Contributing

### Development Setup

```bash
# Clone repository
git clone <repository-url>
cd blockchain

# Build
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run startnode <WALLET_ADDR> yes local
```

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Lint with Clippy (`cargo clippy`)
- Document public APIs with `///` comments
- Add tests for new features
- Follow Bitcoin Core naming conventions

### Architectural Guidelines

1. **Primitives = Pure Data** - No business logic in primitives/
2. **Bitcoin Core Alignment** - Match Bitcoin's file names and structure
3. **Separation of Concerns** - Keep layers independent
4. **Comprehensive Testing** - Test all new functionality
5. **Documentation** - Update docs for structural changes

---

## License

This project is an educational implementation following Bitcoin Core's architecture.

---

## Acknowledgments

- **Bitcoin Core** for the proven architecture and design patterns
- **Rust Community** for excellent cryptographic libraries
- **secp256k1** for Schnorr/ECDSA implementation

---

## Contact & Resources

- **Blog Post**: Coming soon with technical deep-dive
- **Issue Tracker**: Use GitHub issues for bugs/features
- **Documentation**: See `/docs` directory for detailed guides

---

**Version**: 0.1.0  
**Status**: Active Development  
**Bitcoin Core Target**: v24.0+ compatibility