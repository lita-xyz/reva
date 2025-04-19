use alloy_provider::ReqwestProvider;
use clap::Parser;
use reva_client_executor::{
    ChainVariant, CHAIN_ID_ETH_MAINNET, CHAIN_ID_LINEA_MAINNET, CHAIN_ID_OP_MAINNET,
};
use reva_host_executor::HostExecutor;
use std::path::Path;
use url::Url;

/// The arguments for the host executable.
#[derive(Debug, Clone, Parser)]
struct HostArgs {
    /// The block number of the block to execute.
    #[clap(long)]
    block_number: u64,
    /// The rpc url used to fetch data about the block.
    #[clap(long)]
    rpc_url: Option<Url>,
    /// The chain ID.
    #[clap(long)]
    chain_id: Option<u64>,
}

#[tokio::main]
async fn main() {
    // Initialize the environment variables.
    dotenv::dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // Parse the command line arguments.
    let args = HostArgs::parse();

    let variant = match args.chain_id {
        Some(CHAIN_ID_ETH_MAINNET) => ChainVariant::Ethereum,
        Some(CHAIN_ID_OP_MAINNET) => ChainVariant::Optimism,
        Some(CHAIN_ID_LINEA_MAINNET) => ChainVariant::Linea,
        _ => {
            panic!("unknown chain ID: {}", args.chain_id.unwrap());
        }
    };

    match args.rpc_url {
        Some(rpc_url) => {
            // Setup the provider.
            let provider = ReqwestProvider::new_http(rpc_url);

            // Setup the host executor.
            let host_executor = HostExecutor::new(provider);

            // Execute the host.
            let client_input = host_executor
                .execute(args.block_number, variant)
                .await
                .expect("failed to execute host");

            let input_folder = Path::new("input").join(format!("{}", args.chain_id.unwrap()));
            if !input_folder.exists() {
                std::fs::create_dir_all(&input_folder).unwrap();
            }

            let input_path = input_folder.join(format!("{}.bin", args.block_number));
            let mut cache_file = std::fs::File::create(input_path).unwrap();
            ciborium::into_writer(&client_input, &mut cache_file).unwrap();
        }
        None => {
            panic!("RPC URL not provided")
        }
    };
}
