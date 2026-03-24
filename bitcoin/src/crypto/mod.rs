// Declare and defines a module for the crypto layer
pub mod address;
pub mod hash;
pub mod keypair;
pub mod signature;

// Re-export the modules
pub use address::{base58_decode, base58_encode};
pub use hash::{sha256_digest, taproot_hash};
pub use keypair::{get_schnorr_public_key, new_key_pair, new_schnorr_key_pair};
pub use signature::{
    ecdsa_p256_sha256_sign_digest, ecdsa_p256_sha256_sign_verify, schnorr_sign_digest,
    schnorr_sign_verify,
};
