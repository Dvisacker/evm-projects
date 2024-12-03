use crate::state::State;

use super::types::{Action, Event};
use addressbook::Addressbook;
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
            batch_request::{
                fetch_v3_pool_data_batch_request, get_uniswap_v3_tick_data_batch_request,
                populate_v3_pool_data,
            },
            IUniswapV3Pool, UniswapV3Pool,
        },
        AutomatedMarketMaker, AMM,
    },
    sync::{self},
};
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
use engine::types::Strategy;
use eyre::Result;
use provider::SignerProvider;
use shared::pool_helpers::db_pools_to_amms;
use std::sync::Arc;
use tracing::{debug, info, warn};

#[derive(Debug, Clone)]
pub struct BaseArb {
    pub chain: Chain,
    pub client: Arc<SignerProvider>,
    pub state: State,
    pub db_url: String,
}

impl BaseArb {
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
impl Strategy<Event, Action> for BaseArb {
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

        let mut active_v2_amms = db_pools_to_amms(&active_v2_pools)?;

        sync::populate_amms(&mut active_v2_amms, block_number, self.client.clone()).await?;

        self.state.set_pools(active_v2_amms);

        info!("Updated pools: {:?}", self.state.pools);

        let arb_cycles = self.state.update_cycles();

        info!("{} arbitrage cycles", arb_cycles.len());
        for cycle in arb_cycles {
            info!("{}: Profit: {}", cycle, cycle.get_profit_perc());
        }

        Ok(())
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
        vec![]
    }
}

impl BaseArb {
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
        let result =
            fetch_v3_pool_data_batch_request(&[pool_address], None, self.client.clone()).await;

        let uniswap_v3_log =
            result.map_err(|e| eyre::eyre!("Failed to pool batch request: {:?}", e))?;

        let new_pool = self.parse_univ3_pool_data(uniswap_v3_log, &mut conn, pool_address)?;
        batch_upsert_uni_v3_pools(&mut conn, &vec![new_pool]).unwrap();
        Ok(())
    }

    fn parse_univ2_pool_data(
        &self,
        pool_data: DynSolValue,
        mut conn: &mut PgConnection,
        pool_address: Address,
    ) -> Result<NewDbUniV2Pool> {
        let pool_data = pool_data
            .as_array()
            .ok_or_else(|| eyre::eyre!("Failed to parse pool data"))?;

        for token in pool_data {
            let pool_data = token
                .as_tuple()
                .ok_or_else(|| eyre::eyre!("Failed to parse pool data"))?;

            let address = pool_data[0]
                .as_address()
                .ok_or_else(|| eyre::eyre!("Failed to parse pool data"))?;

            if !address.is_zero() {
                let mut pool = UniswapV2Pool::default();
                pool.address = pool_address;

                let chain = self.chain.named().unwrap().to_string();

                populate_v2_pool_data(&mut pool, &pool_data)?;

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
            } else {
                break;
            };
        }
        return Err(eyre::eyre!("Failed to parse pool data"));
    }

    fn parse_univ3_pool_data(
        &self,
        data: DynSolValue,
        mut conn: &mut PgConnection,
        pool_address: Address,
    ) -> Result<NewDbUniV3Pool> {
        let data = data
            .as_array()
            .ok_or_else(|| eyre::eyre!("Failed to parse pool data"))?;

        for token in data {
            let pool_data = token
                .as_tuple()
                .ok_or_else(|| eyre::eyre!("Failed to parse pool data"))?;

            let address = pool_data[0]
                .as_address()
                .ok_or_else(|| eyre::eyre!("Failed to parse pool data"))?;

            // If the pool token A is not zero, signaling that the pool data was populated
            if !address.is_zero() {
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
            } else {
                break;
            }
        }
        return Err(eyre::eyre!("Failed to parse pool data"));
    }
}
