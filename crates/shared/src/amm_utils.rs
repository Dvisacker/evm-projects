use std::collections::HashMap;
use std::sync::Arc;

use alloy_chains::{Chain, NamedChain};
use amms::amm::common::get_detailed_pool_data_batch_request;
use amms::amm::AutomatedMarketMaker;
use amms::errors::AMMError;
// use amms::config::get_chain_config;
use alloy::network::Network;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::transports::Transport;
use amms::sync::populate_amms;
use amms::{
    amm::{
        factory::Factory,
        uniswap_v2::{factory::UniswapV2Factory, UniswapV2Pool},
        uniswap_v3::{factory::UniswapV3Factory, UniswapV3Pool},
        AMM,
    },
    filters::value::filter_amms_below_usd_threshold,
    sync::{self, checkpoint},
};
use db::models::{NewPool, Pool};
use db::{
    batch_insert_pools, batch_update_filtered, batch_upsert_pools, establish_connection,
    get_all_pools, get_pools, get_pools_by_chain,
};
use types::exchange::{ExchangeName, ExchangeType};
use types::pool::DetailedPool;

use crate::addressbook::Addressbook;
use crate::config::get_chain_config;

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

pub async fn store_uniswap_v2_pools<P, T, N>(
    provider: Arc<P>,
    chain: Chain,
    factory_address: Address,
    db_url: &str,
) -> Result<(), AMMError>
where
    P: Provider<T, N> + 'static,
    T: Transport + Clone,
    N: Network,
{
    let mut conn = establish_connection(db_url);
    let factory = Factory::UniswapV2Factory(UniswapV2Factory::new(factory_address, 0, 3000));

    let (amms, _) = sync::sync_amms(vec![factory], provider.clone(), None, 100000)
        .await
        .unwrap();

    let pools = amms
        .iter()
        .map(|pool| {
            DetailedPool::empty(
                pool.address(),
                chain.named().unwrap(),
                Some(ExchangeType::UniV2),
                Some(ExchangeName::UniswapV2),
            )
        })
        .collect::<Vec<DetailedPool>>();

    for chunk in pools.chunks(50) {
        let mut chunk = chunk.to_vec();
        get_detailed_pool_data_batch_request(&mut chunk, provider.clone()).await?;

        let new_pools = chunk
            .iter()
            .map(|pool| pool.to_new_pool())
            .collect::<Vec<NewPool>>();

        batch_upsert_pools(&mut conn, &new_pools).unwrap();
        println!("Inserted {:?} pools", new_pools.len());
    }

    Ok(())
}

