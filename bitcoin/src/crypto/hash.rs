//! # Hash Functions Module
//!
//! This module provides cryptographic hash functions for the blockchain.
//!
//! ## Key Differences Between `sha256_digest` and `taproot_hash`
//!
//! ### 1. Underlying Libraries
//! - **`sha256_digest`**: Uses the `ring` crate (`ring::digest::Context` and `ring::digest::SHA256`)
//! - **`taproot_hash`**: Uses the `sha2` crate (`sha2::Sha256`)
//!
//! ### 2. Purpose and Context
//!
//! **`sha256_digest`**:
//! - General-purpose SHA-256 hashing
//! - Used throughout the blockchain for various hashing needs
//! - Part of the core cryptographic infrastructure
//! - Used in transaction signing
//!
//! **`taproot_hash`**:
//! - Specifically designed for Taproot (P2TR) compatibility
//! - Part of Bitcoin's Taproot upgrade implementation
//! - Used for P2TR (Pay-to-Taproot) address generation
//! - Replaces RIPEMD160 for better security
//!
//! ## Usage Locations
//!
//! ### `sha256_digest` Usage:
//! - **`src/core/transaction.rs`**: Transaction ID generation (`hash()` method)
//! - **`src/core/block.rs`**: Merkle tree root calculation
//! - **`src/core/proof_of_work.rs`**: Block hash calculation for mining
//! - **`src/crypto/signature.rs`**: Message hashing for Schnorr signatures
//! - **`src/crypto/hash.rs`**: Core hashing infrastructure
//!
//! ### `taproot_hash` Usage:
//! - **`src/wallet/wallet_impl.rs`**: Public key hashing for P2TR addresses (`hash_pub_key()`)
//! - **`src/service/wallet_service.rs`**: Wallet service public key hashing
//! - **`src/crypto/hash.rs`**: Core Taproot hashing infrastructure
//!
//! ### Indirect Usage via `hash_pub_key`:
//! - **`src/core/transaction.rs`**: Transaction input validation (`uses_key()` method)
//! - **`src/core/transaction.rs`**: UTXO transaction creation
//! - **`src/store/file_system_db_chain.rs`**: Transaction summary generation
//! - **`src/main.rs`**: Address validation and transaction processing
//!
//! ### 3. Why Two Different Libraries?
//!
//! This appears to be a legacy issue and inconsistency in the codebase:
//! - **Historical Reasons**: The codebase likely started with `ring` for general hashing
//! - **Taproot Implementation**: When Taproot support was added, `sha2` was chosen for the new functionality
//! - **Different Requirements**: Taproot has specific requirements that might have led to choosing `sha2`
//!
//! ### 4. Technical Differences
//!
//! Both functions produce identical SHA-256 output, but:
//! - **`ring`**: More comprehensive cryptographic library, used by other parts of the codebase
//! - **`sha2`**: More focused on hashing algorithms, potentially lighter weight
//!
//! ## Recommendation
//!
//! The codebase should be refactored to use a single SHA-256 implementation for consistency,
//! reduced dependencies, and improved maintainability.

use ring::digest::{Context, SHA256};
use sha2::{Digest as Sha2Digest, Sha256 as Sha2Hash};

///
/// Hash functions are used to create a unique identifier for a block or transaction.
///
/// The `sha256_digest` function performs a SHA-256 hash operation on the provided data input,
/// returning the resulting hash as a vector of bytes.
/// It initializes a hashing context with SHA-256, updates the context with the input data,
/// generates the hash digest, and converts it to a vector of bytes for output.
///
/// # Usage Examples
///
/// - **Transaction ID generation**: Used in `Transaction::hash()` to create unique transaction identifiers
/// - **Block hashing**: Used in `Block::get_hash()` for Merkle tree root calculation
/// - **Mining**: Used in `ProofOfWork::run()` for block hash calculation during mining
/// - **Signature verification**: Used in Schnorr signature verification for message hashing
///
/// # Arguments
///
/// * `data` - A reference to the input data to be hashed.
///
/// # Returns
///
/// A 32-byte SHA-256 hash as a vector of bytes.
pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}

