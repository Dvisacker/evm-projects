use alloy::primitives::Address;
use alloy::providers::Provider;
use amms::amm::{AutomatedMarketMaker, AMM};
use amms::errors::AMMError;
use amms::sync;
use dashmap::DashMap;
use shared::cycle::Cycle;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Clone)]
pub struct State<P: Provider> {
    provider: Arc<P>,
    pub block_number: u64,
    pub inactive_pools: DashMap<Address, AMM>,
    pub pools: DashMap<Address, AMM>,
    pub pools_cycles_map: DashMap<Address, HashSet<String>>, // map of pool address to all cycles that include the pool
    pub cycles: HashMap<String, Cycle>,                      // map of cycle id to cycle
    pub inventory: Vec<Address>,                             // list of tokens that can be traded
}

impl<P: Provider + 'static> State<P> {
    pub fn new(provider: Arc<P>, inventory: Vec<Address>) -> Self {
        Self {
            provider,
            inventory,
            inactive_pools: DashMap::new(),
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

    pub fn set_inactive_pools(&self, amms: Vec<AMM>) {
        for amm in amms {
            self.inactive_pools.insert(amm.address(), amm);
        }
    }

    pub fn get_updated_cycles(&self, amms: Vec<AMM>) -> Vec<Cycle> {
        // get the cycles that include the amms
        let mut cycles = vec![];
        for amm in amms {
            tracing::info!("Getting pool cycles for: {:?}", amm.name());
            let pool_address = amm.address();
            tracing::info!("Pool address: {:?}", pool_address);
            let pool_cycles = self.pools_cycles_map.get(&pool_address);
            tracing::info!("Updated pool cycles: {:?}", pool_cycles);
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

    pub fn update_cycles(&mut self) -> Vec<Cycle> {
        let mut nb_cycles = 0;
        let mut all_cycles = vec![];
        for token in self.inventory.iter() {
            let pools = self
                .pools
                .iter()
                .map(|entry| entry.value().clone())
                .collect::<Vec<_>>();

            let cycles = Self::get_cycles(
                &pools,
                token.clone(),
                token.clone(),
                3,
                &vec![],
                &mut vec![],
                &mut HashSet::new(),
            );

            all_cycles.extend(cycles);
        }

        tracing::info!("Found {} potential cycles", all_cycles.len());

        let profit_threshold = -0.50;
        let potential_cycles: Vec<Cycle> = all_cycles
            .into_iter()
            .filter(|cycle| cycle.get_profit_perc() > profit_threshold)
            .collect();

        for cycle in potential_cycles.clone() {
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

        info!("Found {} cycles", self.cycles.len());
        info!("Nb cycles: {}", nb_cycles);
        return potential_cycles;
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