pub async fn filter_amms(
    chain: Chain,
    usd_threshold: f64,
    db_url: &str,
) -> Result<Vec<AMM>, AMMError> {
    let chain_config = get_chain_config(chain).await;
    let provider = chain_config.ws;
    let addressbook = Addressbook::load().unwrap();
    let named_chain = chain.named().unwrap();
    let mut conn = establish_connection(db_url);

    let pools = get_pools_by_chain(&mut conn, &named_chain.to_string()).unwrap();

    let block_number = provider.get_block_number().await.unwrap();
    let v2_factories = addressbook.get_v2_factories(&named_chain);
    let v2_factories: Vec<Factory> = v2_factories
        .into_iter()
        .map(|addr| Factory::UniswapV2Factory(UniswapV2Factory::new(addr, 0, 300)))
        .collect();
    let v3_factories = addressbook.get_v3_factories(&named_chain);
    let v3_factories: Vec<Factory> = v3_factories
        .into_iter()
        .map(|addr| Factory::UniswapV3Factory(UniswapV3Factory::new(addr, 0)))
        .collect();

    let amms = pools
        .iter()
        .map(|pool| db_pool_to_amm(pool))
        .collect::<Result<Vec<AMM>, AMMError>>()?;

    let mut v2_pools = amms
        .iter()
        .filter(|amm| matches!(amm, AMM::UniswapV2Pool(_)))
        .cloned()
        .collect::<Vec<AMM>>();

    populate_amms(&mut v2_pools, block_number, provider.clone())
        .await
        .unwrap();

    let mut v3_pools = amms
        .iter()
        .filter(|amm| matches!(amm, AMM::UniswapV3Pool(_)))
        .cloned()
        .collect::<Vec<AMM>>();

    populate_amms(&mut v3_pools, block_number, provider.clone())
        .await
        .unwrap();

    println!("Populated amms: {:?}", v3_pools);

    let weth_address = addressbook.get_weth(&named_chain).unwrap();
    let exchange_name: ExchangeName = ExchangeName::UniswapV2;
    let weth_usdc_address = addressbook
        .get_pool_by_name(&named_chain, exchange_name, "WETH-USDC")
        .unwrap();

    let weth_usdc_pool = AMM::UniswapV2Pool(
        UniswapV2Pool::new_from_address(weth_usdc_address, 300, provider.clone()).await?,
    );

    let weth_value_in_token_to_weth_pool_threshold = U256::from(1000000000000000000_u128); // 10 weth

    println!("Filtering amms");

    let v2_filtered_pools = filter_amms_below_usd_threshold(
        v2_pools,
        &v2_factories,
        weth_usdc_pool.clone(),
        usd_threshold,
        weth_address,
        weth_value_in_token_to_weth_pool_threshold,
        100,
        provider.clone(),
    )
    .await?;

    let v3_filtered_pools = filter_amms_below_usd_threshold(
        v3_pools,
        &v3_factories,
        weth_usdc_pool,
        usd_threshold,
        weth_address,
        weth_value_in_token_to_weth_pool_threshold,
        100,
        provider.clone(),
    )
    .await?;

    // concat v2 and v3 filtered pools
    let filtered_pools = v2_filtered_pools
        .into_iter()
        .chain(v3_filtered_pools.into_iter())
        .collect::<Vec<AMM>>();

    let filtered_addresses = filtered_pools
        .iter()
        .map(|amm| amm.address().to_string())
        .collect::<Vec<String>>();

    // Update filtered flag for filtered pools
    batch_update_filtered(&mut conn, &filtered_addresses, true).unwrap();

    Ok(filtered_pools)
}

pub fn db_pool_to_amm(pool: &Pool) -> Result<AMM, AMMError> {
    let address: Address = pool.address.parse().unwrap();
    let token0: Address = pool.token_a.parse().unwrap();
    let token1: Address = pool.token_b.parse().unwrap();
    let exchange_type: ExchangeType = ExchangeType::from_str(&pool.exchange_type).unwrap();
    let exchange_name: ExchangeName = ExchangeName::from_str(&pool.exchange_name).unwrap();
    let chain: Chain = Chain::try_from(pool.chain.parse::<NamedChain>().unwrap()).unwrap();

    match exchange_type {
        ExchangeType::UniV2 => Ok(AMM::UniswapV2Pool(UniswapV2Pool {
            address,
            token_a: token0,
            token_a_decimals: pool.token_a_decimals as u8,
            token_a_symbol: pool.token_a_symbol.clone(),
            token_b: token1,
            token_b_decimals: pool.token_b_decimals as u8,
            token_b_symbol: pool.token_b_symbol.clone(),
            reserve_0: pool.reserve_0.parse().unwrap(),
            reserve_1: pool.reserve_1.parse().unwrap(),
            fee: pool.fee as u32,
            exchange_name,
            exchange_type,
            chain: chain.named().unwrap(),
        })),
        ExchangeType::UniV3 => Ok(AMM::UniswapV3Pool(UniswapV3Pool {
            address,
            token_a: token0,
            token_a_decimals: pool.token_a_decimals as u8,
            token_a_symbol: pool.token_a_symbol.clone(),
            token_b: token1,
            token_b_decimals: pool.token_b_decimals as u8,
            token_b_symbol: pool.token_b_symbol.clone(),
            liquidity: 0,
            sqrt_price: U256::from(0),
            tick: 0,
            tick_spacing: 0,
            tick_bitmap: HashMap::new(),
            ticks: HashMap::new(),
            fee: pool.fee as u32,
            exchange_name,
            exchange_type,
            chain: chain.named().unwrap(),
        })),
        _ => panic!("Unsupported exchange type"),
    }
}
