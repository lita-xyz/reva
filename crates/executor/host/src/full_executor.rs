use std::{
    fmt::{Debug, Formatter},
    time::Duration,
};

use crate::{Config, ExecutorComponents, HostExecutor};
use alloy_provider::Provider;
use eyre::bail;
use reva_rpc_db::RpcDb;
use tokio::time::sleep;

pub async fn build_executor<C, P>(
    provider: Option<P>,
    evm_config: C::EvmConfig,
    config: Config,
) -> eyre::Result<FullExecutor<C, P>>
where
    C: ExecutorComponents,
    P: Provider<C::Network> + Clone,
{
    if let Some(provider) = provider {
        return Ok(FullExecutor::try_new(provider, evm_config, config).await?);
    }

    bail!("Either a RPC URL or a cache dir must be provided")
}

pub trait BlockExecutor<C: ExecutorComponents> {
    #[allow(async_fn_in_trait)]
    async fn execute(&self, block_number: u64) -> eyre::Result<()>;
}

pub struct FullExecutor<C, P>
where
    C: ExecutorComponents,
    P: Provider<C::Network> + Clone,
{
    provider: P,
    host_executor: HostExecutor<C::EvmConfig>,
    config: Config,
}

impl<C, P> FullExecutor<C, P>
where
    C: ExecutorComponents,
    P: Provider<C::Network> + Clone,
{
    pub async fn try_new(
        provider: P,
        evm_config: C::EvmConfig,
        config: Config,
    ) -> eyre::Result<Self> {
        Ok(Self { provider, host_executor: HostExecutor::new(evm_config), config })
    }

    pub async fn wait_for_block(&self, block_number: u64) -> eyre::Result<()> {
        let block_number = block_number.into();

        while self.provider.get_block_by_number(block_number).await?.is_none() {
            sleep(Duration::from_millis(100)).await;
        }
        Ok(())
    }
}

impl<C, P> BlockExecutor<C> for FullExecutor<C, P>
where
    C: ExecutorComponents,
    P: Provider<C::Network> + Clone,
{
    async fn execute(&self, block_number: u64) -> eyre::Result<()> {
        let rpc_db = RpcDb::new(self.provider.clone(), block_number - 1);

        // Execute the host.
        let client_input = self
            .host_executor
            .execute(
                block_number,
                &rpc_db,
                &self.provider,
                self.config.genesis.clone(),
                self.config.custom_beneficiary,
                self.config.opcode_tracking,
            )
            .await?;

        if let Some(ref cache_dir) = self.config.cache_dir {
            let input_folder = cache_dir.join(format!("input/{}", self.config.chain.id()));
            if !input_folder.exists() {
                std::fs::create_dir_all(&input_folder)?;
            }

            let input_path = input_folder.join(format!("{}.bin", block_number));
            let mut cache_file = std::fs::File::create(input_path)?;

            bincode::serialize_into(&mut cache_file, &client_input)?;
        }

        Ok(())
    }
}

impl<C, P> Debug for FullExecutor<C, P>
where
    C: ExecutorComponents,
    P: Provider<C::Network> + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FullExecutor").field("config", &self.config).finish()
    }
}
