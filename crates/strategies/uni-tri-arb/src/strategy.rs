use crate::state::State;

use super::types::{Action, Event};
use alloy::{
    dyn_abi::DynSolValue, primitives::Address, providers::Provider, rpc::types::Log,
    signers::Signer, sol_types::SolEvent,
};
use alloy_chains::{Chain, NamedChain};
use amms::{
    amm::{
        common::fetch_pool_data_batch_request, uniswap_v3::IUniswapV3Pool, AutomatedMarketMaker,
        AMM,
    },
    errors::AMMError,
    sync::{self, checkpoint::sort_amms},
};
use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings::iuniswapv2pair::IUniswapV2Pair;
use db::queries::{exchange::get_exchanges_by_chain, pool::batch_upsert_pools};
use db::{establish_connection, models::NewPool};
use diesel::PgConnection;
use shared::{addressbook::Addressbook, amm_utils::db_pools_to_amms, utils::bytes32_to_string};
use std::sync::Arc;
use tracing::{error, info};
use types::exchange::ExchangeName;

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

        // update block number
        let block_number = self.client.get_block_number().await.unwrap();
        self.state.update_block_number(block_number).await.unwrap();

        let active_pools = db::queries::pool::get_pools(
            &mut establish_connection(&self.db_url),
            Some(&self.chain.named().unwrap().to_string()),
            None,
            None,
            None,
            Some(true),
        )
        .unwrap();

        info!("{:?} active pools", active_pools.len());

        let inactive_pools = db::queries::pool::get_pools(
            &mut establish_connection(&self.db_url),
            Some(&self.chain.named().unwrap().to_string()),
            None,
            None,
            None,
            None,
        )
        .unwrap();

        info!("{:?} inactive pools", inactive_pools.len());

        let inactive_amms = db_pools_to_amms(&inactive_pools)?;
        self.state.set_inactive_pools(inactive_amms);

        let active_amms = db_pools_to_amms(&active_pools)?;
        let (mut uniswap_v2_pools, mut uniswap_v3_pools, _, mut camelot_v3_pools) =
            sort_amms(active_amms);

        sync::populate_amms(&mut uniswap_v2_pools, block_number, self.client.clone()).await?;
        sync::populate_amms(&mut uniswap_v3_pools, block_number, self.client.clone()).await?;
        sync::populate_amms(&mut camelot_v3_pools, block_number, self.client.clone()).await?;

        let synced_amms = vec![uniswap_v2_pools, uniswap_v3_pools, camelot_v3_pools].concat();
        self.state.set_pools(synced_amms);
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
                    self.handle_uniswap_v2_swap(&mut conn, pool_address, log.clone())
                        .await
                        .unwrap_or_else(|e| {
                            error!(
                                "Failed to handle uniswap v2 swap: {:?}. Pool: {:?}. Log: {:?}",
                                e, pool_address, log
                            );
                        });
                } else if log.topics()[0] == IUniswapV3Pool::Swap::SIGNATURE_HASH {
                    self.handle_uniswap_v3_swap(&mut conn, pool_address, log.clone())
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
    async fn handle_uniswap_v2_swap(
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
        let result = fetch_pool_data_batch_request(vec![pool_address], provider).await;

        let uniswap_v2_log =
            result.map_err(|e| anyhow::anyhow!("Failed to parse pool batch request: {:?}", e))?;

        let new_pool = self.parse_uniswap_v2_log(uniswap_v2_log, &mut conn, pool_address)?;

        batch_upsert_pools(&mut conn, &vec![new_pool]).unwrap();
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
        let result = fetch_pool_data_batch_request(vec![pool_address], self.client.clone()).await;

        let uniswap_v3_log =
            result.map_err(|e| anyhow::anyhow!("Failed to parse pool batch request: {:?}", e))?;

        let new_pool = self.parse_uniswap_v3_log(uniswap_v3_log, &mut conn, pool_address)?;
        batch_upsert_pools(&mut conn, &vec![new_pool]).unwrap();
        Ok(())
    }

    fn parse_uniswap_v2_log(
        &self,
        log: DynSolValue,
        mut conn: &mut PgConnection,
        pool_address: Address,
    ) -> Result<NewPool> {
        let log = log
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Failed to parse pool data"))?;

        for token in log {
            let pool_data = token
                .as_tuple()
                .ok_or_else(|| anyhow::anyhow!("Failed to parse pool data"))?;

            let address = pool_data[0]
                .as_address()
                .ok_or_else(|| anyhow::anyhow!("Failed to parse pool data"))?;

            if !address.is_zero() {
                let token_a = pool_data[0].as_address().unwrap();
                let token_a_symbol = pool_data[1].as_fixed_bytes().unwrap();
                let token_a_decimals = pool_data[2].as_uint().unwrap();
                let token_b = pool_data[3].as_address().unwrap();
                let token_b_symbol = pool_data[4].as_fixed_bytes().unwrap();
                let token_b_decimals = pool_data[5].as_uint().unwrap();
                let factory = pool_data[6].as_address().unwrap();
                let reserve_0 = pool_data[7].as_uint().unwrap();
                let reserve_1 = pool_data[8].as_uint().unwrap();
                let fee = pool_data[9].as_uint().unwrap();
                let chain = self.chain.named().unwrap().to_string();

                let known_exchanges = get_exchanges_by_chain(&mut conn, &chain).unwrap();
                let exchange_name = known_exchanges
                    .iter()
                    .find(|e| *e.factory_address.as_ref().unwrap() == factory.to_string())
                    .map(|e| e.exchange_name.clone())
                    .unwrap_or("unknown".to_string());

                return Ok(NewPool {
                    address: pool_address.to_string(),
                    chain: self.chain.named().unwrap().to_string(),
                    factory_address: factory.to_string(),
                    exchange_name: exchange_name.clone(),
                    exchange_type: "univ2".to_string(),
                    token_a: token_a.to_string(),
                    token_a_symbol: bytes32_to_string(token_a_symbol.0),
                    token_a_decimals: token_a_decimals.0.to::<i32>(),
                    token_b: token_b.to_string(),
                    token_b_symbol: bytes32_to_string(token_b_symbol.0),
                    token_b_decimals: token_b_decimals.0.to::<i32>(),
                    reserve_0: reserve_0.0.to::<u128>().to_string(),
                    reserve_1: reserve_1.0.to::<u128>().to_string(),
                    fee: fee.0.to::<i32>(),
                    filtered: None,
                });
            } else {
                break;
            };
        }
        return Err(anyhow::anyhow!("Failed to parse pool data"));
    }

    fn parse_uniswap_v3_log(
        &self,
        log: DynSolValue,
        conn: &mut PgConnection,
        pool_address: Address,
    ) -> Result<NewPool> {
        let log = log
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Failed to parse pool data"))?;

        for token in log {
            let pool_data = token
                .as_tuple()
                .ok_or_else(|| anyhow::anyhow!("Failed to parse pool data"))?;

            let address = pool_data[0]
                .as_address()
                .ok_or_else(|| anyhow::anyhow!("Failed to parse pool data"))?;

            // If the pool token A is not zero, signaling that the pool data was polulated
            if !address.is_zero() {
                let token_a = pool_data[0].as_address().unwrap();
                let token_a_symbol = pool_data[1].as_fixed_bytes().unwrap();
                let token_a_decimals = pool_data[2].as_uint().unwrap();
                let token_b = pool_data[3].as_address().unwrap();
                let token_b_symbol = pool_data[4].as_fixed_bytes().unwrap();
                let token_b_decimals = pool_data[5].as_uint().unwrap();
                let factory = pool_data[6].as_address().unwrap();
                let reserve_0 = pool_data[7].as_uint().unwrap();
                let reserve_1 = pool_data[8].as_uint().unwrap();
                let fee = pool_data[9].as_uint().unwrap();
                let chain = self.chain.named().unwrap().to_string();

                let mut conn = establish_connection(&self.db_url);
                let known_exchanges = get_exchanges_by_chain(&mut conn, &chain).unwrap();

                let exchange_name = known_exchanges
                    .iter()
                    .find(|e| *e.factory_address.as_ref().unwrap() == factory.to_string())
                    .map(|e| e.exchange_name.clone())
                    .unwrap_or("unknown".to_string());

                if exchange_name == "unknown" {
                    info!(
                        "Unknown v3 pool {:?}:{:?}-{:?}",
                        pool_address,
                        bytes32_to_string(token_a_symbol.0),
                        bytes32_to_string(token_b_symbol.0)
                    );
                }

                return Ok(NewPool {
                    address: pool_address.to_string(),
                    chain: chain,
                    factory_address: factory.to_string(),
                    exchange_name: exchange_name.clone(),
                    exchange_type: "univ3".to_string(),
                    token_a: token_a.to_string(),
                    token_a_symbol: bytes32_to_string(token_a_symbol.0),
                    token_a_decimals: token_a_decimals.0.to::<i32>(),
                    token_b: token_b.to_string(),
                    token_b_symbol: bytes32_to_string(token_b_symbol.0),
                    token_b_decimals: token_b_decimals.0.to::<i32>(),
                    reserve_0: reserve_0.0.to::<u128>().to_string(),
                    reserve_1: reserve_1.0.to::<u128>().to_string(),
                    fee: fee.0.to::<i32>(),
                    filtered: None,
                });
            } else {
                break;
            }
        }
        return Err(anyhow::anyhow!("Failed to parse pool data"));
    }
}
