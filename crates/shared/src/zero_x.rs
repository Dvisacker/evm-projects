use alloy_chains::{Chain, NamedChain};
use alloy_primitives::{Address, U256};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZeroExQuote {
    pub buy_amount: String,
    pub price: String,
    pub sell_amount: String,
    pub allowance_target: String,
    pub price_impact: String,
    pub gas_price: String,
    pub estimated_gas: String,
    pub gas: String,
    pub protocol_fee: String,
    pub minimum_protocol_fee: String,
    pub to: String,
    pub data: String,
}

#[derive(Debug)]
pub struct ZeroExClient {
    api_key: String,
    client: reqwest::Client,
}

impl ZeroExClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_quote(
        &self,
        sell_token: Address,
        buy_token: Address,
        sell_amount: U256,
        taker_address: Address,
        chain: NamedChain,
    ) -> Result<ZeroExQuote, Box<dyn Error>> {
        let chain_id = Chain::from(chain).id();
        let mut headers = HeaderMap::new();
        headers.insert("0x-api-key", HeaderValue::from_str(&self.api_key).unwrap());
        headers.insert("0x-version", HeaderValue::from_static("v2"));

        let url = format!(
            "https://api.0x.org/swap/permit2/quote?chainId={}&sellToken={:#x}&buyToken={:#x}&sellAmount={}&taker={:#x}",
            chain_id, sell_token, buy_token, sell_amount, taker_address
        );

        let response = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .json::<ZeroExQuote>()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::provider::get_default_anvil_signer;
    use addressbook::Addressbook;

    use super::*;
    use std::env;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_zero_ex_real_quote() {
        dotenv::dotenv().ok();
        let api_key = env::var("ZEROX_API_KEY").expect("ZEROX_API_KEY must be set");
        let chain = NamedChain::Arbitrum;
        let addressbook = Addressbook::load(None).unwrap();
        let anvil_signer = get_default_anvil_signer();
        let user_address = anvil_signer.address();
        let weth = addressbook.get_weth(&chain).unwrap();
        let usdc = addressbook.get_usdc(&chain).unwrap();
        let input_amount = U256::from(1000000000000000000u128); // 1 WETH
        let client = ZeroExClient::new(api_key);

        // Test getting a quote for WETH -> USDC on Ethereum mainnet
        let result = client
            .get_quote(weth, usdc, input_amount, user_address, chain)
            .await;

        assert!(result.is_ok());

        let quote = result.unwrap();

        // Basic validation of the response
        assert!(!quote.buy_amount.is_empty());
        assert!(!quote.sell_amount.is_empty());
        assert!(!quote.to.is_empty());
        assert!(!quote.data.is_empty());

        // The sell amount should match what we requested
        assert_eq!(quote.sell_amount, "1000000000000000000");

        // USDC has 6 decimals, so the buy amount should be around 1800_000000 (1800 USDC)
        // This is a rough check since price will vary
        let buy_amount = quote.buy_amount.parse::<f64>().unwrap() / 1_000_000.0;
        assert!(buy_amount > 1000.0); // Should be worth more than 1000 USDC
        assert!(buy_amount < 3000.0); // Should be worth less than 3000 USDC

        println!("Quote received:");
        println!("Selling 1 WETH for {} USDC", buy_amount);
        println!("Price impact: {}%", quote.price_impact);
        println!("Gas estimate: {} wei", quote.estimated_gas);
    }
}
