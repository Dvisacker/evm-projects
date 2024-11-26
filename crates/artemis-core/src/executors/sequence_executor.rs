use std::sync::Arc;

use crate::types::Executor;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::signers::local::PrivateKeySigner;
use alloy_chains::NamedChain;
use async_trait::async_trait;
use eyre::{Context, Result};
use provider::ProviderMap;

/// SequenceExecutor is responsible for executing complex sequences of transactions
/// across multiple chains and protocols. It can handle:
/// - Cross-chain token transfers (bridges)
/// - DEX swaps on various chains
/// - Multi-step MEV opportunities
pub struct SequenceExecutor {
    /// Map of providers for different chains
    providers: Arc<ProviderMap>,
    /// Address of the wallet executing transactions
    wallet_address: Address,
}

/// Represents a token swap operation on a specific chain and DEX.
/// This is used as part of a transaction sequence to specify
/// token swaps within a single chain.
#[derive(Debug, Clone)]
pub struct SwapBlock {
    /// The chain where the swap will occur
    pub chain: NamedChain,
    /// The DEX/AMM to use for the swap
    pub exchange_name: types::exchange::ExchangeName,
    /// The address of the token to receive from the swap
    pub token_out: Address,
}

/// Represents a cross-chain bridge operation.
/// This is used as part of a transaction sequence to move
/// tokens between different chains.
#[derive(Debug, Clone)]
pub struct BridgeBlock {
    /// The chain to bridge tokens to
    pub destination_chain: NamedChain,
    /// The token address on the destination chain
    pub destination_token: Address,
    /// The bridge protocol to use
    pub bridge_name: types::bridge::BridgeName,
}

/// Represents a single operation in a transaction sequence.
/// Can be either a swap within a chain or a bridge between chains.
#[derive(Debug, Clone)]
pub enum TxBlock {
    /// A token swap operation
    Swap(SwapBlock),
    /// A cross-chain bridge operation
    Bridge(BridgeBlock),
}

/// Represents a complete sequence of transactions to be executed.
/// This can include multiple swaps and bridges across different chains,
/// forming a complete path for a complex MEV opportunity.
#[derive(Debug, Clone)]
pub struct TxSequence {
    origin_chain: NamedChain,
    amount_in: U256,
    token_in: Address,
    txs: Vec<TxBlock>,
}

impl TxSequence {
    /// Creates a new transaction sequence.
    ///
    /// # Arguments
    /// * `origin_chain` - The chain where the sequence starts
    /// * `amount_in` - The amount of input tokens
    /// * `token_in` - The address of the input token
    pub fn new(origin_chain: NamedChain, amount_in: U256, token_in: Address) -> Self {
        Self {
            origin_chain,
            amount_in,
            token_in,
            txs: vec![],
        }
    }

    /// Sets the complete sequence of operations.
    /// This replaces any existing operations in the sequence.
    pub fn set_sequence(&mut self, sequence: Vec<TxBlock>) {
        self.txs = sequence;
    }

    /// Adds a swap operation to the end of the sequence.
    pub fn add_swap(&mut self, swap: SwapBlock) {
        self.txs.push(TxBlock::Swap(swap));
    }

    /// Adds a bridge operation to the end of the sequence.
    pub fn add_bridge(&mut self, bridge: BridgeBlock) {
        self.txs.push(TxBlock::Bridge(bridge));
    }
}

impl SequenceExecutor {
    /// Creates a new SequenceExecutor instance.
    ///
    /// # Arguments
    /// * `providers` - Map of providers for different chains
    /// * `wallet_address` - Address of the wallet that will execute transactions
    pub fn new(providers: Arc<ProviderMap>, wallet_address: Address) -> Self {
        Self {
            providers,
            wallet_address,
        }
    }
}

/// Implementation of the [Executor] trait for [SequenceExecutor].
/// This implementation:
/// 1. Executes each operation in the sequence in order
/// 2. Tracks token amounts and addresses across operations
/// 3. Handles errors and transaction confirmations
/// 4. Updates state between operations
#[async_trait]
impl Executor<TxSequence> for SequenceExecutor {
    /// Executes a complete transaction sequence.
    ///
    /// # Arguments
    /// * `sequence` - The sequence of operations to execute
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all operations in the sequence succeeded
    ///
    /// # Errors
    /// Returns an error if:
    /// * Any swap operation fails
    /// * Any bridge operation fails
    /// * Provider communication fails
    /// * Transaction confirmation fails
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
    use addressbook::Addressbook;
    use alloy::{
        network::{EthereumWallet, NetworkWallet},
        primitives::utils::parse_units,
        providers::WalletProvider,
        signers::local::PrivateKeySigner,
    };
    use provider::{get_default_wallet, get_provider_map};
    use shared::token_helpers::parse_token_units;
    use shared::token_manager::TokenManager;
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
