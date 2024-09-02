use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy_chains::NamedChain;
use amms::amm::uniswap_v2::UniswapV2Pool;
use amms::amm::{AutomatedMarketMaker, AMM};
use amms::errors::AMMError;
use amms::sync;
use dashmap::DashMap;
use db::establish_connection;
use shared::types::Cycle;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tracing::info;
use types::exchange::{ExchangeName, ExchangeType};

#[derive(Debug, Clone)]
pub struct PoolState<P: Provider> {
    provider: Arc<P>,
    pub block_number: u64,
    pub pools: DashMap<Address, AMM>,
    pub pools_cycles_map: DashMap<Address, HashSet<String>>, // map of pool address to all cycles that include the pool
    pub cycles: HashMap<String, Cycle>,                      // map of cycle id to cycle
    pub inventory: Vec<Address>,                             // list of tokens that can be traded
}

impl<P: Provider + 'static> PoolState<P> {
    pub fn new(provider: Arc<P>, inventory: Vec<Address>) -> Self {
        Self {
            provider,
            inventory,
            block_number: 0,
            pools: DashMap::new(),
            pools_cycles_map: DashMap::new(),
            cycles: HashMap::new(),
        }
    }

    pub fn print_pools(&self) {
        for pool in self.pools.iter() {
            info!("Pool: {}", pool.key().to_string().to_lowercase());
        }
    }

    pub fn get_cycles(
        pairs: &[AMM],
        token_in: Address,
        token_out: Address,
        max_hops: i32,
        current_pairs: &Vec<AMM>,
        circles: &mut Vec<Cycle>,
        seen: &mut HashSet<Address>,
    ) -> Vec<Cycle> {
        let mut circles_copy: Vec<Cycle> = circles.clone();

        for pair in pairs {
            let address = pair.address();
            let tokens = pair.tokens();
            let [token_a, token_b] = tokens.as_slice() else {
                todo!();
            };
            if seen.contains(&address) {
                continue;
            }

            let temp_out: Address;
            if token_in == *token_a {
                temp_out = *token_b;
            } else if token_in == *token_b {
                temp_out = *token_a;
            } else {
                continue;
            }

            let mut new_seen = seen.clone();
            new_seen.insert(address);

            if temp_out == token_out {
                let mut pools = current_pairs.clone();
                pools.push(pair.clone());
                let cycle = Cycle::new(pools);
                circles_copy.push(cycle);
            } else if max_hops > 1 {
                let mut new_pairs: Vec<AMM> = current_pairs.clone();
                new_pairs.push(pair.clone());
                circles_copy = Self::get_cycles(
                    pairs,
                    temp_out,
                    token_out,
                    max_hops - 1,
                    &new_pairs,
                    &mut circles_copy,
                    &mut new_seen,
                );
            }
        }

        circles_copy
    }

    pub fn set_pools(&self, amms: Vec<AMM>) {
        for amm in amms {
            self.pools.insert(amm.address(), amm);
        }
    }

    pub fn get_updated_cycles(&self, amms: Vec<AMM>) -> Vec<Cycle> {
        // get the cycles that include the amms
        let mut cycles = vec![];
        for amm in amms {
            let pool_address = amm.address();
            let pool_cycles = self.pools_cycles_map.get(&pool_address);
            if let Some(c) = pool_cycles {
                // flat map of the cycle ids
                let cycle_ids = c.iter().collect::<Vec<_>>();
                for cycle_id in cycle_ids {
                    let cycle = self.cycles.get(cycle_id).unwrap().clone();
                    cycles.push(cycle);
                }
            }
        }

        return cycles;
    }

    pub fn update_cycles(&mut self) {
        let mut nb_cycles = 0;
        for token in self.inventory.iter() {
            // get all the tracked pools
            let pools = self
                .pools
                .iter()
                .map(|entry| entry.value().clone())
                .collect::<Vec<_>>();

            // compute all potential cycles
            let cycles = Self::get_cycles(
                &pools,
                token.clone(),
                token.clone(),
                3,
                &vec![],
                &mut vec![],
                &mut HashSet::new(),
            );

            info!("Found {} cycles", cycles.len());

            for cycle in cycles {
                nb_cycles += 1;
                let id = cycle.id.clone();
                self.cycles.insert(id.clone(), cycle.clone());

                for pool in &cycle.amms {
                    let pool_address = pool.address();
                    let pool_cycles = self.pools_cycles_map.get_mut(&pool_address);
                    if let Some(mut c) = pool_cycles {
                        c.insert(id.clone());
                    } else {
                        self.pools_cycles_map
                            .insert(pool_address, HashSet::from([id.clone()]));
                    }
                }
            }
        }

        info!("Found {} cycles", self.cycles.len());
        info!("Nb cycles: {}", nb_cycles);
    }

    pub async fn load_pools_from_checkpoint(&self, path: &str) -> Result<(), AMMError> {
        let (_, pools) =
            sync::checkpoint::sync_amms_from_checkpoint(path, 200, self.provider.clone()).await?;
        info!("Loaded {} pools from checkpoint", pools.len());
        for pool in pools {
            self.pools.insert(pool.address(), pool);
        }
        Ok(())
    }

    pub async fn load_pools_from_db(&self, db_url: &str) -> Result<(), AMMError> {
        let mut conn = establish_connection(db_url);
        let db_pools = db::queries::pool::get_filtered_pools(&mut conn, "arbitrum").unwrap();
        // let db_pools = db::queries::pool::get_pools_by_chain(&mut conn, "arbitrum").unwrap();

        info!("Loaded {} pools from db", db_pools.len());
        for pool in db_pools {
            let amm = shared::amm_utils::db_pool_to_amm(&pool).unwrap();
            self.pools.insert(amm.address(), amm);
        }

        Ok(())
    }

    pub async fn add_pools(&self, addresses: Vec<Address>) -> Result<(), AMMError> {
        let mut amms: Vec<AMM> = addresses
            .into_iter()
            .map(|address| {
                AMM::UniswapV2Pool(UniswapV2Pool::new(
                    address,
                    Address::ZERO,
                    0,
                    "".to_string(),
                    Address::ZERO,
                    0,
                    "".to_string(),
                    0,
                    0,
                    3000,
                    ExchangeName::UniswapV2,
                    ExchangeType::UniV2,
                    NamedChain::Arbitrum,
                ))
            })
            .collect();

        sync::populate_amms(&mut amms, self.block_number, self.provider.clone()).await?;

        for amm in amms {
            self.pools.insert(amm.address(), amm);
        }
        Ok(())
    }

    pub async fn update_pools(&self) -> Result<(), AMMError> {
        let mut amms: Vec<AMM> = self
            .pools
            .iter()
            .map(|entry| entry.value().clone())
            .collect();

        sync::populate_amms(&mut amms, self.block_number, self.provider.clone()).await?;

        for amm in amms {
            self.pools.insert(amm.address(), amm);
        }

        // for mut pool in self.pools.iter_mut() {
        //     pool.value_mut().sync(self.provider.clone()).await?;
        // }
        Ok(())
    }

    pub async fn update_block_number(&mut self, block_number: u64) -> Result<(), AMMError> {
        self.block_number = block_number;
        Ok(())
    }

    pub async fn get_all_pools(&self) -> Vec<AMM> {
        self.pools
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }
}
