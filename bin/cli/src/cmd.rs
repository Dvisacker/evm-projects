use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::{network::EthereumWallet, signers::local::PrivateKeySigner};
use alloy_chains::{Chain, NamedChain};
use alloy_primitives::U256;
use artemis_core::executors::sequence_executor::{
    BridgeBlock, SequenceExecutor, SwapBlock, TxBlock, TxSequence,
};
use artemis_core::types::Executor;
use eyre::{Error, Result};
use shared::addressbook::Addressbook;
use shared::amm_utils::{store_uniswap_v2_pools, store_uniswap_v3_pools};
use shared::provider::{get_default_signer, get_default_wallet, get_provider_map};
use shared::token_manager::TokenManager;
use shared::{
    bridge::bridge_lifi, config::get_chain_config, helpers::get_contract_creation_block,
    provider::get_provider,
};
use std::{str::FromStr, sync::Arc};
use tracing::info;
use types::bridge::BridgeName;
use types::exchange::ExchangeName;
use types::token::TokenIsh;

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

pub async fn bridge_command(
    from_chain_name: &NamedChain,
    to_chain_name: &NamedChain,
    token: &TokenIsh,
    amount: &str,
) -> Result<(), Error> {
    let signer = get_default_signer();
    let wallet_address = signer.address();
    let wallet = EthereumWallet::new(signer);
    let origin_chain = Chain::from_named(*from_chain_name);
    let destination_chain = Chain::from_named(*to_chain_name);

    let origin_provider = get_provider(origin_chain, wallet.clone()).await;
    let destination_provider = get_provider(destination_chain, wallet.clone()).await;
    let token_manager = TokenManager::instance().await;

    // Convert TokenIsh to &TokenIsh for the token_manager.get() calls
    let token_ref = &token;
    let from_token = token_manager.get(&from_chain_name, token_ref).unwrap();
    let to_token = token_manager.get(&to_chain_name, token_ref).unwrap();

    // Parse amount
    let amount = U256::from_str(amount).map_err(|_| eyre::eyre!("Invalid amount"))?;

    println!(
        "Starting bridge from {} to {}",
        from_chain_name, to_chain_name
    );
    println!("Amount: {}", amount);

    let result = bridge_lifi(
        origin_provider,
        destination_provider,
        from_chain_name,
        to_chain_name,
        *from_token.address(),
        *to_token.address(),
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
    origin_chain_name: NamedChain,
    destination_chain_name: NamedChain,
    origin_token_in: TokenIsh,
    bridge_token: TokenIsh, //name of the token to bridge (USDC or WETH)
    destination_token_out: TokenIsh,
    amount_in: &str,
) -> Result<(), Error> {
    let signer = get_default_signer();
    let wallet_address = signer.address();
    let token_manager = TokenManager::instance().await;

    let origin_token_in = token_manager
        .get(&origin_chain_name, &origin_token_in)
        .unwrap();
    let destination_token_out = token_manager
        .get(&destination_chain_name, &destination_token_out)
        .unwrap();
    let amount_in = U256::from_str(amount_in).unwrap();
    let origin_bridge_token = token_manager
        .get(&origin_chain_name, &bridge_token)
        .unwrap();
    let destination_bridge_token = token_manager
        .get(&destination_chain_name, &bridge_token)
        .unwrap();
    let providers = get_provider_map().await;
    let default_wallet: EthereumWallet = get_default_wallet();
    let default_wallet_address = default_wallet.default_signer().address();

    let swap_executor = SequenceExecutor::new(providers, default_wallet_address);

    let mut seq = TxSequence::new(origin_chain_name, amount_in, *origin_token_in.address());

    seq.set_sequence(vec![
        TxBlock::Swap(SwapBlock {
            chain: NamedChain::Arbitrum,
            exchange_name: ExchangeName::UniswapV3,
            token_out: *origin_bridge_token.address(),
        }),
        TxBlock::Bridge(BridgeBlock {
            destination_chain: destination_chain_name,
            destination_token: *destination_bridge_token.address(),
            bridge_name: BridgeName::StargateV2,
        }),
        TxBlock::Swap(SwapBlock {
            chain: destination_chain_name,
            exchange_name: ExchangeName::UniswapV3,
            token_out: *destination_token_out.address(),
        }),
    ]);
    swap_executor.execute(seq).await?;

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
    use types::token::{NamedToken, TokenIsh};

    const ORIGIN_CHAIN: NamedChain = NamedChain::Arbitrum;
    const DESTINATION_CHAIN: NamedChain = NamedChain::Base;

    #[tokio::test]
    async fn test_cross_chain_swap_command() {
        dotenv::dotenv().ok();

        cross_chain_swap_command(
            ORIGIN_CHAIN,
            DESTINATION_CHAIN,
            TokenIsh::Named(NamedToken::USDC),
            TokenIsh::Named(NamedToken::WETH),
            TokenIsh::Named(NamedToken::USDT),
            "1000000",
        )
        .await
        .unwrap();
    }
}
