use crate::state::State;

use super::types::{Action, Event};
use alloy::{
    dyn_abi::DynSolValue, primitives::Address, providers::Provider, rpc::types::Log,
    signers::Signer, sol_types::SolEvent,
};
use alloy_chains::Chain;
use amms::{
    amm::{
        uniswap_v2::{
            batch_request::{fetch_v2_pool_data_batch_request, populate_v2_pool_data},
            UniswapV2Pool,
        },
        uniswap_v3::{
            batch_request::{fetch_v3_pool_data_batch_request, populate_v3_pool_data},
            IUniswapV3Pool, UniswapV3Pool,
        },
        AutomatedMarketMaker, AMM,
    },
    sync::{self},
};
use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings::iuniswapv2pair::IUniswapV2Pair;
use db::{
    establish_connection,
    models::{db_pool::DbPool, NewDbUniV2Pool},
    queries::{
        uni_v2_pool::{batch_upsert_uni_v2_pools, get_uni_v2_pools},
        uni_v3_pool::{batch_upsert_uni_v3_pools, get_uni_v3_pools},
    },
};
use db::{models::NewDbUniV3Pool, queries::exchange::get_exchanges_by_chain};
use diesel::PgConnection;
use shared::{addressbook::Addressbook, amm_utils::db_pools_to_amms};
use std::sync::Arc;
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct UniTriArb<P: Provider + 'static, S: Signer> {
    pub chain: Chain,
    pub client: Arc<P>,
    pub state: State<P>,
    pub tx_signer: S,
    pub db_url: String,
}

impl<P: Provider + 'static, S: Signer> UniTriArb<P, S> {
    pub fn new(chain: Chain, client: Arc<P>, signer: S, db_url: String) -> Self {
        let addressbook = Addressbook::load().unwrap();
        let weth = addressbook.get_weth(&chain.named().unwrap()).unwrap();
        Self {
            chain,
            client: client.clone(),
            state: State::new(client.clone(), vec![weth]),
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
        info!("Initializing state...");

        let block_number = self.client.get_block_number().await.unwrap();
        let chain = self.chain.named().unwrap();
        self.state.update_block_number(block_number).await.unwrap();

        let active_v2_pools = get_uni_v2_pools(
            &mut establish_connection(&self.db_url),
            Some(&chain.to_string()),
            None,
            None,
            None,
            Some(true),
        )
        .unwrap()
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<DbPool>>();

        let active_v3_pools = get_uni_v3_pools(
            &mut establish_connection(&self.db_url),
            Some(&chain.to_string()),
            None,
            None,
            None,
            Some(true),
        )
        .unwrap()
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<DbPool>>();

        let active_camelot_v3_pools = get_uni_v3_pools(
            &mut establish_connection(&self.db_url),
            Some(&chain.to_string()),
            None,
            None,
            None,
            Some(true),
        )
        .unwrap()
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<DbPool>>();

        let mut active_v2_amms = db_pools_to_amms(&active_v2_pools)?;
        let mut active_v3_amms = db_pools_to_amms(&active_v3_pools)?;
        let mut active_camelot_v3_amms = db_pools_to_amms(&active_camelot_v3_pools)?;

        sync::populate_amms(&mut active_v2_amms, block_number, self.client.clone()).await?;
        sync::populate_amms(&mut active_v3_amms, block_number, self.client.clone()).await?;
        sync::populate_amms(
            &mut active_camelot_v3_amms,
            block_number,
            self.client.clone(),
        )
        .await?;

        let synced_amms = vec![active_v2_amms, active_v3_amms, active_camelot_v3_amms].concat();
        self.state.set_pools(synced_amms);

        info!("Updated pools: {:?}", self.state.pools);

        let arb_cycles = self.state.update_cycles();

        info!("{} arbitrage cycles", arb_cycles.len());
        for cycle in arb_cycles {
            info!("{}: Profit: {}", cycle, cycle.get_profit_perc());
        }

        Ok(())

        // info!("{:?} active pools", active_pools.len());

        // let inactive_v2_pools = get_uni_v2_pools(
        //     &mut establish_connection(&self.db_url),
        //     Some(&chain.to_string()),
        //     None,
        //     None,
        //     None,
        //     None,
        // )
        // .unwrap();

        // let inactive_v2_pools = info!("{:?} inactive pools", inactive_v2_pools.len());

        // let inactive_amms = db_pools_to_amms(
        //     &inactive_v2_pools
        //         .into_iter()
        //         .map(|p| p.into())
        //         .collect::<Vec<DbPool>>(),
        // )?;

        // self.state.set_inactive_pools(inactive_amms);

        // let uniswap_v2_amms = db_pools_to_amms(&active_pools)?;

        // let active_amms = db_pools_to_amms(&active_pools)?;

        // let (mut uniswap_v2_pools, mut uniswap_v3_pools, _, mut camelot_v3_pools) =
        //     sort_amms(active_amms);

        // take only 50 uniswap v3 pools for testing
        // let mut uniswap_v3_pools: Vec<AMM> = uniswap_v3_pools
        //     .into_iter()
        //     .filter(|pool| matches!(pool, AMM::UniswapV3Pool(_)))
        //     .take(50)
        //     .collect();

        // sync::populate_amms(&mut uniswap_v3_pools, block_number, self.client.clone()).await?;
        // sync::populate_amms(&mut camelot_v3_pools, block_number, self.client.clone()).await?;

        // for pool in &mut uniswap_v3_pools {
        //     if let AMM::UniswapV3Pool(uniswap_v3_pool) = pool {
        //         let tick_start = uniswap_v3_pool.tick - 20;
        //         let num_ticks = 40;
        //         let (tick_data, _) = get_uniswap_v3_tick_data_batch_request(
        //             &uniswap_v3_pool,
        //             tick_start,
        //             false,
        //             num_ticks,
        //             Some(block_number),
        //             self.client.clone(),
        //         )
        //         .await?;
        //         uniswap_v3_pool.populate_ticks_from_tick_data(tick_data);
        //         println!(
        //             "Populated tick data for pool: {:?}",
        //             uniswap_v3_pool.address
        //         );
        //     }
        // }
    }

    async fn sync_state(&mut self) -> Result<()> {
        info!("Syncing state...");
        self.state
            .update_pools()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to sync pools: {}", e))?;

        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::NewBlock(event) => {
                // info!("New block: {:?}", event);
                let block_number = event.number.to::<u64>();
                self.state.update_block_number(block_number).await.unwrap();
                return vec![];
            }
            Event::UniswapV2Swap(swap) => {
                info!(
                    "New UniswapV2 swap from {:?} on pool {:?}",
                    swap.sender, swap.to
                );
                return vec![];
            }
            Event::UniswapV3Swap(swap) => {
                info!(
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
                let block_number = log.block_number.unwrap();
                let mut conn = establish_connection(&self.db_url);
                self.state.update_block_number(block_number).await.unwrap();

                if log.topics()[0] == IUniswapV2Pair::Swap::SIGNATURE_HASH {
                    // self.handle_uniswap_v2_swap(&mut conn, pool_address, log.clone())
                    //     .await
                    //     .unwrap_or_else(|e| {
                    //         error!(
                    //             "Failed to handle uniswap v2 swap: {:?}. Pool: {:?}. Log: {:?}",
                    //             e, pool_address, log
                    //         );
                    //     });
                } else if log.topics()[0] == IUniswapV3Pool::Swap::SIGNATURE_HASH {
                    self.handle_uniswap_v3_swap(&mut conn, pool_address, log.clone())
                        .await
                        .unwrap_or_else(|e| {
                            error!(
                                "Failed to handle uniswap v3 swap: {:?}. Pool: {:?}. Log: {:?}",
                                e, pool_address, log
                            );
                        });
                } else if log.topics()[0] == IUniswapV2Pair::Sync::SIGNATURE_HASH {
                    self.handle_uniswap_v2_sync(&mut conn, pool_address, log.clone())
                        .await
                        .unwrap_or_else(|e| {
                            error!(
                                "Failed to handle uniswap v3 swap: {:?}. Pool: {:?}. Log: {:?}",
                                e, pool_address, log
                            );
                        });
                }
            }
        }
        vec![]
    }
}

