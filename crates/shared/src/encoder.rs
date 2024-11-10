use alloy::network::{Ethereum, Network};
use alloy::providers::Provider;
use alloy::transports::Transport;
use alloy_primitives::{Address, Bytes, FixedBytes, TxHash};
use executor_binding::executor::Executor::{ExecutorInstance, Swap, SwapData};
use eyre::Result;

// a calldata enum that can be either a swap or a multicall
pub enum CallData {
    Swap(SwapData),
    Multicall(Vec<Bytes>),
}

pub struct ExecutorClient<T, P>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum>,
{
    executor: ExecutorInstance<T, P>,
    calldata: Option<CallData>,
}

impl<T, P> ExecutorClient<T, P>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum>,
{
    pub fn new(address: Address, provider: P) -> Self {
        let executor = ExecutorInstance::new(address, provider);
        Self {
            executor,
            calldata: None,
        }
    }

    pub fn push_call_data(&mut self, call_data: CallData) {
        match call_data {
            CallData::Swap(calldata) => self.push_swap_call_data(calldata),
            CallData::Multicall(calls) => self.push_multicall_call_data(calls),
        }
    }

    pub fn push_multicall_call_data(&mut self, calldata: Vec<Bytes>) {
        self.calldata = Some(CallData::Multicall(calldata));
    }

    pub fn push_swap_call_data(&mut self, calldata: SwapData) {
        self.calldata = Some(CallData::Swap(calldata));
    }

    pub fn push_swap(&mut self, swap: Swap) {
        if let Some(CallData::Swap(calldata)) = self.calldata.as_mut() {
            calldata.swaps.push(swap);
        }
    }

    pub async fn execute_tx(&self) -> Result<(bool, TxHash)> {
        let calldata = self.calldata.as_ref().unwrap();
        let pending_tx = match calldata {
            CallData::Swap(calldata) => {
                let call = self.executor.swap(calldata.clone());
                let result = call.send().await?;
                result
            }
            CallData::Multicall(calldata) => {
                let call = self.executor.multicall(calldata.clone());
                let result = call.send().await?;
                result
            }
        };
        let receipt = pending_tx.get_receipt().await?;
        let status = receipt.status();
        let tx_hash = receipt.transaction_hash;

        Ok((status, tx_hash))
    }
}
