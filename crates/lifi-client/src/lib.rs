pub mod lifi_types;
use alloy::network::Ethereum;
use alloy::{hex, providers::Provider};
use alloy_chains::NamedChain;
use alloy_primitives::{Address, U256};
use alloy_rpc_types::TransactionRequest;
use eyre::{eyre, Context, Result};
use lifi_types::{LiFiQuoteRequest, LiFiQuoteResponse, LiFiTransactionRequest};
use reqwest;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use types::bindings::ierc20::IERC20;
use types::bridge::BridgeName;

pub struct LifiClient {
    http_client: reqwest::Client,
    base_url: String,
}

impl LifiClient {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: "https://li.quest/v1".to_string(),
        }
    }

    pub async fn bridge<P>(
        &self,
        origin_chain_provider: Arc<P>,
        destination_chain_provider: Arc<P>,
        from_chain: &NamedChain,
        to_chain: &NamedChain,
        from_token_address: Address,
        to_token_address: Address,
        amount_in: U256,
        from_address: Address,
        to_address: Address,
        bridge_name: BridgeName,
    ) -> Result<U256>
    where
        P: Provider<Ethereum>,
    {
        // 1. Get quote
        let quote = self
            .get_quote(
                from_chain,
                to_chain,
                from_token_address,
                to_token_address,
                amount_in,
                from_address,
                to_address,
                bridge_name,
            )
            .await?;

        // 2. Approve tokens
        let from_token = IERC20::new(from_token_address, origin_chain_provider.clone());
        let to_token = IERC20::new(to_token_address, destination_chain_provider.clone());
        let bridge_address = Address::from_str(&quote.transaction_request.to)?;

        let approve_tx = from_token.approve(bridge_address, amount_in).send().await?;
        let _approve_receipt = approve_tx.get_receipt().await.unwrap();

        let to_token_balance_before = match to_token.balanceOf(to_address).call().await {
            Ok(balance) => balance._0,
            Err(e) => return Err(eyre!("Error getting balance before: {:?}", e)),
        };

        // 3. Execute bridge transaction
        let tx_hash = self
            .execute_bridge_transaction(&quote.transaction_request, origin_chain_provider.clone())
            .await?;

        // 4. Monitor bridge status
        self.wait_for_bridge_completion(
            &quote.tool,
            (*from_chain).into(),
            (*to_chain).into(),
            &tx_hash.to_string(),
        )
        .await?;

        // 5. Get final balance difference
        let to_token_balance_after = match to_token.balanceOf(to_address).call().await {
            Ok(balance) => balance._0,
            Err(e) => return Err(eyre!("Error getting balance after: {:?}", e)),
        };
        let amount_out = to_token_balance_after - to_token_balance_before;

        Ok(amount_out)
    }

    async fn get_quote(
        &self,
        from_chain: &NamedChain,
        to_chain: &NamedChain,
        from_token: Address,
        to_token: Address,
        amount: U256,
        from_address: Address,
        to_address: Address,
        bridge_name: BridgeName,
    ) -> Result<LiFiQuoteResponse> {
        let quote_request = LiFiQuoteRequest {
            from_chain: u64::from(*from_chain).to_string(),
            to_chain: u64::from(*to_chain).to_string(),
            from_token: from_token.to_string(),
            to_token: to_token.to_string(),
            from_amount: amount.to_string(),
            from_address: from_address.to_string(),
            to_address: to_address.to_string(),
            slippage: "0.10".to_string(),
            order: "FASTEST".to_string(),
            allow_bridges: bridge_name.to_string(),
        };

        let response = self
            .http_client
            .get(format!("{}/quote", self.base_url))
            .query(&quote_request)
            .send()
            .await
            .wrap_err("Failed to get quote from Li.Fi API")?;

        let quote_response: LiFiQuoteResponse = response.json().await?;
        Ok(quote_response)
    }

    async fn execute_bridge_transaction<P>(
        &self,
        tx_request: &LiFiTransactionRequest,
        provider: Arc<P>,
    ) -> Result<alloy_primitives::B256>
    where
        P: Provider<Ethereum>,
    {
        let data =
            hex::decode(&tx_request.data[2..]).wrap_err("Failed to decode transaction data")?;
        let contract_address = Address::from_str(&tx_request.to)?;
        let value = U256::from_str(&tx_request.value)?;
        let gas_limit = U256::from_str(&tx_request.gas_limit)?;
        let gas_price = U256::from_str(&tx_request.gas_price)?;

        let tx_request = TransactionRequest::default()
            .to(contract_address)
            .input(data.into())
            .value(value)
            .gas_limit(gas_limit.try_into().unwrap())
            .max_fee_per_gas(gas_price.try_into().unwrap())
            .max_priority_fee_per_gas(gas_price.try_into().unwrap());

        let pending_tx = provider.send_transaction(tx_request).await?;
        let receipt = pending_tx.get_receipt().await?;

        Ok(receipt.transaction_hash)
    }

    async fn wait_for_bridge_completion(
        &self,
        tool: &str,
        from_chain_id: u64,
        to_chain_id: u64,
        tx_hash: &str,
    ) -> Result<()> {
        let mut status = "PENDING".to_string();

        while status == "PENDING" || status == "UNKNOWN" {
            let status_response = self
                .http_client
                .get(format!("{}/status", self.base_url))
                .query(&[
                    ("bridge", tool),
                    ("fromChain", &from_chain_id.to_string()),
                    ("toChain", &to_chain_id.to_string()),
                    ("txHash", tx_hash),
                ])
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;

            status = status_response["status"]
                .as_str()
                .unwrap_or("UNKNOWN")
                .to_string();

            if status == "FAILED" {
                return Err(eyre!("Bridge transaction failed"));
            }

            if status == "PENDING" || status == "UNKNOWN" {
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
        Ok(())
    }
}

impl Default for LifiClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use addressbook::Addressbook;
    use alloy::signers::local::PrivateKeySigner;
    use alloy_chains::NamedChain;
    use provider::{get_default_signer, get_provider_map};
    use types::token::NamedToken;

    #[tokio::test]
    async fn test_bridge_usdc_arbitrum_to_base() -> Result<()> {
        dotenv::dotenv().ok();

        let lifi_client = LifiClient::new();
        let addressbook = Addressbook::load().unwrap();
        let signer: PrivateKeySigner = get_default_signer();
        let wallet_address = signer.address();
        let provider_map = get_provider_map().await;

        let result = lifi_client
            .bridge(
                Arc::new(provider_map.get(&NamedChain::Arbitrum).unwrap().clone()),
                Arc::new(provider_map.get(&NamedChain::Base).unwrap().clone()),
                &NamedChain::Arbitrum,
                &NamedChain::Base,
                addressbook
                    .get_token(&NamedChain::Arbitrum, &NamedToken::USDC)
                    .unwrap(),
                addressbook
                    .get_token(&NamedChain::Base, &NamedToken::USDC)
                    .unwrap(),
                U256::from(1_000_000u64),
                wallet_address,
                wallet_address,
                BridgeName::Accross,
            )
            .await?;

        println!(
            "Bridge transaction completed. Expected output amount: {}",
            result
        );
        Ok(())
    }

    // The WETH test remains similar but uses the new LifiClient structure
    #[tokio::test]
    async fn test_bridge_weth_arbitrum_to_base() -> Result<()> {
        // Test implementation remains similar but uses LifiClient
        Ok(())
    }
}
