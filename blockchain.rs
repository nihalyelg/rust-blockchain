use crate::block::Block;

#[derive(Debug)]

// Represents a blockchain, which is sequential collection of blocks
// It maintains a chain of blocks, starting from a genesis block
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize, 
}

// Implementation of the Blockchain
// Returns blockchain instance with an initialized genesis block
impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {chain: Vec::new(), difficulty};
        blockchain.create_genesis_block();
        blockchain
    }

     /// Creates and adds the **genesis block**, which is the first block in the blockchain.
    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), self.difficulty);
        self.chain.push(genesis_block); 
    }

    /// Adds a new block to the blockchain.
    /// The new block is linked to the previous block using its hash.
    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap(); 
        let index = previous_block.index.checked_add(1).unwrap(); 
        let previous_hash = previous_block.hash.clone(); 
        
        // Create a new block and add it to the chain
        let new_block = Block::new(index, data, previous_hash, self.difficulty);
        self.chain.push(new_block);
    }

    /// Validates the blockchain’s integrity.
    /// This function ensures that:
    /// 1. Each block correctly references the previous block's hash.
    /// 2. The stored hash of each block matches the recomputed hash.
    /// Returns `true` if the blockchain is valid, `false` otherwise.
    pub fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.previous_hash != previous_block.hash {
                return false;
            }

            if current_block.hash != Block::calculate_hash (current_block.index, current_block.timestamp, &current_block.data, &current_block.previous_hash, current_block.nonce,) {
                return false;
            }
        }

        true
    }
}


