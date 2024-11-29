use crate::token_manager::TokenManager;
use alloy::network::Ethereum;
use alloy::providers::WalletProvider;
use alloy::{
    dyn_abi::DynSolType, network::Network, primitives::Address, providers::Provider, sol,
    transports::Transport,
};
use alloy_chains::NamedChain;
use alloy_primitives::utils::parse_units;
use alloy_primitives::U256;
use amms::errors::AMMError;
use bindings::geterc20tokendatabatchrequest::GetERC20TokenDataBatchRequest;
use bindings::ierc20::IERC20;
use eyre::Context;
use eyre::{eyre, Error, Result};
use provider::SignerProvider;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::sync::Arc;
use types::token::TokenIsh;

pub struct ERC20TokenData {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
}

pub async fn get_token_balance(
    provider: Arc<SignerProvider>,
    token: Address,
    holder: Address,
) -> Result<U256> {
    let token = IERC20::new(token, provider.clone());
    let balance = token.balanceOf(holder).call().await?;
    Ok(balance._0)
}

pub async fn get_token_allowance(
    provider: Arc<SignerProvider>,
    token: Address,
    holder: Address,
    spender: Address,
) -> Result<U256> {
    let token = IERC20::new(token, provider.clone());
    let allowance = token.allowance(holder, spender).call().await?;
    Ok(allowance._0)
}

pub async fn approve_token_if_needed(
    provider: Arc<SignerProvider>,
    token: Address,
    spender: Address,
    amount: U256,
) -> Result<(), Error> {
    let token = IERC20::new(token, provider.clone());
    let wallet_address = provider.default_signer_address();

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

pub async fn transfer_token_if_needed(
    provider: Arc<SignerProvider>,
    token: Address,
    to: Address,
    amount: U256,
) -> Result<(), Error> {
    let token = IERC20::new(token, provider.clone());
    let wallet_address = provider.default_signer_address();
    let balance = token.balanceOf(wallet_address).call().await?;
    if balance._0 < amount {
        token.transfer(to, amount).send().await?;
    }
    Ok(())
}

pub async fn transfer_approve_token_if_needed(
    provider: Arc<SignerProvider>,
    token: Address,
    to: Address,
    amount: U256,
) -> Result<(), Error> {
    transfer_token_if_needed(provider.clone(), token, to, amount).await?;
    approve_token_if_needed(provider.clone(), token, to, amount).await?;
    Ok(())
}

pub async fn parse_token_units(chain: &NamedChain, token: &TokenIsh, amount: &str) -> Result<U256> {
    let token_manager = TokenManager::instance().await;
    let token = token_manager.get(chain, token).unwrap();
    let decimals = match token.decimals().call().await {
        Ok(decimals) => decimals._0,
        Err(e) => return Err(eyre!("Failed to get decimals: {}", e)),
    };
    let amount = parse_units(amount, decimals).unwrap().into();
    Ok(amount)
}

pub async fn get_erc20_data_batch_request<T, N, P>(
    token_addresses: Vec<Address>,
    provider: Arc<P>,
) -> Result<Vec<ERC20TokenData>, AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    const BATCH_SIZE: usize = 50;
    let mut all_token_data = Vec::new();

    for chunk in token_addresses.chunks(BATCH_SIZE) {
        let deployer =
            GetERC20TokenDataBatchRequest::deploy_builder(provider.clone(), chunk.to_vec());
        let res = deployer.call().await?;

        let constructor_return = DynSolType::Array(Box::new(DynSolType::Tuple(vec![
            DynSolType::Address,
            DynSolType::String,
            DynSolType::Uint(8),
            DynSolType::Uint(256),
        ])));

        let return_data_tokens = constructor_return.abi_decode_sequence(&res)?;

        if let Some(tokens_arr) = return_data_tokens.as_array() {
            for token in tokens_arr {
                if let Some(token_data) = token.as_tuple() {
                    if token_data.len() == 4 {
                        let address = token_data[0].as_address().unwrap_or_default();
                        let symbol = token_data[1].as_str().unwrap_or_default().to_string();
                        let decimals = token_data[2].as_uint().unwrap_or_default().0.to::<u8>();
                        let total_supply =
                            token_data[3].as_uint().unwrap_or_default().0.to::<u128>();

                        all_token_data.push(ERC20TokenData {
                            address,
                            symbol,
                            decimals,
                            total_supply,
                        });
                    }
                }
            }
        }
    }

    Ok(all_token_data)
}

