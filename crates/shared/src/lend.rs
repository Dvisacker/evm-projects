use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, WalletProvider};
use alloy::sol;
use alloy_chains::NamedChain;
use eyre::{Context, Result};
use std::sync::Arc;

use crate::token_helpers::approve_token_if_needed;
use addressbook::Addressbook;

sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IAaveV3Pool {
        function supply(address asset, uint256 amount, address onBehalfOf, uint16 referralCode);
        function withdraw(address asset, uint256 amount, address to);
    }
}

pub async fn aave_v3_supply<P: Provider + WalletProvider>(
    provider: Arc<P>,
    chain: NamedChain,
    token_address: Address,
    amount: U256,
) -> Result<()> {
    let addressbook = Addressbook::load().unwrap();
    let pool_address = addressbook
        .get_lending_pool(&chain, "aave_v3")
        .context("Failed to get Aave V3 pool address")?;

    let pool = IAaveV3Pool::new(pool_address, provider.clone());

    approve_token_if_needed(provider.clone(), token_address, pool_address, amount).await?;

    let wallet_address = provider.default_signer_address();
    let pending_tx = pool
        .supply(token_address, amount, wallet_address, 0u16)
        .send()
        .await?;

    let receipt = pending_tx.get_receipt().await?;
    println!("Aave supply receipt: {:?}", receipt);

    Ok(())
}

pub async fn aave_v3_withdraw<P: Provider + WalletProvider>(
    provider: Arc<P>,
    chain: NamedChain,
    token: Address,
    amount: U256,
) -> Result<()> {
    let addressbook = Addressbook::load().unwrap();
    let pool_address = addressbook
        .get_lending_pool(&chain, "aave_v3")
        .context("Failed to get Aave V3 pool address")?;

    // Create contract instance
    let pool = IAaveV3Pool::new(pool_address, provider.clone());

    // Withdraw from Aave
    let wallet_address = provider.default_signer_address();
    let pending_tx = pool.withdraw(token, amount, wallet_address).send().await?;
    let receipt = pending_tx.get_receipt().await?;
    println!("Aave withdraw receipt: {:?}", receipt);

    Ok(())
}
