use artemis_core::{
    collectors::block_collector::NewBlock, executors::mempool_executor::SubmitTxToMempool,
};
use ethers::types::H160;

/// Core Event enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Event {
    NewBlock(NewBlock),
    UniswapOrder,
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

#[derive(Debug, serde::Deserialize)]
pub struct V2V3PoolRecord {
    pub token_address: H160,
    pub v3_pool: H160,
    pub v2_pool: H160,
    pub weth_token0: bool,
}
