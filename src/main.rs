use warp::Filter;

use ::block_chain::block_chain::BlockChain;
use ::block_chain::transaction::Transaction;

async fn start_server(blockchain: BlockChain, port: u16) {
    let blockchain = warp::any().map(move || blockchain.clone());

    // 创建交易
    let create_transaction = warp::path("transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain.clone())
        .map(|tx: Transaction, mut blockchain: BlockChain| {
            blockchain.add_transaction(tx.clone());
            blockchain.broadcast_transaction(tx, vec!["127.0.0.1:3031".to_string()]);
            warp::reply::json(&"Transaction created and broadcasted")
        });

    // 挖矿
    let mine = warp::path("mine")
        .and(warp::post())
        .and(blockchain.clone())
        .map(|mut blockchain: BlockChain| {
            blockchain.mine_block();
            warp::reply::json(&"New block mined")
        });

    // 查看区块链
    let get_chain = warp::path("chain")
        .and(warp::get())
        .and(blockchain.clone())
        .map(|blockchain: BlockChain| warp::reply::json(&blockchain.blocks));

    // 合并路由
    let routes = create_transaction.or(mine).or(get_chain);

    // 启动服务器
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

#[tokio::main]
async fn main() {
    // 创建区块链
    let blockchain = BlockChain::new(4);

    // 启动 HTTP 服务器
    let server_handle = tokio::spawn(start_server(blockchain.clone(), 3030));

    // 等待服务器关闭
    server_handle.await.unwrap();
}
