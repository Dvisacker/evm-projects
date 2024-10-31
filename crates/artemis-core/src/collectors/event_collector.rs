use crate::types::{Collector, CollectorStream};
use alloy::{primitives::Log, providers::Provider, rpc::types::Filter, sol_types::SolEvent};
use async_trait::async_trait;
use eyre::Result;
use std::sync::Arc;
use tokio_stream::StreamExt;

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
