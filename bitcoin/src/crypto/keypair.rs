//! # Keypair Generation Module
//!
//! This module provides cryptographic key pair generation for the blockchain, supporting both
//! traditional ECDSA signatures and modern Schnorr signatures. The dual-library approach ensures
//! compatibility with both legacy Bitcoin implementations and modern Bitcoin features.
//!
//! ## Two Keypair Libraries
//!
//! ### 1. Ring Library (Traditional ECDSA)
//! - **Curve**: ECDSA P-256 (secp256r1)
//! - **Format**: PKCS#8 format (variable length)
//! - **Purpose**: Legacy ECDSA-based wallet operations
//! - **Use Cases**: Traditional Bitcoin addresses, backward compatibility
//! - **Signature Type**: ECDSA signatures
//!
//! ### 2. secp256k1 Library (Modern Bitcoin/Schnorr)
//! - **Curve**: secp256k1 (Bitcoin's standard curve)
//! - **Format**: Raw 32-byte private keys, 33-byte compressed public keys
//! - **Purpose**: Modern Bitcoin operations, P2TR (Pay-to-Taproot) addresses
//! - **Use Cases**: Schnorr signatures, Taproot transactions, modern Bitcoin features
//! - **Signature Type**: Schnorr signatures
//!
//! ## Why Two Libraries?
//!
//! ### 1. Bitcoin Evolution & Compatibility
//! - **Legacy Support**: Traditional Bitcoin wallets used ECDSA signatures
//! - **Modern Bitcoin**: Bitcoin uses secp256k1 curve for proper compatibility
//! - **Backward Compatibility**: Supports existing ECDSA-based implementations
//!
//! ### 2. Signature Scheme Evolution
//! - **ECDSA**: Traditional scheme with larger signature sizes and higher fees
//! - **Schnorr**: Modern scheme with smaller signatures, better security, and signature aggregation
//!
//! ### 3. Address Type Support
//! - **Traditional Addresses**: Use ECDSA signatures (Ring library)
//! - **P2TR (Pay-to-Taproot) Addresses**: Use Schnorr signatures (secp256k1 library)
//!
//! ### 4. Development Flexibility
//! - Support users with existing ECDSA-based wallets
//! - Provide modern Schnorr-based wallets for new users
//! - Enable testing and comparison of both approaches
//! - Allow gradual migration from ECDSA to Schnorr
//!
//! ## Key Differences Summary
//!
//! | Aspect | Ring (ECDSA) | secp256k1 (Schnorr) |
//! |--------|--------------|---------------------|
//! | **Curve** | P-256 (secp256r1) | secp256k1 |
//! | **Key Format** | PKCS#8 (variable length) | Raw 32-byte private, 33-byte compressed public |
//! | **Use Case** | Legacy ECDSA wallets | Modern Bitcoin (P2TR, Schnorr) |
//! | **Bitcoin Compatibility** | Limited | Full Bitcoin compatibility |
//! | **Signature Type** | ECDSA | Schnorr |
//! | **Address Type** | Traditional Bitcoin addresses | Taproot (P2TR) addresses |
//!
//! ## Usage Examples
//!
//! ```rust
//! use blockchain::crypto::keypair::{new_key_pair, new_schnorr_key_pair, get_schnorr_public_key};
//!
//! fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     // Generate traditional ECDSA key pair
//!     let ecdsa_private_key = new_key_pair()?;
//!
//!     // Generate modern Schnorr key pair
//!     let schnorr_private_key = new_schnorr_key_pair()?;
//!     let schnorr_public_key = get_schnorr_public_key(&schnorr_private_key)?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! All functions return `Result<Vec<u8>, BtcError>` and will return `BtcError::WalletKeyPairError`
//! if key generation fails due to insufficient randomness or other cryptographic errors.

use crate::error::{BtcError, Result};
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair};
use secp256k1::{PublicKey, Secp256k1, SecretKey};

