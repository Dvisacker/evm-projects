use crate::state::State;

use super::types::{Action, Event};
use addressbook::Addressbook;
use alloy::{primitives::Address, providers::Provider, rpc::types::Log, sol_types::SolEvent};
use alloy_chains::Chain;
use amms::{
    amm::{
        uniswap_v2::{
            batch_request::{fetch_v2_pool_data_batch_request, populate_v2_pool_data},
            IUniswapV2Pair, UniswapV2Pool,
        },
        uniswap_v3::{
            batch_request::{fetch_v3_pool_data_batch_request, populate_v3_pool_data},
            IUniswapV3Pool, UniswapV3Pool,
        },
        AutomatedMarketMaker, AMM,
    },
    bindings::{
        getuniv2pooldata::PoolHelpers::UniswapV2PoolData,
        getuniv3pooldata::PoolHelpers::UniswapV3PoolData,
    },
    sync::{self},
};
use async_trait::async_trait;
use db::{
    establish_connection,
    models::{db_pool::DbPool, NewDbUniV2Pool},
    queries::{
        uni_v2_pool::{batch_upsert_uni_v2_pools, get_uni_v2_pools},
        uni_v3_pool::batch_upsert_uni_v3_pools,
    },
};
use db::{models::NewDbUniV3Pool, queries::exchange::get_exchanges_by_chain};
use diesel::PgConnection;
use engine::types::Strategy;
use eyre::Result;
use provider::SignerProvider;
use shared::pool_helpers::db_pools_to_amms;
use std::sync::Arc;
use tracing::{debug, info, warn};

#[derive(Debug, Clone)]
pub struct GeneralizedArb {
    pub chain: Chain,
    pub client: Arc<SignerProvider>,
    pub state: State,
    pub db_url: String,
}

impl GeneralizedArb {
    pub fn new(chain: Chain, client: Arc<SignerProvider>, db_url: String) -> Self {
        let addressbook = Addressbook::load().unwrap();
        let weth = addressbook.get_weth(&chain.named().unwrap()).unwrap();
        Self {
            chain,
            client: client.clone(),
            state: State::new(client.clone(), vec![weth]),
            db_url,
        }
    }
}

#[async_trait]
impl Strategy<Event, Action> for GeneralizedArb {
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
            None,
        )
        .unwrap()
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<DbPool>>();

        let mut active_v2_amms = db_pools_to_amms(&active_v2_pools)?;

        sync::populate_amms(&mut active_v2_amms, block_number, self.client.clone(), true).await?;
        let synced_amms = vec![active_v2_amms].concat();
        // let synced_amms = vec![active_v2_amms, active_v3_amms, active_camelot_v3_amms].concat();
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
    }

    async fn sync_state(&mut self) -> Result<()> {
        info!("Syncing state...");
        self.state
            .update_pools()
            .await
            .map_err(|e| eyre::eyre!("Failed to sync pools: {}", e))?;

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
                            debug!("Failed to handle uniswap v3 swap. Pool: {:?}", e);
                            warn!("Failed to handle uniswap v3 swap: {:?}", pool_address);
                        });
                } else if log.topics()[0] == IUniswapV2Pair::Sync::SIGNATURE_HASH {
                    self.handle_uniswap_v2_sync(&mut conn, pool_address, log.clone())
                        .await
                        .unwrap_or_else(|e| {
                            warn!("Failed to handle uniswap v2 swap: {:?}", pool_address);
                            debug!("Error: {:?}. Pool: {:?}. Log: {:?}", e, pool_address, log);
                        });
                }
            }
        }
        vec![]
    }
}

impl GeneralizedArb {
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
            result.map_err(|e| eyre::eyre!("Failed to parse pool batch request: {:?}", e))?;
        let pool_data = pool_data[0].clone();

        let new_pool = self.parse_univ2_pool_data(pool_data, &mut conn, pool_address)?;

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
        let logs = fetch_v3_pool_data_batch_request(&[pool_address], None, self.client.clone())
            .await
            .expect("Failed to fetch v3 pool data");

        let pool_data = logs.get(0).expect("Failed to get pool data");

        let new_pool = self.parse_univ3_pool_data(pool_data, &mut conn, pool_address)?;
        batch_upsert_uni_v3_pools(&mut conn, &vec![new_pool]).unwrap();
        Ok(())
    }

    fn parse_univ2_pool_data(
        &self,
        pool_data: UniswapV2PoolData,
        mut conn: &mut PgConnection,
        pool_address: Address,
    ) -> Result<NewDbUniV2Pool> {
        let address = pool_data.tokenA;

        if !address.is_zero() {
            let mut pool = UniswapV2Pool::default();
            pool.address = pool_address;

            let chain = self.chain.named().unwrap().to_string();

            populate_v2_pool_data(&mut pool, pool_data)?;

            let known_exchanges = get_exchanges_by_chain(&mut conn, &chain).unwrap();
            let exchange_name = known_exchanges
                .iter()
                .find(|e| *e.factory_address.as_ref().unwrap() == pool.factory.to_string())
                .map(|e| e.exchange_name.clone())
                .unwrap_or("unknown".to_string());

            let mut db_pool: NewDbUniV2Pool = pool.into();
            db_pool.exchange_name = Some(exchange_name);
            db_pool.exchange_type = Some("univ2".to_string());
            db_pool.chain = chain;

            return Ok(db_pool);
        }
        return Err(eyre::eyre!("Failed to parse pool data"));
    }

    fn parse_univ3_pool_data(
        &self,
        pool_data: &UniswapV3PoolData,
        mut conn: &mut PgConnection,
        pool_address: Address,
    ) -> Result<NewDbUniV3Pool> {
        let mut pool = UniswapV3Pool::default();
        pool.address = pool_address;

        let chain = self.chain.named().unwrap().to_string();
        let known_exchanges = get_exchanges_by_chain(&mut conn, &chain).unwrap();

        populate_v3_pool_data(&mut pool, &pool_data)?;

        let exchange_name = known_exchanges
            .iter()
            .find(|e| *e.factory_address.as_ref().unwrap() == pool.factory.to_string())
            .map(|e| e.exchange_name.clone())
            .unwrap_or("unknown".to_string());

        let mut db_pool: NewDbUniV3Pool = pool.into();
        db_pool.exchange_name = Some(exchange_name);
        db_pool.exchange_type = Some("univ3".to_string());
        db_pool.chain = chain;

        info!(
            "Parsed pool: Factory: {}, Exchange: {}",
            db_pool.factory_address.as_ref().unwrap(),
            db_pool.exchange_name.as_ref().unwrap()
        );

        return Ok(db_pool);
    }
}
