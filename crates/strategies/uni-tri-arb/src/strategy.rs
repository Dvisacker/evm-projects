use super::types::{Action, Event};
use crate::state::PoolState;
use alloy::{providers::Provider, signers::Signer};
use alloy_chains::NamedChain;
use amms::sync::{self, checkpoint::sort_amms};
use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use shared::{addressbook::Addressbook, types::Cycle};
use std::{collections::HashSet, sync::Arc};
use tracing::info;

#[derive(Debug, Clone)]
pub struct UniTriArb<P: Provider + 'static, S: Signer> {
    addressbook: Addressbook,
    pub client: Arc<P>,
    pub pool_state: PoolState<P>,
    pub tx_signer: S,
    pub db_url: String,
}

impl<P: Provider + 'static, S: Signer> UniTriArb<P, S> {
    pub fn new(client: Arc<P>, signer: S, db_url: String) -> Self {
        let addressbook = Addressbook::load().unwrap();
        Self {
            addressbook,
            client: client.clone(),
            pool_state: PoolState::new(client.clone()),
            tx_signer: signer,
            db_url,
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

        self.pool_state
            .load_pools_from_db(&self.db_url)
            .await
            .unwrap();

        let weth = self.addressbook.get_weth(&NamedChain::Arbitrum).unwrap();
        let amms = self.pool_state.get_all_pools().await;
        let (mut uniswap_v2_pools, mut uniswap_v3_pools, _) = sort_amms(amms);

        sync::populate_amms(&mut uniswap_v2_pools, block_number, self.client.clone()).await?;
        sync::populate_amms(&mut uniswap_v3_pools, block_number, self.client.clone()).await?;
        let synced_amms = vec![uniswap_v2_pools, uniswap_v3_pools].concat();

        println!("Computing arbitrage cycles...");
        let arb_cycles = PoolState::<P>::get_cycles(
            &synced_amms,
            weth,
            weth,
            3,
            &vec![],
            &mut vec![],
            &mut HashSet::new(),
        );

        let profit_threshold = -0.01;
        let arb_cycles: Vec<Cycle> = arb_cycles
            .into_iter()
            .filter(|cycle| cycle.get_profit_perc() > profit_threshold)
            .collect();

        println!("Found {} arbitrage cycles", arb_cycles.len());
        for cycle in arb_cycles {
            println!("{}: Profit: {}", cycle, cycle.get_profit_perc());
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
                println!("New block: {:?}", event);
                let block_number = event.number.to::<u64>();
                self.pool_state
                    .update_block_number(block_number)
                    .await
                    .unwrap();
                return vec![];
            }
            Event::UniswapV2Swap(swap) => {
                println!(
                    "New UniswapV2 swap from {:?} on pool {:?}",
                    swap.sender, swap.to
                );
                return vec![];
            }
            Event::UniswapV3Swap(swap) => {
                println!(
                    "New UniswapV3 swap from {:?} on pool {:?}",
                    swap.sender, swap.recipient
                );
                return vec![];
            }
            Event::UniswapV2Sync(_) => {
                return vec![];
            }
        }
    }
}
