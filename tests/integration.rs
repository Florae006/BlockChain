#[cfg(test)]
mod tests {
    use block_chain::block_chain::Block;
    use block_chain::block_chain::BlockChain;
    use block_chain::serialization::serialize_blockchain as serialize;
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
}
