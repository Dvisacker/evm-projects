use std::{collections::HashMap, sync::Arc};

use crate::types::Executor;
use alloy::{
    primitives::{Address, U256},
    signers::k256::elliptic_curve::generic_array::sequence,
};
use alloy_chains::NamedChain;
use async_trait::async_trait;
use eyre::{Context, Result};
use shared::provider::SignerProvider;

/// An executor that sends transactions to the mempool.
pub struct SequenceExecutor {
    providers: HashMap<NamedChain, Arc<SignerProvider>>,
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

enum Tx {
    Swap(SwapBlock),
    Bridge(BridgeBlock),
}

pub struct TxSequence {
    origin_chain: NamedChain,
    amount_in: U256,
    token_in: Address,
    txs: Vec<Tx>,
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

    pub fn add_swap(&mut self, swap: SwapBlock) {
        self.txs.push(Tx::Swap(swap));
    }

    pub fn add_bridge(&mut self, bridge: BridgeBlock) {
        self.txs.push(Tx::Bridge(bridge));
    }
}

impl SequenceExecutor {
    pub fn new(
        providers: HashMap<NamedChain, Arc<SignerProvider>>,
        wallet_address: Address,
    ) -> Self {
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
                Tx::Swap(swap) => {
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
                Tx::Bridge(bridge) => {
                    amount_in = shared::bridge::bridge_lifi(
                        self.providers[&current_chain].clone(),
                        self.providers[&bridge.destination_chain].clone(),
                        current_chain.into(),
                        bridge.destination_chain.into(),
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
