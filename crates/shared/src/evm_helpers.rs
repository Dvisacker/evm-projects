use addressbook::Addressbook;
use alloy::{
    network::Network,
    providers::{ext::DebugApi, Provider},
    sol,
    transports::TransportResult,
};
use alloy_chains::NamedChain;
use alloy_primitives::{aliases::U24, keccak256, Address, Bytes};
use alloy_rpc_types::{
    trace::geth::{GethDebugTracingCallOptions, GethDebugTracingOptions, GethTrace},
    BlockId, BlockNumberOrTag, TransactionRequest,
};
use alloy_sol_types::SolValue;
use eyre::{eyre, Result};
use futures::future::join_all;
use std::sync::Arc;
use types::exchange::ExchangeName;

pub async fn get_trace_call<P, N>(
    provider: Arc<P>,
    tx_request: TransactionRequest,
) -> TransportResult<GethTrace>
where
    P: Provider<N>,
    N: Network,
{
    let s = "{\"tracer\": \"callTracer\"}";
    let opts = serde_json::from_str::<GethDebugTracingOptions>(s).unwrap();
    let tracing_call_options = GethDebugTracingCallOptions::new(opts);

    provider
        .debug_trace_call(
            tx_request,
            BlockNumberOrTag::Latest.into(),
            tracing_call_options,
        )
        .await
}

pub async fn get_contract_creation_block_binary<P, N>(
    provider: Arc<P>,
    contract_address: Address,
    start_block: u64,
    end_block: u64,
) -> Result<u64>
where
    P: Provider<N>,
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

// N-ary search for the contract creation block
pub async fn get_contract_creation_block_n_ary<P, N>(
    provider: Arc<P>,
    contract_address: Address,
    start_block: u64,
    end_block: u64,
    n: u64,
) -> Result<u64>
where
    P: Provider<N>,
    N: Network,
{
    if n < 2 {
        return Err(eyre!("n must be at least 2"));
    }

    let mut low = start_block;
    let mut high = end_block;

    while low <= high {
        // Generate n points including low and high
        let mut points = Vec::with_capacity(n as usize);
        points.push(low);
        let range = high - low;
        for i in 1..n - 1 {
            points.push(low + (range * i) / (n - 1));
        }
        points.push(high);

        // Create futures for all point checks in parallel
        let point_futures: Vec<_> = points
            .iter()
            .map(|&point| get_code_at_block(provider.clone(), contract_address, point))
            .collect();

        // Execute all point checks in parallel
        let point_results = join_all(point_futures).await;
        let codes: Result<Vec<_>> = point_results.into_iter().collect();
        let codes = codes?;

        // Find the first point where code exists
        let mut transition_idx = None;
        for (i, code) in codes.iter().enumerate() {
            if !code.is_empty() {
                transition_idx = Some(i);
                break;
            }
        }

        match transition_idx {
            None => {
                // No code found in any point, search in next range
                low = high + 1;
            }
            Some(0) => {
                // First point has code, check if it's the creation block
                if points[0] == start_block {
                    return Ok(points[0]);
                }
                // Check if previous block is empty
                let prev_code =
                    get_code_at_block(provider.clone(), contract_address, points[0] - 1).await?;
                if prev_code.is_empty() {
                    return Ok(points[0]);
                }
                // Code exists before our range, search earlier
                high = points[0] - 1;
            }
            Some(i) => {
                // Found a transition point, narrow search to this range
                let prev_point = points[i - 1];
                let curr_point = points[i];
                if curr_point - prev_point <= 1 {
                    return Ok(curr_point);
                }
                low = prev_point + 1;
                high = curr_point;
            }
        }
    }

    Err(eyre!("Contract creation block not found"))
}

async fn get_code_at_block<P, N>(provider: Arc<P>, address: Address, block: u64) -> Result<Bytes>
where
    P: Provider<N>,
    N: Network,
{
    tracing::info!("Getting code at block: {}", block);
    let block_number = BlockNumberOrTag::Number(block.into());
    let block_id = BlockId::Number(block_number);
    let result = provider.get_code_at(address).block_id(block_id).await?;
    Ok(result)
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
    let addressbook = Addressbook::load().unwrap();
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

    let init_code_v2_hash = "96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f";
    let encode_packed = [token0.abi_encode_packed(), token1.abi_encode_packed()].concat();
    let salt = keccak256(encode_packed);

    Ok(get_create2_address(
        factory_address,
        salt,
        init_code_v2_hash,
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
    let addressbook = Addressbook::load().unwrap();
    let factory_address = addressbook
        .get_factory(chain, exchange_name)
        .ok_or_else(|| eyre!("Factory address not found"))?;

    let (token0, token1) =
        if token_a.to_string().to_lowercase() < token_b.to_string().to_lowercase() {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        };

    let init_code_v3_hash = "e34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54";
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
        init_code_v3_hash,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_chains::Chain;
    use provider::get_basic_provider_arc;

    #[tokio::test]
    async fn test_get_contract_creation_block() {
        let provider = get_basic_provider_arc(Chain::from_id(1)).await;
        // USDC contract address on Ethereum mainnet
        let contract_address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
            .parse()
            .unwrap();

        let result =
            get_contract_creation_block_binary(provider, contract_address, 0, 21_000_000).await;

        assert!(result.is_ok());
        let creation_block = result.unwrap();
        println!("USDC contract creation block: {}", creation_block);
        assert!(creation_block >= 6_000_000 && creation_block <= 7_000_000);
    }

    #[tokio::test]
    async fn test_get_code_at_block() {
        let provider = get_basic_provider_arc(Chain::from_id(1)).await;
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
        let provider = get_basic_provider_arc(Chain::from_id(1)).await;
        // Use a random address that's unlikely to be a contract
        let contract_address = Address::random();

        let result =
            get_contract_creation_block_binary(provider, contract_address, 14_000_000, 14_001_000)
                .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Contract creation block not found"
        );
    }

    #[tokio::test]
    async fn test_get_contract_creation_block_2() {
        let provider = get_basic_provider_arc(Chain::from_id(1)).await;
        // USDC contract address on Ethereum mainnet
        let contract_address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
            .parse()
            .unwrap();

        let result =
            get_contract_creation_block_n_ary(provider, contract_address, 0, 21_000_000, 4).await;

        assert!(result.is_ok());
        let creation_block = result.unwrap();
        println!("USDC contract creation block (n-ary): {}", creation_block);
        assert_eq!(creation_block, 6_082_465);
    }

    #[tokio::test]
    async fn test_get_contract_creation_block_n_ary_invalid_n() {
        let provider = get_basic_provider_arc(Chain::from_id(1)).await;
        let contract_address = Address::random();

        let result = get_contract_creation_block_n_ary(
            provider,
            contract_address,
            14_000_000,
            14_001_000,
            1,
        )
        .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "n must be at least 2");
    }
}
