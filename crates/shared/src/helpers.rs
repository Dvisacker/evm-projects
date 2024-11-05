use alloy::{
    network::Network,
    providers::{Provider, WalletProvider},
    transports::Transport,
};
use alloy_primitives::{Address, Bytes, U256};
use alloy_rpc_types::{BlockId, BlockNumberOrTag};
use bindings::ierc20::IERC20;
use eyre::{eyre, Error, Result};
use std::sync::Arc;

use crate::provider::SignerProvider;

pub async fn get_contract_creation_block<P, T, N>(
    provider: Arc<P>,
    contract_address: Address,
    start_block: u64,
    end_block: u64,
) -> Result<u64>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    let mut low = start_block;
    let mut high = end_block;

    while low <= high {
        let mid = (low + high) / 2;
        let code = get_code_at_block(provider.clone(), contract_address, mid).await?;

        if code.is_empty() {
            low = mid + 1;
        } else {
            if mid == start_block
                || get_code_at_block(provider.clone(), contract_address, mid - 1)
                    .await?
                    .is_empty()
            {
                return Ok(mid);
            }
            high = mid - 1;
        }
    }

    Err(eyre!("Contract creation block not found"))
}

async fn get_code_at_block<P, T, N>(provider: Arc<P>, address: Address, block: u64) -> Result<Bytes>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    tracing::info!("Getting code at block: {}", block);
    let block_number = BlockNumberOrTag::Number(block.into());
    let block_id = BlockId::Number(block_number);
    let result = provider.get_code_at(address).block_id(block_id).await?;
    Ok(result)
}

pub async fn approve_token_if_needed(
    provider: Arc<SignerProvider>,
    token: Address,
    spender: Address,
    amount: U256,
) -> Result<(), Error> {
    let token = IERC20::new(token, provider.clone());
    let wallet_address = provider.default_signer_address();

    // Check current allowance
    let allowance: U256 = match token.allowance(wallet_address, spender).call().await {
        Ok(allowance) => allowance._0,
        Err(e) => {
            return Err(eyre!("Failed to get allowance: {}", e));
        }
    };

    if allowance < amount {
        token.approve(spender, amount).send().await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        providers::{ProviderBuilder, RootProvider},
        transports::http::{Client, Http},
    };
    use std::env;

    fn setup_provider() -> Arc<RootProvider<Http<Client>>> {
        let rpc_endpoint = env::var("MAINNET_RPC_URL").expect("MAINNET_RPC_URL must be set");

        Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()))
    }

    #[tokio::test]
    async fn test_get_contract_creation_block() {
        let provider = setup_provider();
        // USDC contract address on Ethereum mainnet
        let contract_address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
            .parse()
            .unwrap();

        let result =
            get_contract_creation_block(provider, contract_address, 6_000_000, 7_000_000).await;

        assert!(result.is_ok());
        let creation_block = result.unwrap();
        println!("USDC contract creation block: {}", creation_block);
        assert!(creation_block >= 6_000_000 && creation_block <= 7_000_000);
    }

    #[tokio::test]
    async fn test_get_code_at_block() {
        let provider = setup_provider();
        // USDC contract address on Ethereum mainnet
        let contract_address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
            .parse()
            .unwrap();
        let block_number = 6_082_465; // A block after USDC deployment

        let result = get_code_at_block(provider, contract_address, block_number).await;

        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(!code.is_empty());
        println!(
            "USDC contract code length at block {}: {} bytes",
            block_number,
            code.len()
        );
    }

    #[tokio::test]
    async fn test_get_contract_creation_block_not_found() {
        let provider = setup_provider();
        // Use a random address that's unlikely to be a contract
        let contract_address = Address::random();

        let result =
            get_contract_creation_block(provider, contract_address, 14_000_000, 14_001_000).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Contract creation block not found"
        );
    }
}
