mod block;
mod blockchain;
use blockchain::Blockchain;

// This main function initializes the blockchain, adds multiple transactions(blocks).
// prints entire blockchain with validation status

fn main() {
    
    //Creates a new blockchain instance
    let mut blockchain = Blockchain::new(4);

    // Adds multiple blocks with transaction data
    blockchain.add_block("Transaction: A pays B 10 BTC".to_string());
    blockchain.add_block("Transaction: C pays D 20 BTC".to_string());
    blockchain.add_block("Transaction: U pays V 30 BTC".to_string());
    blockchain.add_block("Transaction: M pays N 40 BTC".to_string());
    blockchain.add_block("Transaction: X pays Y 50 BTC".to_string());

      // Prints header for blockchain
      println!("--- Blockchain ---");

      // Iterate over the blockchain and prints each block in a structured way
      for (i, block) in blockchain.chain.iter().enumerate() {
          println!("\nBlock #{}:", i);  // Display block index
          println!("----------------------");
          println!("Index: {}", block.index);
          println!("Timestamp: {}", block.timestamp);
          println!("Data: {}", block.data);
          println!("Previous Hash: {}", block.previous_hash);
          println!("Hash: {}", block.hash);
          println!("Nonce: {}", block.nonce);
          println!("----------------------");
      }

    // Validates the blockchain and prints the result
    let is_valid = blockchain.validate_chain();
    println!("Is blockchain valid? {}", is_valid);

  }
