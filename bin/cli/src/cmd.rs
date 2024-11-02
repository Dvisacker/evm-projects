use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::{network::EthereumWallet, signers::local::PrivateKeySigner};
use alloy_chains::{Chain, ChainKind, NamedChain};
use alloy_primitives::U256;
use artemis_core::executors::crosschain_executor::{
    Bridge, CrossChainSwap, CrossChainSwapExecutor, Swap,
};
use artemis_core::types::Executor;
use eyre::{Error, Result};
use shared::{
    bridge::{bridge_lifi, ARBITRUM_CHAIN_ID, BASE_CHAIN_ID, USDC_ARBITRUM, USDC_BASE},
    config::get_chain_config,
    helpers::get_contract_creation_block,
    provider::get_provider,
};
use std::{str::FromStr, sync::Arc};
use tracing::info;
use types::bridge::BridgeName;
// use eyre::{Error};
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
    let addressbook = Addressbook::load(None).unwrap();
    let named_chain = chain.named().unwrap();
    let factory_address = addressbook.get_factory(&named_chain, exchange).unwrap();

    // let factory_address = match chain.kind() {
    //     ChainKind::Named(NamedChain::Arbitrum) => match exchange {
    //         ExchangeName::UniswapV2 => addressbook.arbitrum.exchanges.univ2.uniswapv2.factory,
    //         ExchangeName::SushiswapV2 => addressbook.arbitrum.exchanges.univ2.sushiswapv2.factory,
    //         _ => panic!("Choose a uniswap v2 type exchange"),
    //     },
    //     ChainKind::Named(NamedChain::Mainnet) => match exchange {
    //         ExchangeName::UniswapV2 => addressbook.mainnet.exchanges.univ2.uniswapv2.factory,
    //         ExchangeName::SushiswapV2 => addressbook.mainnet.exchanges.univ2.sushiswapv2.factory,
    //         _ => panic!("Choose a uniswap v2 type exchange"),
    //     },
    //     _ => panic!("Unsupported chain"),
    // };

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
    let addressbook = Addressbook::load(None).unwrap();
    let named_chain = chain.named().unwrap();
    let factory_address = addressbook.get_factory(&named_chain, exchange).unwrap();

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

    let origin_chain = Chain::from_named(NamedChain::from_str(from_chain).unwrap());
    let destination_chain = Chain::from_named(NamedChain::from_str(to_chain).unwrap());

    let origin_provider = get_provider(origin_chain, wallet.clone()).await;
    let destination_provider = get_provider(destination_chain, wallet.clone()).await;

    // Parse amount
    let amount = U256::from_str(amount).map_err(|_| eyre::eyre!("Invalid amount"))?;

    println!("Starting bridge from {} to {}", from_chain, to_chain);
    println!("Amount: {}", amount);

    let result = bridge_lifi(
        origin_provider,
        destination_provider,
        from_chain_id,
        to_chain_id,
        from_token,
        to_token,
        amount,
        wallet_address,
        wallet_address,
        BridgeName::Accross,
    )
    .await?;

    println!("Bridge initiated successfully!");
    println!("Expected output amount: {}", result);

    Ok(())
}

pub async fn cross_chain_swap_command(
    origin_chain: &str,
    destination_chain: &str,
    origin_token_in_address: &str,
    bridge_token: &str, //name of the token to bridge (USDC or WETH)
    destination_token_out_address: &str,
    amount_in: &str,
) -> Result<(), Error> {
    let signer: PrivateKeySigner = std::env::var("DEV_PRIVATE_KEY")
        .expect("PRIVATE_KEY must be set")
        .parse()
        .expect("should parse private key");
    let wallet = EthereumWallet::new(signer.clone());
    let wallet_address = signer.address();

    let origin_chain_name = NamedChain::from_str(origin_chain).unwrap();
    let origin_chain = Chain::from_named(origin_chain_name);
    let destination_chain_name = NamedChain::from_str(destination_chain).unwrap();
    let destination_chain = Chain::from_named(destination_chain_name);
    let origin_token_in = Address::from_str(origin_token_in_address).unwrap();
    let destination_token_out = Address::from_str(destination_token_out_address).unwrap();
    let amount_in = U256::from_str(amount_in).unwrap();
    let origin_provider = get_provider(origin_chain, wallet.clone()).await;
    let destination_provider = get_provider(destination_chain, wallet.clone()).await;
    let addressbook = Addressbook::load(None).unwrap();
    let origin_bridge_token = addressbook
        .get_token(&origin_chain_name, bridge_token)
        .unwrap();
    let destination_bridge_token = addressbook
        .get_token(&destination_chain_name, bridge_token)
        .unwrap();

    let swap_executor =
        CrossChainSwapExecutor::new(origin_provider, destination_provider, wallet_address);

    let cc_swap = CrossChainSwap {
        swap1: Swap {
            chain: origin_chain_name,
            exchange_name: ExchangeName::UniswapV3,
            token_in: origin_token_in,
            token_out: origin_bridge_token,
            amount_in,
        },
        bridge: Bridge {
            origin_chain: origin_chain_name,
            origin_token: origin_bridge_token,
            destination_chain: destination_chain_name,
            destination_token: destination_bridge_token,
            bridge_name: BridgeName::StargateV2,
        },
        swap2: Swap {
            chain: destination_chain_name,
            exchange_name: ExchangeName::UniswapV3,
            token_in: destination_bridge_token,
            token_out: destination_token_out,
            amount_in: U256::from(0),
        },
    };

    swap_executor.execute(cc_swap).await?;

    Ok(())
}

mod cmd_test {
    use crate::cmd::cross_chain_swap_command;
    use alloy::network::EthereumWallet;
    use alloy::primitives::{Address, U256};
    use alloy::signers::local::PrivateKeySigner;
    use alloy_chains::{Chain, NamedChain};
    use shared::addressbook::Addressbook;
    use shared::provider::get_provider;
    use std::ptr::eq;
    use std::str::FromStr;
    use std::sync::Arc;

    const ORIGIN_CHAIN: NamedChain = NamedChain::Arbitrum;
    const DESTINATION_CHAIN: NamedChain = NamedChain::Base;

    #[tokio::test]
    async fn test_cross_chain_swap_command() {
        // Set up environment variable for private key
        dotenv::dotenv().ok();

        let addressbook = Addressbook::load(None).unwrap();
        let origin_usdc = addressbook.get_token(&ORIGIN_CHAIN, "usdc").unwrap();
        let destination_usdt = addressbook.get_token(&DESTINATION_CHAIN, "usdt").unwrap();

        cross_chain_swap_command(
            "arbitrum",
            "base",
            origin_usdc.to_string().as_str(),
            "weth",
            destination_usdt.to_string().as_str(),
            "1000000",
        )
        .await
        .unwrap();
    }
}
