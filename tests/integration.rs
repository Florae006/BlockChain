#[cfg(test)]
mod tests {
    use block_chain::block_chain::{Block, BlockChain};
    use block_chain::hash_function::sha256_hash;
    use block_chain::serialization::{deserialize_bc, serialize_bc};
    use block_chain::transaction::{Transaction, TxIn, TxOut};
    use ring::rand::SystemRandom;
    use ring::signature::Ed25519KeyPair;

    #[test]
    fn test_block_chain() {
        let mut block_chain = BlockChain::new();
        block_chain.add_block(Block::new());
        assert_eq!(block_chain.blocks.len(), 1);
        block_chain.add_block(Block::new());
        assert_eq!(block_chain.blocks.len(), 2);
    }
    #[test]
    fn test_add_block() {
        let mut block_chain = BlockChain::new();
        block_chain.add_block(Block::new());
        assert_eq!(block_chain.blocks.len(), 1);
        block_chain.add_block(Block::new());
        assert_eq!(block_chain.blocks.len(), 2);
    }
    #[test]
    fn test_serialize_deserialize() {
        let blockchain: Vec<String> = vec![
            "Block1".to_string(),
            "Block2".to_string(),
            "Block3".to_string(),
        ];

        // 序列化
        let serialized: Vec<u8> = serialize_bc(&blockchain).unwrap();

        // 反序列化
        let deserialized: Result<Vec<String>, _> = deserialize_bc(&serialized);

        match deserialized {
            Ok(data) => assert_eq!(blockchain, data),
            Err(err) => panic!("Deserialization failed: {:?}", err),
        }
    }

    #[test]
    fn test_hash_function() {
        let data = b"hello world";
        let hash = sha256_hash(data);
        // 预期的 SHA-256 哈希值（可以通过在线工具或命令行计算）
        let expected_hash =
            hex::decode("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
                .expect("Failed to decode expected hash");

        // 比较计算出的哈希值和预期值
        assert_eq!(
            hash.as_ref(),
            expected_hash.as_slice(),
            "Hashes do not match"
        );
    }
    #[test]
    fn test_sign_and_verify() {
        // 生成一个随机的 Ed25519 密钥对
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();

        // 创建一个交易
        let tx_in = TxIn::new([0; 32], Vec::new(), 0);
        let tx_out = TxOut::new(100, Vec::new());
        let mut tx = Transaction::new(1, vec![tx_in], vec![tx_out], 0);

        // 对交易进行签名
        tx.sign(&key_pair, 0);

        // 验证签名
        assert!(tx.verify_signature(0), "Signature verification failed");

        // 篡改交易数据，验证签名是否失败
        let mut tampered_tx = tx.clone();
        tampered_tx.outputs[0].value = 200; // 修改交易输出
        assert!(
            !tampered_tx.verify_signature(0),
            "Signature verification should fail after tampering"
        );
    }
}
