use alloy::network::Ethereum;
use alloy::{hex, sol};
use alloy::{providers::Provider, transports::Transport};
use alloy_primitives::{Address, U256};
use alloy_rpc_types::TransactionRequest;
use eyre::{eyre, Context, Result};

use reqwest;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;

pub const ARBITRUM_CHAIN_ID: u64 = 42161;
pub const BASE_CHAIN_ID: u64 = 8453;
pub const USDC_ARBITRUM: &str = "0xaf88d065e77c8cc2239327c5edb3a432268e5831";
pub const USDC_BASE: &str = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913";

sol! {
    /// Interface of the ERC20 token
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);
    }
}

#[derive(Debug, Serialize)]
struct LiFiQuoteRequest {
    fromChain: String,
    toChain: String,
    fromToken: String,
    toToken: String,
    fromAmount: String,
    fromAddress: String,
    toAddress: String,
    slippage: String,
    allowBridges: String,
    order: String,
}

#[derive(Debug, Deserialize)]
pub struct LiFiQuoteResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub quote_type: String,
    pub tool: String,
    // #[serde(rename = "toolDetails")]
    // pub tool_details: LiFiToolDetails,
    pub action: LiFiAction,
    pub estimate: LiFiEstimate,
    pub integrator: String,
    #[serde(rename = "transactionRequest")]
    pub transaction_request: LiFiTransactionRequest,
    // #[serde(rename = "includedSteps")]
    // pub included_steps: Vec<LiFiIncludedStep>,
}

