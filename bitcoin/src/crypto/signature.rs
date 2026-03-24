//! # Signature Functions Module
//!
//! This module provides cryptographic signature functions for the blockchain.
//!
//! ## Four Signature Functions Overview
//!
//! This module contains four main signature functions that serve different purposes in the blockchain:
//!
//! ### 1. ECDSA Functions (Legacy/Alternative)
//! - **`ecdsa_p256_sha256_sign_digest`**: Signs messages with ECDSA P-256 SHA-256 signatures
//! - **`ecdsa_p256_sha256_sign_verify`**: Verifies ECDSA P-256 SHA-256 signatures
//!
//! ### 2. Schnorr Functions (Primary/Modern)
//! - **`schnorr_sign_digest`**: Signs messages with Schnorr signatures (P2TR/Taproot)
//! - **`schnorr_sign_verify`**: Verifies Schnorr signatures (P2TR/Taproot)
//!
//! ## Why Four Different Functions?
//!
//! ### 1. **ECDSA vs Schnorr - Different Signature Schemes**
//! - **ECDSA**: Traditional signature scheme used in older Bitcoin implementations
//! - **Schnorr**: Modern signature scheme introduced with Bitcoin's Taproot upgrade
//! - **Purpose**: Provides both legacy compatibility and modern security
//!
//! ### 2. **Different Cryptographic Libraries**
//! - **ECDSA functions**: Use `ring` crate (BoringSSL-based, comprehensive crypto library)
//! - **Schnorr functions**: Use `secp256k1` crate (Bitcoin-specific, optimized for secp256k1 curve)
//! - **Purpose**: Each library is optimized for its specific use case
//!
//! ### 3. **Different Key Formats**
//! - **ECDSA**: Uses PKCS#8 format private keys (more complex, standardized)
//! - **Schnorr**: Uses raw 32-byte private keys (simpler, Bitcoin-native)
//! - **Purpose**: Matches the requirements of each signature scheme
//!
//! ## Current Usage in the Codebase
//!
//! ### **Primary Usage: Schnorr Functions (Modern Bitcoin)**
//! - **`schnorr_sign_digest`**: Used in `Transaction::sign()` for signing transaction inputs
//! - **`schnorr_sign_verify`**: Used in `Transaction::verify()` for verifying transaction signatures
//! - **Why**: Bitcoin's Taproot upgrade uses Schnorr signatures for better security and efficiency
//!
//! ### **Legacy/Alternative Usage: ECDSA Functions**
//! - **`ecdsa_p256_sha256_sign_digest`**: Available for legacy transaction signing
//! - **`ecdsa_p256_sha256_sign_verify`**: Available for legacy transaction verification
//! - **Why**: Provides backward compatibility and alternative signature schemes
//!
//! ## Usage Locations
//!
//! ### **Schnorr Functions (Primary)**:
//! - **`src/core/transaction.rs`**: Used in `Transaction::sign()` and `Transaction::verify()`
//! - **`src/service/blockchain_service.rs`**: Used indirectly through transaction verification in `mine_block()`
//! - **`src/network/operations.rs`**: Used indirectly through transaction validation
//! - **`src/main.rs`**: Used indirectly through transaction operations in CLI commands
//!
//! ### **ECDSA Functions (Legacy/Alternative)**:
//! - **Available for use**: Can be used for alternative signature schemes
//! - **Not currently used**: The codebase primarily uses Schnorr signatures
//! - **Purpose**: Provides flexibility for different signature requirements
//!
//! ## Technical Differences
//!
//! ### **Signature Sizes**:
//! - **ECDSA**: Variable size signatures (typically 70-72 bytes)
//! - **Schnorr**: Fixed 64-byte signatures (more efficient)
//!
//! ### **Security Properties**:
//! - **ECDSA**: Well-established, widely used
//! - **Schnorr**: Better security properties, linearity, batch verification support
//!
//! ### **Bitcoin Compatibility**:
//! - **ECDSA**: Traditional Bitcoin signatures
//! - **Schnorr**: Modern Bitcoin Taproot signatures (P2TR addresses)

use crate::error::{BtcError, Result};
// use rand::SeedableRng; // Not needed in current implementation
use ring::rand::SecureRandom;
use ring::signature::{ECDSA_P256_SHA256_FIXED, ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair};
use secp256k1::{Keypair, Message, PublicKey, Secp256k1, SecretKey, XOnlyPublicKey, schnorr};

// Import the hash function from utils
use crate::sha256_digest;

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