///
/// The `new_key_pair` function generates a new ECDSA key pair and returns the private key as a byte vector.
/// It utilizes EcdsaKeyPair and SystemRandom from the ring crate to generate a private key in PKCS#8 format
/// and converts it to a byte vector.
///
/// # Usage Examples
///
/// - **Wallet creation**: Used in wallet generation for creating new Bitcoin addresses
/// - **Key management**: Used for generating secure cryptographic key pairs
/// - **ECDSA operations**: Used as the foundation for ECDSA signing and verification
/// - **Legacy support**: Used for traditional ECDSA-based wallet operations
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/wallet/wallet_impl.rs`**: Used in `Wallet::new()` for generating new wallet key pairs
/// - **`src/service/wallet_service.rs`**: Used in wallet service for key pair generation
///
/// ### Indirect Usage via Wallet Creation:
/// - **`src/main.rs`**: Used indirectly through wallet creation in CLI commands
/// - **`src/network/server.rs`**: Used indirectly through wallet operations in server functionality
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the private key in PKCS#8 format as a byte vector.
///
/// # Error Handling
///
/// Returns `BtcError::WalletKeyPairError` if key generation fails due to insufficient randomness or other cryptographic errors.
///
pub fn new_key_pair() -> Result<Vec<u8>> {
    let rng = SystemRandom::new();
    // Generates new key pair serialized as a PKCS#8 document.
    let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
        .map_err(|e| BtcError::WalletKeyPairError(e.to_string()))?;
    Ok(pkcs8.as_ref().to_vec())
}

///
/// The `new_schnorr_key_pair` function generates a new Schnorr key pair using secp256k1.
/// This is used for P2TR (Pay-to-Taproot) addresses with true Schnorr signatures.
///
/// # Usage Examples
///
/// - **P2TR wallet creation**: Used in wallet generation for creating new Taproot addresses
/// - **Schnorr operations**: Used as the foundation for Schnorr signing and verification
/// - **Modern Bitcoin**: Used for Bitcoin's Taproot upgrade implementation
/// - **Enhanced security**: Used for improved signature schemes with better security properties
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/wallet/wallet_impl.rs`**: Used in `Wallet::new()` for generating new Schnorr-based wallet key pairs
/// - **`src/service/wallet_service.rs`**: Used in wallet service for Schnorr key pair generation
///
/// ### Indirect Usage via Wallet Creation:
/// - **`src/main.rs`**: Used indirectly through wallet creation in CLI commands
/// - **`src/network/server.rs`**: Used indirectly through wallet operations in server functionality
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the private key as a 32-byte vector.
///
/// # Error Handling
///
/// Returns `BtcError::WalletKeyPairError` if key generation fails due to insufficient randomness or other cryptographic errors.
///
pub fn new_schnorr_key_pair() -> Result<Vec<u8>> {
    let mut secret_key_bytes = [0u8; 32];
    ring::rand::SystemRandom::new()
        .fill(&mut secret_key_bytes)
        .map_err(|e| BtcError::WalletKeyPairError(e.to_string()))?;
    let _secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array(secret_key_bytes)
        .map_err(|e| BtcError::WalletKeyPairError(e.to_string()))?;
    Ok(secret_key.secret_bytes().to_vec())
}

