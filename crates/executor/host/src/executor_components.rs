use std::marker::PhantomData;

use alloy_evm::EthEvmFactory;
use alloy_network::Ethereum;
use alloy_provider::Network;
use op_alloy_network::Optimism;
use reth_ethereum_primitives::EthPrimitives;
use reth_evm::ConfigureEvm;
use reth_evm_ethereum::EthEvmConfig;
use reth_optimism_evm::OpEvmConfig;
use reth_optimism_primitives::OpPrimitives;
use reth_primitives_traits::NodePrimitives;
use reva_client_executor::{
    custom::CustomEvmFactory, IntoInput, IntoPrimitives, ValidateBlockPostExecution,
};
use serde::de::DeserializeOwned;

pub trait ExecutorComponents {
    type Network: Network;

    type Primitives: NodePrimitives
        + DeserializeOwned
        + IntoPrimitives<Self::Network>
        + IntoInput
        + ValidateBlockPostExecution;

    type EvmConfig: ConfigureEvm<Primitives = Self::Primitives>;
}

#[derive(Debug, Default)]
pub struct EthExecutorComponents<H, P = ()> {
    phantom: PhantomData<(H, P)>,
}

impl<H> ExecutorComponents for EthExecutorComponents<H> {
    type Network = Ethereum;

    type Primitives = EthPrimitives;

    type EvmConfig = EthEvmConfig<CustomEvmFactory<EthEvmFactory>>;
}

#[derive(Debug, Default)]
pub struct OpExecutorComponents<H> {
    phantom: PhantomData<H>,
}

impl<H> ExecutorComponents for OpExecutorComponents<H> {
    type Network = Optimism;

    type Primitives = OpPrimitives;

    type EvmConfig = OpEvmConfig;
}
