use chrono::Utc;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use crate::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    /// Create a new unmined block
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
    ) -> Self {
        let timestamp = Utc::now().timestamp();

        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };

        block.hash = block.calculate_hash();
        block
    }

    /// Calculate block hash
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        let tx_data = serde_json::to_string(&self.transactions).unwrap();

        let data = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            tx_data,
            self.previous_hash,
            self.nonce
        );

        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    /// Proof-of-Work mining
    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        println!("Block mined: {}", self.hash);
    }
}
