use alloy::sol_types::SolType;
use alloy_chains::{Chain, NamedChain};
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol_data::Bytes as SolBytes;
use alloy_sol_types::{SolCall, SolValue};
use eyre::Result;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct OdosInputToken {
    amount: String,
    #[serde(rename = "tokenAddress")]
    token_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OdosOutputToken {
    proportion: String,
    #[serde(rename = "tokenAddress")]
    token_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OdosQuoteRequest {
    #[serde(rename = "chainId")]
    chain_id: u64,
    #[serde(rename = "inputTokens")]
    input_tokens: Vec<OdosInputToken>,
    #[serde(rename = "outputTokens")]
    output_tokens: Vec<OdosOutputToken>,
    #[serde(rename = "userAddr")]
    user_addr: String,
    #[serde(rename = "slippageLimitPercent")]
    slippage_limit_percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OdosQuoteResponse {
    #[serde(rename = "inTokens")]
    pub in_tokens: Vec<String>,
    #[serde(rename = "outTokens")]
    pub out_tokens: Vec<String>,
    #[serde(rename = "inAmounts")]
    pub in_amounts: Vec<String>,
    #[serde(rename = "outAmounts")]
    pub out_amounts: Vec<String>,
    #[serde(rename = "gasEstimate")]
    pub gas_estimate: f64,
    #[serde(rename = "dataGasEstimate")]
    pub data_gas_estimate: u64,
    #[serde(rename = "gweiPerGas")]
    pub gwei_per_gas: f64,
    #[serde(rename = "gasEstimateValue")]
    pub gas_estimate_value: f64,
    #[serde(rename = "inValues")]
    pub in_values: Vec<f64>,
    #[serde(rename = "outValues")]
    pub out_values: Vec<f64>,
    #[serde(rename = "netOutValue")]
    pub net_out_value: f64,
    #[serde(rename = "priceImpact")]
    pub price_impact: f64,
    #[serde(rename = "percentDiff")]
    pub percent_diff: f64,
    #[serde(rename = "partnerFeePercent")]
    pub partner_fee_percent: f64,
    #[serde(rename = "pathId")]
    pub path_id: String,
    #[serde(rename = "pathViz")]
    pub path_viz: Option<serde_json::Value>,
    #[serde(rename = "pathVizImage")]
    pub path_viz_image: Option<String>,
    #[serde(rename = "blockNumber")]
    pub block_number: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OdosTokenInfo {
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OdosSimulationError {
    #[serde(rename = "type")]
    pub error_type: String,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OdosSimulation {
    #[serde(rename = "isSuccess")]
    pub is_success: bool,
    #[serde(rename = "amountsOut")]
    pub amounts_out: Vec<f64>,
    #[serde(rename = "gasEstimate")]
    pub gas_estimate: u64,
    #[serde(rename = "simulationError")]
    pub simulation_error: Option<OdosSimulationError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OdosTransaction {
    pub gas: u64,
    #[serde(rename = "gasPrice")]
    pub gas_price: u64,
    pub value: String,
    pub to: String,
    pub from: String,
    pub data: String,
    pub nonce: u64,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OdosAssembleRequest {
    #[serde(rename = "userAddr")]
    user_addr: String,
    #[serde(rename = "pathId")]
    path_id: String,
    simulate: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OdosAssembleResponse {
    pub deprecated: Option<String>,
    #[serde(rename = "blockNumber")]
    pub block_number: u64,
    #[serde(rename = "gasEstimate")]
    pub gas_estimate: u64,
    #[serde(rename = "gasEstimateValue")]
    pub gas_estimate_value: f64,
    #[serde(rename = "inputTokens")]
    pub input_tokens: Vec<OdosTokenInfo>,
    #[serde(rename = "outputTokens")]
    pub output_tokens: Vec<OdosTokenInfo>,
    #[serde(rename = "netOutValue")]
    pub net_out_value: f64,
    #[serde(rename = "outValues")]
    pub out_values: Vec<String>,
    pub transaction: OdosTransaction,
    pub simulation: Option<OdosSimulation>,
}

pub async fn get_odos_quote(
    chain_id: NamedChain,
    input_token: Address,
    input_amount: U256,
    output_token: Address,
    user_address: Address,
    slippage: f64,
) -> Result<OdosQuoteResponse> {
    let client = reqwest::Client::new();
    let chain_id = Chain::from_named(chain_id).id();

    let request = OdosQuoteRequest {
        chain_id,
        input_tokens: vec![OdosInputToken {
            amount: input_amount.to_string(),
            token_address: input_token.to_string(),
        }],
        output_tokens: vec![OdosOutputToken {
            proportion: "1".to_string(),
            token_address: output_token.to_string(),
        }],
        slippage_limit_percent: slippage,
        user_addr: user_address.to_string(),
    };

    let response = client
        .post("https://api.odos.xyz/sor/quote/v2")
        .json(&request)
        .send()
        .await?;

    let response_text = response.text().await?;
    match serde_json::from_str::<OdosQuoteResponse>(&response_text) {
        Ok(parsed) => Ok(parsed),
        Err(e) => {
            println!("Failed to parse Odos response: {}", e);
            println!("Raw response: {}", response_text);
            Err(eyre::eyre!("Failed to parse Odos response"))
        }
    }
}

pub async fn assemble_odos_swap(
    quote: &OdosQuoteResponse,
    user_address: Address,
    simulate: bool,
) -> Result<OdosAssembleResponse> {
    let client = reqwest::Client::new();

    let request = OdosAssembleRequest {
        user_addr: user_address.to_string(),
        path_id: quote.path_id.clone(),
        simulate,
    };

    let response = client
        .post("https://api.odos.xyz/sor/assemble")
        .json(&request)
        .send()
        .await?;

    // Get the raw response text
    let response_text = response.text().await?;

    // Try to parse the response, if it fails print the raw response
    match serde_json::from_str::<OdosAssembleResponse>(&response_text) {
        Ok(parsed) => Ok(parsed),
        Err(e) => {
            println!("Failed to parse Odos assemble response: {}", e);
            println!("Raw response: {}", response_text);
            Err(eyre::eyre!("Failed to parse Odos assemble response"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use addressbook::Addressbook;
    use alloy::hex::hex;
    use alloy::{network::EthereumWallet, signers::local::PrivateKeySigner};
    use alloy::{
        network::{NetworkWallet, TransactionBuilder},
        providers::Provider,
    };
    use alloy_chains::{Chain, NamedChain};
    use alloy_primitives::Bytes;
    use alloy_rpc_types::TransactionRequest;
    use executor_binding::{ierc20::IERC20, weth::WETH};
    use provider::{get_default_anvil_provider, get_default_anvil_signer};
    use shared::token_helpers::{get_token_allowance, get_token_balance, parse_token_units};
    use types::token::TokenIsh;

    use super::*;

    #[tokio::test]
    async fn test_get_odos_quote() {
        let chain = NamedChain::Arbitrum;
        let addressbook = Addressbook::load().unwrap();
        let anvil_signer = get_default_anvil_signer();
        let user_address = anvil_signer.address();
        let weth = addressbook.get_weth(&chain).unwrap();
        let usdc = addressbook.get_usdc(&chain).unwrap();
        let input_amount = U256::from(1000000000000000000u128); // 1 WETH
        let slippage = 1.0;

        let response = get_odos_quote(chain, weth, input_amount, usdc, user_address, slippage)
            .await
            .unwrap();

        assert!(!response.in_tokens.is_empty());
        assert!(!response.out_tokens.is_empty());
        assert!(!response.in_amounts.is_empty());
        assert!(!response.out_amounts.is_empty());

        // Verify output amount is non-zero (should be ~$2000-3000 worth of USDC)
        let output_amount =
            U256::from_str_radix(&response.out_amounts[0].replace("0x", ""), 16).unwrap();
        assert!(output_amount > U256::ZERO);
    }

    #[tokio::test]
    async fn test_assemble_odos_swap() {
        // First get a quote
        let chain = NamedChain::Arbitrum;
        let addressbook = Addressbook::load().unwrap();
        let anvil_signer = get_default_anvil_signer();
        let user_address = anvil_signer.address();
        let weth = addressbook.get_weth(&chain).unwrap();
        let usdc = addressbook.get_usdc(&chain).unwrap();
        let input_amount = U256::from(1000000000000000000u128); // 1 WETH
        let slippage = 0.5;

        let quote = get_odos_quote(chain, weth, input_amount, usdc, user_address, slippage)
            .await
            .unwrap();
        let response = assemble_odos_swap(&quote, user_address, false)
            .await
            .unwrap();
        assert!(!response.transaction.data.is_empty());
        assert!(!response.transaction.to.is_empty());
        assert!(response.transaction.gas > 1000000);
    }

    #[tokio::test]
    async fn send_odos_swap() {
        dotenv::dotenv().ok();
        let chain = NamedChain::Base;
        let addressbook = Addressbook::load().unwrap();
        let anvil_signer = get_default_anvil_signer();
        let user_address = anvil_signer.address();
        let anvil_provider = get_default_anvil_provider().await;
        // let providers = get_provider_map().await;
        // let anvil_provider: &std::sync::Arc<alloy::providers::fillers::FillProvider<alloy::providers::fillers::JoinFill<alloy::providers::fillers::JoinFill<alloy::providers::Identity, alloy::providers::fillers::JoinFill<alloy::providers::fillers::GasFiller, alloy::providers::fillers::JoinFill<alloy::providers::fillers::BlobGasFiller, alloy::providers::fillers::JoinFill<alloy::providers::fillers::NonceFiller, alloy::providers::fillers::ChainIdFiller>>>>, alloy::providers::fillers::WalletFiller<EthereumWallet>>, alloy::providers::RootProvider<alloy::transports::BoxTransport>, alloy::transports::BoxTransport, alloy::network::Ethereum>> = providers.get(&chain).unwrap();
        // let signer = get_default_wallet().default_signer();
        // let user_address = signer.address();
        let weth = addressbook.get_weth(&chain).unwrap();
        let usdc = addressbook.get_usdc(&chain).unwrap();
        let input_amount = parse_token_units(&chain, &TokenIsh::Address(weth), "0.001")
            .await
            .unwrap();
        let slippage = 2.0;
        let router = Address::from_str("0x19cEeAd7105607Cd444F5ad10dd51356436095a1").unwrap();

        let call = WETH::depositCall {};
        let encoded = call.abi_encode();

        let wrap_eth_tx = TransactionRequest::default()
            .with_to(weth)
            .with_input(Bytes::from(encoded))
            .with_value(input_amount);

        let pending_wrap_eth_tx = anvil_provider.send_transaction(wrap_eth_tx).await.unwrap();
        pending_wrap_eth_tx.get_receipt().await.unwrap();

        let approve_tx = IERC20::approveCall {
            spender: router,
            value: input_amount,
        };
        let encoded = approve_tx.abi_encode();
        let approve_tx = TransactionRequest::default()
            .with_to(weth)
            .with_input(Bytes::from(encoded));

        let pending_approve_tx = anvil_provider.send_transaction(approve_tx).await.unwrap();

        pending_approve_tx.get_receipt().await.unwrap();

        let quote = get_odos_quote(chain, weth, input_amount, usdc, user_address, slippage)
            .await
            .unwrap();

        let assemble_response = assemble_odos_swap(&quote, user_address, false)
            .await
            .unwrap();

        let input = hex::decode(
            assemble_response
                .transaction
                .data
                .strip_prefix("0x")
                .unwrap(),
        )
        .unwrap();

        let tx = TransactionRequest::default()
            .with_from(user_address)
            .with_to(Address::from_str(&assemble_response.transaction.to).unwrap())
            .with_value(U256::from_str(&assemble_response.transaction.value).unwrap())
            .with_gas_price(assemble_response.transaction.gas_price as u128)
            .with_gas_limit(assemble_response.transaction.gas)
            .with_input(input)
            .with_chain_id(Chain::from_named(chain).id());

        let pending_tx = anvil_provider.send_transaction(tx).await.unwrap();
        let receipt = pending_tx.get_receipt().await.unwrap();
        assert_eq!(receipt.status(), true);
    }
}
