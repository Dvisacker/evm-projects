use super::types::{Action, Event};
use crate::state::State;
use addressbook::Addressbook;
use alloy::primitives::aliases::U24;
use alloy::primitives::utils::parse_units;
use alloy::primitives::{Bytes, I256, U256};
use alloy::providers::Provider;
use alloy::{primitives::Address, rpc::types::Log};
use alloy_chains::{Chain, NamedChain};
use alloy_sol_types::SolEvent;
use amms::amm::{
    uniswap_v2::{
        batch_request::{fetch_v2_pool_data_batch_request, populate_v2_pool_data},
        UniswapV2Pool,
    },
    AutomatedMarketMaker, AMM,
};
use amms::bindings::getuniv2pooldata::PoolHelpers::UniswapV2PoolData;
use amms::bindings::iaerodromepool::IAerodromePool;
use amms::bindings::iuniswapv2pool::IUniswapV2Pool;
use async_trait::async_trait;
use db::queries::uni_v3_pool::get_uni_v3_pools;
use db::{
    establish_connection,
    models::{db_pool::DbPool, NewDbUniV2Pool},
    queries::{
        exchange::get_exchanges_by_chain,
        uni_v2_pool::{batch_upsert_uni_v2_pools, get_uni_v2_pools},
    },
};
use diesel::PgConnection;
use engine::executors::encoded_tx_executor::SubmitEncodedTx;
use engine::types::Strategy;
use eyre::Result;
use shared::cycle::{get_most_profitable_cycles, Cycle};
use shared::pool_helpers::db_pools_to_amms;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, info, warn};
use tx_executor::encoder::BatchExecutorClient;
// use tx_executor::{get_default_encoder, BasicEncoder};
use tx_simulator::simulator::TxSimulatorClient;

pub struct BaseArb<P: Provider> {
    pub chain: Chain,
    pub client: Arc<P>,
    pub encoder: Option<BatchExecutorClient<P>>,
    pub addressbook: Addressbook,
    pub state: State<P>,
    pub db_url: String,
    pub simulator: Option<TxSimulatorClient<P>>,
}

impl<P: Provider> BaseArb<P> {
    pub fn new(chain: Chain, client: Arc<P>, db_url: String) -> Self {
        let addressbook = Addressbook::load().expect("Failed to load addressbook");
        let chain_name = chain.named().expect("Chain must be named");
        let weth = addressbook
            .get_weth(&chain_name)
            .expect("Failed to get WETH address");

        Self {
            chain,
            addressbook,
            client: client.clone(),
            encoder: None,
            simulator: None,
            state: State::new(client.clone(), vec![weth]),
            db_url,
        }
    }

    async fn load_encoder(&mut self) -> Result<()> {
        let named_chain = NamedChain::try_from(self.chain).unwrap();
        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        self.encoder = Some(
            BatchExecutorClient::new(executor_address, named_chain, self.client.clone()).await,
        );
        Ok(())
    }

    async fn load_simulator(&mut self) -> Result<()> {
        let simulator = TxSimulatorClient::new(
            Address::from_str(&env::var("SIMULATOR_ADDRESS").unwrap())
                .expect("Failed to parse simulator address"),
            self.client.clone(),
        )
        .await;
        self.simulator = Some(simulator);
        Ok(())
    }

