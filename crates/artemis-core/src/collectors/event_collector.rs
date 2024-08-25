use crate::types::{Collector, CollectorStream};
use alloy::{
    primitives::{Address, Log, U256},
    providers::Provider,
    rpc::types::{Filter, RawLog},
    sol_types::SolEvent,
};
use alloy_sol_types::sol;
use anyhow::Result;
use async_trait::async_trait;
use bindings::iuniswapv2pair::IUniswapV2Pair;
// use bindings::iuniswapv2router02::UniswapV2Router02;
use std::sync::Arc;
use tokio_stream::StreamExt;

// // abi = "Swap(address,uint256,uint256,uint256,uint256,address)"
// #[derive(Debug, Clone, SolEvent)]
// #[sol(event, name = "Swap")]
// pub struct UniswapV2SwapEvent {
//     #[sol(indexed)]
//     pub sender: Address,
//     pub amount0_in: U256,
//     pub amount1_in: U256,
//     pub amount0_out: U256,
//     pub amount1_out: U256,
//     #[sol(indexed)]
//     pub to: Address,
// }

pub struct EventCollector<M, E> {
    provider: Arc<M>,
    filter: Filter,
    _phantom: std::marker::PhantomData<E>,
}

impl<M, E> EventCollector<M, E>
where
    E: SolEvent + Send + Sync + 'static,
{
    pub fn new(provider: Arc<M>, filter: Filter) -> Self {
        // let event = IUniswapV2Pair::Swap;
        Self {
            provider,
            filter,
            _phantom: std::marker::PhantomData,
        }
    }
    fn parse_event_log(log: &Log) -> Option<E> {
        let log = E::decode_log(&log, true).ok();
        let data = log.map(|log| log.data);
        data
    }
}

#[async_trait]
impl<M, E> Collector<E> for EventCollector<M, E>
where
    M: Provider,
    E: SolEvent + Send + Sync + 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, E>> {
        let sub = self.provider.subscribe_logs(&self.filter).await?;
        let stream = sub.into_stream().take(256);
        let stream = stream.filter_map(|log| Self::parse_event_log(&log.inner));
        Ok(Box::pin(stream))
    }
}
