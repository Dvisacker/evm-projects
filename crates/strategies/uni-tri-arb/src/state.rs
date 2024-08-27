use alloy::primitives::Address;
use alloy::providers::Provider;
use amms::amm::{
    factory::Factory, uniswap_v2::factory::UniswapV2Factory, uniswap_v2::UniswapV2Pool,
};
use amms::amm::{AutomatedMarketMaker, AMM};
use amms::errors::AMMError;
use amms::sync;
use dashmap::DashMap;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PoolState<P: Provider> {
    provider: Arc<P>,
    pub block_number: u64,
    pub pools: DashMap<Address, UniswapV2Pool>,
}

pub type Cycle = Vec<UniswapV2Pool>;

impl<P: Provider + 'static> PoolState<P> {
    pub fn new(provider: Arc<P>) -> Self {
        Self {
            provider,
            block_number: 0,
            pools: DashMap::new(),
        }
    }

    pub fn get_cycles(
        pairs: &[UniswapV2Pool],
        token_in: Address,
        token_out: Address,
        max_hops: i32,
        current_pairs: &Vec<UniswapV2Pool>,
        circles: &mut Vec<Cycle>,
        seen: &mut HashSet<Address>,
    ) -> Vec<Cycle> {
        let mut circles_copy: Vec<Cycle> = circles.clone();

        for pair in pairs {
            if seen.contains(&pair.address) {
                continue;
            }

            let temp_out: Address;
            if token_in == pair.token_a {
                temp_out = pair.token_b;
            } else if token_in == pair.token_b {
                temp_out = pair.token_a;
            } else {
                continue;
            }

            let mut new_seen = seen.clone();
            new_seen.insert(pair.address);

            if temp_out == token_out {
                let mut new_cycle = current_pairs.clone();
                new_cycle.push(pair.clone());
                circles_copy.push(new_cycle);
            } else if max_hops > 1 {
                let mut new_pairs: Vec<UniswapV2Pool> = current_pairs.clone();
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
            println!("Loaded pool");
            if let AMM::UniswapV2Pool(pool) = pool {
                self.pools.insert(pool.address, pool);
            }
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
            if let AMM::UniswapV2Pool(pool) = amm {
                self.pools.insert(pool.address, pool);
            }
        }
        Ok(())
    }

    pub async fn update_pools(&self) -> Result<(), AMMError> {
        let mut amms: Vec<AMM> = self
            .pools
            .iter()
            .map(|entry| AMM::UniswapV2Pool(entry.value().clone()))
            .collect();

        sync::populate_amms(&mut amms, self.block_number, self.provider.clone()).await?;

        for amm in amms {
            if let AMM::UniswapV2Pool(pool) = amm {
                self.pools.insert(pool.address, pool);
            }
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

    pub async fn get_all_pools(&self) -> Vec<UniswapV2Pool> {
        self.pools
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }
}
