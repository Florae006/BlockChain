use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TxIn {
    pub previous_output: [u8; 32], // 交易输入的哈希值
    pub script_sig: Vec<u8>,       // 解锁脚本
    pub sequence: u32,             // 序列号
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxOut {
    pub value: u64,             // 交易输出金额
    pub script_pubkey: Vec<u8>, // 锁定脚本
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub version: u32,        // 版本号
    pub inputs: Vec<TxIn>,   // 交易输入
    pub outputs: Vec<TxOut>, // 交易输出
    pub lock_time: u32,      // 锁定时间
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockChain {
    pub blocks: Vec<Block>, // 区块列表
}

impl Block {
    // 创建一个新的区块
    pub fn new() -> Block {
        Block {
            header: BlockHeader {
                version: 0,
                prev_block_hash: [0; 32],
                merkle_root: [0; 32],
                timestamp: 0,
                bits: 0,
                nonce: 0,
            },
            transactions: vec![],
        }
    }
}

impl BlockChain {
    // 创建一个新的区块链
    pub fn new() -> BlockChain {
        BlockChain { blocks: vec![] }
    }
    
    pub fn add_block(&mut self, data: Block) {
        self.blocks.push(data);
    }
}
