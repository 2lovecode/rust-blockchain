use serde::{Serialize, Deserialize};
use chrono::Utc;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    prev_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: String, prev_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp: Utc::now().timestamp(),
            data,
            prev_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let input = format!("{}{}{}{}{}", 
            self.index, 
            self.timestamp, 
            self.data, 
            self.prev_hash, 
            self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    fn mine(&mut self, difficulty: usize) {
        while !self.hash.starts_with(&"0".repeat(difficulty)) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Blockchain {
            chain: Vec::new(),
            difficulty: 2,
        };
        chain.create_genesis_block();
        chain
    }

    fn create_genesis_block(&mut self) {
        let mut genesis = Block::new(0, "Genesis Block".into(), "0".into());
        genesis.mine(self.difficulty);
        self.chain.push(genesis);
    }

    pub fn add_block(&mut self, data: String) {
        let prev_block = self.chain.last().unwrap();
        let mut new_block = Block::new(
            prev_block.index + 1,
            data,
            prev_block.hash.clone()
        );
        new_block.mine(self.difficulty);
        self.chain.push(new_block);
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!();
            println!("Index: {}", block.index);
            println!("Timestamp: {}", block.timestamp);
            println!("Data: {}", block.data);
            println!("Prev Hash: {}", block.prev_hash);
            println!("Hash: {}", block.hash);
            println!("Nonce: {}", block.nonce);
        }
    }
}