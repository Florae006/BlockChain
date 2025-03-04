use ::block_chain::serialization;
use block_chain::block_chain;

fn main() {
    let _block = block_chain::Block::new();
    let blockchain = block_chain::BlockChain::new();
    let bytes = serialization::serialize_bc(&blockchain).unwrap();
    let blockchain2: block_chain::BlockChain = serialization::deserialize_bc(&bytes).unwrap();
    println!("{:?}", blockchain2);
    println!("Hello, world!");
}
