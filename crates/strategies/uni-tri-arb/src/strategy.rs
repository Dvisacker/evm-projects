use std::collections::HashMap;
use std::sync::Arc;

use alloy::{primitives::Address, providers::Provider, signers::Signer};
use async_trait::async_trait;

use anyhow::Result;
use artemis_core::types::Strategy;
use bindings::iuniswapv2pair::IUniswapV2Pair;

// use ethers::signers::Signer;

// use ethers::providers::Middleware;
// use ethers::types::H160;

use super::types::{Action, Event};

/// Information about a uniswap v2 pool.
#[derive(Debug, Clone)]
pub struct V2PoolInfo {
    /// Address of the v2 pool.
    pub v2_pool: Address,
    /// Whether the pool has weth as token0.
    pub is_weth_token0: bool,
}

#[derive(Debug, Clone)]
pub struct UniTriArb<M, S> {
    client: Arc<M>,
    pool_map: HashMap<Address, V2PoolInfo>,
    tx_signer: S,
}

impl<M: Provider + 'static, S: Signer> UniTriArb<M, S> {
    /// Create a new instance of the strategy.
    pub fn new(client: Arc<M>, signer: S) -> Self {
        Self {
            client: client.clone(),
            pool_map: HashMap::new(),
            tx_signer: signer,
        }
    }
}

#[async_trait]
impl<M: Provider + 'static, S: Signer + Send + Sync + 'static> Strategy<Event, Action>
    for UniTriArb<M, S>
{
    async fn sync_state(&mut self) -> Result<()> {
        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders.
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::NewBlock(event) => {
                println!("New block: {:?}", event);
                return vec![];
            }
            Event::UniswapV2Swap(swap) => {
                println!("New swap from {:?} on pool {:?}", swap.sender, swap.to);
                return vec![];
            }
        }
    }
}
