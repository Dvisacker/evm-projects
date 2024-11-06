use std::sync::Arc;

use crate::types::Executor;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::signers::local::PrivateKeySigner;
use alloy_chains::NamedChain;
use async_trait::async_trait;
use eyre::{Context, Result};
use shared::provider::ProviderMap;

/// An executor that sends transactions to the mempool.
pub struct SequenceExecutor {
    providers: Arc<ProviderMap>,
    wallet_address: Address,
}

/// Information about the gas bid for a transaction.

#[derive(Debug, Clone)]
pub struct SwapBlock {
    pub chain: NamedChain,
    pub exchange_name: types::exchange::ExchangeName,
    pub token_out: Address,
}

#[derive(Debug, Clone)]
pub struct BridgeBlock {
    pub destination_chain: NamedChain,
    pub destination_token: Address,
    pub bridge_name: types::bridge::BridgeName,
}

pub enum TxBlock {
    Swap(SwapBlock),
    Bridge(BridgeBlock),
}

pub struct TxSequence {
    origin_chain: NamedChain,
    amount_in: U256,
    token_in: Address,
    txs: Vec<TxBlock>,
}

impl TxSequence {
    pub fn new(origin_chain: NamedChain, amount_in: U256, token_in: Address) -> Self {
        Self {
            origin_chain,
            amount_in,
            token_in,
            txs: vec![],
        }
    }

    pub fn set_sequence(&mut self, sequence: Vec<TxBlock>) {
        self.txs = sequence;
    }

    pub fn add_swap(&mut self, swap: SwapBlock) {
        self.txs.push(TxBlock::Swap(swap));
    }

    pub fn add_bridge(&mut self, bridge: BridgeBlock) {
        self.txs.push(TxBlock::Bridge(bridge));
    }
}

impl SequenceExecutor {
    pub fn new(providers: Arc<ProviderMap>, wallet_address: Address) -> Self {
        Self {
            providers,
            wallet_address,
        }
    }
}

#[async_trait]
impl Executor<TxSequence> for SequenceExecutor {
    async fn execute(&self, sequence: TxSequence) -> Result<()> {
        let mut current_chain = sequence.origin_chain;
        let mut amount_in = sequence.amount_in;
        let mut token_in = sequence.token_in;
        for block in sequence.txs {
            match block {
                TxBlock::Swap(swap) => {
                    amount_in = shared::swap::swap(
                        self.providers[&swap.chain].clone(),
                        current_chain,
                        swap.exchange_name,
                        self.wallet_address,
                        token_in,
                        swap.token_out,
                        amount_in,
                    )
                    .await
                    .context("Error making swap")?;

                    current_chain = swap.chain;
                    token_in = swap.token_out;
                }
                TxBlock::Bridge(bridge) => {
                    amount_in = shared::bridge::bridge_lifi(
                        self.providers[&current_chain].clone(),
                        self.providers[&bridge.destination_chain].clone(),
                        &current_chain,
                        &bridge.destination_chain,
                        token_in,
                        bridge.destination_token,
                        amount_in,
                        self.wallet_address,
                        self.wallet_address,
                        bridge.bridge_name,
                    )
                    .await
                    .context("Error bridging tokens")?;

                    current_chain = bridge.destination_chain;
                    token_in = bridge.destination_token;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        network::{EthereumWallet, NetworkWallet},
        primitives::utils::parse_units,
        providers::WalletProvider,
        signers::local::PrivateKeySigner,
    };
    use shared::{
        addressbook::Addressbook,
        provider::{get_default_wallet, get_provider_map},
        token_manager::TokenManager,
    };
    use shared::{helpers::parse_token_units, provider::SignerProvider};
    use types::{
        bridge::BridgeName,
        exchange::ExchangeName,
        token::{NamedToken, TokenIsh},
    };

    #[tokio::test]
    async fn test_sequence_executor() {
        dotenv::dotenv().ok();
        let token_manager = TokenManager::instance().await;
        let providers = get_provider_map().await;
        let default_wallet: EthereumWallet = get_default_wallet();
        let default_wallet_address = default_wallet.default_signer().address();
        let executor = SequenceExecutor::new(providers, default_wallet_address);

        let usdc_arbitrum = token_manager
            .get(&NamedChain::Arbitrum, &TokenIsh::Named(NamedToken::USDC))
            .unwrap();
        let weth_arbitrum = token_manager
            .get(&NamedChain::Arbitrum, &TokenIsh::Named(NamedToken::WETH))
            .unwrap();
        let weth_base = token_manager
            .get(&NamedChain::Base, &TokenIsh::Named(NamedToken::WETH))
            .unwrap();
        let usdt_base = token_manager
            .get(&NamedChain::Base, &TokenIsh::Named(NamedToken::USDT))
            .unwrap();

        let amount_usdc = parse_token_units(
            &NamedChain::Arbitrum,
            &TokenIsh::Address(*usdc_arbitrum.address()),
            "1",
        )
        .await
        .unwrap();

        let mut txsequence =
            TxSequence::new(NamedChain::Arbitrum, amount_usdc, *usdc_arbitrum.address());

        txsequence.set_sequence(vec![
            TxBlock::Swap(SwapBlock {
                chain: NamedChain::Arbitrum,
                exchange_name: ExchangeName::UniswapV3,
                token_out: *weth_arbitrum.address(),
            }),
            TxBlock::Bridge(BridgeBlock {
                destination_chain: NamedChain::Base,
                destination_token: *weth_base.address(),
                bridge_name: BridgeName::StargateV2,
            }),
            TxBlock::Swap(SwapBlock {
                chain: NamedChain::Base,
                exchange_name: ExchangeName::UniswapV3,
                token_out: *usdt_base.address(),
            }),
        ]);

        executor.execute(txsequence).await.unwrap();
    }
}
