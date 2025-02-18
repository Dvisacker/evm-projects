use crate::types::{Collector, CollectorStream};
use alloy::providers::Provider;
use alloy::rpc::types::{Filter, Log};
use async_trait::async_trait;
use eyre::Result;
use std::sync::Arc;

pub struct MultiLogCollector<P: Provider> {
    provider: Arc<P>,
    filters: Vec<Filter>,
}

impl<P: Provider> MultiLogCollector<P> {
    pub fn new(provider: Arc<P>, filters: Vec<Filter>) -> Self {
        Self { provider, filters }
    }
}

#[async_trait]
impl<P: Provider + 'static> Collector<Log> for MultiLogCollector<P> {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Log>> {
        let mut streams = Vec::new();
        for filter in &self.filters {
            let stream = self.provider.subscribe_logs(&filter).await?;
            let stream = stream.into_stream();
            streams.push(stream);
        }

        let combined_stream = futures::stream::select_all(streams);
        Ok(Box::pin(combined_stream))
    }
}
