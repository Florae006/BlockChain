#[cfg(test)]
mod tests {
    use block_chain::block_chain::Block;
    use block_chain::block_chain::BlockChain;
    use block_chain::hash_function::sha256_hash;
    use block_chain::serialization::deserialize_bc as deserialize;
    use block_chain::serialization::serialize_bc as serialize;
    use serde::Serialize;
    use serde_json;

    #[derive(Serialize)]
    struct Transaction {
        sender: String,
        receiver: String,
        amount: u64,
    }

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
        let transaction: Transaction = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 100,
        };
        let serialized = serde_json::to_vec(&transaction).unwrap();
        let hash = sha256_hash(&serialized);
        assert_eq!(hash.as_ref().len(), 32);
    }
}
