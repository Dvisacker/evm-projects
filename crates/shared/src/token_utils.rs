use alloy::{
    dyn_abi::DynSolType, network::Network, primitives::Address, providers::Provider, sol,
    transports::Transport,
};
use amms::errors::AMMError;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::sync::Arc;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IGetERC20TokenDataBatchRequest,
    "src/abis/GetERC20TokenDataBatchRequest.json"
}

pub struct ERC20TokenData {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
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
    let deployer = IGetERC20TokenDataBatchRequest::deploy_builder(provider, token_addresses);
    let res = deployer.call().await?;

    let constructor_return = DynSolType::Array(Box::new(DynSolType::Tuple(vec![
        DynSolType::Address,
        DynSolType::String,
        DynSolType::Uint(8),
        DynSolType::Uint(256),
    ])));

    let return_data_tokens = constructor_return.abi_decode_sequence(&res)?;
    let mut token_data_vec = Vec::new();

    if let Some(tokens_arr) = return_data_tokens.as_array() {
        for token in tokens_arr {
            if let Some(token_data) = token.as_tuple() {
                if token_data.len() == 4 {
                    let address = token_data[0].as_address().unwrap_or_default();
                    let symbol = token_data[1].as_str().unwrap_or_default().to_string();
                    let decimals = token_data[2].as_uint().unwrap_or_default().0.to::<u8>();
                    let total_supply = token_data[3].as_uint().unwrap_or_default().0.to::<u128>();

                    token_data_vec.push(ERC20TokenData {
                        address,
                        symbol,
                        decimals,
                        total_supply,
                    });
                }
            }
        }
    }

    Ok(token_data_vec)
}

pub async fn load_pools_and_fetch_token_data<T, N, P>(provider: Arc<P>) -> Result<()>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    // Load the filtered pools JSON file
    println!("Loading filtered pools...");
    let filtered_pools_json: String = fs::read_to_string("checkpoints/filtered-pools.json")?;
    println!("Deserializing pools...");
    let mut filtered_pools: Value = serde_json::from_str(&filtered_pools_json)?;
    // Extract unique token addresses
    let mut token_addresses = HashSet::new();
    println!("Extracting token addresses...");
    if let Some(amms) = filtered_pools["amms"].as_array() {
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
    println!("Converting token addresses to Address type...");
    // Convert token addresses to Address type
    let token_addresses: Vec<Address> = token_addresses
        .into_iter()
        .filter_map(|addr| addr.parse().ok())
        .collect();

    // Fetch token data
    println!("Fetching token data...");
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

    // println!("token_map type: {:?}", type_of(&token_map));
    // println!("token_map: {:?}", type token_map);
    println!("token_map keys: {:?}", token_map.keys().collect::<Vec<_>>());
    let keys = token_map.keys().collect::<Vec<_>>();
    println!("token_map.get(key0): {:?}", token_map.get(keys[0]));
    println!(
        "token_map.get(string): {:?}",
        token_map.get("0x82af49447d8a07e3bd95bd0d56f35241523fbab1")
    );

    println!("Adding token data to the filtered pools JSON...");
    // Add token data to the filtered pools JSON
    if let Some(amms) = filtered_pools["amms"].as_array_mut() {
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

    // println!("filtered_pools: {:?}", filtered_pools);

    // Save updated filtered pools data to filtered-pools-named.json
    let filtered_pools_named_json = serde_json::to_string_pretty(&filtered_pools)?;
    fs::write(
        "checkpoints/filtered-pools-named.json",
        filtered_pools_named_json,
    )?;

    println!("Updated filtered pools data saved to filtered-pools-named.json");

    Ok(())
}
