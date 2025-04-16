#![no_main]
valida_rs::entrypoint!(main);

use reva_client_executor::{ClientExecutor, EthereumVariant};
use std::io::{self};

pub fn main() {
    // Read the input.
    let input = ciborium::from_reader(io::stdin()).unwrap();

    // Execute the block.
    let executor = ClientExecutor;
    let header = executor.execute::<EthereumVariant>(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    // Commit the block hash.
    println!("{:?}", block_hash);
}
