#![no_main]
valida_rs::entrypoint!(main);

use reva_client_executor::{executor::EthClientExecutor, io::EthClientExecutorInput};
use std::{
    io::{self},
    sync::Arc,
};

pub fn main() {
    // Read the input.
    let input = bincode::deserialize_from::<_, EthClientExecutorInput>(io::stdin()).unwrap();

    // Execute the block.
    let executor = EthClientExecutor::eth(
        Arc::new((&input.genesis).try_into().unwrap()),
        input.custom_beneficiary,
    );
    let header = executor.execute(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    // Commit the block hash.
    println!("{:?}", block_hash);
}