    async fn load_pools(&mut self) -> Result<()> {
        let chain = self.chain.named().expect("Chain must be named").to_string();
        let mut conn = establish_connection(&self.db_url);

        // aerodrome pools are "univ2-ish" pools
        let aerodrome_pools = get_uni_v2_pools(
            &mut conn,
            Some(&chain),
            Some("aerodrome"),
            Some("ve33"),
            None,
            None,
        )?;

        let most_traded_univ3_pools = get_uni_v3_pools(
            &mut conn,
            Some(&chain),
            Some("uniswapv3"),
            Some("univ3"),
            Some(10),
            Some("univ3-base-most-traded"),
        )?;

        println!("Most traded univ3 pools: {:?}", most_traded_univ3_pools);

        let most_traded_univ2_pools = get_uni_v2_pools(
            &mut conn,
            Some(&chain),
            Some("uniswapv2"),
            Some("univ2"),
            Some(3),
            Some("univ2-base-most-traded"),
        )?;

        let aerodrome_db_pools = aerodrome_pools
            .into_iter()
            .map(|p| p.into())
            .collect::<Vec<DbPool>>();

        let most_traded_univ2_db_pools = most_traded_univ2_pools
            .into_iter()
            .map(|p| p.into())
            .collect::<Vec<DbPool>>();

        let most_traded_univ3_db_pools = most_traded_univ3_pools
            .into_iter()
            .map(|p| p.into())
            .collect::<Vec<DbPool>>();

        let mut db_pools = vec![];
        db_pools.extend(aerodrome_db_pools);
        db_pools.extend(most_traded_univ2_db_pools);
        db_pools.extend(most_traded_univ3_db_pools);

        let amms = db_pools_to_amms(&db_pools)?;

        self.state.set_pools(amms);
        self.state.update_pools().await?;

        Ok(())
    }

    fn log_arbitrage_cycles(&self, cycles: &[impl std::fmt::Display]) {
        for cycle in cycles {
            info!("{}", cycle);
        }
    }

    // this encode the cycle through the executor contract
    async fn get_cycle_calldata(
        &self,
        token_first: Address,
        amount_in: U256,
        cycle: &Cycle,
    ) -> Result<(Vec<Bytes>, U256)> {
        let named_chain = NamedChain::try_from(self.chain).unwrap();
        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let mut encoder =
            BatchExecutorClient::new(executor_address, named_chain, self.client.clone()).await;

        let amms = cycle.amms.clone();
        let first_amm = amms.first().unwrap();
        let remaining_amms = amms[1..].to_vec();
        let first_amm_tokens = first_amm.tokens();
        let token_a = first_amm_tokens[0];
        let token_b = first_amm_tokens[1];

        let token_in: Address;
        let token_out: Address;

        if token_a == token_first {
            token_in = token_a;
            token_out = token_b;
        } else {
            token_in = token_b;
            token_out = token_a;
        };

        let amount_in = amount_in;
        let stable = if let AMM::Ve33Pool(ve33_pool) = first_amm {
            Some(ve33_pool.stable)
        } else {
            None
        };
        let fee = if let AMM::UniswapV3Pool(uniswap_v3_pool) = first_amm {
            Some(U24::from(uniswap_v3_pool.fee))
        } else {
            None
        };

        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap())
            .expect("Failed to parse executor address");
        let weth = self
            .addressbook
            .get_weth(&self.chain.named().unwrap())
            .expect("Failed to get WETH address");

        encoder
            .add_wrap_eth(weth, amount_in)
            .add_transfer_erc20(weth, executor_address, amount_in)
            .add_swap(
                first_amm.exchange_name(),
                token_in,
                token_out,
                amount_in,
                None,
                stable,
                fee,
            );

        let mut last_token = token_out;

        for amm in remaining_amms {
            let tokens = amm.tokens();
            let token_a = tokens[0];
            let token_b = tokens[1];
            let token_out: Address;
            let token_in: Address;

            if last_token == token_a {
                token_out = token_b;
                token_in = token_a;
            } else {
                token_out = token_a;
                token_in = token_b;
            };

            let stable = if let AMM::Ve33Pool(ve33_pool) = &amm {
                Some(ve33_pool.stable)
            } else {
                None
            };
            let fee = if let AMM::UniswapV3Pool(uniswap_v3_pool) = &amm {
                Some(U24::from(uniswap_v3_pool.fee))
            } else {
                None
            };

            encoder.add_swap_all(amm.exchange_name(), token_in, token_out, None, stable, fee);
            last_token = token_out;
        }

        encoder.require_profitable(
            weth,
            U256::from(amount_in) * U256::from(50) / U256::from(100),
        );

