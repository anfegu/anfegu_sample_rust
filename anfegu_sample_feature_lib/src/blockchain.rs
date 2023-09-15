use ring::digest;

use crate::encryption::{generate_key, encrypt_file, decrypt_file};

#[derive(Debug, Clone)]
pub struct Block {
    data: Vec<u8>,
    prev_hash: String,
    hash: String,
}

impl Block {
    fn new(data: Vec<u8>, prev_hash: &str) -> Block {
        let mut context = digest::Context::new(&digest::SHA256);
        let data_str = String::from_utf8_lossy(&data).to_string();
        let input = format!("{}{}", data_str, prev_hash);

        context.update(input.as_bytes());
        let hash = context.finish();

        Block {
            data,
            prev_hash: prev_hash.to_string(),
            hash: hex::encode(hash),
        }
    }
}

pub fn create_blockchain() -> Vec<Block> {
    let mut blockchain = vec![];
    let key = generate_key();

    let mut prev_hash = String::from("GenesisBlock");

    for _i in 0..3 {
        let encrypted_data = encrypt_file("input.txt", "encrypted.txt", &key).unwrap();
        let block = Block::new(encrypted_data.clone(), &prev_hash);
        prev_hash = block.hash.clone();
        blockchain.push(block);

        decrypt_file("encrypted.txt", "decrypted.txt", &key).unwrap();
    }

    blockchain
}