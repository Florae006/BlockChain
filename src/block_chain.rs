use crate::hash_function::{calculate_merkle_root, hash_block_header};

use crate::transaction::Transaction;
use chrono::Utc;
use rand::Rng;

use serde::de::{self, Visitor};
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::VecDeque;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct BlockChain {
    pub blocks: Vec<Block>, // 区块列表
    pub transaction_pool: Arc<Mutex<VecDeque<Transaction>>>,
    difficulty: usize,
}

// 手动实现 Serialize 和 Deserialize
impl Serialize for BlockChain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let transaction_pool = self.transaction_pool.lock().unwrap();
        let mut state = serializer.serialize_struct("BlockChain", 3)?;
        state.serialize_field("blocks", &self.blocks)?;
        state.serialize_field("transaction_pool", &*transaction_pool)?;
        state.serialize_field("difficulty", &self.difficulty)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for BlockChain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Blocks,
            TransactionPool,
            Difficulty,
        }

        struct BlockChainVisitor;

        impl<'de> Visitor<'de> for BlockChainVisitor {
            type Value = BlockChain;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct BlockChain")
            }

            fn visit_map<V>(self, mut map: V) -> Result<BlockChain, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut blocks = None;
                let mut transaction_pool = None;
                let mut difficulty = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Blocks => {
                            if blocks.is_some() {
                                return Err(de::Error::duplicate_field("blocks"));
                            }
                            blocks = Some(map.next_value()?);
                        }
                        Field::TransactionPool => {
                            if transaction_pool.is_some() {
                                return Err(de::Error::duplicate_field("transaction_pool"));
                            }
                            let pool: VecDeque<Transaction> = map.next_value()?;
                            transaction_pool = Some(Arc::new(Mutex::new(pool)));
                        }
                        Field::Difficulty => {
                            if difficulty.is_some() {
                                return Err(de::Error::duplicate_field("difficulty"));
                            }
                            difficulty = Some(map.next_value()?);
                        }
                    }
                }

                let blocks = blocks.ok_or_else(|| de::Error::missing_field("blocks"))?;
                let transaction_pool =
                    transaction_pool.ok_or_else(|| de::Error::missing_field("transaction_pool"))?;
                let difficulty =
                    difficulty.ok_or_else(|| de::Error::missing_field("difficulty"))?;

                Ok(BlockChain {
                    blocks,
                    transaction_pool,
                    difficulty,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["blocks", "transaction_pool", "difficulty"];
        deserializer.deserialize_struct("BlockChain", FIELDS, BlockChainVisitor)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockHeader {
    pub version: u32,              // 版本号
    pub prev_block_hash: [u8; 32], // 前一个区块的哈希值
    pub merkle_root: [u8; 32],     // Merkle树根哈希值
    pub timestamp: u32,            // 时间戳
    pub bits: u32,                 // 难度值
    pub nonce: u32,                // 随机数
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

impl BlockChain {
    // 创建一个新的区块链
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block::new();
        BlockChain {
            blocks: vec![genesis_block],
            transaction_pool: Arc::new(Mutex::new(VecDeque::new())),
            difficulty,
        }
    }
    pub fn broadcast_transaction(&self, tx: Transaction, peers: Vec<String>) {
        for peer in peers {
            let client = reqwest::Client::new();
            let url = format!("http://{}/transaction", peer);
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                let res = client.post(&url).json(&tx_clone).send().await;
                match res {
                    Ok(_) => println!("Transaction broadcasted to {}", peer),
                    Err(e) => println!("Failed to broadcast to {}: {}", peer, e),
                }
            });
        }
    }

    // 添加交易到交易池
    pub fn add_transaction(&mut self, transaction: Transaction) {
        let mut pool = self.transaction_pool.lock().unwrap();
        pool.push_back(transaction);
    }

    pub fn mine_block(&mut self) {
        let mut new_block = Block::new();
        // 将交易池中的交易添加到新区块的交易列表
        // 获取当前区块高度和时间戳
        let current_height = self.blocks.len() as u32;
        let current_timestamp = Utc::now().timestamp() as u32;

        // 过滤交易池中的交易
        let valid_transactions: Vec<Transaction> = self
            .transaction_pool
            .lock()
            .unwrap()
            .iter()
            .filter(|tx| {
                if tx.lock_time == 0 {
                    // lock_time 为 0，表示交易立即生效
                    true
                } else if tx.lock_time < 500_000_000 {
                    // lock_time 表示区块高度
                    current_height >= tx.lock_time
                } else {
                    // lock_time 表示时间戳
                    current_timestamp >= tx.lock_time
                }
            })
            .cloned()
            .collect();

        // 将有效的交易添加到新区块的交易列表
        new_block.transactions = valid_transactions;
        // 清空交易池
        self.transaction_pool.lock().unwrap().clear();
        // 计算新区块的 Merkle Root（假设交易列表已经设置）
        new_block.header.merkle_root = calculate_merkle_root(&new_block.transactions);
        println!("Mining block...");
        // 计算新区块的哈希值
        let mut hash = hash_block_header(&new_block.header);
        while !hash.starts_with(&[0; 32][..self.difficulty]) {
            new_block.header.nonce += 1;
            hash = hash_block_header(&new_block.header);
        }
        // 将新区块添加到区块链
        self.add_block(new_block);
    }

    pub fn add_block(&mut self, data: Block) {
        let mut new_block = data;
        println!("{:?}", self.blocks.len());
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

        // 将新区块添加到区块链
        self.blocks.push(new_block);
    }
}
