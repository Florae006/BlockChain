// 启动HTTP本地服务器，广播服务

use std::sync::Arc;
use std::sync::Mutex;
use warp::Filter;

use block_chain::block_chain;
use block_chain::transaction;
use block_chain::transaction::Transaction;

async fn start_server(node: Transaction, port: u16) {
    // 创建一个 Warp 过滤器
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    // 启动 Warp 服务器
    warp::serve(hello).run(([127, 0, 0, 1], port)).await;
}

#[tokio::main]
async fn main() {
    // 创建一个新的区块链
    let mut block_chain = block_chain::BlockChain::new();

    // 创建一个新的交易
    let tx_in = transaction::TxIn::new([0; 32], vec![0; 32], 0);
    let tx_out = transaction::TxOut::new(100, vec![0; 32]);
    let transaction = Transaction::new(1, vec![tx_in], vec![tx_out], 0);

    // 启动服务器
    start_server(transaction, 3030).await;
    // 添加交易到交易池
    block_chain.add_transaction(transaction);
}
