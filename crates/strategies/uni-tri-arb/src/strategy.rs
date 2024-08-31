use super::types::{Action, Event};
use crate::state::PoolState;
use alloy::{providers::Provider, signers::Signer, sol_types::SolEvent};
use alloy_chains::NamedChain;
use amms::{
    amm::{AutomatedMarketMaker, AMM},
    sync::{self, checkpoint::sort_amms},
};
use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings::{iuniswapv2pair::IUniswapV2Pair, iuniswapv3pool::IUniswapV3Pool};
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
        let weth = addressbook.get_weth(&NamedChain::Arbitrum).unwrap();
        Self {
            addressbook,
            client: client.clone(),
            pool_state: PoolState::new(client.clone(), vec![weth]),
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

        let amms = self.pool_state.get_all_pools().await;
        let (mut uniswap_v2_pools, mut uniswap_v3_pools, _) = sort_amms(amms);

        sync::populate_amms(&mut uniswap_v2_pools, block_number, self.client.clone()).await?;
        sync::populate_amms(&mut uniswap_v3_pools, block_number, self.client.clone()).await?;
        let synced_amms = vec![uniswap_v2_pools, uniswap_v3_pools].concat();

        self.pool_state.set_pools(synced_amms);
        self.pool_state.update_cycles();
        // let profit_threshold = -0.99;

        let arb_cycles = self
            .pool_state
            .cycles
            .iter()
            .map(|entry| entry.1.clone())
            .collect::<Vec<_>>();

        println!("Printing pools...");
        self.pool_state.print_pools();

        println!("Found {} arbitrage cycles", arb_cycles.len());
        for cycle in arb_cycles {
            println!("{}: Profit: {}", cycle, cycle.get_profit_perc());
        }

        // println!("Computing arbitrage cycles...");
        // let arb_cycles = PoolState::<P>::get_cycles(
        //     &synced_amms,
        //     weth,
        //     weth,
        //     3,
        //     &vec![],
        //     &mut vec![],
        //     &mut HashSet::new(),
        // );

        // let arb_cycles: Vec<Cycle> = arb_cycles
        //     .into_iter()
        //     .collect();

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
                // println!("New block: {:?}", event);
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
            Event::Log(log) => {
                let pool_address = log.address();
                // let tx_hash = log.transaction_hash.unwrap();
                // check if log is a swap in uniswap v2 or v3
                if log.topics()[0] == IUniswapV2Pair::Swap::SIGNATURE_HASH {
                    let pool = self.pool_state.pools.get_mut(&pool_address);
                    if pool.is_some() {
                        let mut pool_ref = pool.unwrap();
                        let pool = pool_ref.value_mut();
                        println!("New uniswap v2 swap on pool {:?}", pool.name());
                        let mut amm_slice: &mut [AMM] = std::slice::from_mut(pool);
                        sync::populate_amms(
                            &mut amm_slice,
                            self.pool_state.block_number,
                            self.client.clone(),
                        )
                        .await
                        .unwrap();

                        let updated_cycles = self.pool_state.get_updated_cycles(amm_slice.to_vec());
                        println!("Found {} updated cycles", updated_cycles.len());
                        for cycle in updated_cycles {
                            println!("{}: Profit: {}", cycle, cycle.get_profit_perc());
                        }
                    } else {
                        println!("New uniswap v2 swap on unknown pool {:?}", pool_address);
                    }
                } else if log.topics()[0] == IUniswapV3Pool::Swap::SIGNATURE_HASH {
                    let pool = self.pool_state.pools.get_mut(&pool_address);
                    if pool.is_some() {
                        let mut pool_ref = pool.unwrap();
                        let pool = pool_ref.value_mut();
                        println!("New uniswap v3 swap on pool {:?}", pool.name());
                        let amm_slice: &mut [AMM] = std::slice::from_mut(pool);
                        sync::populate_amms(
                            amm_slice,
                            self.pool_state.block_number,
                            self.client.clone(),
                        )
                        .await
                        .unwrap();

                        let updated_cycles = self.pool_state.get_updated_cycles(amm_slice.to_vec());
                        println!("Found {} updated cycles", updated_cycles.len());
                        for cycle in updated_cycles {
                            println!("{}: Profit: {}", cycle, cycle.get_profit_perc());
                        }
                    } else {
                        println!("New uniswap v3 swap on unknown pool {:?}", pool_address);
                    }

                    // if let Some(p) = updated_pool {
                    //     println!("New uniswap v3 swap on pool {:?}", p.name());
                    //     // let mut amm_slice: &mut [AMM] = std::slice::from_mut(pool);
                    //     sync::populate_amms(
                    //         &mut [updated_pool],
                    //         self.pool_state.block_number,
                    //         self.client.clone(),
                    //     )
                    //     .await
                    //     .unwrap();
                    // }

                    //     self.pool_state.set_pools(vec![pool.clone()]);
                    //     println!("Set pools");
                }
                return vec![];
            }
        }
    }
}
