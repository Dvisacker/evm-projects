use crate::types::{Collector, CollectorStream};
use alloy::{
    primitives::{U256, U64},
    providers::Provider,
};
use async_trait::async_trait;
use eyre::Result;
use std::sync::Arc;
use tokio_stream::StreamExt;

/// BlockCollector is responsible for monitoring new blocks on the blockchain.
/// It provides a stream of [NewBlock] events that contain information about
/// each new block as it is mined.
///
/// Type Parameters:
/// - M: The provider type that implements the Provider trait
///
/// # Example
/// ```rust,no_run
/// use std::sync::Arc;
/// use engine::collectors::BlockCollector;
///
/// async fn example(provider: Arc<impl Provider>) {
///     let collector = BlockCollector::new(provider);
///     let mut stream = collector.get_event_stream().await.unwrap();
///     while let Some(block) = stream.next().await {
///         println!("New block: {} ({})", block.number, block.hash);
///     }
/// }
/// ```
pub struct BlockCollector<M: Provider> {
    /// The blockchain provider used to subscribe to new blocks
    provider: Arc<M>,
}

/// Represents a newly mined block on the blockchain.
/// This event is emitted by the [BlockCollector] when a new block is detected.
#[derive(Debug, Clone)]
pub struct NewBlock {
    /// The block hash as a 256-bit number
    pub hash: U256,
    /// The block number
    pub number: U64,
}

impl<M: Provider> BlockCollector<M> {
    /// Creates a new BlockCollector instance.
    ///
    /// # Arguments
    /// * `provider` - An Arc-wrapped provider that implements the Provider trait
    pub fn new(provider: Arc<M>) -> Self {
        Self { provider }
    }
}

/// Implementation of the [Collector] trait for [BlockCollector].
/// This implementation:
/// 1. Subscribes to new blocks using the provider
/// 2. Converts block headers into NewBlock events
/// 3. Provides a stream of these events
#[async_trait]
impl<M> Collector<NewBlock> for BlockCollector<M>
where
    M: Provider,
{
    /// Returns a stream of NewBlock events.
    /// The stream will emit a new event each time a block is mined.
    ///
    /// # Returns
    /// * `Result<CollectorStream<'_, NewBlock>>` - A stream of NewBlock events
    ///
    /// # Errors
    /// Returns an error if:
    /// * The subscription to new blocks fails
    /// * The provider connection is lost
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewBlock>> {
        let sub = self.provider.subscribe_blocks().await?;
        let stream = sub.into_stream().take(256);
        let stream = stream.filter_map(|header| {
            Some(NewBlock {
                hash: header.hash.into(),
                number: U64::from(header.number),
            })
        });
        Ok(Box::pin(stream))
    }
}
