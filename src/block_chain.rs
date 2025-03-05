use crate::hash_function::{calculate_merkle_root, hash_block_header};
use crate::serialization::serialize_bc;
use crate::transaction::Transaction;
use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub version: u32,              // 版本号
    pub prev_block_hash: [u8; 32], // 前一个区块的哈希值
    pub merkle_root: [u8; 32],     // Merkle树根哈希值
    pub timestamp: u32,            // 时间戳
    pub bits: u32,                 // 难度值
    pub nonce: u32,                // 随机数
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>, // 交易列表
}

impl Block {
    // 创建一个新的区块
    pub fn new() -> Block {
        // 生成随机数 nonce
        let nonce = rand::rng().random();
        // let nonce = 0;
        // 生成当前时间戳
        let timestamp = Utc::now().timestamp() as u32;
        Block {
            header: BlockHeader {
                version: 1,
                prev_block_hash: [0; 32],
                merkle_root: [0; 32],
                timestamp,
                bits: 0,
                nonce,
            },
            transactions: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockChain {
    pub blocks: Vec<Block>, // 区块列表
    pub transaction_pool: Vec<Transaction>,
}

impl BlockChain {
    // 创建一个新的区块链
    pub fn new() -> Self {
        let genesis_block = Block::new();
        BlockChain {
            blocks: vec![genesis_block],
            transaction_pool: vec![],
        }
    }
    // 添加交易到交易池
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transaction_pool.push(transaction);
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let mut new_block = Block::new();
        // 将交易池中的交易添加到新区块的交易列表
        new_block.transactions = self.transaction_pool.clone();
        // 清空交易池
        self.transaction_pool.clear();
        // 计算新区块的 Merkle Root（假设交易列表已经设置）
        new_block.header.merkle_root = calculate_merkle_root(&new_block.transactions);
        // 计算新区块的哈希值
        let mut hash = hash_block_header(&new_block.header);
        while !hash.starts_with(&[0; 32][..difficulty]) {
            new_block.header.nonce += 1;
            hash = hash_block_header(&new_block.header);
        }
        // 将新区块添加到区块链
        self.add_block(new_block);
    }
    pub fn add_block(&mut self, data: Block) {
        let mut new_block = data;
        if let Some(last_block) = self.blocks.last() {
            // 获取前一个区块的哈希值
            let prev_block_hash = hash_block_header(&last_block.header);
            // 设置新区块的前一个区块哈希值
            new_block.header.prev_block_hash = prev_block_hash;
        } else {
            // 如果是创世区块，前一个区块哈希值为全零
            new_block.header.prev_block_hash = [0; 32];
        }

        // 计算新区块的 Merkle Root（假设交易列表已经设置）
        new_block.header.merkle_root = calculate_merkle_root(&new_block.transactions);

        // new_block.mine_block(4); // 假设难度是4

        // 将新区块添加到区块链
        self.blocks.push(new_block);
    }
}
