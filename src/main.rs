use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use warp::Filter;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        }
    }

    fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        hex::encode(hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}

#[derive(Clone)]
struct Blockchain {
    chain: Vec<Block>,
    transaction_pool: Arc<Mutex<VecDeque<Transaction>>>,
    difficulty: usize,
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let genesis_block = Block::new(0, vec![], "0".to_string());
        let mut blockchain = Blockchain {
            chain: vec![genesis_block],
            transaction_pool: Arc::new(Mutex::new(VecDeque::new())),
            difficulty,
        };
        blockchain.chain[0].hash = blockchain.chain[0].calculate_hash();
        blockchain
    }

    fn add_transaction(&self, tx: Transaction) {
        let mut pool = self.transaction_pool.lock().unwrap();
        pool.push_back(tx);
    }

    fn mine(&mut self) {
        let mut pool = self.transaction_pool.lock().unwrap();
        let transactions: Vec<Transaction> = pool.drain(..).collect();

        if !transactions.is_empty() {
            let previous_block = self.chain.last().unwrap();
            let mut new_block = Block::new(
                previous_block.index + 1,
                transactions,
                previous_block.hash.clone(),
            );
            new_block.mine_block(self.difficulty);
            self.chain.push(new_block);
        }
    }

    fn broadcast_transaction(&self, tx: Transaction, peers: Vec<String>) {
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
}

async fn start_server(blockchain: Blockchain, port: u16) {
    let blockchain = warp::any().map(move || blockchain.clone());

    // 创建交易
    let create_transaction = warp::path("transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain.clone())
        .map(|tx: Transaction, blockchain: Blockchain| {
            blockchain.add_transaction(tx.clone());
            blockchain.broadcast_transaction(tx, vec!["127.0.0.1:3031".to_string()]);
            warp::reply::json(&"Transaction created and broadcasted")
        });

    // 挖矿
    let mine = warp::path("mine")
        .and(warp::post())
        .and(blockchain.clone())
        .map(|mut blockchain: Blockchain| {
            blockchain.mine();
            warp::reply::json(&"New block mined")
        });

    // 查看区块链
    let get_chain = warp::path("chain")
        .and(warp::get())
        .and(blockchain.clone())
        .map(|blockchain: Blockchain| warp::reply::json(&blockchain.chain));

    // 合并路由
    let routes = create_transaction.or(mine).or(get_chain);

    // 启动服务器
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

#[tokio::main]
async fn main() {
    // 创建区块链
    let blockchain = Blockchain::new(4);

    // 启动 HTTP 服务器
    let server_handle = tokio::spawn(start_server(blockchain.clone(), 3030));

    // 等待服务器关闭
    server_handle.await.unwrap();
}
