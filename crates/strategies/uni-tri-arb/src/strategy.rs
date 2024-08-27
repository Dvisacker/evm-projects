use super::types::{Action, Event};
use crate::state::PoolState;
use alloy::{providers::Provider, signers::Signer};
use alloy_chains::NamedChain;
use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use shared::addressbook::Addressbook;
use std::{collections::HashSet, sync::Arc};
use tracing::info;

#[derive(Debug, Clone)]
pub struct UniTriArb<P: Provider + 'static, S: Signer> {
    addressbook: Addressbook,
    pub client: Arc<P>,
    pub pool_state: PoolState<P>,
    pub tx_signer: S,
    pub checkpoint_path: Option<&'static str>,
}

impl<P: Provider + 'static, S: Signer> UniTriArb<P, S> {
    pub fn new(client: Arc<P>, signer: S, checkpoint_path: Option<&'static str>) -> Self {
        let addressbook = Addressbook::load().unwrap();
        Self {
            addressbook,
            client: client.clone(),
            pool_state: PoolState::new(client.clone()),
            tx_signer: signer,
            checkpoint_path,
        }
    }
}

#[async_trait]
impl<P: Provider + 'static, S: Signer + Send + Sync + 'static> Strategy<Event, Action>
    for UniTriArb<P, S>
{
    async fn init_state(&mut self) -> Result<()> {
        println!("Initializing state...");

        // update block number
        let block_number = self.client.get_block_number().await.unwrap();
        self.pool_state
            .update_block_number(block_number)
            .await
            .unwrap();

        // update pool state
        if let Some(checkpoint_path) = self.checkpoint_path {
            println!("Loading pools from checkpoint...");
            self.pool_state
                .load_pools_from_checkpoint(checkpoint_path)
                .await
                .unwrap();
        } else {
            println!("Loading pools from addressbook...");
            let pools = self.addressbook.get_pools_by_chain(&NamedChain::Arbitrum);
            self.pool_state.add_pools(pools).await.unwrap();
        }

        let weth = self.addressbook.get_weth(&NamedChain::Arbitrum).unwrap();
        let pools = self.pool_state.get_all_pools().await;

        println!("Computing arbitrage cycles...");
        let arb_cycles = PoolState::<P>::get_cycles(
            pools.as_ref(),
            weth,
            weth,
            3,
            &vec![],
            &mut vec![],
            &mut HashSet::new(),
        );

        println!("Found {} arbitrage cycles", arb_cycles.len());
        for cycle in arb_cycles {
            let profit = cycle.get_profit();
            println!("Cycle: {}: Profit: {}", cycle, profit);
        }

        Ok(())
    }

    async fn sync_state(&mut self) -> Result<()> {
        info!("Syncing state...");
        self.pool_state
            .update_pools()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to sync pools: {}", e))?;

        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::NewBlock(event) => {
                let block_number = event.number.to::<u64>();
                self.pool_state
                    .update_block_number(block_number)
                    .await
                    .unwrap();
                println!("New block: {:?}", event);
                return vec![];
            }
            Event::UniswapV2Swap(swap) => {
                println!("New swap from {:?} on pool {:?}", swap.sender, swap.to);
                return vec![];
            }
            Event::UniswapV2Sync(sync) => {
                return vec![];
            }
        }
    }
}
