use alloy::primitives::Address;
use alloy::providers::Provider;
use amms::amm::AutomatedMarketMaker;
use amms::amm::{
    factory::Factory, uniswap_v2::factory::UniswapV2Factory, uniswap_v2::UniswapV2Pool,
};
use dashmap::DashMap;
use std::sync::Arc;

pub struct PoolState<M: Provider> {
    provider: Arc<M>,
    pub pools: DashMap<Address, UniswapV2Pool>,
}

impl<M: Provider + 'static> PoolState<M> {
    pub async fn new(provider: Arc<M>) -> Self {
        Self {
            provider,
            pools: DashMap::new(),
        }
    }

    pub async fn add_pool(&self, address: Address) -> Result<(), Box<dyn std::error::Error>> {
        let pool = UniswapV2Pool::new_from_address(address, 3000, self.provider.clone()).await?;
        self.pools.insert(address.clone(), pool);
        Ok(())
    }

    pub async fn sync_pools(&self) -> Result<(), Box<dyn std::error::Error>> {
        for mut pool in self.pools.iter_mut() {
            pool.value_mut().sync(self.provider.clone()).await?;
        }
        Ok(())
    }
}

// pub struct DexState<M: Provider> {
//     provider: Arc<M>,
//     dexes: Vec<Dex>,
//     poolstate: PoolState<M>,
// }

// impl<M: Provider + 'static> DexState<M> {
//     pub async fn new(provider: Arc<M>, dexes: Vec<Dex>) -> Self {
//         Self {
//             provider: provider.clone(),
//             dexes,
//             poolstate: PoolState::new(provider.clone()).await,
//         }
//     }

//     pub async fn sync_dexes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         let checkpoint_path = ".pools-checkpoint.json";
//         let checkpoint_exists = Path::new(checkpoint_path).exists();
//         let pools = if checkpoint_exists {
//             let (_, pools) =
//                 sync_pools_from_checkpoint(checkpoint_path, 100, self.provider.clone()).await?;
//             pools
//         } else {
//             let pools = sync_pairs(
//                 self.dexes.clone(),
//                 self.provider.clone(),
//                 Some(checkpoint_path),
//             )
//             .await?;
//             pools
//         };

//         for pool in pools {
//             self.poolstate.pools.insert(pool.address(), pool);
//         }

//         Ok(())
//     }
// }
