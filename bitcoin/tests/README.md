# Blockchain Tests

This directory contains comprehensive tests for the blockchain implementation.

## Test Structure

### Unit Tests
Unit tests are located within each module file and test individual components:

- **Block Tests** (`src/domain/block.rs`): Tests for block creation, serialization, and validation
- **Transaction Tests** (`src/domain/transaction.rs`): Tests for transaction creation, validation, and serialization
- **Blockchain Tests** (`src/domain/blockchain.rs`): Tests for blockchain operations, persistence, and integrity
- **Server Operations Tests** (`src/server/operations.rs`): Tests for network operations and message handling

### Integration Tests
Integration tests are located in `tests/integration_tests.rs` and test the interaction between multiple components:

- Blockchain creation and mining
- Wallet creation and management
- UTXO set operations
- Server creation and configuration
- Network node parsing
- Transaction creation and validation
- Blockchain persistence
- Blockchain iteration
- Wallet-to-wallet transactions

### Test Helpers
Test helper functions are located in `tests/test_helpers.rs` and provide common utilities:

- `create_temp_blockchain()`: Creates a temporary blockchain for testing
- `create_blockchain_with_blocks()`: Creates a blockchain with a specified number of blocks
- `create_test_wallets()`: Creates test wallets
- `create_utxo_set()`: Creates a UTXO set from a blockchain
- `cleanup_test_files()`: Cleans up test files
- `create_test_transaction()`: Creates test transactions
- `verify_blockchain_integrity()`: Verifies blockchain integrity
- `create_test_addresses()`: Creates multiple test addresses

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Unit Tests Only. Since the current DB is the file system, to avoid locking run
```bash
cargo test --lib -- --test-threads=1
```

### Run Unit Tests Only
```bash
cargo test --lib
```

### Run Integration Tests Only
```bash
cargo test --test integration_tests
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Specific Test
```bash
cargo test test_name
```

## Test Coverage

The tests cover:

1. **Core Blockchain Operations**
   - Block creation and validation
   - Transaction creation and validation
   - Blockchain persistence
   - Mining operations

2. **Network Operations**
   - Server creation
   - Message handling
   - Node communication

3. **Wallet Operations**
   - Wallet creation
   - Address generation
   - Transaction signing

4. **UTXO Management**
   - UTXO set creation
   - Transaction validation
   - Balance calculation

5. **Data Integrity**
   - Serialization/deserialization
   - Hash validation
   - Chain integrity

## Test Data

Test data files are located in `src/tests/commands/` and contain sample JSON requests for testing network operations.

## Best Practices

1. **Use Temporary Files**: Tests use temporary directories to avoid conflicts
2. **Clean Up**: Tests automatically clean up after themselves
3. **Isolation**: Each test is independent and doesn't rely on other tests
4. **Async Support**: Tests use `tokio` for async operations
5. **Error Handling**: Tests properly handle and verify error conditions

## Adding New Tests

When adding new tests:

1. Add unit tests to the appropriate module file
2. Add integration tests to `tests/integration_tests.rs`
3. Add helper functions to `tests/test_helpers.rs` if needed
4. Update this README with new test descriptions
5. Ensure tests clean up after themselves
6. Use descriptive test names that explain what is being tested

