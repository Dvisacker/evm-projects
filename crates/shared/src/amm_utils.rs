use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use alloy::network::Network;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::transports::Transport;
use alloy_chains::{Chain, NamedChain};
use amms::amm::camelot_v3::CamelotV3Pool;
use amms::amm::common::get_standard_pool_data_batch_request;
use amms::amm::uniswap_v2::IUniswapV2Pair;
use amms::amm::uniswap_v3::IUniswapV3Pool;
use amms::amm::AutomatedMarketMaker;
use amms::errors::AMMError;
use amms::filters::value::get_weth_values_in_amms;
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
use db::models::{DbPool, NewDbPool};
use db::queries::pool::{batch_update_filtered, batch_upsert_pools, get_pools};
use types::exchange::{ExchangeName, ExchangeType};
use types::standard_pool::StandardPool;

use crate::addressbook::Addressbook;
use crate::config::get_chain_config;

/// Determines the exchange type of a pool given its address.
///
/// This function attempts to identify whether a pool is UniswapV2, UniswapV3, CamelotV3, or Unknown
/// by calling specific functions on the pool contract.
///
/// # Arguments
/// * `address` - The address of the pool to check
/// * `provider` - An Arc-wrapped provider for making RPC calls
///
/// # Returns
/// A Result containing the ExchangeType or an AMMError
pub async fn get_pool_type<P, T, N>(
    address: Address,
    provider: Arc<P>,
) -> Result<ExchangeType, AMMError>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    let univ2_pool = IUniswapV2Pair::new(address, provider.clone());
    let univ3_pool = IUniswapV3Pool::new(address, provider.clone());

    match univ3_pool.factory().call().await {
        Ok(_factory) => match univ3_pool.tickSpacing().call().await {
            Ok(_tick_spacing) => {
                return Ok(ExchangeType::UniV3);
            }
            Err(_) => match univ2_pool.getReserves().call().await {
                Ok(_reserves) => {
                    return Ok(ExchangeType::UniV2);
                }
                Err(_) => {
                    return Ok(ExchangeType::Unknown);
                }
            },
        },
        Err(_) => match univ3_pool.tickSpacing().call().await {
            Ok(_tick_spacing) => {
                return Ok(ExchangeType::CamelotV3);
            }
            Err(_) => {
                return Ok(ExchangeType::Unknown);
            }
        },
    }
}

/// Stores Uniswap V3 pools in the database.
///
/// This function fetches Uniswap V3 pools from logs within a specified block range,
/// populates their data, and stores them in the database.
///
/// # Arguments
/// * `provider` - An Arc-wrapped provider for making RPC calls
/// * `chain` - The blockchain on which the pools exist
/// * `factory_address` - The address of the Uniswap V3 factory
/// * `from_block` - The starting block number to fetch pools from
/// * `to_block` - The ending block number to fetch pools to
/// * `step` - The number of blocks to process in each iteration
/// * `db_url` - The URL of the database to store the pools
///
/// # Returns
/// A Result indicating success or an AMMError
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
            StandardPool::empty(
                pool.address(),
                chain.named().unwrap(),
                Some(factory_address),
                Some(ExchangeType::UniV3),
                Some(ExchangeName::UniswapV3),
            )
        })
        .collect::<Vec<StandardPool>>();

    let max_batch_size = 50;
    for chunk in pools.chunks_mut(max_batch_size) {
        get_standard_pool_data_batch_request(chunk, provider.clone()).await?;
    }

    let new_pools = pools
        .iter()
        .map(|pool| pool.to_new_db_pool())
        .collect::<Vec<NewDbPool>>();

    batch_upsert_pools(&mut conn, &new_pools).unwrap();

    tracing::info!(
        "Inserted {:?} pools created from block {:?} to {:?}",
        new_pools.len(),
        from_block,
        to_block
    );

    Ok(())
}

/// Stores Uniswap V2 pools in the database.
///
/// This function syncs Uniswap V2 pools, populates their data, and stores them in the database.
///
/// # Arguments
/// * `provider` - An Arc-wrapped provider for making RPC calls
/// * `chain` - The blockchain on which the pools exist
/// * `exchange_name` - The name of the exchange (e.g., UniswapV2)
/// * `factory_address` - The address of the Uniswap V2 factory
/// * `db_url` - The URL of the database to store the pools
///
/// # Returns
/// A Result indicating success or an AMMError
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
            StandardPool::empty(
                pool.address(),
                chain.named().unwrap(),
                Some(factory_address),
                Some(ExchangeType::UniV2),
                Some(exchange_name),
            )
        })
        .collect::<Vec<StandardPool>>();

    for chunk in pools.chunks(50) {
        let mut chunk = chunk.to_vec();
        get_standard_pool_data_batch_request(&mut chunk, provider.clone()).await?;

        let new_pools = chunk
            .iter()
            .map(|pool| pool.to_new_db_pool())
            .collect::<Vec<NewDbPool>>();

        batch_upsert_pools(&mut conn, &new_pools).unwrap();
        tracing::info!("Inserted {:?} pools", new_pools.len());
    }

    Ok(())
}

