use std::error::Error;
use std::sync::Arc;

use alloy_chains::Chain;
use amms::amm::common::get_detailed_pool_data_batch_request;
use amms::amm::uniswap_v3::factory::UniswapV3Factory;
use amms::amm::AutomatedMarketMaker;
use amms::errors::AMMError;
// use amms::config::get_chain_config;
use alloy::network::Network;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::transports::Transport;
use amms::{
    amm::{
        factory::Factory,
        uniswap_v2::{factory::UniswapV2Factory, UniswapV2Pool},
        AMM,
    },
    filters::value::filter_amms_below_usd_threshold,
    sync::{self, checkpoint},
};
use db::models::NewPool;
use db::{batch_insert_pools, establish_connection};
use types::{DetailedPool, ExchangeName, ExchangeType};

use crate::addressbook::Addressbook;
use crate::config::get_chain_config;

// let chain_config = get_chain_config(chain).await;
// let provider = chain_config.ws;
// let addressbook = Addressbook::load().unwrap();
// let named_chain = chain.named().unwrap();
// let v3_factories = addressbook.get_v3_factories(&named_chain);
// let v3_factories: Vec<UniswapV3Factory> = v3_factories
//     .into_iter()
//     .map(|addr| UniswapV3Factory::new(addr, from_block))
//     .collect();

pub async fn store_uniswap_v3_pools<P, T, N>(
    provider: Arc<P>,
    chain: Chain,
    factory_address: Address,
    from_block: u64,
    to_block: u64,
    step: u64,
    db_url: &str,
) -> Result<(), AMMError>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    let mut conn = establish_connection(db_url);
    let factory = UniswapV3Factory::new(factory_address, from_block);

    let pools = factory
        .get_pools_from_logs(from_block, to_block, step, provider.clone())
        .await?;

    println!("Got {:?} pools", pools.len());

    let mut pools = pools
        .iter()
        .map(|pool| {
            DetailedPool::empty(
                pool.address(),
                chain.named().unwrap(),
                Some(ExchangeType::UniV3),
                Some(ExchangeName::UniswapV3),
            )
        })
        .collect::<Vec<DetailedPool>>();

    get_detailed_pool_data_batch_request(&mut pools, provider.clone()).await?;

    let new_pools = pools
        .iter()
        .map(|pool| pool.to_new_pool())
        .collect::<Vec<NewPool>>();

    batch_insert_pools(&mut conn, &new_pools).unwrap();

    println!(
        "Inserted {:?} pools created from block {:?} to {:?}",
        new_pools.len(),
        from_block,
        to_block
    );

    Ok(())
}

pub async fn get_filtered_amms(chain: Chain, usd_threshold: f64) -> Result<Vec<AMM>, AMMError> {
    let chain_config = get_chain_config(chain).await;
    let provider = chain_config.ws;
    let addressbook = Addressbook::load().unwrap();

    let named_chain = chain.named().unwrap();
    let v2_factories = addressbook.get_v2_factories(&named_chain);
    let v3_factories = addressbook.get_v3_factories(&named_chain);
    let weth_address = addressbook.get_weth(&named_chain).unwrap();
    let exchange_name = ExchangeName::UniswapV2;
    let weth_usdc_address = addressbook
        .get_pool_by_name(&named_chain, exchange_name, "WETH-USDC")
        .unwrap();

    let weth_usdc_pool = AMM::UniswapV2Pool(
        UniswapV2Pool::new_from_address(weth_usdc_address, 300, provider.clone()).await?,
    );
    let start_block = 0;
    let v2_factories: Vec<Factory> = v2_factories
        .into_iter()
        .map(|addr| Factory::UniswapV2Factory(UniswapV2Factory::new(addr, 150442611, 300)))
        .collect();
    let v3_factories: Vec<Factory> = v3_factories
        .into_iter()
        .map(|addr| Factory::UniswapV3Factory(UniswapV3Factory::new(addr, start_block)))
        .collect();

    // let factories = [v2_factories, v3_factories].concat();
    let factories = v3_factories;

    // create a filename dependent on the chain
    let path = format!("./pools_{}.json", chain.named().unwrap());
    let (pools, last_block) = sync::sync_amms(
        factories.clone(),
        provider.clone(),
        Some(path.as_str()),
        100000,
    )
    .await
    .unwrap();

    println!("Synced pools!");

    let weth_value_in_token_to_weth_pool_threshold = U256::from(100000000000000000_u128); // 10 weth

    let filtered_pools = filter_amms_below_usd_threshold(
        pools,
        &factories,
        weth_usdc_pool,
        usd_threshold,
        weth_address,
        weth_value_in_token_to_weth_pool_threshold,
        5000,
        provider.clone(),
    )
    .await?;

    println!("Filtered pools: {:?}", filtered_pools);
    println!("Found {} pools", filtered_pools.len());

    // call checkpoint with empty factorie vector
    let result = checkpoint::construct_checkpoint(
        Vec::new(),
        &filtered_pools,
        last_block,
        "./filtered-pools.json",
    );

    result.unwrap();

    Ok(filtered_pools)
}
