use crate::addressbook::Addressbook;
use ethers::{
    prelude::*,
    providers::{Http, Provider},
};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use uni_tri_arb_bindings::uniswap_v2_factory::UniswapV2Factory;
use uni_tri_arb_bindings::uniswap_v2_pair::UniswapV2Pair;

pub struct UniswapV2Client<M: Middleware + 'static> {
    factory: UniswapV2Factory<M>,
    chain_id: u64,
    client: Arc<M>,
}

impl<M: Middleware + 'static> UniswapV2Client<M> {
    pub fn new(client: Arc<M>, addressbook: &Addressbook, chain_id: u64) -> Self {
        let factory_address = match chain_id {
            42161 => H160::from_str(&addressbook.arbitrum.uniswapv2.factory).unwrap(),
            10 => H160::from_str(&addressbook.optimism.uniswapv2.factory).unwrap(),
            _ => panic!("Unsupported chain ID"),
        };

        let factory = UniswapV2Factory::new(factory_address, client.clone());

        Self {
            factory,
            chain_id,
            client,
        }
    }

    pub async fn get_all_pairs(&self) -> Result<Vec<Address>, Box<dyn std::error::Error>> {
        let pair_count = self.factory.all_pairs_length().call().await?;
        let mut pairs = Vec::with_capacity(pair_count.as_usize());

        for i in 0..pair_count.as_u64() {
            let pair_address = self.factory.all_pairs(i.into()).call().await?;
            pairs.push(pair_address);
        }

        Ok(pairs)
    }

    pub async fn update_reserves(
        &self,
        pairs: &[Address],
    ) -> Result<HashMap<Address, (U256, U256)>, Box<dyn std::error::Error>> {
        let mut reserves = HashMap::new();

        for &pair_address in pairs {
            let pair = UniswapV2Pair::new(pair_address, self.client.clone());
            let (reserve0, reserve1, _) = pair.get_reserves().call().await?;
            reserves.insert(pair_address, (reserve0.into(), reserve1.into()));
        }

        Ok(reserves)
    }
}

pub async fn initialize(
    addressbook: &Addressbook,
    chain_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = match chain_id {
        42161 => "https://arb1.arbitrum.io/rpc",
        10 => "https://mainnet.optimism.io",
        _ => return Err("Unsupported chain ID".into()),
    };

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(provider);

    let uniswap_client = UniswapV2Client::new(client, addressbook, chain_id);
    let all_pairs = uniswap_client.get_all_pairs().await?;
    println!("Total pairs: {}", all_pairs.len());

    let sample_pairs = &all_pairs[..5]; // Get first 5 pairs for example
    let reserves = uniswap_client.update_reserves(sample_pairs).await?;
    for (pair, (reserve0, reserve1)) in reserves {
        println!(
            "Pair {}: Reserve0 = {}, Reserve1 = {}",
            pair, reserve0, reserve1
        );
    }

    Ok(())
}
