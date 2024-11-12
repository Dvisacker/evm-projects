use alloy::{
    network::Network,
    providers::{Provider, WalletProvider},
    sol,
    transports::Transport,
};
use alloy_chains::NamedChain;
use alloy_primitives::{aliases::U24, keccak256, utils::parse_units, Address, Bytes, U256};
use alloy_rpc_types::{BlockId, BlockNumberOrTag};
use alloy_sol_types::SolValue;
use bindings::ierc20::IERC20;
use eyre::{eyre, Error, Result};
use std::sync::Arc;
use types::{exchange::ExchangeName, token::TokenIsh};

use crate::{addressbook::Addressbook, provider::SignerProvider, token_manager::TokenManager};

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

pub fn get_create2_address(
    from: Address,
    salt: impl AsRef<[u8]>,
    init_code: impl AsRef<[u8]>,
) -> Address {
    // Convert the inputs to byte slices
    let from_bytes = from.as_slice();
    let salt_bytes = salt.as_ref();
    let init_code_bytes = init_code.as_ref();

    // Ensure salt is 32 bytes, pad with zeros if needed
    let mut padded_salt = [0u8; 32];
    let salt_len = salt_bytes.len().min(32);
    padded_salt[..salt_len].copy_from_slice(&salt_bytes[..salt_len]);

    // Calculate init code hash if not already provided
    let code_hash = keccak256(init_code_bytes);

    // Pack the data: 0xff ++ from ++ salt ++ keccak256(init_code)
    let mut packed = Vec::with_capacity(1 + 20 + 32 + 32);
    packed.push(0xff);
    packed.extend_from_slice(from_bytes);
    packed.extend_from_slice(&padded_salt);
    packed.extend_from_slice(code_hash.as_slice());

    // Calculate final hash and convert to address
    let hash = keccak256(packed);
    Address::from_slice(&hash[12..])
}

pub fn compute_v2_pool_address(
    chain: &NamedChain,
    exchange_name: ExchangeName,
    token_a: Address,
    token_b: Address,
    a_is_0: Option<bool>,
) -> Result<Address> {
    let addressbook = Addressbook::load(None).unwrap();
    let factory_address = addressbook
        .get_factory(chain, exchange_name)
        .ok_or_else(|| eyre!("Factory address not found"))?;

    let a_is_0 = a_is_0
        .unwrap_or_else(|| token_a.to_string().to_lowercase() < token_b.to_string().to_lowercase());
    let (token0, token1) = if a_is_0 {
        (token_a, token_b)
    } else {
        (token_b, token_a)
    };

    let INIT_CODE_V2_HASH = "96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f";
    let encode_packed = [token0.abi_encode_packed(), token1.abi_encode_packed()].concat();
    let salt = keccak256(encode_packed);

    Ok(get_create2_address(
        factory_address,
        salt,
        INIT_CODE_V2_HASH,
    ))
}

sol! {
    #[allow(missing_docs)]
    #[derive(Debug, PartialEq, Eq)]
    struct PoolParameters {
        address token0;
        address token1;
        uint24 fee;
    }
}

pub fn compute_v3_pool_address(
    chain: &NamedChain,
    exchange_name: ExchangeName,
    token_a: Address,
    token_b: Address,
    fee: u16,
) -> Result<Address> {
    let addressbook = Addressbook::load(None).unwrap();
    let factory_address = addressbook
        .get_factory(chain, exchange_name)
        .ok_or_else(|| eyre!("Factory address not found"))?;

    let (token0, token1) =
        if token_a.to_string().to_lowercase() < token_b.to_string().to_lowercase() {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        };

    let INIT_CODE_V3_HASH = "e34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54";
    let encoded = PoolParameters {
        token0,
        token1,
        fee: U24::from(fee),
    }
    .abi_encode();
    let salt = keccak256(encoded);

    Ok(get_create2_address(
        factory_address,
        salt,
        INIT_CODE_V3_HASH,
    ))
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