impl<P: Provider + 'static, S: Signer + Send + Sync + 'static> UniTriArb<P, S> {
    async fn handle_uniswap_v2_sync(
        &self,
        mut conn: &mut PgConnection,
        pool_address: Address,
        log: Log,
    ) -> Result<()> {
        let pool = self.state.pools.get_mut(&pool_address);
        if pool.is_some() {
            info!("New uniswap v2 swap on known pool {:?}", pool_address);
            let mut pool_ref = pool.unwrap();
            let pool = pool_ref.value_mut();
            let price_before = pool.calculate_price(pool.tokens()[0])?;
            pool.sync_from_log(log)?;
            let price_after = pool.calculate_price(pool.tokens()[0])?;

            info!(
                "New uniswap v2 swap on pool {:?}. Price: {:?} -> {:?}",
                pool.name(),
                price_before,
                price_after
            );

            let amm_slice: &mut [AMM] = std::slice::from_mut(pool);
            let updated_cycles = self.state.get_updated_cycles(amm_slice.to_vec());
            info!("Found {} updated cycles", updated_cycles.len());
            for cycle in updated_cycles {
                info!("{}: Profit: {}", cycle, cycle.get_profit_perc());
            }

            return Ok(());
        }

        if self.state.inactive_pools.contains_key(&pool_address) {
            info!("New uniswap v2 swap on inactive pool {:?}", pool_address);
            return Ok(());
        }

        info!("New uniswap v2 swap on unknown pool {:?}", pool_address);
        let provider = self.client.clone();
        let result = fetch_v2_pool_data_batch_request(&[pool_address], provider).await;

        let pool_data =
            result.map_err(|e| anyhow::anyhow!("Failed to parse pool batch request: {:?}", e))?;

        let new_pool = self.populate_uni_v2_db_pool_data(pool_data, &mut conn, pool_address)?;

        batch_upsert_uni_v2_pools(&mut conn, &vec![new_pool]).unwrap();
        Ok(())
    }

