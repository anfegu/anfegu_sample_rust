# Anfegu Sample 

This is a simple Rust project that demonstrates the creation of a blockchain and low-level file encryption/decryption using XOR encryption and advanced encryption using the Ring crate.

## Project Structure

The project is organized into two main Rust crates:

1. `anfegu_sample_feature`: This crate contains the core blockchain logic and data structures.

2. `anfegu_sample_feature_bin`: This crate serves as the entry point for the project and includes the main function.

## File Management

In this project, low-level file management is utilized for encrypting and decrypting data using XOR encryption. Here's how it works:

- The `encrypt_file` function reads data from an input file and encrypts it using XOR encryption. The encrypted data is then written to an output file, creating a new file if it doesn't exist or overwriting the existing one.

- The `decrypt_file` function reads encrypted data from an input file, decrypts it using XOR decryption, and appends the decrypted data to an output file. If the output file doesn't exist, it will be created.

## Encryption with the Ring Crate

Advanced encryption in this project is implemented using the Ring crate, a popular cryptography library in Rust. It provides secure and efficient encryption algorithms.

- The blockchain's data is encrypted using SHA-256 hashing provided by the Ring crate. This ensures data integrity and security within the blockchain.

## Usage

To build and run the project, follow these steps:

1. Clone the repository:

git clone https://github.com/anfegu/anfegu_sample_rust.git

2. Navigate to the project directory:

cd anfegu_sample_rust

3. Build the project:

cargo build

4. Run the project:

cargo run


This will create a simple blockchain and demonstrate both low-level file encryption/decryption with XOR encryption and advanced encryption using the Ring crate for blockchain data.

## License

This project is licensed under the MIT License - see the [LICENSE](https://opensource.org/license/mit/) file for details.
