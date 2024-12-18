use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/queries/pairs_for_token.graphql",
    response_derives = "Debug,Serialize,Deserialize"
)]
pub struct ListPairsForToken;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/queries/top_tokens.graphql",
    response_derives = "Debug,Serialize,Deserialize"
)]
pub struct ListTopTokens;

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
        let variables = list_top_tokens::Variables {
            networks: Some(networks),
        };

        let request_body = ListTopTokens::build_query(variables);

        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("{}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        let response_body: Response<list_top_tokens::ResponseData> = response.json().await?;

        if let Some(errors) = response_body.errors {
            return Err(anyhow::anyhow!("GraphQL errors: {:?}", errors));
        }

        let data = response_body
            .data
            .ok_or_else(|| anyhow::anyhow!("No data returned"))?;
        Ok(data.list_top_tokens.into_iter().map(Token::from).collect())
    }

    pub async fn get_pairs_for_token(
        &self,
        token_address: String,
        network_id: i64,
    ) -> Result<Vec<PairMetadata>> {
        let variables = list_pairs_for_token::Variables {
            token_address,
            network_id,
        };

        let request_body = ListPairsForToken::build_query(variables);

        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("{}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        let response_body: Response<list_pairs_for_token::ResponseData> = response.json().await?;

        if let Some(errors) = response_body.errors {
            return Err(anyhow::anyhow!("GraphQL errors: {:?}", errors));
        }

        let data = response_body
            .data
            .ok_or_else(|| anyhow::anyhow!("No data returned"))?;

        Ok(data
            .list_pairs_with_metadata_for_token
            .results
            .into_iter()
            .map(|result| PairMetadata {
                pair_address: result.pair.address,
                backing_token_address: result.backing_token.address,
                volume: result.volume,
                liquidity: result.liquidity,
            })
            .collect())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exchange {
    pub id: String,
    pub name: String,
    pub exchange_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: Option<i64>,
    pub top_pair_id: String,
    pub market_cap: Option<String>,
    pub volume: String,
    pub liquidity: String,
    pub exchanges: Vec<Exchange>,
}

impl From<list_top_tokens::ListTopTokensListTopTokens> for Token {
    fn from(token: list_top_tokens::ListTopTokensListTopTokens) -> Self {
        Self {
            address: token.address,
            symbol: token.symbol,
            name: token.name,
            decimals: token.decimals,
            top_pair_id: token.top_pair_id,
            market_cap: token.market_cap,
            volume: token.volume,
            liquidity: token.liquidity,
            exchanges: token
                .exchanges
                .into_iter()
                .map(|e| Exchange {
                    id: e.id,
                    name: e.name,
                    exchange_version: e.exchange_version,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PairMetadata {
    pub pair_address: String,
    pub backing_token_address: String,
    pub volume: String,
    pub liquidity: String,
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
    async fn test_get_pairs_for_token() {
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
}
