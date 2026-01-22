use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub mempool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
            mempool: Vec::new(),
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
        if !tx.is_valid() {
            return false;
        }
        self.mempool.push(tx);
        true
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let reward_tx = Transaction {
            from: "COINBASE".to_string(),
            to: miner_address,
            amount: 100,
            signature: None,
        };

        let mut block_txs = vec![reward_tx];
        block_txs.append(&mut self.mempool);

        let previous_block = self.latest_block();

        let mut new_block = Block::new(
            previous_block.index + 1,
            block_txs,
            previous_block.hash.clone(),
        );

        new_block.mine(self.difficulty);
        self.chain.push(new_block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }

            for tx in &current.transactions {
                if !tx.is_valid() {
                    return false;
                }
            }
        }
        true
    }
}
