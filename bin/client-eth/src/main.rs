#![no_main]
valida_rs::entrypoint!(main);

use reva_client_executor::{io::ClientExecutorInput, ClientExecutor, EthereumVariant};
use std::io::{self, Read};

pub fn main() {
    // Read the input.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    let input = bincode::deserialize::<ClientExecutorInput>(&input).unwrap();

    // Execute the block.
    let executor = ClientExecutor;
    let header = executor.execute::<EthereumVariant>(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    // Commit the block hash.
    println!("{:?}", block_hash);
}
