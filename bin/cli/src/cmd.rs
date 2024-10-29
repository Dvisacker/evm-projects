use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::{network::EthereumWallet, signers::local::PrivateKeySigner};
use alloy_chains::{Chain, ChainKind, NamedChain};
use alloy_primitives::U256;
use eyre::{eyre, Error, Result};
use shared::{
    bridge::{bridge_lifi, ARBITRUM_CHAIN_ID, BASE_CHAIN_ID, USDC_ARBITRUM, USDC_BASE},
    config::get_chain_config,
    helpers::get_contract_creation_block,
    provider::get_provider,
};
use std::{str::FromStr, sync::Arc};
use tracing::info;
// use anyhow::{Error};
use shared::addressbook::Addressbook;
use shared::amm_utils::{store_uniswap_v2_pools, store_uniswap_v3_pools};
use types::exchange::ExchangeName;

pub async fn get_uniswap_v2_pools_command(
    chain_id: u64,
    exchange: ExchangeName,
) -> Result<(), Error> {
    let chain = Chain::try_from(chain_id).expect("Invalid chain ID");
    let chain_config = get_chain_config(chain).await;
    let provider = Arc::new(chain_config.ws);
    let addressbook = Addressbook::load().unwrap();

    let factory_address = match chain.kind() {
        ChainKind::Named(NamedChain::Arbitrum) => match exchange {
            ExchangeName::UniswapV2 => addressbook.arbitrum.exchanges.univ2.uniswapv2.factory,
            ExchangeName::SushiswapV2 => addressbook.arbitrum.exchanges.univ2.sushiswapv2.factory,
            _ => panic!("Choose a uniswap v2 type exchange"),
        },
        ChainKind::Named(NamedChain::Mainnet) => match exchange {
            ExchangeName::UniswapV2 => addressbook.mainnet.exchanges.univ2.uniswapv2.factory,
            ExchangeName::SushiswapV2 => addressbook.mainnet.exchanges.univ2.sushiswapv2.factory,
            _ => panic!("Choose a uniswap v2 type exchange"),
        },
        _ => panic!("Unsupported chain"),
    };

    info!("Downloading pools from {:?}", factory_address);
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    store_uniswap_v2_pools(provider.clone(), chain, exchange, factory_address, &db_url).await?;

    Ok(())
}

pub async fn get_uniswap_v3_pools_command(
    chain_id: u64,
    exchange: ExchangeName,
    from_block: u64,
    step: u64,
) -> Result<(), Error> {
    let chain = Chain::try_from(chain_id).expect("Invalid chain ID");
    let chain_config = get_chain_config(chain).await;
    let provider = Arc::new(chain_config.ws);
    let addressbook = Addressbook::load().unwrap();

    // todo: refactor this
    let factory_address = match chain.kind() {
        ChainKind::Named(NamedChain::Arbitrum) => match exchange {
            ExchangeName::UniswapV3 => addressbook.arbitrum.exchanges.univ3.uniswapv3.factory,
            ExchangeName::SushiswapV3 => addressbook.arbitrum.exchanges.univ3.sushiswapv3.factory,
            ExchangeName::CamelotV3 => addressbook.arbitrum.exchanges.univ3.camelotv3.factory,
            ExchangeName::RamsesV2 => addressbook.arbitrum.exchanges.univ3.ramsesv2.factory,
            ExchangeName::PancakeswapV3 => {
                addressbook.arbitrum.exchanges.univ3.pancakeswapv3.factory
            }
            _ => panic!("Choose a uniswap v3 exchange"),
        },
        ChainKind::Named(NamedChain::Mainnet) => match exchange {
            ExchangeName::UniswapV3 => addressbook.mainnet.exchanges.univ3.uniswapv3.factory,
            ExchangeName::SushiswapV3 => addressbook.mainnet.exchanges.univ3.sushiswapv3.factory,
            ExchangeName::PancakeswapV3 => {
                addressbook.mainnet.exchanges.univ3.pancakeswapv3.factory
            }
            _ => panic!("Choose a uniswap v3 exchange"),
        },
        _ => panic!("Unsupported chain"),
    };

    let from_block = from_block;
    let step = step;
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    store_uniswap_v3_pools(
        provider.clone(),
        chain,
        exchange,
        factory_address,
        Some(from_block),
        None,
        step,
        &db_url,
    )
    .await?;

    Ok(())
}

pub async fn get_contract_creation_block_command(
    chain_id: u64,
    contract_address: &str,
    start_block: Option<u64>,
    end_block: Option<u64>,
) -> Result<(), Error> {
    let chain = Chain::try_from(chain_id).expect("Invalid chain ID");
    let chain_config = get_chain_config(chain).await;
    let provider = Arc::new(chain_config.ws);
    let contract_address = Address::from_str(contract_address).expect("Invalid contract address");

    let start_block = start_block.unwrap_or(0);
    let end_block = match end_block {
        Some(block) => block,
        None => provider.get_block_number().await?,
    };

    match get_contract_creation_block(provider, contract_address, start_block, end_block).await {
        Ok(block) => info!("Contract creation block: {}", block),
        Err(e) => info!("Error finding contract creation block: {}", e),
    }

    Ok(())
}

pub async fn bridge_command(from_chain: &str, to_chain: &str, amount: &str) -> Result<(), Error> {
    let signer: PrivateKeySigner = std::env::var("DEV_PRIVATE_KEY")
        .expect("PRIVATE_KEY must be set")
        .parse()
        .expect("should parse private key");

    let wallet_address = signer.address();
    let wallet = EthereumWallet::new(signer);

    // Determine chain IDs and token addresses based on input
    let (from_chain_id, to_chain_id, from_token, to_token) = match (from_chain, to_chain) {
        ("arbitrum", "base") => (
            ARBITRUM_CHAIN_ID,
            BASE_CHAIN_ID,
            Address::from_str(USDC_ARBITRUM).unwrap(),
            Address::from_str(USDC_BASE).unwrap(),
        ),
        ("base", "arbitrum") => (
            BASE_CHAIN_ID,
            ARBITRUM_CHAIN_ID,
            Address::from_str(USDC_BASE).unwrap(),
            Address::from_str(USDC_ARBITRUM).unwrap(),
        ),
        _ => return Err(eyre::eyre!("Unsupported chain combination")),
    };

    // Get provider for source chain
    let source_chain = match from_chain {
        "arbitrum" => Chain::from_named(NamedChain::Arbitrum),
        "base" => Chain::from_named(NamedChain::Base),
        _ => return Err(eyre::eyre!("Unsupported source chain")),
    };

    let provider = get_provider(source_chain, wallet).await;

    // Parse amount
    let amount = U256::from_str(amount).map_err(|_| eyre::eyre!("Invalid amount"))?;

    println!("Starting bridge from {} to {}", from_chain, to_chain);
    println!("Amount: {}", amount);

    let result = bridge_lifi(
        provider,
        from_chain_id,
        to_chain_id,
        from_token,
        to_token,
        amount,
        wallet_address,
        wallet_address,
    )
    .await?;

    println!("Bridge initiated successfully!");
    println!("Expected output amount: {}", result);

    Ok(())
}
