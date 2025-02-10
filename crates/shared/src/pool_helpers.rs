use std::str::FromStr;
use std::sync::Arc;

use addressbook::Addressbook;
use alloy::network::Network;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy_chains::{Chain, NamedChain};
use amms::amm::camelot_v3::CamelotV3Pool;
use amms::amm::uniswap_v2::IUniswapV2Pair;
use amms::amm::uniswap_v3::{json_to_tickbitmap, json_to_ticks, IUniswapV3Pool};
use amms::amm::ve33::Ve33Pool;
use amms::amm::AutomatedMarketMaker;
use amms::errors::AMMError;
use amms::filters::value::get_weth_values_in_amms;
use amms::sync::populate_amms;
use amms::{
    amm::{
        factory::Factory,
        uniswap_v2::UniswapV2Pool,
        uniswap_v3::{factory::UniswapV3Factory, UniswapV3Pool},
        AMM,
    },
    filters::value::filter_amms_below_usd_threshold,
};
use db::models::db_pool::DbPool;
use db::models::{DbUniV2Pool, DbUniV3Pool};
use provider::get_basic_provider;
use types::exchange::{ExchangeName, ExchangeType};

pub fn extract_v2_pools(amms: &[AMM]) -> Vec<UniswapV2Pool> {
    amms.iter()
        .filter_map(|amm| {
            if let AMM::UniswapV2Pool(pool) = amm {
                Some(pool.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn extract_v3_pools(amms: &[AMM]) -> Vec<UniswapV3Pool> {
    amms.iter()
        .filter_map(|amm| {
            if let AMM::UniswapV3Pool(pool) = amm {
                Some(pool.clone())
            } else {
                None
            }
        })
        .collect()
}

#[allow(dead_code)]
pub fn extract_camelot_v3_pools(amms: &[AMM]) -> Vec<CamelotV3Pool> {
    amms.iter()
        .filter_map(|amm| {
            if let AMM::CamelotV3Pool(pool) = amm {
                Some(pool.clone())
            } else {
                None
            }
        })
        .collect()
}

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
pub async fn get_pool_type<P, N>(
    address: Address,
    provider: Arc<P>,
) -> Result<ExchangeType, AMMError>
where
    P: Provider<N>,
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
    let provider = get_basic_provider(chain).await;
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
/// A Result containing a vector of active AMMs or an AMMError
pub async fn filter_amms(
    chain: Chain,
    usd_threshold: f64,
    amms: Vec<AMM>,
) -> Result<Vec<AMM>, AMMError> {
    let provider = get_basic_provider(chain).await;

    let v2_active_pools =
        filter_univ2_pools(amms.clone(), chain, provider.clone(), usd_threshold).await?;
    let v3_active_pools =
        filter_univ3_pools(amms.clone(), chain, provider.clone(), usd_threshold).await?;

    let active_pools = v2_active_pools
        .into_iter()
        .chain(v3_active_pools.into_iter())
        .collect::<Vec<AMM>>();

    Ok(active_pools)
}

pub async fn filter_univ2_pools<P, N>(
    amms: Vec<AMM>,
    chain: Chain,
    provider: Arc<P>,
    usd_threshold: f64,
) -> Result<Vec<AMM>, AMMError>
where
    P: Provider<N> + 'static,
    N: Network,
{
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

    let mut v2_active_pools = Vec::new();
    if !v2_pools.is_empty() {
        populate_amms(&mut v2_pools, block_number, provider.clone(), true)
            .await
            .unwrap();

        v2_active_pools = filter_amms_below_usd_threshold(
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

    Ok(v2_active_pools)
}

pub async fn filter_univ3_pools<P, N>(
    amms: Vec<AMM>,
    chain: Chain,
    provider: Arc<P>,
    usd_threshold: f64,
) -> Result<Vec<AMM>, AMMError>
where
    P: Provider<N> + 'static,
    N: Network,
{
    let addressbook = Addressbook::load().unwrap();
    let named_chain = chain.named().unwrap();
    let weth_address = addressbook.get_weth(&named_chain).unwrap();
    let exchange_name: ExchangeName = ExchangeName::UniswapV3;
    let weth_usdc_address = addressbook
        .get_pool_by_name(&named_chain, exchange_name, "WETH-USDC")
        .expect("WETH-USDC pool not found");
    let mut weth_usdc_pool = AMM::UniswapV3Pool(UniswapV3Pool {
        address: weth_usdc_address,
        exchange_name: ExchangeName::UniswapV3,
        exchange_type: ExchangeType::UniV3,
        chain: named_chain,
        ..Default::default()
    });
    weth_usdc_pool.populate_data(None, provider.clone()).await?;
    let weth_value_in_token_to_weth_pool_threshold = U256::from(1000000000000000000_u128); // 10 weth
    let block_number = provider.get_block_number().await.unwrap();
    let uniswap_v3_factory = addressbook.get_v3_factories(&named_chain)[0];
    let factories = vec![Factory::UniswapV3Factory(UniswapV3Factory::new(
        uniswap_v3_factory,
        0,
    ))];

    let mut v3_pools = amms
        .iter()
        .filter(|amm| matches!(amm, AMM::UniswapV3Pool(_)))
        .cloned()
        .collect::<Vec<AMM>>();

    let mut v3_active_pools = Vec::new();
    if !v3_pools.is_empty() {
        populate_amms(&mut v3_pools, block_number, provider.clone(), false)
            .await
            .unwrap();

        v3_active_pools = filter_amms_below_usd_threshold(
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

    Ok(v3_active_pools)
}

pub fn db_pools_to_amms(pools: &[DbPool]) -> Result<Vec<AMM>, AMMError> {
    pools.iter().map(|pool| db_pool_to_amm(pool)).collect()
}

pub fn db_pool_to_amm(pool: &DbPool) -> Result<AMM, AMMError> {
    match pool {
        DbPool::UniV2(pool) => db_univ2_pool_to_amm(pool),
        DbPool::UniV3(pool) => db_univ3_pool_to_amm(pool),
        DbPool::ERC4626Vault(_) => Err(AMMError::UnsupportedPoolType),
        DbPool::Curve(_) => Err(AMMError::UnsupportedPoolType),
    }
}

pub fn db_univ2_pool_to_amm(pool: &DbUniV2Pool) -> Result<AMM, AMMError> {
    let address: Address = pool.address.parse().unwrap();
    let token0: Address = pool.token_a.parse().unwrap();
    let token1: Address = pool.token_b.parse().unwrap();
    let exchange_type: ExchangeType =
        ExchangeType::from_str(pool.exchange_type.as_ref().unwrap()).unwrap();
    let exchange_name: ExchangeName =
        ExchangeName::from_str(pool.exchange_name.as_ref().unwrap()).unwrap();
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
            chain: chain.named().ok_or(AMMError::ParseError)?,
            factory: Address::ZERO,
        })),
        ExchangeType::Ve33 => Ok(AMM::Ve33Pool(Ve33Pool {
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
            stable: false, // TODO: fix this
            exchange_name,
            exchange_type,
            chain: chain.named().ok_or(AMMError::ParseError)?,
            factory: Address::ZERO,
        })),
        _ => Err(AMMError::UnsupportedExchangeType),
    }
}

fn db_univ3_pool_to_amm(pool: &DbUniV3Pool) -> Result<AMM, AMMError> {
    let address: Address = pool.address.parse().unwrap();
    let token0: Address = pool.token_a.parse().unwrap();
    let token1: Address = pool.token_b.parse().unwrap();
    let exchange_type: ExchangeType =
        ExchangeType::from_str(&pool.exchange_type.as_ref().unwrap()).unwrap();
    let exchange_name: ExchangeName =
        ExchangeName::from_str(&pool.exchange_name.as_ref().unwrap()).unwrap();
    let chain: Chain = Chain::try_from(pool.chain.parse::<NamedChain>().unwrap()).unwrap();

    match exchange_type {
        ExchangeType::UniV3 => db_univ3_pool_to_univ3_amm(
            pool,
            address,
            token0,
            token1,
            exchange_name,
            exchange_type,
            chain,
        ),
        ExchangeType::CamelotV3 => db_univ3_pool_to_camelotv3_amm(
            pool,
            address,
            token0,
            token1,
            exchange_name,
            exchange_type,
            chain,
        ),
        _ => Err(AMMError::UnsupportedExchangeType),
    }
}

fn db_univ3_pool_to_univ3_amm(
    pool: &DbUniV3Pool,
    address: Address,
    token0: Address,
    token1: Address,
    exchange_name: ExchangeName,
    exchange_type: ExchangeType,
    chain: Chain,
) -> Result<AMM, AMMError> {
    Ok(AMM::UniswapV3Pool(UniswapV3Pool {
        address,
        token_a: token0,
        token_a_decimals: pool.token_a_decimals as u8,
        token_a_symbol: pool.token_a_symbol.clone(),
        token_b: token1,
        token_b_decimals: pool.token_b_decimals as u8,
        token_b_symbol: pool.token_b_symbol.clone(),
        liquidity: pool
            .liquidity
            .as_ref()
            .and_then(|l| l.parse().ok())
            .unwrap_or(0),
        sqrt_price: pool
            .sqrt_price
            .as_ref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(U256::from(0)),
        tick: pool.tick.unwrap_or(0),
        tick_spacing: pool.tick_spacing.unwrap_or(0),
        tick_bitmap: json_to_tickbitmap(pool.tick_bitmap.clone().unwrap_or_default()),
        ticks: json_to_ticks(pool.ticks.clone().unwrap_or_default()),
        fee: pool.fee.unwrap_or(0) as u32,
        exchange_name,
        exchange_type,
        chain: chain.named().ok_or(AMMError::ParseError)?,
        factory: Address::ZERO, //TODO
        liquidity_net: 0,       // TODO
    }))
}

fn db_univ3_pool_to_camelotv3_amm(
    pool: &DbUniV3Pool,
    address: Address,
    token0: Address,
    token1: Address,
    exchange_name: ExchangeName,
    exchange_type: ExchangeType,
    chain: Chain,
) -> Result<AMM, AMMError> {
    Ok(AMM::CamelotV3Pool(CamelotV3Pool {
        address,
        token_a: token0,
        token_a_decimals: pool.token_a_decimals as u8,
        token_a_symbol: pool.token_a_symbol.clone(),
        token_b: token1,
        token_b_decimals: pool.token_b_decimals as u8,
        token_b_symbol: pool.token_b_symbol.clone(),
        liquidity: pool
            .liquidity
            .as_ref()
            .and_then(|l| l.parse().ok())
            .unwrap_or(0),
        sqrt_price: pool
            .sqrt_price
            .as_ref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(U256::from(0)),
        tick: pool.tick.unwrap_or(0),
        tick_spacing: pool.tick_spacing.unwrap_or(0),
        tick_bitmap: json_to_tickbitmap(pool.tick_bitmap.clone().unwrap_or_default()),
        ticks: json_to_ticks(pool.ticks.clone().unwrap_or_default()),
        fee: pool.fee.unwrap_or(0) as u32,
        exchange_name,
        exchange_type,
        chain: chain.named().ok_or(AMMError::ParseError)?,
    }))
}
