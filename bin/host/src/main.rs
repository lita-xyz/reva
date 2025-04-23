#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use reva_host_executor::{
    build_executor, create_eth_block_execution_strategy_factory,
    create_op_block_execution_strategy_factory, BlockExecutor, EthExecutorComponents,
    OpExecutorComponents,
};
use reva_provider::create_provider;

mod cli;
use cli::HostArgs;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Parse the command line arguments.
    let args = HostArgs::parse();
    let block_number = args.block_number;
    let mut config = args.as_config().await?;

    if config.cache_dir.is_none() {
        config.cache_dir = Some(PathBuf::from_str(".")?);
    }

    if config.chain.is_optimism() {
        let block_execution_strategy_factory =
            create_op_block_execution_strategy_factory(&config.genesis);
        let provider = config.rpc_url.as_ref().map(|url| create_provider(url.clone()));

        let executor = build_executor::<OpExecutorComponents<()>, _>(
            provider,
            block_execution_strategy_factory,
            config,
        )
        .await?;

        executor.execute(block_number).await?;
    } else {
        let block_execution_strategy_factory =
            create_eth_block_execution_strategy_factory(&config.genesis, config.custom_beneficiary);
        let provider = config.rpc_url.as_ref().map(|url| create_provider(url.clone()));

        let executor = build_executor::<EthExecutorComponents<()>, _>(
            provider,
            block_execution_strategy_factory,
            config,
        )
        .await?;

        executor.execute(block_number).await?;
    }

    Ok(())
}
