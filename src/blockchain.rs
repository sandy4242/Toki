use crate::block::Block;
use crate::transaction::Transaction;
use std::collections::HashMap;

const BLOCK_REWARD: u64 = 50;
const COINBASE_ADDRESS: &str = "COINBASE";
const TARGET_BLOCK_TIME: i64 = 10; // seconds
const DIFFICULTY_ADJUSTMENT_INTERVAL: usize = 5; // blocks

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub state: HashMap<String, u64>, // account balances
    pub mempool: Vec<Transaction>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            state: HashMap::new(),
            mempool: Vec::new(),
            difficulty,
        };

        blockchain.chain.push(Self::genesis_block());
        blockchain
    }

    fn genesis_block() -> Block {
        Block::new(0, vec![], "0".to_string())
    }

    pub fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        // Cryptographic validity
        if !tx.is_valid() {
            return false;
        }

        // Block coinbase from mempool
        if tx.from == COINBASE_ADDRESS {
            return false;
        }

        // Simulate state including mempool
        let mut temp_state = self.state.clone();
        for pending in &self.mempool {
            Self::apply_transaction(&mut temp_state, pending);
        }

        if !Self::is_tx_valid_against_state(&temp_state, &tx) {
            return false;
        }

        self.mempool.push(tx);
        true
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let total_fees: u64 = self.mempool.iter().map(|tx| tx.fee).sum();

        let coinbase_tx = Transaction {
            from: COINBASE_ADDRESS.to_string(),
            to: miner_address,
            amount: BLOCK_REWARD + total_fees,
            fee: 0,
            nonce: 0,
            signature: None,
        };

        let mut block_txs = vec![coinbase_tx];
        block_txs.append(&mut self.mempool);

        let previous = self.latest_block();

        let mut block = Block::new(previous.index + 1, block_txs, previous.hash.clone());

        let adjusted_difficulty = self.adjust_difficulty();
        block.mine(adjusted_difficulty);

        println!(
            "Block {} mined at difficulty {}",
            block.index, adjusted_difficulty
        );

        for tx in &block.transactions {
            Self::apply_transaction(&mut self.state, tx);
        }

        self.chain.push(block);
        self.difficulty = adjusted_difficulty;
    }

    pub fn is_chain_valid(&self) -> bool {
        let mut temp_state: HashMap<String, u64> = HashMap::new();

        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            //  Hash integrity
            if current.hash != current.calculate_hash() {
                return false;
            }

            // Chain linkage
            if current.previous_hash != previous.hash {
                return false;
            }

            // Difficulty validation (CRITICAL)
            let expected_difficulty = {
                let temp_chain = &self.chain[..i];
                let temp_blockchain = Blockchain {
                    chain: temp_chain.to_vec(),
                    state: HashMap::new(),
                    mempool: vec![],
                    difficulty: self.difficulty,
                };
                temp_blockchain.adjust_difficulty()
            };

            let target = "0".repeat(expected_difficulty);
            if !current.hash.starts_with(&target) {
                return false;
            }

            // Transactions must exist
            if current.transactions.is_empty() {
                return false;
            }

            // Coinbase validation
            let coinbase = &current.transactions[0];
            let total_fees: u64 = current.transactions.iter().skip(1).map(|tx| tx.fee).sum();

            if coinbase.from != COINBASE_ADDRESS
                || coinbase.amount != BLOCK_REWARD + total_fees
                || coinbase.fee != 0
            {
                return false;
            }

            // 6Transaction + state validation
            for tx in &current.transactions {
                if !tx.is_valid() {
                    return false;
                }

                if !Self::is_tx_valid_against_state(&temp_state, tx) {
                    return false;
                }

                Self::apply_transaction(&mut temp_state, tx);
            }
        }

        true
    }

    pub fn try_replace_chain(&mut self, new_chain: Vec<Block>) -> bool {
        // 1. New chain must be longer
        if new_chain.len() <= self.chain.len() {
            return false;
        }

        // 2. Validate new chain
        let temp_blockchain = Blockchain {
            chain: new_chain.clone(),
            state: HashMap::new(),
            mempool: vec![],
            difficulty: self.difficulty,
        };

        if !temp_blockchain.is_chain_valid() {
            return false;
        }

        // 3. Rebuild state from new chain
        let mut new_state: HashMap<String, u64> = HashMap::new();

        for block in new_chain.iter().skip(1) {
            for tx in &block.transactions {
                Self::apply_transaction(&mut new_state, tx);
            }
        }

        // 4. Accept new chain
        self.chain = new_chain;
        self.state = new_state;
        self.mempool.clear();

        true
    }

    pub fn adjust_difficulty(&self) -> usize {
        let len = self.chain.len();

        // Not enough blocks yet → keep current difficulty
        if len < DIFFICULTY_ADJUSTMENT_INTERVAL + 1 {
            return self.difficulty;
        }

        let last_block = &self.chain[len - 1];
        let prev_adjustment_block = &self.chain[len - 1 - DIFFICULTY_ADJUSTMENT_INTERVAL];

        let actual_time = last_block.timestamp - prev_adjustment_block.timestamp;
        let expected_time = TARGET_BLOCK_TIME * DIFFICULTY_ADJUSTMENT_INTERVAL as i64;

        if actual_time < expected_time / 2 {
            // Too fast → increase difficulty
            self.difficulty + 1
        } else if actual_time > expected_time * 2 {
            // Too slow → decrease difficulty
            self.difficulty.saturating_sub(1)
        } else {
            // Within acceptable range
            self.difficulty
        }
    }

    //state helpers

    fn is_tx_valid_against_state(state: &HashMap<String, u64>, tx: &Transaction) -> bool {
        if tx.from == COINBASE_ADDRESS {
            return true;
        }

        let balance = state.get(&tx.from).copied().unwrap_or(0);
        balance >= tx.amount + tx.fee
    }

    fn apply_transaction(state: &mut HashMap<String, u64>, tx: &Transaction) {
        if tx.from != COINBASE_ADDRESS {
            *state.entry(tx.from.clone()).or_insert(0) -= tx.amount + tx.fee;
        }

        *state.entry(tx.to.clone()).or_insert(0) += tx.amount;
    }
}
