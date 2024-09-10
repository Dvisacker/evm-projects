use crate::types::{Collector, CollectorStream};
use alloy::{
    primitives::{U256, U64},
    providers::Provider,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio_stream::StreamExt;

/// A collector that listens for new blocks, and generates a stream of
/// [events](NewBlock) which contain the block number and hash.
pub struct BlockCollector<M: Provider> {
    provider: Arc<M>,
}

/// A new block event, containing the block number and hash.
#[derive(Debug, Clone)]
pub struct NewBlock {
    pub hash: U256,
    pub number: U64,
}

impl<M: Provider> BlockCollector<M> {
    pub fn new(provider: Arc<M>) -> Self {
        Self { provider }
    }
}

/// Implementation of the [Collector](Collector) trait for the [BlockCollector](BlockCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new blocks.
#[async_trait]
impl<M> Collector<NewBlock> for BlockCollector<M>
where
    M: Provider,
    // M::Provider: PubSubFrontend,
    // M::Error: 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewBlock>> {
        let sub = self.provider.subscribe_blocks().await?;
        let stream = sub.into_stream().take(256);
        let stream = stream.filter_map(|block| {
            Some(NewBlock {
                hash: block.header.hash.into(),
                number: U64::from(block.header.number),
            })
        });
        Ok(Box::pin(stream))
    }
}