#[derive(Debug, Deserialize)]
pub struct LiFiToolDetails {
    pub key: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LiFiAction {
    #[serde(rename = "fromChainId")]
    pub from_chain_id: u64,
    #[serde(rename = "toChainId")]
    pub to_chain_id: u64,
    #[serde(rename = "fromToken")]
    pub from_token: LiFiToken,
    #[serde(rename = "toToken")]
    pub to_token: LiFiToken,
    #[serde(rename = "fromAmount")]
    pub from_amount: String,
    pub slippage: f64,
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    #[serde(rename = "toAddress")]
    pub to_address: String,
}

#[derive(Debug, Deserialize)]
pub struct LiFiToken {
    pub address: String,
    pub symbol: String,
    pub decimals: u8,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    pub name: String,
    #[serde(rename = "coinKey", default)]
    pub coin_key: Option<String>,
    #[serde(rename = "priceUSD", default)]
    pub price_usd: Option<String>,
    #[serde(rename = "logoURI", default)]
    pub logo_uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LiFiEstimate {
    #[serde(rename = "fromAmount")]
    pub from_amount: String,
    #[serde(rename = "toAmount")]
    pub to_amount: String,
    #[serde(rename = "toAmountMin")]
    pub to_amount_min: String,
    #[serde(rename = "approvalAddress")]
    pub approval_address: String,
    #[serde(rename = "feeCosts")]
    pub fee_costs: Vec<LiFiFeeCost>,
    #[serde(rename = "gasCosts")]
    pub gas_costs: Vec<LiFiGasCost>,
    // pub data: LiFiEstimateData,
}

#[derive(Debug, Deserialize)]
pub struct LiFiFeeCost {}

#[derive(Debug, Deserialize)]
pub struct LiFiGasCost {
    #[serde(rename = "type")]
    pub gas_type: String,
    pub price: String,
    pub estimate: String,
    pub limit: String,
    pub amount: String,
    #[serde(rename = "amountUSD")]
    pub amount_usd: String,
    pub token: LiFiToken,
}

#[derive(Debug, Deserialize)]
pub struct LiFiEstimateData {
    #[serde(rename = "fromToken")]
    pub from_token: LiFiTokenInfo,
    #[serde(rename = "toToken")]
    pub to_token: LiFiTokenInfo,
    #[serde(rename = "toTokenAmount")]
    pub to_token_amount: String,
    #[serde(rename = "fromTokenAmount")]
    pub from_token_amount: String,
    pub protocols: Vec<Vec<Vec<LiFiProtocol>>>,
    #[serde(rename = "estimatedGas")]
    pub estimated_gas: u64,
}

#[derive(Debug, Deserialize)]
pub struct LiFiTokenInfo {
    pub name: String,
    pub address: String,
    pub symbol: String,
    pub decimals: u8,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct LiFiProtocol {
    pub name: String,
    pub part: u32,
    #[serde(rename = "fromTokenAddress")]
    pub from_token_address: String,
    #[serde(rename = "toTokenAddress")]
    pub to_token_address: String,
}

#[derive(Debug, Deserialize)]
pub struct LiFiTransactionRequest {
    pub from: String,
    pub to: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    pub data: String,
    pub value: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
}

#[derive(Debug, Deserialize)]
pub struct LiFiIncludedStep {
    pub id: String,
    #[serde(rename = "type")]
    pub step_type: String,
    pub tool: String,
    #[serde(rename = "toolDetails")]
    pub tool_details: LiFiToolDetails,
    pub action: LiFiAction,
    pub estimate: LiFiEstimate,
}

pub async fn bridge_lifi<T, P>(
    provider: Arc<P>,
    from_chain_id: u64,
    to_chain_id: u64,
    from_token_address: Address,
    to_token_address: Address,
    amount: U256,
    from_address: Address,
    to_address: Address,
) -> Result<U256>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum>,
{
    // 1. Get quote from Li.Fi API
    let client = reqwest::Client::new();
    let from_token = IERC20::new(from_token_address, provider.clone());

    let quote_request = LiFiQuoteRequest {
        fromChain: from_chain_id.to_string(),
        toChain: to_chain_id.to_string(),
        fromToken: from_token_address.to_string(),
        toToken: to_token_address.to_string(),
        fromAmount: amount.to_string(),
        fromAddress: from_address.to_string(),
        toAddress: to_address.to_string(),
        slippage: "0.10".to_string(),
        order: "FASTEST".to_string(),
        allowBridges: "across".to_string(),
    };

    let response = client
        .get("https://li.quest/v1/quote")
        .query(&quote_request)
        .send()
        .await
        .wrap_err("Failed to get quote from Li.Fi API")?;

    let quote_response: LiFiQuoteResponse = response.json().await?;
    let bridge_address = Address::from_str(&quote_response.transaction_request.to)?;

    // 2. Approve tokens to bridge
    let approve_tx = from_token.approve(bridge_address, amount).send().await?;
    let _approve_receipt = approve_tx.get_receipt().await.unwrap();

    // 3. Execute the bridge transaction
    let data = hex::decode(&quote_response.transaction_request.data[2..])
        .wrap_err("Failed to decode transaction data")?;
    let to_address = Address::from_str(&quote_response.transaction_request.to)
        .wrap_err("Failed to parse 'to' address")?;
    let value = U256::from_str(&quote_response.transaction_request.value)
        .wrap_err("Failed to parse transaction value")?;
    let gas_limit = U256::from_str(&quote_response.transaction_request.gas_limit)
        .wrap_err("Failed to parse gas limit")?;
    let gas_price = U256::from_str(&quote_response.transaction_request.gas_price)
        .wrap_err("Failed to parse gas price")?;

    let tx_request = TransactionRequest::default()
        .to(to_address)
        .input(data.into())
        .value(value)
        .gas_limit(gas_limit.try_into().unwrap())
        .max_fee_per_gas(gas_price.try_into().unwrap())
        .max_priority_fee_per_gas(gas_price.try_into().unwrap());

    // println!("Tx request: {:?}", tx_request);

    let pending_tx = provider.send_transaction(tx_request).await?;
    let receipt = pending_tx.get_receipt().await?;

    println!("Receipt: {:?}", receipt);

    // 4. Monitor bridge status
    let status = "PENDING".to_string();
    while status == "PENDING" {
        let status_response = client
            .get("https://li.quest/v1/status")
            .query(&[
                ("bridge", &quote_response.tool),
                ("fromChain", &from_chain_id.to_string()),
                ("toChain", &to_chain_id.to_string()),
                ("txHash", &receipt.transaction_hash.to_string()),
            ])
            .send()
            .await
            .wrap_err("Failed to get bridge status")?
            .json::<serde_json::Value>()
            .await
            .wrap_err("Failed to parse status response")?;

        let status = status_response["status"]
            .as_str()
            .unwrap_or("UNKNOWN")
            .to_string();

        if status == "FAILED" {
            return Err(eyre!("Bridge transaction failed"));
        }

        if status == "PENDING" {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    let final_amount = U256::from_str(&quote_response.estimate.to_amount)
        .wrap_err("Failed to parse estimated amount")?;

    Ok(final_amount)
}

#[cfg(test)]
mod tests {
    use crate::provider::get_provider;

    use super::*;
    use alloy::{network::EthereumWallet, signers::local::PrivateKeySigner};
    use alloy_chains::{Chain, NamedChain};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_bridge_usdc_arbitrum_to_base() -> Result<()> {
        dotenv::dotenv().ok();

        let signer: PrivateKeySigner = std::env::var("DEV_PRIVATE_KEY")
            .expect("PRIVATE_KEY must be set")
            .parse()
            .expect("should parse private key");

        let wallet_address = signer.address();
        let wallet = EthereumWallet::new(signer);
        let provider = get_provider(Chain::from_named(NamedChain::Arbitrum), wallet).await;
        let from_address = wallet_address;
        let to_address = wallet_address;
        let usdc_arb = Address::from_str(USDC_ARBITRUM).expect("Invalid USDC Arbitrum address");
        let usdc_base = Address::from_str(USDC_BASE).expect("Invalid USDC Base address");

        // Amount to bridge (e.g., 1 USDC = 10_000_000 because USDC has 6 decimals)
        let amount = U256::from(1_000_000u64);

        let result = bridge_lifi(
            provider,
            ARBITRUM_CHAIN_ID,
            BASE_CHAIN_ID,
            usdc_arb,
            usdc_base,
            amount,
            from_address,
            to_address,
        )
        .await?;

        println!(
            "Bridge transaction completed. Expected output amount: {}",
            result
        );
        Ok(())
    }
}
