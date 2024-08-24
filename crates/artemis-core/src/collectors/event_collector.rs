use crate::types::{Collector, CollectorStream};
use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    abi::{AbiDecode, RawLog},
    prelude::{EthEvent, Middleware},
    providers::PubsubClient,
    types::{Address, Filter, Log, U256},
};
use std::sync::Arc;
use tokio_stream::StreamExt;

#[derive(Debug, Clone, EthEvent)]
#[ethevent(
    name = "Swap",
    abi = "Swap(address,uint256,uint256,uint256,uint256,address)"
)]
pub struct UniswapV2SwapEvent {
    #[ethevent(indexed)]
    pub sender: Address,
    pub amount0_in: U256,
    pub amount1_in: U256,
    pub amount0_out: U256,
    pub amount1_out: U256,
    #[ethevent(indexed)]
    pub to: Address,
}

pub struct EventCollector<M, E> {
    provider: Arc<M>,
    filter: Filter,
    _phantom: std::marker::PhantomData<E>,
}

impl<M, E> EventCollector<M, E>
where
    E: EthEvent + Send + Sync + 'static,
{
    pub fn new(provider: Arc<M>, filter: Filter) -> Self {
        Self {
            provider,
            filter,
            _phantom: std::marker::PhantomData,
        }
    }
    fn parse_event_log(log: &Log) -> Option<E> {
        E::decode_log(&RawLog {
            topics: log.topics.clone(),
            data: log.data.to_vec(),
        })
        .ok()
    }
}

#[async_trait]
impl<M, E> Collector<E> for EventCollector<M, E>
where
    M: Middleware,
    M::Provider: PubsubClient,
    M::Error: 'static,
    E: EthEvent + Send + Sync + 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, E>> {
        let stream = self.provider.subscribe_logs(&self.filter).await?;
        let stream = stream.filter_map(|log| Self::parse_event_log(&log));
        Ok(Box::pin(stream))
    }
}
