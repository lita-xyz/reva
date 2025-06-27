use reva_client_executor::{executor::OpClientExecutor, io::OpClientExecutorInput};
use std::{
    io::{self},
    sync::Arc,
};

pub fn main() {
    // Read the input.
    let input = bincode::deserialize_from::<_, OpClientExecutorInput>(io::stdin()).unwrap();

    // Execute the block.
    let executor = OpClientExecutor::optimism(Arc::new((&input.genesis).try_into().unwrap()));
    let header = executor.execute(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    // Commit the block hash.
    println!("{:?}", block_hash);
}
