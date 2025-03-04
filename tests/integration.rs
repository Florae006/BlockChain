#[cfg(test)]
mod tests {
    use block_chain::block_chain::{Block, BlockChain};
    use block_chain::hash_function::sha256_hash;
    use block_chain::serialization::deserialize_bc as deserialize;
    use block_chain::serialization::serialize_bc as serialize;
    use block_chain::transaction::{Transaction, TxIn, TxOut};
    use ring::signature::Ed25519KeyPair;
    use serde::Serialize;
    use serde_json;

    #[test]
    fn test_block_chain() {
        let mut block_chain = BlockChain::new();
        block_chain.add_block(Block::new());
        assert_eq!(block_chain.blocks.len(), 1);
    }
    #[test]
    fn test_serialize() {
        let mut block_chain = BlockChain::new();
        block_chain.add_block(Block::new());
        let bytes = serialize(&block_chain).unwrap();
        let block_chain2: BlockChain = bincode::deserialize(&bytes).unwrap();
        assert_eq!(block_chain2.blocks.len(), 1);
    }

    #[test]
    fn test_serialize_and_deserialize() {
        let mut block_chain = BlockChain::new();
        block_chain.add_block(Block::new());
        let bytes = serialize(&block_chain).unwrap();
        let block_chain2: BlockChain = deserialize(&bytes).unwrap();
        assert_eq!(block_chain2.blocks.len(), 1);
    }

    #[test]
    fn test_hash_function() {
        let data = b"hello world";
        let hash = sha256_hash(data);
        assert_eq!(hash.as_ref().len(), 32);
    }
    #[test]
    fn test_signature() {
        let tx_in = TxIn {
            previous_output: [0; 32],
            script_sig: vec![],
            sequence: 0xFFFFFFFF,
        };

        // 创建一个交易输出
        let tx_out = TxOut {
            value: 100,
            script_pubkey: vec![],
        };

        // 创建一个交易
        let mut tx = Transaction::new(1, vec![tx_in], vec![tx_out], 0);

        // 生成密钥对
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();

        // 签名交易
        tx.sign(&key_pair);

        // 验证签名
        let message = serde_json::to_vec(&tx).unwrap();
        let signature = tx.inputs[0].script_sig.split_at(64).0;
        let public_key = tx.inputs[0].script_sig.split_at(64).1;
        let public_key =
            ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key);
        let is_valid = public_key.verify(&message, signature).is_ok();
        assert!(is_valid);
    }
}