pub async fn load_pools_and_fetch_token_data<T, N, P>(provider: Arc<P>) -> Result<()>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let active_pools_json: String = fs::read_to_string("checkpoints/active-pools.json")?;
    let mut active_pools: Value = serde_json::from_str(&active_pools_json)?;
    let mut token_addresses = HashSet::new();
    if let Some(amms) = active_pools["amms"].as_array() {
        for amm in amms {
            if let Some(pool) = amm["UniswapV2Pool"].as_object() {
                if let (Some(token_a), Some(token_b)) =
                    (pool["token_a"].as_str(), pool["token_b"].as_str())
                {
                    token_addresses.insert(token_a.to_string());
                    token_addresses.insert(token_b.to_string());
                }
            }
        }
    }
    // Convert token addresses to Address type
    let token_addresses: Vec<Address> = token_addresses
        .into_iter()
        .filter_map(|addr| addr.parse().ok())
        .collect();

    let token_data = get_erc20_data_batch_request(token_addresses, provider).await?;

    // Create a map of token addresses to token data
    println!("Creating token map...");
    let token_map: serde_json::Map<String, Value> = token_data
        .into_iter()
        .map(|data| {
            (
                data.address.to_string().to_lowercase(),
                serde_json::json!({
                    "address": data.address.to_string().to_lowercase(),
                    "symbol": data.symbol,
                    "decimals": data.decimals,
                    "total_supply": data.total_supply.to_string()
                }),
            )
        })
        .collect();

    println!("Adding token data to the active pools JSON...");
    // Add token data to the active pools JSON
    if let Some(amms) = active_pools["amms"].as_array_mut() {
        for amm in amms {
            if let Some(pool) = amm["UniswapV2Pool"].as_object_mut() {
                if let (Some(token_a), Some(token_b)) =
                    (pool["token_a"].as_str(), pool["token_b"].as_str())
                {
                    let mut pool_data = serde_json::Map::new();
                    if let Some(token_data) = token_map.get(token_a) {
                        for field in ["symbol", "decimals", "total_supply"] {
                            let key = format!("{}_{}", "token_a", field);
                            pool_data.insert(key, token_data[field].clone());
                        }
                    }

                    if let Some(token_data) = token_map.get(token_b) {
                        for field in ["symbol", "decimals", "total_supply"] {
                            let key = format!("{}_{}", "token_b", field);
                            pool_data.insert(key, token_data[field].clone());
                        }
                    }

                    pool.extend(pool_data);
                }
            }
        }
    }

    // Save updated active pools data to active-pools-named.json
    let active_pools_named_json = serde_json::to_string_pretty(&active_pools)?;
    fs::write(
        "checkpoints/active-pools-named.json",
        active_pools_named_json,
    )?;

    Ok(())
}

pub async fn verify_erc20_interface(
    provider: Arc<SignerProvider>,
    token_address: Address,
) -> Result<(), Error> {
    let token = IERC20::new(token_address, provider.clone());

    // Test name() - Optional in ERC20 but commonly implemented
    token
        .name()
        .call()
        .await
        .wrap_err("Failed to get token name - name() method may not be implemented")?;

    // Test symbol() - Optional in ERC20 but commonly implemented
    token
        .symbol()
        .call()
        .await
        .wrap_err("Failed to get token symbol - symbol() method may not be implemented")?;

    // Test decimals() - Optional in ERC20 but commonly implemented
    token
        .decimals()
        .call()
        .await
        .wrap_err("Failed to get token decimals - decimals() method may not be implemented")?;

    token
        .totalSupply()
        .call()
        .await
        .wrap_err("Failed to get total supply - totalSupply() method not working")?;

    // Test balanceOf() - Required by ERC20
    // Using address(0) as test address
    token
        .balanceOf(Address::ZERO)
        .call()
        .await
        .wrap_err("Failed to get balance - balanceOf() method not working")?;

    // Test allowance() - Required by ERC20
    // Using address(0) for both owner and spender as test addresses
    token
        .allowance(Address::ZERO, Address::ZERO)
        .call()
        .await
        .wrap_err("Failed to get allowance - allowance() method not working")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_chains::{Chain, NamedChain};
    use provider::get_anvil_signer_provider;

    #[tokio::test]
    // TODO make this test independant of anvil
    async fn test_verify_erc20_interface() {
        let usdc_address = "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913"
            .parse()
            .expect("Invalid address");

        let provider = get_anvil_signer_provider().await;

        let result = verify_erc20_interface(provider.clone(), usdc_address).await;
        println!("{:?}", result);
        assert!(
            result.is_ok(),
            "USDC should implement ERC20 interface correctly"
        );

        // Test with invalid/non-contract address
        let invalid_address = Address::ZERO;
        let result = verify_erc20_interface(provider.clone(), invalid_address).await;
        assert!(result.is_err(), "Should fail with invalid token address");
    }

    #[tokio::test]
    async fn test_verify_erc20_interface_with_shitcoin() {
        let usdc_address = "0xef4fC624EA1a2Acfd806240Ada70d6802a81Eaf3"
            .parse()
            .expect("Invalid address");

        let provider = get_anvil_signer_provider().await;

        let result = verify_erc20_interface(provider.clone(), usdc_address).await;
        println!("{:?}", result);
        assert!(
            result.is_ok(),
            "USDC should implement ERC20 interface correctly"
        );

        // Test with invalid/non-contract address
        let invalid_address = Address::ZERO;
        let result = verify_erc20_interface(provider.clone(), invalid_address).await;
        assert!(result.is_err(), "Should fail with invalid token address");
    }
}