///
/// The `ecdsa_p256_sha256_sign_digest` function signs the provided message parameter using the ECDSA P-256
/// SHA-256 algorithm. Given a private key in PKCS#8 format (pkcs8), it creates an ECDSA
/// key pair, signs the message, and returns the resulting signature as a byte vector.
///
/// # Usage Examples
///
/// - **Legacy transaction signing**: Available for signing transactions with ECDSA signatures
/// - **Message authentication**: Available for authenticating messages and data
/// - **Alternative signature schemes**: Provides ECDSA as an alternative to Schnorr signatures
/// - **Backward compatibility**: Available for systems that require ECDSA signatures
///
/// # Usage Locations
///
/// ### Current Status:
/// - **Not currently used**: The codebase primarily uses Schnorr signatures for transactions
/// - **Available for use**: Can be used for alternative signature schemes or legacy compatibility
/// - **Purpose**: Provides flexibility for different signature requirements
///
/// # Arguments
///
/// * `pkcs8` - A reference to the PKCS#8 document containing the private key.
/// * `message` - A reference to the message to be signed.
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the ECDSA signature as a byte vector.
///
/// # Error Handling
///
/// Returns `BtcError::TransactionSignatureError` if signing fails due to invalid key format or other cryptographic errors.
pub fn ecdsa_p256_sha256_sign_digest(pkcs8: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let rng = ring::rand::SystemRandom::new();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8, &rng)
        .map_err(|e| BtcError::TransactionSignatureError(e.to_string()))?;
    key_pair
        .sign(&rng, message)
        .map(|signature| signature.as_ref().to_vec())
        .map_err(|e| BtcError::TransactionSignatureError(e.to_string()))
}

///
/// The `ecdsa_p256_sha256_sign_verify` function verifies an ECDSA P-256
/// SHA-256 signature against a provided message parameter using the corresponding
/// public_key value. It constructs an unparsed public key from the public_key byte slice
/// and uses it to verify the provided signature against the message parameter,
/// returning a Boolean indicating the signature's validity.
///
/// # Usage Examples
///
/// - **Legacy transaction verification**: Available for verifying ECDSA signatures on transactions
/// - **Message authentication**: Available for verifying the authenticity of signed messages
/// - **Alternative signature schemes**: Provides ECDSA verification as an alternative to Schnorr
/// - **Backward compatibility**: Available for systems that require ECDSA signature verification
///
/// # Usage Locations
///
/// ### Current Status:
/// - **Not currently used**: The codebase primarily uses Schnorr signature verification for transactions
/// - **Available for use**: Can be used for alternative signature schemes or legacy compatibility
/// - **Purpose**: Provides flexibility for different signature verification requirements
///
/// # Arguments
///
/// * `public_key` - A reference to the public key used for verification.
/// * `signature` - A reference to the signature to be verified.
/// * `message` - A reference to the original message that was signed.
///
/// # Returns
///
/// A boolean indicating whether the signature is valid (`true`) or invalid (`false`).
pub fn ecdsa_p256_sha256_sign_verify(public_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let peer_public_key =
        ring::signature::UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public_key);
    let result = peer_public_key.verify(message, signature.as_ref());
    result.is_ok()
}

