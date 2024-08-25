use cfmms::checkpoint::sync_pools_from_checkpoint;
use cfmms::dex::DexVariant;
use cfmms::sync::sync_pairs;
use cfmms::{dex::Dex, pool::Pool};
use dashmap::DashMap;
use ethers::prelude::*;
use ethers::types::Address;
use std::path::Path;
use std::sync::Arc;

pub struct PoolState<M: Middleware> {
    provider: Arc<M>,
    pools: DashMap<Address, Pool>,
}

impl<M: Middleware + 'static> PoolState<M> {
    pub async fn new(provider: Arc<M>) -> Self {
        Self {
            provider,
            pools: DashMap::new(),
        }
    }

    pub async fn add_pools(
        &self,
        pools: Vec<(Address, DexVariant)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (address, dex_variant) in pools {
            let pool = Pool::new_from_address(address, dex_variant, self.provider.clone()).await?;
            self.pools.insert(address, pool);
        }
        Ok(())
    }

    pub async fn sync_pools(&self) -> Result<(), Box<dyn std::error::Error>> {
        for mut pool in self.pools.iter_mut() {
            pool.value_mut().sync_pool(self.provider.clone()).await?;
        }
        Ok(())
    }
}

pub struct DexState<M: Middleware> {
    provider: Arc<M>,
    dexes: Vec<Dex>,
    poolstate: PoolState<M>,
}

impl<M: Middleware + 'static> DexState<M> {
    pub async fn new(provider: Arc<M>, dexes: Vec<Dex>) -> Self {
        Self {
            provider: provider.clone(),
            dexes,
            poolstate: PoolState::new(provider.clone()).await,
        }
    }

    pub async fn sync_dexes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let checkpoint_path = ".pools-checkpoint.json";
        let checkpoint_exists = Path::new(checkpoint_path).exists();
        let pools = if checkpoint_exists {
            let (_, pools) =
                sync_pools_from_checkpoint(checkpoint_path, 100, self.provider.clone()).await?;
            pools
        } else {
            let pools = sync_pairs(
                self.dexes.clone(),
                self.provider.clone(),
                Some(checkpoint_path),
            )
            .await?;
            pools
        };

        for pool in pools {
            self.poolstate.pools.insert(pool.address(), pool);
        }

        Ok(())
    }
}
