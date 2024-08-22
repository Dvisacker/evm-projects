use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use anyhow::Result;
use artemis_core::types::Strategy;

use ethers::signers::Signer;

use ethers::providers::Middleware;
use ethers::types::H160;

use super::types::{Action, Event};

/// Information about a uniswap v2 pool.
#[derive(Debug, Clone)]
pub struct V2PoolInfo {
    /// Address of the v2 pool.
    pub v2_pool: H160,
    /// Whether the pool has weth as token0.
    pub is_weth_token0: bool,
}

#[derive(Debug, Clone)]
pub struct UniTriArb<M, S> {
    /// Ethers client.
    client: Arc<M>,
    /// Maps uni v3 pool address to v2 pool information.
    pool_map: HashMap<H160, V2PoolInfo>,
    /// Signer for transactions.
    tx_signer: S,
}

impl<M: Middleware + 'static, S: Signer> UniTriArb<M, S> {
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
impl<M: Middleware + 'static, S: Signer + 'static> Strategy<Event, Action> for UniTriArb<M, S> {
    /// Initialize the strategy. This is called once at startup, and loads
    /// pool information into memory.
    async fn sync_state(&mut self) -> Result<()> {
        // Read pool information from csv file.
        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders.
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::NewBlock(event) => {
                return vec![];
            }
            Event::UniswapOrder => {
                return vec![];
            }
        }
    }
}
