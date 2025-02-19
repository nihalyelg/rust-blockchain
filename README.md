# rust-blockchain
A simple blockchain built in Rust, implementing SHA-256 hashing, block validation, and chain integrity checks. 

## 🚀 Features  
- Automatically generates a **Genesis Block** (first block).  
- Uses **SHA-256 cryptographic hashing** for block integrity.  
- Supports adding new **transaction blocks** dynamically.  
- Implements **blockchain validation** to ensure correctness.  
- Displays the blockchain in a structured, human-readable format.  

## 📥 Installation & Setup  

### 1️⃣ Prerequisites  
Ensure you have **Rust and Cargo** installed. If not, install them with:  
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2️⃣ Clone the Repository
```sh
git clone https://github.com/your-username/rust-blockchain.git
cd rust-blockchain
```

### 3️⃣ Install Dependencies
Ensure your Cargo.toml contains these dependencies:

```toml
[dependencies]
sha2 = "0.9"   # SHA-256 hashing for block integrity
chrono = "0.4" # Timestamping each block
```

### 4️⃣ Build & Run

```sh
cargo run
```

## Blockchain Structure
Each block in the blockchain consists of the following fields:

```rust
pub struct Block {
    pub index: u64,          // Position of the block in the blockchain
    pub timestamp: i64,      // Creation time (Unix timestamp)
    pub data: String,        // Transaction or message stored in the block
    pub previous_hash: String, // Hash of the previous block
    pub hash: String,        // SHA-256 hash of the current block
}
```
Each new block links to the previous block using its hash, ensuring immutability.

## Example Output
```sh
Block mined! Nonce: 17323, Hash: 0000512862acf8bbfcd9eaa0ef7dbfc09dd43dbf52a246e82662da68cb634830
Block mined! Nonce: 451260, Hash: 00006d3b11c3bb761c3115d2eb47141cb1f20f3b2842cecd12afeb10040a4044

--- Blockchain ---    

Block #0:
----------------------
Index: 0
Timestamp: 1739965180 
Data: Genesis Block
Previous Hash: 0
Hash: 0000512862acf8bbfcd9eaa0ef7dbfc09dd43dbf52a246e82662da68cb634830
Nonce: 17323
----------------------

Block #1:
----------------------
Index: 1
Timestamp: 1739965181
Data: Transaction: A pays B 10 BTC
Previous Hash: 0000512862acf8bbfcd9eaa0ef7dbfc09dd43dbf52a246e82662da68cb634830
Hash: 00006d3b11c3bb761c3115d2eb47141cb1f20f3b2842cecd12afeb10040a4044
Nonce: 451260
----------------------

Is blockchain valid? true

```

### 📜 Assumptions
This blockchain focuses on core fundamentals like block linking, cryptographic hashing, and Proof of Work (PoW). The following design choices were made intentionally:

- Runs Locally: No peer-to-peer networking yet, but it can be extended for multi-node communication.
- In-Memory Storage: Blocks are stored in memory for simplicity; can be extended to use databases.
- Basic Proof of Work (PoW): Adjustable difficulty; future versions can add dynamic difficulty adjustment.


