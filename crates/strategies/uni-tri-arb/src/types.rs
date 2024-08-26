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
