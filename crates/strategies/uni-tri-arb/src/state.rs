use alloy::primitives::Address;
use alloy::providers::Provider;
use amms::amm::uniswap_v2::UniswapV2Pool;
use amms::amm::{AutomatedMarketMaker, AMM};
use amms::errors::AMMError;
use amms::sync;
use dashmap::DashMap;
use db::establish_connection;
use shared::types::Cycle;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PoolState<P: Provider> {
    provider: Arc<P>,
    pub block_number: u64,
    pub pools: DashMap<Address, AMM>,
}

impl<P: Provider + 'static> PoolState<P> {
    pub fn new(provider: Arc<P>) -> Self {
        Self {
            provider,
            block_number: 0,
            pools: DashMap::new(),
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
                circles_copy.push(Cycle(pools));
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

    pub async fn load_pools_from_checkpoint(&self, path: &str) -> Result<(), AMMError> {
        let (_, pools) =
            sync::checkpoint::sync_amms_from_checkpoint(path, 200, self.provider.clone()).await?;
        println!("Loaded {} pools from checkpoint", pools.len());
        for pool in pools {
            self.pools.insert(pool.address(), pool);
        }
        Ok(())
    }

    pub async fn load_pools_from_db(&self, db_url: &str) -> Result<(), AMMError> {
        let mut conn = establish_connection(db_url);
        let db_pools = db::get_filtered_pools(&mut conn, "arbitrum").unwrap();

        println!("Loaded {} pools from db", db_pools.len());
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
                    Address::ZERO,
                    0,
                    0,
                    0,
                    3000,
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