        let (calldata, total_value) = encoder.flush();

        Ok((calldata, total_value))
    }
}

#[async_trait]
impl<P: Provider + Clone> Strategy<Event, Action> for BaseArb<P> {
    async fn init_state(&mut self) -> Result<()> {
        info!("Initializing state... 🚀");

        let block_number = self.client.get_block_number().await?;
        self.state.update_block_number(block_number).await?;

        self.load_pools().await?;
        info!("Loaded {} pools 🏊", self.state.pools.len());

        self.load_encoder().await?;
        info!("Loaded encoder 📦");
        self.load_simulator().await?;
        info!("Loaded simulator 📡");

        let arb_cycles = self.state.update_cycles()?;
        self.log_arbitrage_cycles(&arb_cycles);

        Ok(())
    }

    async fn sync_state(&mut self) -> Result<()> {
        info!("Syncing state... 🔄");
        self.state.update_pools().await?;
        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        let mut actions = vec![];
        let mut updated_cycles = vec![];
        match event {
            Event::NewBlock(event) => {
                if let Err(e) = self
                    .state
                    .update_block_number(event.number.to::<u64>())
                    .await
                {
                    warn!("Failed to update block number: {}", e);
                }
            }
            Event::Log(log) => {
                updated_cycles = self.handle_log_event(log).await;
            }
            _ => {}
        }

        info!("Updated cycles: {:?}", updated_cycles.len());
        self.log_arbitrage_cycles(&updated_cycles);
        info!("--------------------------------");

        let most_profitable_cycles = get_most_profitable_cycles(updated_cycles, 3);

        for cycle in most_profitable_cycles {
            let token_first = cycle.get_entry_token();
            let amount_in = parse_units("0.001", 18)
                .expect("Failed to parse amount in")
                .into();
            let amount_out = self
                .simulator
                .as_ref()
                .expect("Simulator must be loaded")
                .simulate_route(token_first, amount_in, &cycle.amms)
                .await
                .unwrap_or_else(|e| {
                    warn!("Failed to simulate route: {}", e);
                    U256::from(0)
                });

            let scale_decimals = 5;
            let scale_multiplier: U256 = parse_units("1", scale_decimals).unwrap().into();
            let percentage_profit = (amount_out - amount_in) * scale_multiplier / amount_in;
            let scaled_percentage: I256 = parse_units("-0.03", scale_decimals).unwrap().into();
            let profitable = I256::try_from(percentage_profit).unwrap() >= scaled_percentage;

            if profitable {
                info!(
                    "Profitable cycle: {} - Profit: {:?} 💰",
                    cycle,
                    amount_out - amount_in
                );
                let (calldata, total_value) = self
                    .get_cycle_calldata(token_first, amount_in, &cycle)
                    .await
                    .unwrap_or_else(|e| {
                        warn!("Failed to get cycle calldata: {}", e);
                        (vec![], U256::from(0))
                    });

                let action = Action::SubmitEncodedTx(SubmitEncodedTx {
                    calldata,
                    total_value,
                    gas_bid_info: None,
                });
                info!("Submitting encoded tx... 📨");
                actions.push(action);
            } else {
                info!(
                    "Negative cycle: {} - Loss: {:?} 📉",
                    cycle,
                    amount_in - amount_out
                );
            }
        }

        actions
    }
}

