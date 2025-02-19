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
--- Blockchain ---

Block #0:
----------------------
Index: 0
Timestamp: 1739832500
Data: Genesis Block
Previous Hash: 0
Hash: c5c23d1b30fbba1171faedff08b3e909d1a7fcd5d69942bc6701b8dbee7fdd81
----------------------

Block #1:
----------------------
Index: 1
Timestamp: 1739832501
Data: Transaction: A pays B 10 BTC
Previous Hash: c5c23d1b30fbba1171faedff08b3e909d1a7fcd5d69942bc6701b8dbee7fdd81
Hash: 51ed09e5886083e91bc64a2fb5ef498d51b60881854e0219ab80a00e8de67fbd
----------------------

Is blockchain valid? true
```