///
/// The `get_schnorr_public_key` function derives the public key from a private key
/// using secp256k1 for Schnorr signatures.
///
/// # Usage Examples
///
/// - **Public key derivation**: Used to derive public keys from private keys for Schnorr operations
/// - **Address generation**: Used in P2TR address generation from private keys
/// - **Signature verification**: Used to get public keys for signature verification
/// - **Wallet operations**: Used in wallet operations that require public key derivation
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/wallet/wallet_impl.rs`**: Used in `Wallet::new()` for deriving public keys from private keys
/// - **`src/service/wallet_service.rs`**: Used in wallet service for public key derivation
///
/// ### Indirect Usage via Wallet Operations:
/// - **`src/main.rs`**: Used indirectly through wallet operations in CLI commands
/// - **`src/network/server.rs`**: Used indirectly through wallet operations in server functionality
///
/// # Arguments
///
/// * `private_key` - A reference to the private key bytes (32 bytes for secp256k1).
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the corresponding public key as a 33-byte compressed public key.
///
/// # Error Handling
///
/// Returns `BtcError::WalletKeyPairError` if the private key is invalid or public key derivation fails.
///
pub fn get_schnorr_public_key(private_key: &[u8]) -> Result<Vec<u8>> {
    let secp = Secp256k1::new();
    let secret_key_array: [u8; 32] = private_key
        .try_into()
        .map_err(|_| BtcError::WalletKeyPairError("Invalid private key length".to_string()))?;
    let secret_key = SecretKey::from_byte_array(secret_key_array)
        .map_err(|e| BtcError::WalletKeyPairError(e.to_string()))?;
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    Ok(public_key.serialize().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::signature::{schnorr_sign_digest, schnorr_sign_verify};
    use ring::signature::KeyPair;

    #[test]
    fn test_schnorr_signature_roundtrip() {
        // Generate a Schnorr key pair
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");

        // Create a test message
        let message = b"Hello, P2TR Schnorr signatures!";

        // Sign the message
        let signature = schnorr_sign_digest(&private_key, message).expect("Failed to sign message");

        // Verify the signature
        let is_valid = schnorr_sign_verify(&public_key, &signature, message);

        assert!(is_valid, "Schnorr signature verification failed");

        // Test with wrong message
        let wrong_message = b"Wrong message";
        let is_invalid = schnorr_sign_verify(&public_key, &signature, wrong_message);

        assert!(
            !is_invalid,
            "Schnorr signature should be invalid for wrong message"
        );
    }

    #[test]
    fn test_schnorr_key_generation() {
        // Generate multiple key pairs to ensure randomness
        let key1 = new_schnorr_key_pair().expect("Failed to generate first key pair");
        let key2 = new_schnorr_key_pair().expect("Failed to generate second key pair");

        // Keys should be different (random)
        assert_ne!(key1, key2, "Generated keys should be different");

        // Keys should be 32 bytes
        assert_eq!(key1.len(), 32, "Private key should be 32 bytes");
        assert_eq!(key2.len(), 32, "Private key should be 32 bytes");
    }

    #[test]
    fn test_schnorr_public_key_derivation() {
        let private_key = new_schnorr_key_pair().expect("Failed to generate key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");

        // Public key should be 33 bytes (compressed format)
        assert_eq!(public_key.len(), 33, "Public key should be 33 bytes");

        // Public key should start with 0x02 or 0x03 (compressed format)
        assert!(
            public_key[0] == 0x02 || public_key[0] == 0x03,
            "Public key should be in compressed format"
        );
    }

    #[test]
    fn test_ecdsa_key_pair_generation() {
        // Generate multiple ECDSA key pairs to ensure randomness
        let key1 = new_key_pair().expect("Failed to generate first ECDSA key pair");
        let key2 = new_key_pair().expect("Failed to generate second ECDSA key pair");

        // Keys should be different (random)
        assert_ne!(key1, key2, "Generated ECDSA keys should be different");

        // Keys should be valid PKCS#8 format
        assert!(!key1.is_empty(), "ECDSA private key should not be empty");
        assert!(!key2.is_empty(), "ECDSA private key should not be empty");

        // Keys should be valid PKCS#8 format (can be parsed)
        let _key_pair1 = ring::signature::EcdsaKeyPair::from_pkcs8(
            &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING,
            &key1,
            &ring::rand::SystemRandom::new(),
        )
        .expect("Failed to parse first ECDSA key pair");

        let _key_pair2 = ring::signature::EcdsaKeyPair::from_pkcs8(
            &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING,
            &key2,
            &ring::rand::SystemRandom::new(),
        )
        .expect("Failed to parse second ECDSA key pair");
    }

    #[test]
    fn test_ecdsa_key_pair_consistency() {
        // Test that key generation is consistent
        let key = new_key_pair().expect("Failed to generate ECDSA key pair");

        // Should be able to create key pair from the generated key multiple times
        for _ in 0..10 {
            let _key_pair = ring::signature::EcdsaKeyPair::from_pkcs8(
                &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING,
                &key,
                &ring::rand::SystemRandom::new(),
            )
            .expect("Failed to create ECDSA key pair from PKCS#8");
        }
    }

    #[test]
    fn test_ecdsa_key_pair_public_key_extraction() {
        let private_key = new_key_pair().expect("Failed to generate ECDSA key pair");
        let key_pair = ring::signature::EcdsaKeyPair::from_pkcs8(
            &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING,
            &private_key,
            &ring::rand::SystemRandom::new(),
        )
        .expect("Failed to create ECDSA key pair from PKCS#8");

        let public_key = key_pair.public_key();

        // Public key should not be empty
        assert!(
            !public_key.as_ref().is_empty(),
            "Public key should not be empty"
        );

        // Public key should be valid length for P-256 (65 bytes uncompressed or 33 bytes compressed)
        let pub_key_len = public_key.as_ref().len();
        assert!(
            pub_key_len == 65 || pub_key_len == 33,
            "Public key should be 65 bytes (uncompressed) or 33 bytes (compressed), got {}",
            pub_key_len
        );
    }

    #[test]
    fn test_schnorr_key_pair_consistency() {
        // Test that Schnorr key generation is consistent
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");

        // Should be able to derive public key multiple times with same result
        let public_key1 = get_schnorr_public_key(&private_key).expect("Failed to get public key");
        let public_key2 = get_schnorr_public_key(&private_key).expect("Failed to get public key");

        assert_eq!(
            public_key1, public_key2,
            "Public key derivation should be consistent"
        );
    }

    #[test]
    fn test_schnorr_key_pair_deterministic() {
        // Test that the same private key always produces the same public key
        let private_key = vec![1u8; 32];

        let public_key1 = get_schnorr_public_key(&private_key).expect("Failed to get public key");
        let public_key2 = get_schnorr_public_key(&private_key).expect("Failed to get public key");

        assert_eq!(
            public_key1, public_key2,
            "Same private key should produce same public key"
        );
    }

    #[test]
    fn test_schnorr_key_pair_invalid_private_key() {
        // Test with invalid private key lengths
        let invalid_keys = vec![
            vec![0u8; 31], // Too short
            vec![0u8; 33], // Too long
            vec![0u8; 0],  // Empty
        ];

        for invalid_key in invalid_keys {
            let result = get_schnorr_public_key(&invalid_key);
            assert!(
                result.is_err(),
                "Should fail with invalid private key length: {}",
                invalid_key.len()
            );
        }
    }

    #[test]
    fn test_schnorr_key_pair_edge_cases() {
        // Test with edge case private keys (excluding invalid ones)
        let edge_cases = vec![
            {
                let mut key = vec![0u8; 32];
                key[0] = 1;
                key[31] = 255;
                key
            },
            {
                let mut key = vec![1u8; 32];
                key[0] = 0;
                key
            },
        ];

        for private_key in edge_cases {
            let result = get_schnorr_public_key(&private_key);
            assert!(result.is_ok(), "Should succeed with edge case private key");

            let public_key = result.expect("Failed to get public key");
            assert_eq!(public_key.len(), 33, "Public key should be 33 bytes");
            assert!(
                public_key[0] == 0x02 || public_key[0] == 0x03,
                "Public key should be in compressed format"
            );
        }
    }

    #[test]
    fn test_key_pair_performance() {
        // Test performance with repeated key generation
        for _ in 0..100 {
            let ecdsa_key = new_key_pair().expect("Failed to generate ECDSA key pair");
            let schnorr_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");

            assert!(!ecdsa_key.is_empty(), "ECDSA key should not be empty");
            assert_eq!(schnorr_key.len(), 32, "Schnorr key should be 32 bytes");
        }
    }

    #[test]
    fn test_key_pair_randomness() {
        // Test that key generation produces different keys
        let mut ecdsa_keys = Vec::new();
        let mut schnorr_keys = Vec::new();

        for _ in 0..50 {
            ecdsa_keys.push(new_key_pair().expect("Failed to generate ECDSA key pair"));
            schnorr_keys.push(new_schnorr_key_pair().expect("Failed to generate Schnorr key pair"));
        }

        // All ECDSA keys should be different
        for i in 0..ecdsa_keys.len() {
            for j in (i + 1)..ecdsa_keys.len() {
                assert_ne!(
                    ecdsa_keys[i], ecdsa_keys[j],
                    "ECDSA keys should be different"
                );
            }
        }

        // All Schnorr keys should be different
        for i in 0..schnorr_keys.len() {
            for j in (i + 1)..schnorr_keys.len() {
                assert_ne!(
                    schnorr_keys[i], schnorr_keys[j],
                    "Schnorr keys should be different"
                );
            }
        }
    }

    #[test]
    fn test_key_pair_public_key_uniqueness() {
        // Test that different private keys produce different public keys
        let mut public_keys = Vec::new();

        for _ in 0..20 {
            let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
            let public_key =
                get_schnorr_public_key(&private_key).expect("Failed to get public key");
            public_keys.push(public_key);
        }

        // All public keys should be different
        for i in 0..public_keys.len() {
            for j in (i + 1)..public_keys.len() {
                assert_ne!(
                    public_keys[i], public_keys[j],
                    "Public keys should be different"
                );
            }
        }
    }

    #[test]
    fn test_key_pair_error_handling() {
        // Test proper error handling
        let invalid_private_key = vec![0u8; 31]; // Wrong length

        let result = get_schnorr_public_key(&invalid_private_key);
        assert!(result.is_err(), "Should fail with invalid private key");

        if let Err(BtcError::WalletKeyPairError(msg)) = result {
            assert!(!msg.is_empty(), "Error message should not be empty");
        } else {
            panic!("Expected WalletKeyPairError");
        }
    }

    #[test]
    fn test_key_pair_integration_with_signatures() {
        // Test that generated keys work with signature functions
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");

        let message = b"Integration test message";

        // Sign with the private key
        let signature = schnorr_sign_digest(&private_key, message).expect("Failed to sign message");

        // Verify with the public key
        let is_valid = schnorr_sign_verify(&public_key, &signature, message);
        assert!(is_valid, "Signature should be valid");
    }

    #[test]
    fn test_key_pair_secp256k1_compatibility() {
        // Test that generated keys are compatible with secp256k1
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");

        // Test that we can create secp256k1 objects from our keys
        let secp = Secp256k1::new();
        let secret_key_array: [u8; 32] = private_key
            .try_into()
            .expect("Failed to convert private key to array");
        let secret_key =
            SecretKey::from_byte_array(secret_key_array).expect("Failed to create SecretKey");
        let public_key_array: [u8; 33] = public_key
            .try_into()
            .expect("Failed to convert public key to array");
        let public_key_obj = PublicKey::from_byte_array_compressed(public_key_array)
            .expect("Failed to create PublicKey");

        // Test that the public key matches what secp256k1 would generate
        let expected_public_key = PublicKey::from_secret_key(&secp, &secret_key);
        assert_eq!(
            public_key_obj, expected_public_key,
            "Public key should match secp256k1 derivation"
        );
    }
}
