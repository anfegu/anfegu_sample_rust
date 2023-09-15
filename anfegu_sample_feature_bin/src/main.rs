extern crate anfegu_sample_feature; 

use anfegu_sample_feature::blockchain;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let blockchain = blockchain::create_blockchain();

    println!("Blockchain:");
    for (i, block) in blockchain.iter().enumerate() {
        println!("Block {}: {:?}", i, block);
    }

    Ok(())
}