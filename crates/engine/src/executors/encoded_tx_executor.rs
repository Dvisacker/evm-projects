use std::{env, str::FromStr, sync::Arc};

use crate::types::Executor;
use alloy::{
    contract::Error,
    primitives::{Address, Bytes, U256},
    providers::Provider,
};
use async_trait::async_trait;
use eyre::Result;
use tracing::{info, warn};
use tx_executor::bindings::batchexecutor::BatchExecutor::BatchExecutorInstance;

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
    pub total_value: U256,
    /// Optional gas bidding information
    pub gas_bid_info: Option<GasBidInfo>,
}

// The EncodedTxExecutor (to be renamed BundledTxExecutor)
pub struct EncodedTxExecutor<P: Provider> {
    #[allow(unused)]
    client: Arc<P>,
    executor: BatchExecutorInstance<(), Arc<P>>,
}

impl<P: Provider> EncodedTxExecutor<P> {
    pub fn new(client: Arc<P>) -> Self {
        let address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let executor: BatchExecutorInstance<(), Arc<P>> =
            BatchExecutorInstance::new(address, client.clone());
        Self {
            client: client.clone(),
            executor,
        }
    }
}

#[async_trait]
impl<P: Provider> Executor<SubmitEncodedTx> for EncodedTxExecutor<P> {
    async fn execute(&self, action: SubmitEncodedTx) -> Result<()> {
        let total_value = action.total_value;
        let calldata = action.calldata.clone();
        let gas_bid_info = action.gas_bid_info.clone();
        let owner = self.executor.OWNER().call().await.unwrap()._0;

        let call = self
            .executor
            .batchCall(calldata)
            .value(total_value)
            .from(owner);

        let result = call.estimate_gas().await;

        if let Err(e) = result {
            match e {
                Error::TransportError(rpc_err) => {
                    let error_response = rpc_err.as_error_resp();
                    let message = error_response.unwrap().message.clone();
                    let code = error_response.unwrap().code;
                    warn!("message: {:?} (error code: {})", message, code);
                    return Ok(());
                }
                _ => {
                    warn!("Error estimating gas: {:?}", e);
                    return Ok(());
                }
            }
        }

        let gas_usage = result.unwrap();

        let bid_gas_price: u128;
        if let Some(gas_bid_info) = gas_bid_info {
            let breakeven_gas_price: u128 =
                gas_bid_info.total_profit.to::<u128>() / u128::from(gas_usage);
            bid_gas_price = breakeven_gas_price * u128::from(gas_bid_info.bid_percentage) / 100;
        } else {
            bid_gas_price = self.client.get_gas_price().await.unwrap();
        }

        info!("Sending tx with gas price: {}", bid_gas_price);
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
