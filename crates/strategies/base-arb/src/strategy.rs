use super::types::{Action, Event};
use crate::state::State;
use addressbook::Addressbook;
use alloy::primitives::utils::parse_units;
use alloy::primitives::{Bytes, I256, U256};
use alloy::providers::Provider;
use alloy::{dyn_abi::DynSolValue, primitives::Address, rpc::types::Log};
use alloy_chains::Chain;
use alloy_sol_types::SolEvent;
use amms::bindings::iaerodromepool::IAerodromePool;
use amms::bindings::iuniswapv2pool::IUniswapV2Pool;
use amms::{
    amm::{
        uniswap_v2::{
            batch_request::{fetch_v2_pool_data_batch_request, populate_v2_pool_data},
            UniswapV2Pool,
        },
        AutomatedMarketMaker, AMM,
    },
    sync,
};
use async_trait::async_trait;
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
use provider::SignerProvider;
use shared::cycle::{get_most_profitable_cycles, Cycle};
use shared::pool_helpers::db_pools_to_amms;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, info, warn};
use tx_executor::{get_default_encoder, BasicEncoder};
use tx_simulator::simulator::TxSimulatorClient;
use tx_simulator::SimulatorClient;

#[derive(Clone)]
pub struct BaseArb {
    pub chain: Chain,
    pub client: Arc<SignerProvider>,
    pub encoder: Option<Arc<BasicEncoder>>,
    pub state: State,
    pub db_url: String,
    pub simulator: Option<SimulatorClient>,
}

impl BaseArb {
    pub fn new(chain: Chain, client: Arc<SignerProvider>, db_url: String) -> Self {
        let addressbook = Addressbook::load().expect("Failed to load addressbook");
        let chain_name = chain.named().expect("Chain must be named");
        let weth = addressbook
            .get_weth(&chain_name)
            .expect("Failed to get WETH address");

        Self {
            chain,
            client: client.clone(),
            encoder: None,
            simulator: None,
            state: State::new(client.clone(), vec![weth]),
            db_url,
        }
    }

    async fn load_encoder(&mut self) -> Result<()> {
        self.encoder = Some(Arc::new(get_default_encoder(self.chain).await));
        Ok(())
    }

    async fn load_simulator(&mut self) -> Result<()> {
        let simulator = TxSimulatorClient::new(
            Address::from_str(&env::var("SIMULATOR_ADDRESS").unwrap()).unwrap(),
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

        let db_pools = aerodrome_pools
            .into_iter()
            .map(|p| p.into())
            .collect::<Vec<DbPool>>();

        let mut amms = db_pools_to_amms(&db_pools)?;

        let block_number = self.state.block_number;
        sync::populate_amms(&mut amms, block_number, self.client.clone()).await?;
        self.state.set_pools(amms);

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
    ) -> Result<Vec<Bytes>> {
        let mut encoder = get_default_encoder(self.chain).await;

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
            ve33_pool.stable
        } else {
            eyre::bail!("Pool is not a aerodrome pool");
        };

        let _executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();

        // encoder.add_transfer_erc20(token_in, executor_address, amount_in);
        // encoder.add_wrap_eth(
        //     Address::from_str("0x4200000000000000000000000000000000000006").unwrap(),
        //     amount_in,
        // );
        encoder.add_aerodrome_router_swap(amount_in, token_in, token_out, None, Some(stable));

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
            } else if last_token == token_b {
                token_out = token_a;
                token_in = token_b;
            } else {
                todo!()
            }

            let stable = if let AMM::Ve33Pool(ve33_pool) = amm {
                ve33_pool.stable
            } else {
                eyre::bail!("Pool is not an aerodrome pool");
            };

            encoder.add_aerodrome_swap_all(token_in, token_out, None, Some(stable));
            last_token = token_out;
        }

        let (calldata, _) = encoder.flush();

        Ok(calldata)
    }
}

#[async_trait]
impl Strategy<Event, Action> for BaseArb {
    async fn init_state(&mut self) -> Result<()> {
        info!("Initializing state... 🚀");

        let block_number = self.client.get_block_number().await?;
        self.state.update_block_number(block_number).await?;

        self.load_pools().await?;
        info!("Loaded {} pools 🏊", self.state.pools.len());

        self.load_encoder().await?;
        self.load_simulator().await?;

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

        let most_profitable_cycles = get_most_profitable_cycles(updated_cycles, 1);

        for cycle in most_profitable_cycles {
            let token_first = cycle.get_entry_token();
            let amount_in = parse_units("0.00001", 18).unwrap().into();
            // 100000000000000
            println!("Amount in: {:?}", amount_in);
            let amount_out = self
                .simulator
                .as_ref()
                .unwrap()
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
                let calldata = self
                    .get_cycle_calldata(token_first, amount_in, &cycle)
                    .await
                    .unwrap();

                let action = Action::SubmitEncodedTx(SubmitEncodedTx {
                    calldata,
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
impl BaseArb {
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

        if self.state.inactive_pools.contains_key(&pool_address) {
            self.handle_inactive_pool_sync(pool_address).await?;
            return Ok(vec![]);
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

    async fn handle_inactive_pool_sync(&self, pool_address: Address) -> Result<()> {
        info!("New sync on inactive pool {:?}", pool_address);
        Ok(())
    }

    async fn handle_unknown_pool_sync(&self, pool_address: Address) -> Result<()> {
        info!("New v2 sync on unknown pool {:?}", pool_address);

        let pool_data = fetch_v2_pool_data_batch_request(&[pool_address], self.client.clone())
            .await
            .map_err(|e| eyre::eyre!("Failed to fetch pool data: {}", e))?;

        let mut conn = establish_connection(&self.db_url);
        let new_pool = self.parse_univ2_pool_data(pool_data, &mut conn, pool_address)?;
        batch_upsert_uni_v2_pools(&mut conn, &vec![new_pool])?;

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
}
