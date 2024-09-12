use alloy::providers::Provider;
use alloy_primitives::{Address, Bytes};
use alloy_rpc_types::{BlockId, BlockNumberOrTag};
use std::sync::Arc;

pub async fn get_contract_creation_block<P: Provider>(
    provider: Arc<P>,
    contract_address: Address,
    start_block: u64,
    end_block: u64,
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut low = start_block;
    let mut high = end_block;

    while low <= high {
        let mid = (low + high) / 2;
        let code = get_code_at_block(&provider, contract_address, mid).await?;

        if code.is_empty() {
            low = mid + 1;
        } else {
            if mid == start_block
                || get_code_at_block(&provider, contract_address, mid - 1)
                    .await?
                    .is_empty()
            {
                return Ok(mid);
            }
            high = mid - 1;
        }
    }

    Err("Contract creation block not found".into())
}

async fn get_code_at_block<P: Provider>(
    provider: &Arc<P>,
    address: Address,
    block: u64,
) -> Result<Bytes, Box<dyn std::error::Error>> {
    tracing::info!("Getting code at block: {}", block);
    let block_number = BlockNumberOrTag::Number(block.into());
    let block_id = BlockId::Number(block_number);
    let result = provider.get_code_at(address).block_id(block_id).await?;
    Ok(result)
}
