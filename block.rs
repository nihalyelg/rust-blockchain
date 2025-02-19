use chrono::Utc; // For timestamps
use sha2::{Digest, Sha256}; // For SHA-256 hashing

#[derive(Debug, Clone)]

// Represents a single block in the blockchain
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    
    /// Creates a new block with a given index, data, and previous block hash.
    /// Returns a new `Block` instance with a computed SHA-256 hash.
    pub fn new(index: u64, data: String, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().timestamp(); 
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash : String::new(),
            nonce : 0,
        };
        block.mine_block(difficulty);
        block
    }

    // Method to calculate the hash of a block
    pub fn calculate_hash(index: u64, timestamp: i64, data: &str, previous_hash: &str, nonce:u64) -> String {
        let input = format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input);
        
        let result = hasher.finalize();
        format!("{:x}", result) 
    }

    /// Increments `nonce` until a valid hash is found that satisfies the difficulty target.
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty); // Create the difficulty target (e.g., "0000")
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = Block::calculate_hash(self.index, self.timestamp, &self.data, &self.previous_hash, self.nonce);
        }
        println!("Block mined! Nonce: {}, Hash: {}", self.nonce, self.hash);
    }
}
