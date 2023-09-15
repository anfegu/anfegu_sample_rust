use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use rand::Rng;
use rand::distributions::Alphanumeric;

pub fn generate_key() -> String {
    let rng = rand::thread_rng();
    let key: Vec<u8> = rng.sample_iter(&Alphanumeric)
        .take(32)
        .collect(); 

    // Convert the Vec<u8> into a String
    let key_string = String::from_utf8(key).expect("Failed to convert key to string");

    key_string
}

pub fn encrypt_file(input_path: &str, output_path: &str, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut input_file = File::open(input_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    let encrypted_data: Vec<u8> = buffer
        .iter()
        .zip(key.as_bytes())
        .map(|(byte, key_byte)| byte ^ key_byte)
        .collect();

    let mut output_file = if Path::new(output_path).exists() {
        OpenOptions::new().append(true).open(output_path)?
    } else {
        File::create(output_path)?
    };

    output_file.write_all(&encrypted_data)?;

    Ok(encrypted_data)
}

pub fn decrypt_file(input_path: &str, output_path: &str, key: &str) -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open(input_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    // Decrypt the data (reverse the XOR operation)
    let decrypted_data: Vec<u8> = buffer
        .iter()
        .map(|byte| byte ^ key.as_bytes()[0])
        .collect();

    let mut output_file = if Path::new(output_path).exists() {
        OpenOptions::new().append(true).open(output_path)?
    } else {
        File::create(output_path)?
    };

    output_file.write_all(&decrypted_data)?;

    Ok(())
}