use super::block::Block;
use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};
use std::borrow::Borrow;
use std::ops::ShlAssign;
use tracing::debug;

pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

const TARGET_BITS: i32 = 8;

const MAX_NONCE: i64 = i64::MAX;

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        // Target: This is a 256-bit number that all miners aim to find a hash value below.
        let mut target = BigInt::from(1);

        // Left shifting a number by n bits is equivalent to multiplying that number by 2^n.
        // For example, x << 3 is the same as x * 8 (since 2^3 = 8).
        // TARGET_BITS is the difficulty of the block.
        // So, target << 256 - TARGET_BITS is the same as target * 2^(256 - TARGET_BITS).
        // This is because the target is a 256-bit number and TARGET_BITS is the difficulty of the block.
        // Here is a breakdown of the formula: (2^{256}):
        // This represents the total range of possible outcomes for a 256-bit hash.
        // Bitcoin uses the SHA-256 algorithm, which produces a 256-bit hash value, or a number between 0 and (2^{256}-1).
        // TARGET_BITS: This term is a bit misleading.
        // The formula doesn't use TARGET_BITS directly but rather the exponent from the compact nBits format in the block header.
        // The nBits(TARGET_BITS) field is a compact, 32-bit representation of the full 256-bit target.
        // The exponent is the first byte of nBits, which indicates how many zeroes are at the end of the target.
        // The calculation: The formula (2^{256-TARGET_BITS}) essentially approximates the number of hashes required, on average, to find a valid block.
        // It expresses the mining difficulty relative to the maximum possible target.
        target.shl_assign(256 - TARGET_BITS);
        ProofOfWork { block, target }
    }

    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let pre_block_hash = self.block.get_pre_block_hash();
        let transactions_hash = self.block.hash_transactions();
        let timestamp = self.block.get_timestamp();
        let mut data_bytes = vec![];
        data_bytes.extend(pre_block_hash.as_bytes());
        data_bytes.extend(transactions_hash);
        data_bytes.extend(timestamp.to_be_bytes());
        data_bytes.extend(TARGET_BITS.to_be_bytes());
        data_bytes.extend(nonce.to_be_bytes());
        data_bytes
    }

    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        debug!("Mining the block");
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = crate::sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());

            if hash_int.lt(self.target.borrow()) {
                debug!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
            }
        }
        (nonce, HEXLOWER.encode(hash.as_slice()))
    }
}
