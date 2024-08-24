use artemis_core::collectors::event_collector::UniswapV2SwapEvent;
use artemis_core::{
    collectors::block_collector::NewBlock, executors::mempool_executor::SubmitTxToMempool,
};
use ethers::prelude::*;
use ethers::types::{Address, Transaction, H160, H256};
use serde::{Deserialize, Serialize};

/// Core Event enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Event {
    NewBlock(NewBlock),
    UniswapV2Swap(UniswapV2SwapEvent),
    UniswapOrder(Transaction),
}

/// Core Action enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Action {
    SubmitTx(SubmitTxToMempool),
}

#[derive(Debug, serde::Deserialize)]
pub struct PoolRecord {
    pub token_address: H160,
    pub uni_pool_address: H160,
    pub sushi_pool_address: H160,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniV2Dex {
    pub factory: Address,
    pub router: Address,
    pub fee: U256,
    pub init_code_hash: H256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniV2Pool {
    pub address: Address,
    pub token0: Address,
    pub token1: Address,
    pub reserve0: U256,
    pub reserve1: U256,
    pub router_fee: U256,
    pub fees0: U256, // fee for token0
    pub fees1: U256, // fee for token1
}
