use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use ed25519_dalek::Signature;
use hex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: Option<String>, // hex encoded
}

impl Transaction {
    /// Create unsigned transaction
    pub fn new(from: String, to: String, amount: u64) -> Self {
        Transaction {
            from,
            to,
            amount,
            signature: None,
        }
    }

    /// Hash transaction data (deterministic)
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        let data = format!("{}{}{}", self.from, self.to, self.amount);
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Attach signature (hex encoded)
    pub fn sign(&mut self, signature: &Signature) {
        self.signature = Some(hex::encode(signature.to_bytes()));
    }

    /// Verify transaction signature
    pub fn is_valid(&self) -> bool {
    // Coinbase transaction (mining reward)
    if self.from == "COINBASE" {
        return true;
    }

    let sig_hex = match &self.signature {
        Some(sig) => sig,
        None => return false,
    };

    let sig_vec = match hex::decode(sig_hex) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    // Convert Vec<u8> → [u8; 64]
    let sig_bytes: [u8; 64] = match sig_vec.try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };

    let signature = Signature::from_bytes(&sig_bytes);

    crate::wallet::Wallet::verify(
        &self.from,
        &self.hash(),
        &signature,
    )
}

}
    