

# Toki ⛓️🦀

**A Minimal Blockchain Network Built in Rust**

Toki is an **educational, open-source blockchain implementation written entirely in Rust**, designed to demonstrate how a blockchain works at a low level — from cryptography and proof-of-work to peer-to-peer networking and consensus.

> ⚠️ **Disclaimer**
> Toki is built for learning and research purposes only.
> It is **not audited** and **not intended for production or financial use**.

---

## ✨ Features

* 🧱 **Blockchain Core**

  * Block structure with hash linkage
  * Genesis block initialization
  * Full chain validation

* ⛏️ **Proof-of-Work Mining**

  * Adjustable difficulty (leading zero hashes)
  * Nonce-based mining
  * Miner rewards (coinbase transactions)

* 💸 **Transaction System**

  * Ed25519-based public/private key cryptography
  * Transaction signing & verification
  * Mempool for unconfirmed transactions

* 👛 **Wallet**

  * Secure key pair generation
  * Transaction signing using `ed25519-dalek`

* 🌐 **Peer-to-Peer Network**

  * Built using `libp2p`
  * Peer discovery via Kademlia DHT
  * Transaction & block propagation
  * Longest-chain consensus for fork resolution

* ⚙️ **Async & Concurrent**

  * Powered by `tokio`
  * Memory-safe concurrency via Rust

---

## 🧠 Architecture Overview

```
+------------------+
|      Wallet      |
|  (Keys & Sign)   |
+--------+---------+
         |
         v
+------------------+        +------------------+
|   Transaction    | -----> |     Mempool      |
+------------------+        +------------------+
                                      |
                                      v
+------------------+        +------------------+
|     Mining       | -----> |      Block       |
|  (PoW Engine)    |        +------------------+
+--------+---------+                |
         |                          v
         |                  +------------------+
         |                  |   Blockchain     |
         |                  |  (Validation)    |
         |                  +------------------+
         |                          |
         v                          v
+------------------------------------------------+
|                P2P Network (libp2p)            |
|  - Peer Discovery                              |
|  - Block & Tx Propagation                      |
|  - Chain Sync                                  |
+------------------------------------------------+
```

---

## 📁 Project Structure

```
toki/
├── src/
│   ├── main.rs          # CLI & node startup
│   ├── block.rs         # Block structure & hashing
│   ├── blockchain.rs   # Chain management & validation
│   ├── transaction.rs  # Transactions & signatures
│   ├── wallet.rs       # Key management
│   ├── mining.rs       # Proof-of-Work logic
│   ├── network.rs      # libp2p networking
│   └── consensus.rs    # Longest-chain rules
├── Cargo.toml
└── README.md
```

---

## 📦 Dependencies

```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sha2 = "0.10"
chrono = "0.4"
tokio = { version = "1.0", features = ["full"] }
libp2p = "0.53"
ed25519-dalek = "2.0"
hex = "0.4"
```

---

## 🔁 Block Structure

Each block contains:

* `index`: Position in the chain
* `timestamp`: Block creation time
* `transactions`: List of transactions
* `previous_hash`: Hash of previous block
* `nonce`: Proof-of-work counter
* `hash`: SHA-256 block hash

Hashing formula:

```
SHA256(index + timestamp + transactions + previous_hash + nonce)
```

---

## ⛏️ Mining (Proof of Work)

* Miner collects pending transactions
* Nonce is incremented until:

```
hash.starts_with("0000")
```

* Difficulty can be adjusted dynamically
* Successful blocks are broadcast to peers

---

## 🔐 Wallet & Transactions

* Wallets use **Ed25519 public/private key pairs**
* Transactions are signed by the sender
* Signature verification is mandatory before acceptance
* Supports **account-based model** (initial version)

Transaction format:

```
from: PublicKey
to: PublicKey
amount: u64
signature: Signature
```

---

## 🌍 P2P Networking

* Node discovery via **Kademlia DHT**
* TCP transport with multiplexing
* Bootstrap node support
* Message types:

  * `NewTransaction`
  * `NewBlock`
  * `GetChain`
  * `ChainResponse`

Conflict resolution:

> **Longest valid chain wins**

---

## 🖥️ CLI Commands (Planned)

```
toki start-node --port 8000
toki new-transaction --to <ADDRESS> --amount 50
toki mine
toki get-chain
toki validate
```

---

## 🧪 Testing

* ✅ Unit tests

  * Block hashing
  * Chain validation
  * Signature verification

* 🔄 Integration tests

  * Multi-node simulation
  * Fork resolution

* 📊 Benchmarks

  * Mining speed
  * Transaction throughput

---

## 🚧 Roadmap

### v1.0

* Core blockchain
* PoW mining
* Wallets & transactions
* libp2p networking

### v1.1

* Persistent storage (SQLite)
* Transaction fees
* Difficulty adjustment

### v2.0

* REST / JSON-RPC API
* Proof-of-Stake (experimental)
* WASM smart contracts (research)

---

## 🤝 Contributing

Contributions are welcome!

* Fork the repo
* Create a feature branch
* Submit a pull request

Good first issues will be labeled.

---

## 📜 License

MIT License
See `LICENSE` file for details.

---

If you want, next we can:

* Add **badges** (build, license, rust version)
* Create a **CONTRIBUTING.md**
* Design a **logo**
* Set up **GitHub Actions CI**
* Write the **first `good first issue`**

Just say the word 🚀