    async fn handle_uniswap_v3_swap(
        &self,
        mut conn: &mut PgConnection,
        pool_address: Address,
        log: Log,
    ) -> Result<()> {
        let pool = self.state.pools.get_mut(&pool_address);
        if pool.is_some() {
            let mut pool_ref = pool.unwrap();
            let pool = pool_ref.value_mut();
            let price_before = pool.calculate_price(pool.tokens()[0])?;
            pool.sync_from_log(log)?;
            let price_after = pool.calculate_price(pool.tokens()[0])?;
            info!(
                "New uniswap v3 swap on pool {:?}. Price: {:?} -> {:?}",
                pool.name(),
                price_before,
                price_after
            );

            let amm_slice: &mut [AMM] = std::slice::from_mut(pool);
            let updated_cycles = self.state.get_updated_cycles(amm_slice.to_vec());
            info!("Found {} updated cycles", updated_cycles.len());
            for cycle in updated_cycles {
                info!("{}: Profit: {}", cycle, cycle.get_profit_perc());
            }

            return Ok(());
        }

        if self.state.inactive_pools.contains_key(&pool_address) {
            info!("New uniswap v3 swap on inactive pool {:?}", pool_address);
            return Ok(());
        }

        info!("New uniswap v3 swap on unknown pool {:?}", pool_address);
        let result =
            fetch_v3_pool_data_batch_request(&[pool_address], None, self.client.clone()).await;

        let uniswap_v3_log =
            result.map_err(|e| anyhow::anyhow!("Failed to parse pool batch request: {:?}", e))?;

        let new_pool = self.parse_univ3_pool_data(uniswap_v3_log, &mut conn, pool_address)?;
        batch_upsert_uni_v3_pools(&mut conn, &vec![new_pool]).unwrap();
        Ok(())
    }

    fn populate_uni_v2_db_pool_data(
        &self,
        pool_data: DynSolValue,
        mut conn: &mut PgConnection,
        pool_address: Address,
    ) -> Result<NewDbUniV2Pool> {
        let pool_data = pool_data
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Failed to parse pool data"))?;

        for token in pool_data {
            let pool_data = token
                .as_tuple()
                .ok_or_else(|| anyhow::anyhow!("Failed to parse pool data"))?;

            let address = pool_data[0]
                .as_address()
                .ok_or_else(|| anyhow::anyhow!("Failed to parse pool data"))?;

            if !address.is_zero() {
                let mut pool = UniswapV2Pool::default();
                pool.address = pool_address;

                let chain = self.chain.named().unwrap().to_string();
                let known_exchanges = get_exchanges_by_chain(&mut conn, &chain).unwrap();
                let exchange_name = known_exchanges
                    .iter()
                    .find(|e| *e.factory_address.as_ref().unwrap() == pool.factory.to_string())
                    .map(|e| e.exchange_name.clone())
                    .unwrap_or("unknown".to_string());

                populate_v2_pool_data(&mut pool, &pool_data)?;

                let mut db_pool: NewDbUniV2Pool = pool.into();
                db_pool.exchange_name = Some(exchange_name);
                db_pool.exchange_type = Some("univ2".to_string());
                db_pool.chain = chain;

                return Ok(db_pool);
            } else {
                break;
            };
        }
        return Err(anyhow::anyhow!("Failed to parse pool data"));
    }

    fn parse_univ3_pool_data(
        &self,
        data: DynSolValue,
        mut conn: &mut PgConnection,
        pool_address: Address,
    ) -> Result<NewDbUniV3Pool> {
        let data = data
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Failed to parse pool data"))?;

        for token in data {
            let pool_data = token
                .as_tuple()
                .ok_or_else(|| anyhow::anyhow!("Failed to parse pool data"))?;

            let address = pool_data[0]
                .as_address()
                .ok_or_else(|| anyhow::anyhow!("Failed to parse pool data"))?;

            // If the pool token A is not zero, signaling that the pool data was polulated
            if !address.is_zero() {
                let mut pool = UniswapV3Pool::default();
                pool.address = pool_address;

                let chain = self.chain.named().unwrap().to_string();
                let known_exchanges = get_exchanges_by_chain(&mut conn, &chain).unwrap();
                let exchange_name = known_exchanges
                    .iter()
                    .find(|e| *e.factory_address.as_ref().unwrap() == pool.factory.to_string())
                    .map(|e| e.exchange_name.clone())
                    .unwrap_or("unknown".to_string());

                populate_v3_pool_data(&mut pool, &pool_data)?;

                let mut db_pool: NewDbUniV3Pool = pool.into();
                db_pool.exchange_name = Some(exchange_name);
                db_pool.exchange_type = Some("univ3".to_string());
                db_pool.chain = chain;

                // if exchange_name == "unknown" {
                //     info!(
                //         "Unknown v3 pool {:?}:{:?}-{:?}",
                //         pool_address,
                //         bytes32_to_string(token_a_symbol.0),
                //         bytes32_to_string(token_b_symbol.0)
                //     );
                // }

                return Ok(db_pool);
            } else {
                break;
            }
        }
        return Err(anyhow::anyhow!("Failed to parse pool data"));
    }
}
