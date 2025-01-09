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

    // unfortunately, doesn't seem to ƒetch all exchanges (for example aerodrome is missing)
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

    // pub async fn get_verified_token_by_symbol(&self, symbol: String) -> Result<FilteredTokens, Error> {
    //     let tokens = self.get_most_traded_tokens(8453, 100).await.unwrap();
    //     let token = tokens.into_iter().find(|t| t.token.unwrap().symbol.unwrap() == symbol);
    //     Ok(token)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    fn setup() -> MetadataClient {
        MetadataClient::new()
    }

    #[tokio::test]
    async fn test_get_most_traded_tokens() {
        let client = setup();
        let network_id = 1; // Ethereum mainnet
        let limit = 10;

        let result = client.get_most_traded_tokens(network_id, limit).await;

        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(!tokens.is_empty());
        assert!(tokens.len() <= limit as usize);

        // Check first token has required fields
        println!("{:?}", tokens);
    }

    #[tokio::test]
    async fn test_get_most_traded_pools() {
        let client = setup();
        let network_id = 8453; // Ethereum mainnet
        let limit = 10;

        let result = client.get_most_traded_pools(network_id, limit).await;

        assert!(result.is_ok());
        let pools = result.unwrap();
        assert!(!pools.is_empty());
        assert!(pools.len() <= limit as usize);

        // Check first pool has required fields
        println!("{:?}", pools);
    }

    #[tokio::test]
    async fn test_get_exchanges_by_network() {
        let client = setup();
        let networks = vec![8453]; // Ethereum mainnet
        let limit = 100;

        let result = client.get_exchanges_by_network(networks, limit).await;

        assert!(result.is_ok());
        let exchanges = result.unwrap();
        assert!(!exchanges.is_empty());
        assert!(exchanges.len() <= limit as usize);

        // Check first exchange has required fields
        println!(
            "{:?}",
            exchanges
                .into_iter()
                .map(|e| format!(
                    "{}({}): Volume={}",
                    e.clone()
                        .exchange
                        .unwrap_or_default()
                        .name
                        .unwrap_or_default(),
                    e.clone().exchange.unwrap_or_default().address,
                    e.clone().volume_usd24.unwrap()
                ))
                .collect::<Vec<String>>()
        );
    }
}