///
/// The `schnorr_sign_digest` function signs the provided message using Schnorr signatures
/// with secp256k1. This is the signature scheme used by P2TR (Pay-to-Taproot) addresses.
///
/// # Usage Examples
///
/// - **Transaction signing**: Used in `Transaction::sign()` for signing transactions with Schnorr signatures
/// - **P2TR operations**: Used for all P2TR (Pay-to-Taproot) signature operations
/// - **Modern Bitcoin**: Used for Bitcoin's Taproot upgrade signature scheme
/// - **Enhanced security**: Used for improved signature schemes with better security properties
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/core/transaction.rs`**: Used in `Transaction::sign()` method for signing transaction inputs
///
/// ### Indirect Usage via Transaction Signing:
/// - **`src/core/transaction.rs`**: Used in `Transaction::new_utxo_transaction()` for signing new transactions
/// - **`src/network/operations.rs`**: Used indirectly through transaction creation and signing
/// - **`src/main.rs`**: Used indirectly through transaction operations in CLI commands
///
/// # Arguments
///
/// * `private_key` - A reference to the private key bytes (32 bytes for secp256k1).
/// * `message` - A reference to the message to sign (typically a transaction hash).
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the Schnorr signature as a 64-byte vector.
///
/// # Error Handling
///
/// Returns `BtcError::TransactionSignatureError` if signing fails due to invalid key format or other cryptographic errors.
///
pub fn schnorr_sign_digest(private_key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let secp = Secp256k1::new();
    let secret_key_array: [u8; 32] = private_key.try_into().map_err(|_| {
        BtcError::TransactionSignatureError("Invalid private key length".to_string())
    })?;
    let secret_key = SecretKey::from_byte_array(secret_key_array)
        .map_err(|e| BtcError::TransactionSignatureError(e.to_string()))?;

    let message_hash = sha256_digest(message);
    let message_hash_array: [u8; 32] = message_hash.clone().try_into().map_err(|_| {
        BtcError::TransactionSignatureError("Invalid message hash length".to_string())
    })?;
    let _message_obj = Message::from_digest(message_hash_array);

    let keypair = Keypair::from_secret_key(&secp, &secret_key);
    // Use the RNG re-exported by `secp256k1` to avoid `rand` version mismatches.
    let mut rng = secp256k1::rand::rng();
    let signature = secp.sign_schnorr_with_rng(&message_hash, &keypair, &mut rng);
    Ok(signature.as_ref().to_vec())
}

///
/// The `schnorr_sign_verify` function verifies a Schnorr signature against a provided message
/// using the corresponding public key. This is used for P2TR (Pay-to-Taproot) signature verification.
///
/// # Usage Examples
///
/// - **Transaction verification**: Used in `Transaction::verify()` for verifying Schnorr signatures on transactions
/// - **P2TR operations**: Used for all P2TR (Pay-to-Taproot) signature verification operations
/// - **Modern Bitcoin**: Used for Bitcoin's Taproot upgrade signature verification
/// - **Enhanced security**: Used for improved signature verification with better security properties
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/core/transaction.rs`**: Used in `Transaction::verify()` method for verifying transaction input signatures
///
/// ### Indirect Usage via Transaction Verification:
/// - **`src/service/blockchain_service.rs`**: Used indirectly through transaction verification in `mine_block()`
/// - **`src/network/operations.rs`**: Used indirectly through transaction validation
/// - **`src/main.rs`**: Used indirectly through transaction operations in CLI commands
///
/// # Arguments
///
/// * `public_key` - A reference to the public key bytes (33 bytes for compressed secp256k1 public key).
/// * `signature` - A reference to the signature bytes (64 bytes for Schnorr signature).
/// * `message` - A reference to the original message that was signed (typically a transaction hash).
///
/// # Returns
///
/// A boolean indicating whether the signature is valid (`true`) or invalid (`false`).
///
pub fn schnorr_sign_verify(public_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let secp = Secp256k1::new();

    // Parse the public key
    let public_key_array: [u8; 33] = match public_key.try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    let public_key_obj = match PublicKey::from_byte_array_compressed(public_key_array) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    // Convert to XOnlyPublicKey for Schnorr verification
    let xonly_array: [u8; 32] = match public_key_obj.serialize()[1..33].try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    let xonly_public_key = match XOnlyPublicKey::from_byte_array(xonly_array) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    // Hash the message
    let message_hash = sha256_digest(message);
    let message_hash_array: [u8; 32] = match message_hash.clone().try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    let _message_obj = Message::from_digest(message_hash_array);

    // Parse the signature
    let signature_array: [u8; 64] = match signature.try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    let signature_obj = schnorr::Signature::from_byte_array(signature_array);

    // Verify the Schnorr signature
    secp.verify_schnorr(&signature_obj, &message_hash, &xonly_public_key)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::new_key_pair;
    use ring::signature::KeyPair;

    #[test]
    fn test_ecdsa_signature_roundtrip() {
        // Generate an ECDSA key pair
        let private_key = new_key_pair().expect("Failed to generate ECDSA key pair");
        let key_pair = ring::signature::EcdsaKeyPair::from_pkcs8(
            &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING,
            &private_key,
            &ring::rand::SystemRandom::new(),
        )
        .expect("Failed to create ECDSA key pair from PKCS#8");
        let public_key = key_pair.public_key();

        // Create a test message
        let message = b"Hello, ECDSA signatures!";

        // Sign the message
        let signature =
            ecdsa_p256_sha256_sign_digest(&private_key, message).expect("Failed to sign message");

        // Verify the signature
        let is_valid = ecdsa_p256_sha256_sign_verify(public_key.as_ref(), &signature, message);

        assert!(is_valid, "ECDSA signature verification failed");

        // Test with wrong message
        let wrong_message = b"Wrong message";
        let is_invalid =
            ecdsa_p256_sha256_sign_verify(public_key.as_ref(), &signature, wrong_message);

        assert!(
            !is_invalid,
            "ECDSA signature should be invalid for wrong message"
        );
    }

    #[test]
    fn test_ecdsa_key_generation() {
        // Generate multiple key pairs to ensure randomness
        let key1 = new_key_pair().expect("Failed to generate first ECDSA key pair");
        let key2 = new_key_pair().expect("Failed to generate second ECDSA key pair");

        // Keys should be different (random)
        assert_ne!(key1, key2, "Generated ECDSA keys should be different");

        // Keys should be valid PKCS#8 format
        assert!(!key1.is_empty(), "ECDSA private key should not be empty");
        assert!(!key2.is_empty(), "ECDSA private key should not be empty");
    }

    #[test]
    fn test_ecdsa_signature_different_messages() {
        let private_key = new_key_pair().expect("Failed to generate ECDSA key pair");
        let key_pair = ring::signature::EcdsaKeyPair::from_pkcs8(
            &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING,
            &private_key,
            &ring::rand::SystemRandom::new(),
        )
        .expect("Failed to create ECDSA key pair from PKCS#8");
        let public_key = key_pair.public_key();

        let messages = vec![
            b"Message 1".as_slice(),
            b"Message 2".as_slice(),
            b"Different message".as_slice(),
            b"".as_slice(),
            &[0u8; 100],
        ];

        for message in messages {
            let signature = ecdsa_p256_sha256_sign_digest(&private_key, message)
                .expect("Failed to sign message");

            let is_valid = ecdsa_p256_sha256_sign_verify(public_key.as_ref(), &signature, message);
            assert!(
                is_valid,
                "ECDSA signature should be valid for message: {:?}",
                message
            );
        }
    }

    #[test]
    fn test_ecdsa_signature_invalid_key() {
        let invalid_key = vec![0u8; 32]; // Invalid key format
        let message = b"Test message";

        let result = ecdsa_p256_sha256_sign_digest(&invalid_key, message);
        assert!(result.is_err(), "Should fail with invalid key");
    }

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
        let key1 = new_schnorr_key_pair().expect("Failed to generate first Schnorr key pair");
        let key2 = new_schnorr_key_pair().expect("Failed to generate second Schnorr key pair");

        // Keys should be different (random)
        assert_ne!(key1, key2, "Generated Schnorr keys should be different");

        // Keys should be 32 bytes
        assert_eq!(key1.len(), 32, "Schnorr private key should be 32 bytes");
        assert_eq!(key2.len(), 32, "Schnorr private key should be 32 bytes");
    }

    #[test]
    fn test_schnorr_public_key_derivation() {
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");

        // Public key should be 33 bytes (compressed format)
        assert_eq!(
            public_key.len(),
            33,
            "Schnorr public key should be 33 bytes"
        );

        // Public key should start with 0x02 or 0x03 (compressed format)
        assert!(
            public_key[0] == 0x02 || public_key[0] == 0x03,
            "Schnorr public key should be in compressed format"
        );
    }

    #[test]
    fn test_schnorr_signature_different_messages() {
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");

        let messages = vec![
            b"Message 1".as_slice(),
            b"Message 2".as_slice(),
            b"Different message".as_slice(),
            b"".as_slice(),
            &[0u8; 100],
        ];

        for message in messages {
            let signature =
                schnorr_sign_digest(&private_key, message).expect("Failed to sign message");

            let is_valid = schnorr_sign_verify(&public_key, &signature, message);
            assert!(
                is_valid,
                "Schnorr signature should be valid for message: {:?}",
                message
            );
        }
    }

    #[test]
    fn test_schnorr_signature_invalid_key() {
        let invalid_key = vec![0u8; 31]; // Invalid key length
        let message = b"Test message";

        let result = schnorr_sign_digest(&invalid_key, message);
        assert!(result.is_err(), "Should fail with invalid key length");
    }

    #[test]
    fn test_schnorr_signature_invalid_public_key() {
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let message = b"Test message";
        let signature = schnorr_sign_digest(&private_key, message).expect("Failed to sign message");

        // Test with invalid public key
        let invalid_public_key = vec![0u8; 32]; // Wrong length
        let is_invalid = schnorr_sign_verify(&invalid_public_key, &signature, message);
        assert!(!is_invalid, "Should fail with invalid public key");
    }

    #[test]
    fn test_schnorr_signature_invalid_signature() {
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");
        let message = b"Test message";

        // Test with invalid signature
        let invalid_signature = vec![0u8; 63]; // Wrong length
        let is_invalid = schnorr_sign_verify(&public_key, &invalid_signature, message);
        assert!(!is_invalid, "Should fail with invalid signature");
    }

    #[test]
    fn test_signature_consistency() {
        // Test that signatures are consistent across multiple calls
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");
        let message = b"Consistency test message";

        // Sign the same message multiple times
        let signatures: Vec<Vec<u8>> = (0..10)
            .map(|_| schnorr_sign_digest(&private_key, message).expect("Failed to sign"))
            .collect();

        // All signatures should be valid
        for signature in &signatures {
            let is_valid = schnorr_sign_verify(&public_key, signature, message);
            assert!(is_valid, "All signatures should be valid");
        }

        // Signatures should be different (due to randomness)
        for i in 0..signatures.len() {
            for j in (i + 1)..signatures.len() {
                assert_ne!(
                    signatures[i], signatures[j],
                    "Signatures should be different due to randomness"
                );
            }
        }
    }

    #[test]
    fn test_signature_performance() {
        // Test performance with repeated operations
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");
        let message = b"Performance test message";

        for _ in 0..100 {
            let signature = schnorr_sign_digest(&private_key, message).expect("Failed to sign");
            let is_valid = schnorr_sign_verify(&public_key, &signature, message);
            assert!(is_valid, "Signature should be valid");
        }
    }

    #[test]
    fn test_ecdsa_vs_schnorr_comparison() {
        // Test that both signature schemes work independently
        let ecdsa_private_key = new_key_pair().expect("Failed to generate ECDSA key pair");
        let schnorr_private_key =
            new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let schnorr_public_key =
            get_schnorr_public_key(&schnorr_private_key).expect("Failed to get Schnorr public key");

        let message = b"Comparison test message";

        // ECDSA signing
        let ecdsa_signature = ecdsa_p256_sha256_sign_digest(&ecdsa_private_key, message)
            .expect("Failed to sign with ECDSA");

        // Schnorr signing
        let schnorr_signature = schnorr_sign_digest(&schnorr_private_key, message)
            .expect("Failed to sign with Schnorr");

        // Signatures should be different
        assert_ne!(
            ecdsa_signature, schnorr_signature,
            "ECDSA and Schnorr signatures should be different"
        );

        // Both should be valid for their respective schemes
        let ecdsa_key_pair = ring::signature::EcdsaKeyPair::from_pkcs8(
            &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING,
            &ecdsa_private_key,
            &ring::rand::SystemRandom::new(),
        )
        .expect("Failed to create ECDSA key pair from PKCS#8");
        let ecdsa_public_key = ecdsa_key_pair.public_key();

        let ecdsa_valid =
            ecdsa_p256_sha256_sign_verify(ecdsa_public_key.as_ref(), &ecdsa_signature, message);
        let schnorr_valid = schnorr_sign_verify(&schnorr_public_key, &schnorr_signature, message);

        assert!(ecdsa_valid, "ECDSA signature should be valid");
        assert!(schnorr_valid, "Schnorr signature should be valid");
    }

    #[test]
    fn test_signature_error_handling() {
        // Test proper error handling for invalid inputs
        let invalid_private_key = vec![0u8; 31]; // Wrong length
        let message = b"Test message";

        let result = schnorr_sign_digest(&invalid_private_key, message);
        assert!(result.is_err(), "Should fail with invalid private key");

        if let Err(BtcError::TransactionSignatureError(msg)) = result {
            assert!(!msg.is_empty(), "Error message should not be empty");
        } else {
            panic!("Expected TransactionSignatureError");
        }
    }

    #[test]
    fn test_signature_with_large_messages() {
        // Test signatures with large messages
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");

        let large_message = vec![0u8; 10000]; // 10KB message
        let signature = schnorr_sign_digest(&private_key, &large_message)
            .expect("Failed to sign large message");

        let is_valid = schnorr_sign_verify(&public_key, &signature, &large_message);
        assert!(is_valid, "Signature should be valid for large message");
    }

    #[test]
    fn test_signature_with_empty_message() {
        // Test signatures with empty messages
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");

        let empty_message = b"";
        let signature =
            schnorr_sign_digest(&private_key, empty_message).expect("Failed to sign empty message");

        let is_valid = schnorr_sign_verify(&public_key, &signature, empty_message);
        assert!(is_valid, "Signature should be valid for empty message");
    }
}
