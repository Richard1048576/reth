use crate::{OpEthApi, OpEthApiError, eth::RpcNodeCore};
use reth_evm::{SpecFor, TxEnvFor};
use reth_rpc_eth_api::{
    FromEvmError, RpcConvert,
    helpers::{Call, EthCall, estimate::EstimateCall},
};

impl<N, Rpc> EthCall for OpEthApi<N, Rpc>
where
    N: RpcNodeCore,
    OpEthApiError: FromEvmError<N::Evm>,
    Rpc: RpcConvert<
            Primitives = N::Primitives,
            Error = OpEthApiError,
            TxEnv = TxEnvFor<N::Evm>,
            Spec = SpecFor<N::Evm>,
        >,
{
}

impl<N, Rpc> EstimateCall for OpEthApi<N, Rpc>
where
    N: RpcNodeCore,
    OpEthApiError: FromEvmError<N::Evm>,
    Rpc: RpcConvert<
            Primitives = N::Primitives,
            Error = OpEthApiError,
            TxEnv = TxEnvFor<N::Evm>,
            Spec = SpecFor<N::Evm>,
        >,
{
}

impl<N, Rpc> Call for OpEthApi<N, Rpc>
where
    N: RpcNodeCore,
    OpEthApiError: FromEvmError<N::Evm>,
    Rpc: RpcConvert<
            Primitives = N::Primitives,
            Error = OpEthApiError,
            TxEnv = TxEnvFor<N::Evm>,
            Spec = SpecFor<N::Evm>,
        >,
{
    #[inline]
    fn call_gas_limit(&self) -> u64 {
        self.inner.eth_api.gas_cap()
    }

    #[inline]
    fn max_simulate_blocks(&self) -> u64 {
        self.inner.eth_api.max_simulate_blocks()
    }
}
