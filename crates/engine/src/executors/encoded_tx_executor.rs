use std::{
    env,
    ops::{Div, Mul},
    str::FromStr,
    sync::Arc,
};

use crate::types::Executor;
use alloy::{
    primitives::{Address, Bytes, U256},
    providers::Provider,
    transports::BoxTransport,
};
use async_trait::async_trait;
use eyre::{Context, Result};
use provider::SignerProvider;
use tracing::info;
use tx_executor::bindings::batchexecutor::BatchExecutor::BatchExecutorInstance;

pub struct EncodedTxExecutor {
    client: Arc<SignerProvider>,
    executor: BatchExecutorInstance<BoxTransport, Arc<SignerProvider>>,
}

#[derive(Debug, Clone)]
pub struct GasBidInfo {
    /// Total profit expected from opportunity in wei
    pub total_profit: U256,
    /// Percentage of profit to use for gas (0-100)
    /// For example, 50 means 50% of profit will be used for gas
    pub bid_percentage: u64,
}

#[derive(Debug, Clone)]
pub struct SubmitEncodedTx {
    /// The calldata to submit
    pub calldata: Vec<Bytes>,
    /// Optional gas bidding information
    pub gas_bid_info: Option<GasBidInfo>,
}

impl EncodedTxExecutor {
    pub fn new(client: Arc<SignerProvider>) -> Self {
        let address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let executor: BatchExecutorInstance<BoxTransport, Arc<SignerProvider>> =
            BatchExecutorInstance::new(address, client.clone());
        Self {
            client: client.clone(),
            executor,
        }
    }
}

#[async_trait]
impl Executor<SubmitEncodedTx> for EncodedTxExecutor {
    async fn execute(&self, action: SubmitEncodedTx) -> Result<()> {
        let total_value = U256::ZERO;
        let calldata = action.calldata.clone();
        let gas_bid_info = action.gas_bid_info.clone();

        let call = self.executor.batchCall(calldata).value(total_value);
        let gas_usage = call.estimate_gas().await.unwrap();
        let bid_gas_price: u128;

        if let Some(gas_bid_info) = gas_bid_info {
            // gas price at which we'd break even, meaning 100% of profit goes to validator
            let breakeven_gas_price: u128 =
                gas_bid_info.total_profit.to::<u128>() / u128::from(gas_usage);
            bid_gas_price = breakeven_gas_price
                .mul(u128::from(gas_bid_info.bid_percentage))
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

        let receipt = call
            .gas_price(bid_gas_price)
            .send()
            .await?
            .get_receipt()
            .await?;

        info!("Transaction receipt: {:?}", receipt);
        Ok(())
    }
}
