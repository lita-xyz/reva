#![no_main]
valida_rs::entrypoint!(main);

use reva_client_executor::{io::ClientExecutorInput, ClientExecutor, LineaVariant};
use std::io::{self, Read};

pub fn main() {
    // Read the input.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let input = serde_json::de::from_slice::<ClientExecutorInput>(&input.as_slice()).unwrap();

    // Execute the block.
    let executor = ClientExecutor;
    let header = executor.execute::<LineaVariant>(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    // Commit the block hash.
    println!("{:?}", block_hash);
}
