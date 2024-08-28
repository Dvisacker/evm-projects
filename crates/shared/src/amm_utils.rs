use crate::addressbook::{Addressbook, ExchangeName};
use crate::config::get_chain_config;
use alloy::primitives::U256;
use alloy_chains::Chain;
use amms::{
    amm::{
        factory::Factory,
        uniswap_v2::{factory::UniswapV2Factory, UniswapV2Pool},
        AMM,
    },
    filters::value::filter_amms_below_usd_threshold,
    sync::{self, checkpoint},
};
use anyhow::{Error, Result};

pub async fn get_filtered_amms(chain: Chain, usd_threshold: f64) -> Result<Vec<AMM>, Error> {
    let chain_config = get_chain_config(chain).await;
    let provider = chain_config.ws;
    let addressbook = Addressbook::load().unwrap();

    let named_chain = chain.named().unwrap();
    let factories = addressbook.get_factories_by_chain(&named_chain);
    let weth_address = addressbook.get_weth(&named_chain).unwrap();
    let exchange_name = ExchangeName::UniswapV2;
    let weth_usdc_address = addressbook
        .get_pool_by_name(&named_chain, exchange_name, "WETH-USDC")
        .unwrap();

    let weth_usdc_pool = AMM::UniswapV2Pool(
        UniswapV2Pool::new_from_address(weth_usdc_address, 300, provider.clone()).await?,
    );
    let factories: Vec<Factory> = factories
        .into_iter()
        .map(|addr| Factory::UniswapV2Factory(UniswapV2Factory::new(addr, 0, 300)))
        .collect();

    // create a filename dependent on the chain
    let path = format!("./pools_{}.json", chain.named().unwrap());
    let (pools, last_block) = sync::sync_amms(
        factories.clone(),
        provider.clone(),
        Some(path.as_str()),
        1000,
    )
    .await
    .unwrap();

    let weth_value_in_token_to_weth_pool_threshold = U256::from(100000000000000000_u128); // 10 weth

    let filtered_pools = filter_amms_below_usd_threshold(
        pools,
        &factories,
        weth_usdc_pool,
        usd_threshold,
        weth_address,
        weth_value_in_token_to_weth_pool_threshold,
        200,
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
