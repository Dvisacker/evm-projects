use std::{
    ops::{Div, Mul},
    sync::Arc,
};

use crate::types::Executor;
use alloy::{primitives::U256, providers::Provider, rpc::types::TransactionRequest};
use async_trait::async_trait;
use eyre::{Context, Result};

/// MempoolExecutor is responsible for submitting transactions to the public mempool.
/// It supports both basic transaction submission and gas price optimization based on
/// expected profit from MEV opportunities.
///
/// Type Parameters:
/// - M: The provider type that implements the Provider trait
///
/// # Example
/// ```rust,no_run
/// use std::sync::Arc;
/// use artemis_core::executors::{MempoolExecutor, SubmitTxToMempool};
///
/// async fn example(provider: Arc<impl Provider>) {
///     let executor = MempoolExecutor::new(provider);
///     let tx = SubmitTxToMempool {
///         tx: TransactionRequest::default(),
///         gas_bid_info: None,
///     };
///     executor.execute(tx).await.unwrap();
/// }
/// ```
pub struct MempoolExecutor<M> {
    /// The blockchain provider used to submit transactions
    client: Arc<M>,
}

/// Information about the gas bid for a transaction.
#[derive(Debug, Clone)]
pub struct GasBidInfo {
    /// Total profit expected from opportunity in wei
    pub total_profit: U256,

    /// Percentage of profit to use for gas (0-100)
    /// For example, 50 means 50% of profit will be used for gas
    pub bid_percentage: u64,
}

/// Action to submit a transaction to the public mempool.
/// Can optionally include gas bidding information for
/// profit-based gas price optimization.
#[derive(Debug, Clone)]
pub struct SubmitTxToMempool {
    /// The transaction to submit
    pub tx: TransactionRequest,
    /// Optional gas bidding information
    pub gas_bid_info: Option<GasBidInfo>,
}

impl<M: Provider> MempoolExecutor<M> {
    pub fn new(client: Arc<M>) -> Self {
        Self { client }
    }
}

/// Implementation of the [Executor] trait for [MempoolExecutor].
/// This implementation:
/// 1. Estimates gas usage for the transaction
/// 2. Calculates optimal gas price (either based on profit or current market price)
/// 3. Submits the transaction to the mempool
/// 4. Waits for and returns the transaction receipt
#[async_trait]
impl<M> Executor<SubmitTxToMempool> for MempoolExecutor<M>
where
    M: Provider,
{
    /// Executes a transaction submission to the mempool.
    ///
    /// # Arguments
    /// * `action` - The transaction submission action containing the transaction
    ///             and optional gas bidding information
    ///
    /// # Returns
    /// * `Result<()>` - Ok if the transaction was successfully submitted and mined
    ///
    /// # Errors
    /// Returns an error if:
    /// * Gas estimation fails
    /// * Gas price calculation fails
    /// * Transaction submission fails
    /// * Transaction receipt retrieval fails
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
