use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/queries/query_pairs_for_token.graphql",
    response_derives = "Debug,Serialize,Deserialize"
)]
pub struct QueryCodexPairsForToken;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/queries/query_top_tokens.graphql",
    response_derives = "Debug,Serialize,Deserialize"
)]
pub struct QueryCodexTopTokens;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/queries/filter_tokens.graphql",
    response_derives = "Debug,Serialize,Deserialize,Default"
)]
pub struct QueryCodexFilterTokens;

pub type Token = query_codex_top_tokens::QueryCodexTopTokensData;
pub type Pairs = query_codex_pairs_for_token::QueryCodexPairsForTokenDataResults;
pub type FilteredTokens = query_codex_filter_tokens::QueryCodexFilterTokensDataResults;

pub struct CodexClient {
    client: reqwest::Client,
    api_key: String,
    endpoint: String,
}

impl CodexClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            endpoint: "https://graph.codex.io/graphql".to_string(),
        }
    }

    pub async fn get_top_tokens(&self, networks: Vec<i64>) -> Result<Vec<Token>> {
        let variables = query_codex_top_tokens::Variables {
            networks: Some(networks),
        };

        let request_body = QueryCodexTopTokens::build_query(variables);

        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("{}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        let response_body: Response<query_codex_top_tokens::ResponseData> = response.json().await?;

        if let Some(errors) = response_body.errors {
            return Err(anyhow::anyhow!("GraphQL errors: {:?}", errors));
        }

        let data = response_body
            .data
            .ok_or_else(|| anyhow::anyhow!("No data returned"))?;

        Ok(data.data.unwrap())
    }

    pub async fn get_pairs_for_token(
        &self,
        token_address: String,
        network_id: i64,
    ) -> Result<Vec<Pairs>> {
        let variables = query_codex_pairs_for_token::Variables {
            token_address,
            network_id,
        };

        let request_body = QueryCodexPairsForToken::build_query(variables);

        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("{}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        let response_body: Response<query_codex_pairs_for_token::ResponseData> =
            response.json().await?;

        if let Some(errors) = response_body.errors {
            return Err(anyhow::anyhow!("GraphQL errors: {:?}", errors));
        }

        let data = response_body
            .data
            .ok_or_else(|| anyhow::anyhow!("No data returned"))?;

        Ok(data.data.results)
    }

    pub async fn filter_tokens(
        &self,
        liquidity_min: Option<f64>,
        txn_count_min: Option<f64>,
        networks: Vec<i64>,
        exchange_addresses: Option<Vec<String>>,
        fdv_min: Option<f64>,
        market_cap_min: Option<f64>,
        limit: Option<i64>,
        ranking_attribute: query_codex_filter_tokens::TokenRankingAttribute,
        ranking_direction: query_codex_filter_tokens::RankingDirection,
    ) -> Result<Vec<FilteredTokens>> {
        let variables = query_codex_filter_tokens::Variables {
            filters: query_codex_filter_tokens::TokenFilters {
                fdv: fdv_min.map(|min| query_codex_filter_tokens::NumberFilter {
                    gt: Some(min),
                    gte: None,
                    lt: None,
                    lte: None,
                }),
                trending_ignored: None,
                market_cap: market_cap_min.map(|min| query_codex_filter_tokens::NumberFilter {
                    gt: Some(min),
                    gte: None,
                    lt: None,
                    lte: None,
                }),
                liquidity: liquidity_min.map(|min| query_codex_filter_tokens::NumberFilter {
                    gt: Some(min),
                    gte: None,
                    lt: None,
                    lte: None,
                }),
                txn_count24: txn_count_min.map(|min| query_codex_filter_tokens::NumberFilter {
                    gt: Some(min),
                    gte: None,
                    lt: None,
                    lte: None,
                }),
                age: None,
                exchange_address: exchange_addresses
                    .map(|addresses| addresses.into_iter().map(|e| e.into()).collect()),
                include_scams: None,
                is_verified: None,
                potential_scam: None,
                exchange_id: None,
                holders: None,
                txn_count1: None,
                txn_count12: None,
                volume1: None,
                volume4: None,
                volume12: None,
                volume24: None,
                network: Some(networks.into_iter().map(|n| n.into()).collect()),
                created_at: None,
                last_transaction: None,
                change1: None,
                change12: None,
                change24: None,
                change4: None,
                high1: None,
                high12: None,
                high24: None,
                high4: None,
                low1: None,
                low12: None,
                low24: None,
                low4: None,
                volume_change1: None,
                volume_change12: None,
                volume_change24: None,
                volume_change4: None,
                price_usd: None,
                sell_count1: None,
                sell_count12: None,
                sell_count24: None,
                sell_count4: None,
                txn_count4: None,
                unique_buys1: None,
                unique_buys12: None,
                unique_buys24: None,
                unique_buys4: None,
                unique_sells1: None,
                unique_sells12: None,
                unique_sells24: None,
                unique_sells4: None,
                unique_transactions1: None,
                unique_transactions12: None,
                unique_transactions24: None,
                unique_transactions4: None,
                buy_count1: None,
                buy_count12: None,
                buy_count24: None,
                buy_count4: None,
            },
            rankings: Some(vec![query_codex_filter_tokens::TokenRanking {
                attribute: Some(ranking_attribute),
                direction: Some(ranking_direction),
            }]),
            limit,
        };

        let request_body = QueryCodexFilterTokens::build_query(variables);

        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("{}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        let response_body: Response<query_codex_filter_tokens::ResponseData> =
            response.json().await?;

        if let Some(errors) = response_body.errors {
            return Err(anyhow::anyhow!("GraphQL errors: {:?}", errors));
        }

        let data = response_body
            .data
            .ok_or_else(|| anyhow::anyhow!("No data returned"))?;

        let result = data
            .data
            .unwrap()
            .results
            .unwrap()
            .into_iter()
            .filter_map(|r| r)
            .collect();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_top_tokens() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("CODEX_API_KEY").unwrap();
        let client = CodexClient::new(api_key);
        let tokens = client.get_top_tokens(vec![8453]).await.unwrap();
        assert!(!tokens.is_empty());
        println!("First token: {:?}", tokens.first().unwrap());
    }

    #[tokio::test]
    async fn test_query_pairs_for_token() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("CODEX_API_KEY").unwrap();
        let client = CodexClient::new(api_key);
        // Using WETH address on Base network
        let pairs = client
            .get_pairs_for_token(
                "0x4200000000000000000000000000000000000006".to_string(),
                8453,
            )
            .await
            .unwrap();
        assert!(!pairs.is_empty());
        println!("First pair: {:?}", pairs.first().unwrap());
    }

    #[tokio::test]
    async fn test_filter_tokens() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("CODEX_API_KEY").unwrap();
        let client = CodexClient::new(api_key);
        let tokens = client
            .filter_tokens(
                Some(100000.0),
                None,
                vec![8453],
                None,
                Some(10000.0),
                Some(1000.0),
                Some(10),
                query_codex_filter_tokens::TokenRankingAttribute::liquidity,
                query_codex_filter_tokens::RankingDirection::DESC,
            )
            .await
            .unwrap();
        assert!(!tokens.is_empty());
        println!("First filtered token: {:?}", tokens.first().unwrap());
    }
}
