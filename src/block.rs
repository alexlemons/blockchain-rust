use super::*;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash), // display the hash (an integer) in hexadecimal format
            &self.timestamp,
            &self.payload,
            &self.nonce,
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,
        payload: String,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0, 32],
            prev_block_hash,
            nonce,
            payload,
            difficulty,
        }
    }
    pub fn mine(&mut self) {
        // loop through every u64 integer until we find correct nonce
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash(); // computationally expensive
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

// concatenate together all the bytes composing the block's fields (except the hash itself)
impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![]; // immutable by default -> set mutable
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&u128_bytes(&self.difficulty));
        bytes //implicit return
    }
}

pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}