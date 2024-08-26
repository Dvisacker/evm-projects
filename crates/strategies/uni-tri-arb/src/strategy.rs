use super::types::{Action, Event};
use crate::{addressbook::Addressbook, state::PoolState};
use alloy::{providers::Provider, signers::Signer};
use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use std::{collections::HashSet, sync::Arc};

#[derive(Debug, Clone)]
pub struct UniTriArb<P: Provider + 'static, S: Signer> {
    addressbook: Addressbook,
    client: Arc<P>,
    pub pool_state: PoolState<P>,
    tx_signer: S,
}

impl<P: Provider + 'static, S: Signer> UniTriArb<P, S> {
    /// Create a new instance of the strategy.
    pub fn new(client: Arc<P>, signer: S) -> Self {
        let addressbook = Addressbook::load().unwrap();
        Self {
            addressbook,
            client: client.clone(),
            pool_state: PoolState::new(client.clone()),
            tx_signer: signer,
        }
    }
}

#[async_trait]
impl<P: Provider + 'static, S: Signer + Send + Sync + 'static> Strategy<Event, Action>
    for UniTriArb<P, S>
{
    async fn init_state(&mut self) -> Result<()> {
        let weth = self.addressbook.get_weth("arbitrum").unwrap();
        let pools = self.addressbook.get_pools_by_chain("arbitrum");
        self.pool_state.add_pools(pools).await?;

        let pools = self.pool_state.get_all_pools().await;

        let arb_cycles = PoolState::<P>::get_cycles(
            pools.as_ref(),
            weth,
            weth,
            3,
            &vec![],
            &mut vec![],
            &mut HashSet::new(),
        );

        println!("Arb cycles: {:?}", arb_cycles);

        Ok(())
    }

    async fn sync_state(&mut self) -> Result<()> {
        self.pool_state
            .sync_pools()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to sync pools: {}", e))?;

        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders.
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::NewBlock(event) => {
                println!("New block: {:?}", event);

                println!("Syncing state...");
                // self.sync_state().await;

                return vec![];
            }
            Event::UniswapV2Swap(swap) => {
                println!("New swap from {:?} on pool {:?}", swap.sender, swap.to);
                return vec![];
            }
        }
    }
}