// Private implementation details
impl<P: Provider + Clone> BaseArb<P> {
    async fn handle_log_event(&mut self, log: Log) -> Vec<Cycle> {
        let pool_address = log.address();
        let block_number = log.block_number.expect("Log must have block number");

        if let Err(e) = self.state.update_block_number(block_number).await {
            warn!("Failed to update block number: {}", e);
            return vec![];
        }

        match log.topics()[0] {
            topic if topic == IUniswapV2Pool::Swap::SIGNATURE_HASH => {
                debug!("New uniswap v2 swap on pool {:?}", pool_address);
                return vec![];
            }
            topic if topic == IUniswapV2Pool::Sync::SIGNATURE_HASH => {
                let updated_cycles = self.handle_v2_sync(pool_address, log.clone()).await;
                return updated_cycles.unwrap_or_else(|e| {
                    warn!("Failed to handle uniswap v2 sync: {}", e);
                    debug!("Pool: {:?}, Log: {:?}", pool_address, log);
                    vec![]
                });
            }
            topic if topic == IAerodromePool::Sync::SIGNATURE_HASH => {
                let updated_cycles = self.handle_v2_sync(pool_address, log.clone()).await;
                return updated_cycles.unwrap_or_else(|e| {
                    warn!("Failed to handle aerodrome sync: {}", e);
                    debug!("Pool: {:?}, Log: {:?}", pool_address, log);
                    vec![]
                });
            }
            _ => {}
        }

        vec![]
    }

    // handles sync for both uniswap v2 and ve33 pools
    async fn handle_v2_sync(&mut self, pool_address: Address, log: Log) -> Result<Vec<Cycle>> {
        if let Some(mut pool) = self.state.pools.get_mut(&pool_address) {
            return self.handle_known_pool_sync(&mut pool, log).await;
        }

        self.handle_unknown_pool_sync(pool_address).await?;
        Ok(vec![])
    }

    async fn handle_known_pool_sync(&self, pool: &mut AMM, log: Log) -> Result<Vec<Cycle>> {
        let price_before = pool.calculate_price(pool.tokens()[0])?;
        pool.sync_from_log(log)?;
        let price_after = pool.calculate_price(pool.tokens()[0])?;

        info!(
            "Pool {} price update: {} -> {} 📊",
            pool.name(),
            price_before,
            price_after
        );

        let amms: &mut [AMM] = std::slice::from_mut(pool);
        let updated_cycles = self.state.get_updated_cycles(amms.to_vec())?;
        Ok(updated_cycles)
    }

    async fn handle_unknown_pool_sync(&self, pool_address: Address) -> Result<()> {
        info!("New v2 sync on unknown pool {:?}", pool_address);

        let pool_data = fetch_v2_pool_data_batch_request(&[pool_address], self.client.clone())
            .await
            .map_err(|e| eyre::eyre!("Failed to fetch pool data: {}", e))?;

        let pool_data = pool_data[0].clone();

        let mut conn = establish_connection(&self.db_url);
        let new_pool = self.parse_univ2_pool_data(pool_data, &mut conn, pool_address)?;

        if new_pool.exchange_type != Some("unknown".to_string()) {
            batch_upsert_uni_v2_pools(&mut conn, &vec![new_pool])?;
        }

        Ok(())
    }

    fn parse_univ2_pool_data(
        &self,
        pool_data: UniswapV2PoolData,
        mut conn: &mut PgConnection,
        pool_address: Address,
    ) -> Result<NewDbUniV2Pool> {
        let pool_data = pool_data;

        if !pool_data.tokenA.is_zero() {
            let mut pool = UniswapV2Pool::default();
            pool.address = pool_address;

            let chain = self.chain.named().unwrap().to_string();

            populate_v2_pool_data(&mut pool, pool_data)?;

            let known_exchanges = get_exchanges_by_chain(&mut conn, &chain).unwrap();
            let exchange = known_exchanges
                .iter()
                .find(|e| *e.factory_address.as_ref().unwrap() == pool.factory.to_string())
                .ok_or(eyre::eyre!("Failed to find exchange"))?;

            let exchange_name = exchange.exchange_name.clone();
            let exchange_type = exchange.exchange_type.clone();
            let mut db_pool: NewDbUniV2Pool = pool.into();
            db_pool.exchange_name = Some(exchange_name);
            db_pool.exchange_type = Some(exchange_type);
            db_pool.chain = chain;

            return Ok(db_pool);
        }
        return Err(eyre::eyre!("Failed to parse pool data"));
    }
}
