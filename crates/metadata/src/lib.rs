use codex_client::{
    query_codex_filter_exchanges, query_codex_filter_pairs, query_codex_filter_tokens, CodexClient,
    FilteredExchanges, FilteredPairs, FilteredTokens,
};
use eyre::Error;

pub struct MetadataClient {
    client: CodexClient,
}

impl MetadataClient {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let api_key = std::env::var("CODEX_API_KEY").unwrap();
        Self {
            client: CodexClient::new(api_key),
        }
    }

    pub async fn get_most_traded_tokens(
        &self,
        network_id: i64,
        limit: i64,
    ) -> Result<Vec<FilteredTokens>, Error> {
        let tokens = self
            .client
            .filter_tokens(
                Some(100000.0),
                None,
                vec![network_id],
                None,
                Some(10000.0),
                Some(1000.0),
                Some(limit),
                query_codex_filter_tokens::TokenRankingAttribute::volume24,
                query_codex_filter_tokens::RankingDirection::DESC,
            )
            .await
            .unwrap();
        Ok(tokens)
    }

    pub async fn get_most_traded_pools(
        &self,
        network_id: i64,
        limit: i64,
    ) -> Result<Vec<FilteredPairs>, Error> {
        let pairs = self
            .client
            .filter_pairs(
                Some(100000.0),
                None,
                vec![network_id],
                None,
                Some(10000.0),
                Some(1000.0),
                Some(limit),
                query_codex_filter_pairs::PairRankingAttribute::volumeUSD24,
                query_codex_filter_pairs::RankingDirection::DESC,
            )
            .await
            .unwrap();

        Ok(pairs)
    }

    pub async fn get_exchanges_by_network(
        &self,
        networks: Vec<i64>,
        limit: i64,
    ) -> Result<Vec<FilteredExchanges>, Error> {
        let exchanges = self
            .client
            .filter_exchanges(
                networks,
                None,
                None,
                Some(limit),
                query_codex_filter_exchanges::ExchangeRankingAttribute::volumeUSD24,
                query_codex_filter_exchanges::RankingDirection::DESC,
            )
            .await
            .unwrap();
        Ok(exchanges)
    }
}