///
/// The `taproot_hash` function calculates the Taproot-compatible hash of the input data.
///
/// For P2TR (Pay-to-Taproot), we use SHA256 as the primary hash function instead of RIPEMD160.
/// This provides better security and is compatible with Bitcoin's Taproot upgrade.
/// The function takes input data and returns a 32-byte hash suitable for P2TR addresses.
///
/// # Usage Examples
///
/// - **Address generation**: Used in `hash_pub_key()` for P2TR address creation
/// - **Wallet operations**: Used in wallet service for public key hashing
/// - **Transaction validation**: Used indirectly through `hash_pub_key()` in transaction input validation
/// - **UTXO management**: Used in UTXO set operations for address-based lookups
///
/// # Arguments
///
/// * `data` - A reference to the input data to be hashed (typically a public key).
///
/// # Returns
///
/// A 32-byte hash as a vector of bytes, suitable for P2TR address generation.
pub fn taproot_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha2Hash::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_digest_basic() {
        let data = b"Block Chain Project";
        let hash = sha256_digest(data);

        // SHA-256 should always produce 32 bytes
        assert_eq!(hash.len(), 32);

        // Hash should be deterministic
        let hash2 = sha256_digest(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_sha256_digest_empty() {
        let data = b"";
        let hash = sha256_digest(data);

        // Empty input should still produce 32-byte hash
        assert_eq!(hash.len(), 32);

        // Known empty string SHA-256 hash
        let expected =
            hex::decode("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
                .expect("Failed to decode expected hash");
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_sha256_digest_different_inputs() {
        let data1 = b"Rust";
        let data2 = b"CPlusPlus";
        let data3 = b"Block Chain Project";

        let hash1 = sha256_digest(data1);
        let hash2 = sha256_digest(data2);
        let hash3 = sha256_digest(data3);

        // Different inputs should produce different hashes
        assert_ne!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_ne!(hash2, hash3);

        // All hashes should be 32 bytes
        assert_eq!(hash1.len(), 32);
        assert_eq!(hash2.len(), 32);
        assert_eq!(hash3.len(), 32);
    }

    #[test]
    fn test_sha256_digest_large_input() {
        let data = vec![0u8; 10000]; // 10KB of zeros
        let hash = sha256_digest(&data);

        assert_eq!(hash.len(), 32);

        // Hash should be deterministic even for large inputs
        let hash2 = sha256_digest(&data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_sha256_digest_known_values() {
        // Test with known SHA-256 values
        let test_cases = vec![
            (
                b"abc".as_slice(),
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".as_slice(),
                "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
            ),
        ];

        for (input, expected_hex) in test_cases {
            let hash = sha256_digest(input);
            let expected = hex::decode(expected_hex).expect("Failed to decode expected hash");
            assert_eq!(hash, expected, "Hash mismatch for input: {:?}", input);
        }
    }

    #[test]
    fn test_taproot_hash_basic() {
        let data = b"Hello, Taproot!";
        let hash = taproot_hash(data);

        // Taproot hash should always produce 32 bytes
        assert_eq!(hash.len(), 32);

        // Hash should be deterministic
        let hash2 = taproot_hash(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_taproot_hash_empty() {
        let data = b"";
        let hash = taproot_hash(data);

        // Empty input should still produce 32-byte hash
        assert_eq!(hash.len(), 32);

        // Known empty string SHA-256 hash (same as sha256_digest)
        let expected =
            hex::decode("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
                .expect("Failed to decode expected hash");
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_taproot_hash_consistency_with_sha256() {
        // Both functions should produce identical results for the same input
        let test_data = vec![
            b"Block Chain Project".as_slice(),
            b"Bitcoin Taproot".as_slice(),
            b"P2TR Address".as_slice(),
            b"".as_slice(),
            &[0u8; 100],
        ];

        for data in test_data {
            let sha256_result = sha256_digest(data);
            let taproot_result = taproot_hash(data);

            assert_eq!(
                sha256_result, taproot_result,
                "sha256_digest and taproot_hash should produce identical results for input: {:?}",
                data
            );
        }
    }

    #[test]
    fn test_taproot_hash_different_inputs() {
        let data1 = b"Public Key 1";
        let data2 = b"Public Key 2";
        let data3 = b"Different Key";

        let hash1 = taproot_hash(data1);
        let hash2 = taproot_hash(data2);
        let hash3 = taproot_hash(data3);

        // Different inputs should produce different hashes
        assert_ne!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_ne!(hash2, hash3);

        // All hashes should be 32 bytes
        assert_eq!(hash1.len(), 32);
        assert_eq!(hash2.len(), 32);
        assert_eq!(hash3.len(), 32);
    }

    #[test]
    fn test_taproot_hash_public_key_simulation() {
        // Simulate hashing public keys (33 bytes for compressed secp256k1)
        let pub_key1 = vec![0x02u8; 33]; // Compressed public key starting with 0x02
        let pub_key2 = vec![0x03u8; 33]; // Compressed public key starting with 0x03

        let hash1 = taproot_hash(&pub_key1);
        let hash2 = taproot_hash(&pub_key2);

        assert_eq!(hash1.len(), 32);
        assert_eq!(hash2.len(), 32);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_avalanche_effect() {
        // Test that small changes in input produce completely different hashes
        let data1 = b"Block Chain Project";
        let data2 = b"Hello, World?";
        let data3 = b"Hello, World.";

        let hash1 = sha256_digest(data1);
        let hash2 = sha256_digest(data2);
        let hash3 = sha256_digest(data3);

        // Count how many bits are different
        let diff_bits_1_2 = hash1
            .iter()
            .zip(hash2.iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum::<u32>();
        let diff_bits_1_3 = hash1
            .iter()
            .zip(hash3.iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum::<u32>();

        // Should have significant differences (avalanche effect)
        assert!(diff_bits_1_2 > 100, "Hash should show avalanche effect");
        assert!(diff_bits_1_3 > 100, "Hash should show avalanche effect");
    }

    #[test]
    fn test_hash_performance_consistency() {
        // Test that hash functions are consistent under repeated calls
        let data = b"Performance test data";

        for _ in 0..100 {
            let hash1 = sha256_digest(data);
            let hash2 = taproot_hash(data);

            assert_eq!(hash1.len(), 32);
            assert_eq!(hash2.len(), 32);
            assert_eq!(hash1, hash2);
        }
    }
}
