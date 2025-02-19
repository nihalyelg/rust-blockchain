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

4️⃣ Build & Run

```sh
cargo run
```


