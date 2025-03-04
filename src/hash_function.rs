use ring::digest::{Context, Digest, SHA256};

use crate::block_chain;
use crate::serialization::serialize_bc;
use crate::transaction::Transaction;
use block_chain::BlockHeader;

/// 计算 SHA-256 哈希值
pub fn sha256_hash(data: &[u8]) -> Digest {
    let mut context = Context::new(&SHA256);
    context.update(data);
    context.finish()
}

/// 计算两个哈希值的组合哈希
pub fn hash_pair(a: &[u8], b: &[u8]) -> Digest {
    let mut context = Context::new(&SHA256);
    context.update(a);
    context.update(b);
    context.finish()
}

/// 计算 Merkle Root
pub fn calculate_merkle_root(transactions: &[Transaction]) -> [u8; 32] {
    if transactions.is_empty() {
        return [0; 32]; // 如果没有交易，返回全零的哈希值
    }

    // 计算每个交易的哈希值
    let mut hashes: Vec<[u8; 32]> = transactions
        .iter()
        .map(|tx| {
            let serialized = serialize_bc(tx).unwrap(); // 序列化交易
            sha256_hash(&serialized).as_ref().try_into().unwrap() // 计算哈希值
        })
        .collect();

    // 递归计算 Merkle Root
    while hashes.len() > 1 {
        let mut next_level = Vec::new();

        for i in (0..hashes.len()).step_by(2) {
            let left = &hashes[i];
            let right = if i + 1 < hashes.len() {
                &hashes[i + 1]
            } else {
                left // 如果交易数量为奇数，复制最后一个哈希值
            };

            let combined_hash = hash_pair(left, right);
            next_level.push(combined_hash.as_ref().try_into().unwrap());
        }

        hashes = next_level;
    }

    hashes[0] // 返回 Merkle Root
}

/// 计算区块头的哈希值
pub fn hash_block_header(header: &BlockHeader) -> [u8; 32] {
    let serialized = serialize_bc(header).unwrap(); // 假设 serialize_bc 是序列化函数
    let hash = sha256_hash(&serialized);
    hash.as_ref().try_into().unwrap()
}
