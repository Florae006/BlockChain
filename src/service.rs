use warp::Filter;

use ::block_chain::block_chain::BlockChain;
use ::block_chain::transaction::Transaction;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

#[derive(Deserialize)]
struct CreateTransactionRequest {
    lock_time: u32,
    value: u64,
}
async fn start_server(blockchain: Arc<AsyncMutex<BlockChain>>, port: u16) {
    let blockchain = warp::any().map(move || blockchain.clone());

    // 创建交易
    let create_transaction = warp::path("transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain.clone())
        .and_then(
            |req: CreateTransactionRequest, blockchain: Arc<AsyncMutex<BlockChain>>| async move {
                let tx = Transaction::new(req.value, req.lock_time);
                let mut blockchain = blockchain.lock().await;
                blockchain.add_transaction(tx.clone());
                blockchain.broadcast_transaction(tx, vec!["127.0.0.1:3031".to_string()]);
                Ok::<_, warp::Rejection>(warp::reply::json(&"Transaction created and broadcasted"))
            },
        );

    // 挖矿
    let mine = warp::path("mine")
        .and(warp::post())
        .and(blockchain.clone())
        .and_then(|blockchain: Arc<AsyncMutex<BlockChain>>| async move {
            let mut blockchain = blockchain.lock().await;
            blockchain.mine_block();
            Ok::<_, warp::Rejection>(warp::reply::json(&"New block mined"))
        });

    // 查看区块链
    let get_chain = warp::path("chain")
        .and(warp::get())
        .and(blockchain.clone())
        .and_then(|blockchain: Arc<AsyncMutex<BlockChain>>| async move {
            let blockchain = blockchain.lock().await;
            Ok::<_, warp::Rejection>(warp::reply::json(&*blockchain))
        });

    // 查看区块链区块部分
    let get_blocks = warp::path("blocks")
        .and(warp::get())
        .and(blockchain.clone())
        .and_then(|blockchain: Arc<AsyncMutex<BlockChain>>| async move {
            let blockchain = blockchain.lock().await;
            Ok::<_, warp::Rejection>(warp::reply::json(&blockchain.blocks))
        });

    // 查看区块链交易池
    let get_transaction_pool = warp::path("pool")
        .and(warp::get())
        .and(blockchain.clone())
        .and_then(|blockchain: Arc<AsyncMutex<BlockChain>>| async move {
            let blockchain = blockchain.lock().await;
            let pool = blockchain.transaction_pool.lock().unwrap();
            Ok::<_, warp::Rejection>(warp::reply::json(&*pool))
        });

    // 合并路由
    let routes = create_transaction
        .or(mine)
        .or(get_chain)
        .or(get_blocks)
        .or(get_transaction_pool);

    // 启动服务器
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

#[tokio::main]
async fn main() {
    // 创建区块链
    let blockchain = Arc::new(AsyncMutex::new(BlockChain::new(1)));

    // 启动 HTTP 服务器
    let server_handle = tokio::spawn(start_server(blockchain.clone(), 3030));

    // 等待服务器关闭
    server_handle.await.unwrap();
}
