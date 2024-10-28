use std::{
    ops::{Div, Mul},
    sync::Arc,
};

use crate::types::Executor;
use alloy::{primitives::U256, providers::Provider, rpc::types::TransactionRequest};
use anyhow::{Context, Result};
use async_trait::async_trait;

/// An executor that sends transactions to the mempool.
pub struct MempoolExecutor<M> {
    client: Arc<M>,
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
pub struct SubmitTxToMempool {
    pub tx: TransactionRequest,
    pub gas_bid_info: Option<GasBidInfo>,
}

impl<M: Provider> MempoolExecutor<M> {
    pub fn new(client: Arc<M>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<M> Executor<SubmitTxToMempool> for MempoolExecutor<M>
where
    M: Provider,
{
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempool) -> Result<()> {
        let gas_usage = self
            .client
            .estimate_gas(&action.tx)
            .await
            .context("Error estimating gas usage: {}")?;

        let bid_gas_price: u64;
        if let Some(gas_bid_info) = action.gas_bid_info {
            // gas price at which we'd break even, meaning 100% of profit goes to validator
            let breakeven_gas_price: u64 = gas_bid_info.total_profit.to::<u64>() / gas_usage;
            // gas price corresponding to bid percentage
            bid_gas_price = breakeven_gas_price
                .mul(u64::from(gas_bid_info.bid_percentage))
                .div(100);
        } else {
            bid_gas_price = self
                .client
                .get_gas_price()
                .await
                .context("Error getting gas price: {}")?
                .try_into()
                .context("Error converting gas price to u64: {}")?;
        }

        action.tx.gas_price = Some(bid_gas_price.into());
        let receipt = self
            .client
            .send_transaction(action.tx)
            .await?
            .get_receipt()
            .await?;

        println!("Transaction receipt: {:?}", receipt);
        Ok(())
    }
}
