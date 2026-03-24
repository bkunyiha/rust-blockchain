use crate::error::{BtcError, Result};

///
/// The `base58_encode` function encodes the given byte slice using the Base58 encoding scheme
/// and returns the encoded string representation.
/// It utilizes bs58 crate to perform the encoding and converts the byte data into a Base58-encoded string.
///
/// # Usage Examples
///
/// - **Address generation**: Used in `convert_address()` to encode P2TR Bitcoin addresses
/// - **Address validation**: Used in `validate_address()` for address checksum verification
/// - **Transaction display**: Used indirectly through `convert_address()` in transaction formatting
/// - **UTXO management**: Used in transaction summary generation for address display
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/wallet/wallet_impl.rs`**: Used in `convert_address()` function for P2TR address encoding
///
/// ### Indirect Usage via `convert_address()`:
/// - **`src/core/transaction.rs`**: Transaction output address display in debug logging
/// - **`src/store/file_system_db_chain.rs`**: Transaction summary generation for input/output addresses
/// - **`src/main.rs`**: Transaction input/output formatting for CLI display
///
/// # Arguments
///
/// * `data` - A reference to the input data to be Base58 encoded (typically address payload with version, hash, and checksum).
///
/// # Returns
///
/// A Base58-encoded string representation of the input data.
pub fn base58_encode(data: &[u8]) -> Result<String> {
    Ok(bs58::encode(data).into_string())
}

///
/// The `base58_decode` function decodes a Base58-encoded string back to its original byte representation.
/// It uses the bs58 crate to decode the input string and returns the decoded byte vector.
///
/// # Usage Examples
///
/// - **Address validation**: Used in `validate_address()` to decode and verify Bitcoin address structure
/// - **Public key extraction**: Used in `get_pub_key_hash()` to extract public key hash from addresses
/// - **Address parsing**: Used to decode Bitcoin addresses into their component parts (version, hash, checksum)
/// - **Transaction processing**: Used indirectly through address validation in transaction handling
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/wallet/wallet_impl.rs`**: Used in `validate_address()` function for address structure validation
/// - **`src/wallet/wallet_impl.rs`**: Used in `get_pub_key_hash()` function to extract public key hash from addresses
///
/// ### Indirect Usage via `get_pub_key_hash()`:
/// - **`src/core/transaction.rs`**: Used in `TXOutput::lock()` for address validation during output creation
/// - **`src/core/utxo_set.rs`**: Used in `get_balance()` for address validation and public key hash extraction
/// - **`src/store/file_system_db_chain.rs`**: Used in transaction summary generation for address processing
///
/// # Arguments
///
/// * `data` - A reference to the Base58-encoded string to be decoded (typically a Bitcoin address).
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the decoded byte vector, or an error if decoding fails.
///
/// # Error Handling
///
/// Returns `BtcError::AddressDecodingError` if the input string is not valid Base58 or cannot be decoded.
pub fn base58_decode(data: &str) -> Result<Vec<u8>> {
    bs58::decode(data)
        .into_vec()
        .map_err(|e| BtcError::AddressDecodingError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base58_encode_basic() {
        let data = b"Block Chain Project";
        let encoded = base58_encode(data).expect("Failed to encode");

        // Should not be empty
        assert!(!encoded.is_empty());

        // Should be deterministic
        let encoded2 = base58_encode(data).expect("Failed to encode");
        assert_eq!(encoded, encoded2);
    }

    #[test]
    fn test_base58_encode_empty() {
        let data = b"";
        let encoded = base58_encode(data).expect("Failed to encode empty data");

        // Empty input should produce empty output
        assert_eq!(encoded, "");
    }

    #[test]
    fn test_base58_encode_different_inputs() {
        let data1 = b"Block";
        let data2 = b"Chain";
        let data3 = b"Block Chain Project";

        let encoded1 = base58_encode(data1).expect("Failed to encode data1");
        let encoded2 = base58_encode(data2).expect("Failed to encode data2");
        let encoded3 = base58_encode(data3).expect("Failed to encode data3");

        // Different inputs should produce different encodings
        assert_ne!(encoded1, encoded2);
        assert_ne!(encoded1, encoded3);
        assert_ne!(encoded2, encoded3);
    }

    #[test]
    fn test_base58_encode_known_values() {
        // Test with known Base58 encodings
        let test_cases = vec![
            (b"Hello".as_slice(), "9Ajdvzr"),
            (b"".as_slice(), ""),
            (b"a".as_slice(), "2g"),
            (b"ab".as_slice(), "8Qq"),
        ];

        for (input, expected) in test_cases {
            let encoded = base58_encode(input).expect("Failed to encode");
            assert_eq!(
                encoded, expected,
                "Encoding mismatch for input: {:?}",
                input
            );
        }
    }

    #[test]
    fn test_base58_encode_large_data() {
        let data = vec![0u8; 1000]; // 1KB of zeros
        let encoded = base58_encode(&data).expect("Failed to encode large data");

        // Should not be empty
        assert!(!encoded.is_empty());

        // Should be deterministic
        let encoded2 = base58_encode(&data).expect("Failed to encode large data");
        assert_eq!(encoded, encoded2);
    }

    #[test]
    fn test_base58_encode_special_characters() {
        let data = b"\x00\x01\x02\xff\xfe\xfd";
        let encoded = base58_encode(data).expect("Failed to encode special characters");

        // Should not contain ambiguous characters (0, O, I, l)
        assert!(!encoded.contains('0'));
        assert!(!encoded.contains('O'));
        assert!(!encoded.contains('I'));
        assert!(!encoded.contains('l'));
    }

    #[test]
    fn test_base58_decode_basic() {
        let original = b"Block Chain Project";
        let encoded = base58_encode(original).expect("Failed to encode");
        let decoded = base58_decode(&encoded).expect("Failed to decode");

        assert_eq!(decoded, original);
    }

    #[test]
    fn test_base58_decode_empty() {
        let encoded = "";
        let decoded = base58_decode(encoded).expect("Failed to decode empty string");

        assert_eq!(decoded, b"");
    }

    #[test]
    fn test_base58_decode_known_values() {
        // Test with known Base58 decodings
        let test_cases = vec![
            ("9Ajdvzr", b"Hello".as_slice()),
            ("", b"".as_slice()),
            ("2g", b"a".as_slice()),
            ("8Qq", b"ab".as_slice()),
        ];

        for (encoded, expected) in test_cases {
            let decoded = base58_decode(encoded).expect("Failed to decode");
            assert_eq!(
                decoded, expected,
                "Decoding mismatch for input: {}",
                encoded
            );
        }
    }

    #[test]
    fn test_base58_decode_invalid_characters() {
        // Test with invalid Base58 characters
        let invalid_encodings = vec![
            "0",      // Contains '0' which is ambiguous
            "O",      // Contains 'O' which is ambiguous
            "I",      // Contains 'I' which is ambiguous
            "l",      // Contains 'l' which is ambiguous
            "Hello0", // Contains invalid character
            "WorldO", // Contains invalid character
        ];

        for invalid in invalid_encodings {
            let result = base58_decode(invalid);
            assert!(
                result.is_err(),
                "Should fail to decode invalid Base58: {}",
                invalid
            );
        }
    }

    #[test]
    fn test_base58_decode_invalid_strings() {
        // Test with completely invalid strings
        let invalid_strings = vec!["!@#$%^&*()", "Hello World!", "1234567890-=", "qwertyuiop[]"];

        for invalid in invalid_strings {
            let result = base58_decode(invalid);
            assert!(
                result.is_err(),
                "Should fail to decode invalid string: {}",
                invalid
            );
        }
    }

    #[test]
    fn test_base58_roundtrip() {
        // Test roundtrip encoding/decoding
        let test_data = vec![
            b"Block Chain Project".as_slice(),
            b"Bitcoin Address".as_slice(),
            b"P2TR Test".as_slice(),
            b"".as_slice(),
            &[0u8; 50],
            &[255u8; 50],
            b"\x00\x01\x02\x03\x04\x05".as_slice(),
        ];

        for data in test_data {
            let encoded = base58_encode(data).expect("Failed to encode");
            let decoded = base58_decode(&encoded).expect("Failed to decode");
            assert_eq!(decoded, data, "Roundtrip failed for data: {:?}", data);
        }
    }

    #[test]
    fn test_base58_roundtrip_random_data() {
        // Test roundtrip with random data
        use rand::RngExt;
        let mut rng = rand::rng();

        for _ in 0..100 {
            let len = rng.random_range(1..=100);
            let data: Vec<u8> = (0..len).map(|_| rng.random()).collect();

            let encoded = base58_encode(&data).expect("Failed to encode random data");
            let decoded = base58_decode(&encoded).expect("Failed to decode random data");
            assert_eq!(decoded, data, "Roundtrip failed for random data");
        }
    }

    #[test]
    fn test_base58_consistency() {
        // Test that encoding/decoding is consistent across multiple calls
        let data = b"Consistency test data";

        for _ in 0..50 {
            let encoded1 = base58_encode(data).expect("Failed to encode");
            let encoded2 = base58_encode(data).expect("Failed to encode");
            assert_eq!(encoded1, encoded2, "Encoding should be consistent");

            let decoded1 = base58_decode(&encoded1).expect("Failed to decode");
            let decoded2 = base58_decode(&encoded2).expect("Failed to decode");
            assert_eq!(decoded1, decoded2, "Decoding should be consistent");
            assert_eq!(decoded1, data, "Decoded data should match original");
        }
    }

    #[test]
    fn test_base58_address_simulation() {
        // Simulate Bitcoin address encoding/decoding
        let version_byte = 0x00; // Mainnet P2PKH
        let hash160 = vec![0x12; 20]; // 20-byte hash
        let checksum = vec![0x34, 0x56, 0x78, 0x9a]; // 4-byte checksum

        let mut address_data = vec![version_byte];
        address_data.extend_from_slice(&hash160);
        address_data.extend_from_slice(&checksum);

        let encoded = base58_encode(&address_data).expect("Failed to encode address");
        let decoded = base58_decode(&encoded).expect("Failed to decode address");

        assert_eq!(decoded, address_data);
        assert_eq!(decoded.len(), 25); // 1 + 20 + 4 = 25 bytes
    }

    #[test]
    fn test_base58_performance() {
        // Test performance with repeated operations
        let data = b"Performance test data for Base58 encoding and decoding";

        for _ in 0..1000 {
            let encoded = base58_encode(data).expect("Failed to encode");
            let decoded = base58_decode(&encoded).expect("Failed to decode");
            assert_eq!(decoded, data);
        }
    }

    #[test]
    fn test_base58_error_handling() {
        // Test proper error handling
        let result = base58_decode("InvalidBase58StringWithSpecialChars!@#");
        assert!(result.is_err());

        if let Err(BtcError::AddressDecodingError(msg)) = result {
            assert!(!msg.is_empty(), "Error message should not be empty");
        } else {
            panic!("Expected AddressDecodingError");
        }
    }
}
