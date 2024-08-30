use crate::types::{Collector, CollectorStream};
use alloy::providers::Provider;
use alloy::rpc::types::{Filter, Log};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct MultiLogCollector<M: Provider> {
    provider: Arc<M>,
    filters: Vec<Filter>,
}

impl<M: Provider> MultiLogCollector<M> {
    pub fn new(provider: Arc<M>, filters: Vec<Filter>) -> Self {
        Self { provider, filters }
    }
}

#[async_trait]
impl<M: Provider + 'static> Collector<Log> for MultiLogCollector<M> {
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
