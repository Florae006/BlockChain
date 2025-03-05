use ::block_chain::block_chain::Block;
use ::block_chain::serialization;
use ::block_chain::transaction;
use block_chain::block_chain;

fn main() {
    // 0. 创世区块
    let mut blockchain = block_chain::BlockChain::new();
    // 1. 新建交易：Alice声称转账给Bob 10个比特币
    let tx1 = transaction::Transaction::new(
        1,
        vec![transaction::TxIn::new([0; 32], vec![], 0)],
        vec![transaction::TxOut::new(10, vec![1; 32])],
        0,
    );
    // 2. 将交易添加到交易池
    blockchain.add_transaction(tx1);
    // 3. 挖矿并将节点加入账本
    blockchain.mine_block(4);
    // 4. 新建交易：Bob声称转账给Alice 5个比特币
}
