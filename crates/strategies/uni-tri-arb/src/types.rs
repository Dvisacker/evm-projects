use alloy::primitives::{Address, B256, U256};
use artemis_core::{
    collectors::block_collector::NewBlock, executors::mempool_executor::SubmitTxToMempool,
};
use bindings::iuniswapv2pair::IUniswapV2Pair;
use serde::{Deserialize, Serialize};

/// Core Event enum for the current strategy.
#[derive(Clone)]
pub enum Event {
    NewBlock(NewBlock),
    UniswapV2Swap(IUniswapV2Pair::Swap),
    UniswapV2Sync(IUniswapV2Pair::Sync),
}

/// Core Action enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Action {
    SubmitTx(SubmitTxToMempool),
}

#[derive(Debug, serde::Deserialize)]
pub struct PoolRecord {
    pub token_address: Address,
    pub uni_pool_address: Address,
    pub sushi_pool_address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniV2Dex {
    pub factory: Address,
    pub router: Address,
    pub fee: U256,
    pub init_code_hash: B256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniV2Pool {
    pub address: Address,
    pub token0: Address,
    pub token1: Address,
    pub reserve0: U256,
    pub reserve1: U256,
}
