use std::sync::Arc;

use crate::types::Executor;
use alloy::primitives::{Address, U256};
use alloy_chains::NamedChain;
use async_trait::async_trait;
use eyre::{Context, Result};
use shared::provider::SignerProvider;

/// An executor that sends transactions to the mempool.
pub struct CrossChainSwapExecutor {
    origin_chain_provider: Arc<SignerProvider>,
    destination_chain_provider: Arc<SignerProvider>,
    wallet_address: Address,
}

/// Information about the gas bid for a transaction.
#[derive(Debug, Clone)]
pub struct GasBidInfo {
    /// Total profit expected from opportunity
    pub total_profit: U256,
    /// Percentage of bid profit to use for gas
    pub bid_percentage: u64,
}

#[derive(Debug, Clone)]
pub struct CrossChainSwap {
    pub swap1: Swap,
    pub bridge: Bridge,
    pub swap2: Swap,
}

#[derive(Debug, Clone)]
pub struct Swap {
    pub chain: NamedChain,
    pub exchange_name: types::exchange::ExchangeName,
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: U256,
}

#[derive(Debug, Clone)]
pub struct Bridge {
    pub origin_chain: NamedChain,
    pub origin_token: Address,
    pub destination_chain: NamedChain,
    pub destination_token: Address,
    pub bridge_name: types::bridge::BridgeName,
}

impl CrossChainSwapExecutor {
    pub fn new(
        origin_chain_provider: Arc<SignerProvider>,
        destination_chain_provider: Arc<SignerProvider>,
        wallet_address: Address,
    ) -> Self {
        Self {
            origin_chain_provider,
            destination_chain_provider,
            wallet_address,
        }
    }
}

#[async_trait]
impl Executor<CrossChainSwap> for CrossChainSwapExecutor {
    async fn execute(&self, action: CrossChainSwap) -> Result<()> {
        let amount_out = shared::swap::swap(
            self.origin_chain_provider.clone(),
            action.swap1.chain,
            action.swap1.exchange_name,
            self.wallet_address,
            action.swap1.token_in,
            action.swap1.token_out,
            action.swap1.amount_in,
        )
        .await
        .context("Error making swap on origin chain")?;

        let amount_out = shared::bridge::bridge_lifi(
            self.origin_chain_provider.clone(),
            self.destination_chain_provider.clone(),
            action.bridge.origin_chain.into(),
            action.bridge.destination_chain.into(),
            action.bridge.origin_token,
            action.bridge.destination_token,
            amount_out,
            self.wallet_address,
            self.wallet_address,
            action.bridge.bridge_name,
        )
        .await
        .context("Error bridging tokens")?;

        shared::swap::swap(
            self.destination_chain_provider.clone(),
            action.swap2.chain,
            action.swap2.exchange_name,
            self.wallet_address,
            action.swap2.token_in,
            action.swap2.token_out,
            amount_out,
        )
        .await
        .context("Error making swap on destination chain")?;

        Ok(())
    }
}
