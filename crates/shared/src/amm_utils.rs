use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use alloy::network::Network;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::transports::Transport;
use alloy_chains::{Chain, NamedChain};
use amms::amm::camelot_v3::CamelotV3Pool;
use amms::amm::common::get_detailed_pool_data_batch_request;
use amms::amm::AutomatedMarketMaker;
use amms::errors::AMMError;
use amms::sync::populate_amms;
use amms::{
    amm::{
        factory::Factory,
        uniswap_v2::{factory::UniswapV2Factory, UniswapV2Pool},
        uniswap_v3::{factory::UniswapV3Factory, UniswapV3Pool},
        AMM,
    },
    filters::value::filter_amms_below_usd_threshold,
    sync::{self},
};
use db::establish_connection;
use db::models::{NewPool, Pool};
use db::queries::pool::{batch_update_filtered, batch_upsert_pools, get_pools};
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

    let mut pools = pools
        .iter()
        .map(|pool| {
            DetailedPool::empty(
                pool.address(),
                chain.named().unwrap(),
                Some(factory_address),
                Some(ExchangeType::UniV3),
                Some(ExchangeName::UniswapV3),
            )
        })
        .collect::<Vec<DetailedPool>>();

    let max_batch_size = 50;
    for chunk in pools.chunks_mut(max_batch_size) {
        get_detailed_pool_data_batch_request(chunk, provider.clone()).await?;
    }

    let new_pools = pools
        .iter()
        .map(|pool| pool.to_new_pool())
        .collect::<Vec<NewPool>>();

    batch_upsert_pools(&mut conn, &new_pools).unwrap();

    tracing::info!(
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
    exchange_name: ExchangeName,
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

    tracing::info!("Syncing uniswap v2 pools");
    let (amms, _) = sync::sync_amms(vec![factory], provider.clone(), None, 100000)
        .await
        .unwrap();

    let pools = amms
        .iter()
        .map(|pool| {
            DetailedPool::empty(
                pool.address(),
                chain.named().unwrap(),
                Some(factory_address),
                Some(ExchangeType::UniV2),
                Some(exchange_name),
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
        tracing::info!("Inserted {:?} pools", new_pools.len());
    }

    Ok(())
}

pub async fn activate_pools(
    chain: Chain,
    exchange_name: ExchangeName,
    usd_threshold: f64,
    db_url: &str,
) -> Result<Vec<AMM>, AMMError> {
    let named_chain = chain.named().unwrap();
    let mut conn = establish_connection(db_url);
    let pools = get_pools(
        &mut conn,
        Some(&named_chain.to_string()),
        Some(&exchange_name.to_string()),
        None,
        None,
        None,
    )
    .unwrap();
    let amms = db_pools_to_amms(&pools)?;

    println!("Got {:?} pools", pools.len());

    let filtered_pools = filter_amms(chain, usd_threshold, amms).await?;
    let filtered_addresses = filtered_pools
        .iter()
        .map(|amm| amm.address().to_string())
        .collect::<Vec<String>>();

    println!("Filtered pools: {:?}", filtered_pools.len());

    batch_update_filtered(&mut conn, &filtered_addresses, true).unwrap();
    Ok(filtered_pools)
}

pub async fn get_amm_value(chain: Chain, pool_address: Address) -> Result<U256, AMMError> {
    let chain_config = get_chain_config(chain).await;
    let provider = chain_config.ws;
    let addressbook = Addressbook::load().unwrap();
    let named_chain = chain.named().unwrap();
    let _weth_address = addressbook.get_weth(&named_chain).unwrap();

    let weth_usdc = Address::from_str("0xc31e54c7a869b9fcbecc14363cf510d1c41fa443").unwrap();
    let _weth_usdc_pool = AMM::UniswapV2Pool(
        UniswapV2Pool::new_from_address(weth_usdc, 300, provider.clone()).await?,
    );
    let block_number = provider.get_block_number().await.unwrap();
    let _amm =
        UniswapV3Pool::new_from_address(pool_address, block_number, provider.clone()).await?;

    // let factory = Factory::UniswapV3Factory(UniswapV3Factory::new(
    //     addressbook.arbitrum.exchanges.univ3.uniswapv3.factory,
    //     0,
    // ));
    // let factories = vec![factory];

    // let weth_usd_price = weth_usdc_pool.calculate_price(weth_address)?;
    // println!("Weth usd price: {:?}", weth_usd_price);

    // let weth_values_in_pools = get_weth_values_in_amms(
    //     &[amm],
    //     &vec![],
    //     weth_address,
    //     weth_value_in_token_to_weth_pool_threshold,
    //     100,
    //     provider,
    // )
    // .await?;

    // let weth_value_in_amm = weth_values_in_pools[0];

    Ok(U256::from(0))
}

pub async fn filter_amms(
    chain: Chain,
    usd_threshold: f64,
    amms: Vec<AMM>,
) -> Result<Vec<AMM>, AMMError> {
    let chain_config = get_chain_config(chain).await;
    let provider = chain_config.ws;
    let addressbook = Addressbook::load().unwrap();
    let named_chain = chain.named().unwrap();

    let weth_address = addressbook.get_weth(&named_chain).unwrap();
    let exchange_name: ExchangeName = ExchangeName::UniswapV2;
    let weth_usdc_address = addressbook
        .get_pool_by_name(&named_chain, exchange_name, "WETH-USDC")
        .unwrap();

    let weth_usdc_pool = AMM::UniswapV2Pool(
        UniswapV2Pool::new_from_address(weth_usdc_address, 300, provider.clone()).await?,
    );
    let weth_value_in_token_to_weth_pool_threshold = U256::from(1000000000000000000_u128); // 10 weth
    let block_number = provider.get_block_number().await.unwrap();

    let factory = Factory::UniswapV3Factory(UniswapV3Factory::new(
        addressbook.arbitrum.exchanges.univ3.uniswapv3.factory,
        0,
    ));
    let factories = vec![factory];

    let mut v2_pools = amms
        .iter()
        .filter(|amm| matches!(amm, AMM::UniswapV2Pool(_)))
        .cloned()
        .collect::<Vec<AMM>>();

    let mut v2_filtered_pools = Vec::new();
    let mut v3_filtered_pools = Vec::new();
    if !v2_pools.is_empty() {
        populate_amms(&mut v2_pools, block_number, provider.clone())
            .await
            .unwrap();

        v2_filtered_pools = filter_amms_below_usd_threshold(
            v2_pools,
            &factories,
            weth_usdc_pool.clone(),
            usd_threshold,
            weth_address,
            weth_value_in_token_to_weth_pool_threshold,
            100,
            provider.clone(),
        )
        .await?;
    }

    let mut v3_pools = amms
        .iter()
        .filter(|amm| matches!(amm, AMM::UniswapV3Pool(_)))
        .cloned()
        .collect::<Vec<AMM>>();

    if !v3_pools.is_empty() {
        populate_amms(&mut v3_pools, block_number, provider.clone())
            .await
            .unwrap();

        v3_filtered_pools = filter_amms_below_usd_threshold(
            v3_pools,
            &factories,
            weth_usdc_pool,
            usd_threshold,
            weth_address,
            weth_value_in_token_to_weth_pool_threshold,
            100,
            provider.clone(),
        )
        .await?;
    }

    // concat v2 and v3 filtered pools
    let filtered_pools = v2_filtered_pools
        .into_iter()
        .chain(v3_filtered_pools.into_iter())
        .collect::<Vec<AMM>>();

    Ok(filtered_pools)
}

pub fn db_pools_to_amms(pools: &[Pool]) -> Result<Vec<AMM>, AMMError> {
    pools
        .iter()
        .map(|pool| {
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
                ExchangeType::CamelotV3 => Ok(AMM::CamelotV3Pool(CamelotV3Pool {
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
        })
        .collect()
}