/// Activates pools that meet a certain USD value threshold.
///
/// This function retrieves pools from the database, filters them based on a USD threshold,
/// and marks the filtered pools as active in the database.
///
/// # Arguments
/// * `chain` - The blockchain on which the pools exist
/// * `exchange_name` - The name of the exchange to filter pools for
/// * `usd_threshold` - The minimum USD value for a pool to be considered active
/// * `db_url` - The URL of the database to retrieve and update pools
///
/// # Returns
/// A Result containing a vector of activated AMMs or an AMMError
pub async fn activate_pools(
    chain: Chain,
    exchange_name: ExchangeName,
    usd_threshold: f64,
    db_url: &str,
) -> Result<(), AMMError> {
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

    for chunk in amms.chunks(1000) {
        let result = filter_amms(chain, usd_threshold, chunk.to_vec()).await;
        if result.is_err() {
            tracing::error!("Error filtering amms: {:?}", result.err());
            continue;
        }
        let filtered_pools = result.unwrap();
        let active_pools = filtered_pools
            .iter()
            .map(|amm| amm.address().to_string())
            .collect::<Vec<String>>();

        let inactive_pools = chunk
            .iter()
            .filter(|amm| !active_pools.contains(&amm.address().to_string()))
            .map(|amm| amm.address().to_string())
            .collect::<Vec<String>>();

        batch_update_filtered(&mut conn, &active_pools, true).unwrap();
        batch_update_filtered(&mut conn, &inactive_pools, false).unwrap();
        tracing::info!(
            "Processed pool chunk. Active pools: {:?}. Inactive pools: {:?}",
            active_pools.len(),
            inactive_pools.len()
        );
    }

    Ok(())
}

/// Calculates the value of a specific AMM pool.
///
/// This function determines the type of pool, creates the appropriate AMM object,
/// and calculates its value in terms of WETH.
///
/// # Arguments
/// * `chain` - The blockchain on which the pool exists
/// * `pool_address` - The address of the pool to value
///
/// # Returns
/// A Result containing the U256 value of the pool or an AMMError
pub async fn get_amm_value(chain: Chain, pool_address: Address) -> Result<U256, AMMError> {
    let chain_config = get_chain_config(chain).await;
    let provider = chain_config.ws;
    let addressbook = Addressbook::load().unwrap();
    let named_chain = chain.named().unwrap();
    let weth_address = addressbook.get_weth(&named_chain).unwrap();

    let weth_usdc_address =
        Address::from_str("0xc31e54c7a869b9fcbecc14363cf510d1c41fa443").unwrap();
    let mut weth_usdc_pool = AMM::UniswapV3Pool(UniswapV3Pool {
        address: weth_usdc_address,
        exchange_name: ExchangeName::UniswapV3,
        exchange_type: ExchangeType::UniV3,
        chain: named_chain,
        ..Default::default()
    });
    weth_usdc_pool.populate_data(None, provider.clone()).await?;
    let pool_type = get_pool_type(pool_address, provider.clone()).await?;

    let mut amm: AMM;
    match pool_type {
        ExchangeType::UniV2 => {
            let pool = UniswapV2Pool {
                address: pool_address,
                exchange_name: ExchangeName::UniswapV2,
                exchange_type: ExchangeType::UniV2,
                chain: named_chain,
                ..Default::default()
            };
            amm = AMM::UniswapV2Pool(pool);
        }
        ExchangeType::UniV3 => {
            let pool = UniswapV3Pool {
                address: pool_address,
                exchange_name: ExchangeName::UniswapV3,
                exchange_type: ExchangeType::UniV3,
                chain: named_chain,
                ..Default::default()
            };
            amm = AMM::UniswapV3Pool(pool);
        }
        ExchangeType::CamelotV3 => {
            let pool = CamelotV3Pool {
                address: pool_address,
                exchange_name: ExchangeName::CamelotV3,
                exchange_type: ExchangeType::CamelotV3,
                chain: named_chain,
                ..Default::default()
            };
            amm = AMM::CamelotV3Pool(pool);
        }
        _ => {
            return Err(AMMError::UnknownPoolType);
        }
    }

    amm.populate_data(None, provider.clone()).await?;

    let v3_factories = addressbook.get_v3_factories(&named_chain);

    let factories = v3_factories
        .iter()
        .map(|factory| Factory::UniswapV3Factory(UniswapV3Factory::new(*factory, 0)))
        .collect::<Vec<Factory>>();

    let weth_values_in_pools = get_weth_values_in_amms(
        &[amm],
        &factories,
        weth_address,
        U256::from(10000000000000000000_u128),
        100,
        provider,
    )
    .await?;

    let weth_value_in_amm = weth_values_in_pools[0];
    println!("Weth value in amm: {:?}", weth_value_in_amm);

    Ok(U256::from(0))
}

/// Filters AMMs based on a USD value threshold.
///
/// This function takes a list of AMMs and filters out those below a specified USD value threshold.
///
/// # Arguments
/// * `chain` - The blockchain on which the AMMs exist
/// * `usd_threshold` - The minimum USD value for an AMM to be included
/// * `amms` - A vector of AMMs to filter
///
/// # Returns
/// A Result containing a vector of filtered AMMs or an AMMError
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
    let uniswap_v3_factory = addressbook.get_v3_factories(&named_chain)[0];
    let factories = vec![Factory::UniswapV3Factory(UniswapV3Factory::new(
        uniswap_v3_factory,
        0,
    ))];

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

/// Converts database pool objects to AMM objects.
///
/// This function takes a slice of Pool objects from the database and converts them
/// into the appropriate AMM objects based on their exchange type.
///
/// # Arguments
/// * `pools` - A slice of DbPool objects from the database
///
/// # Returns
/// A Result containing a vector of AMM objects or an AMMError
pub fn db_pools_to_amms(pools: &[DbPool]) -> Result<Vec<AMM>, AMMError> {
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
