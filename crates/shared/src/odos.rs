use alloy_chains::{Chain, NamedChain};
use alloy_primitives::{Address, U256};
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
    slippage: f64,
    #[serde(rename = "userAddr")]
    user_addr: String,
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
        slippage,
        user_addr: user_address.to_string(),
    };

    println!("{:?}", request);

    let response = client
        .post("https://api.odos.xyz/sor/quote/v2")
        .json(&request)
        .send()
        .await?;

    // Get the raw response text
    let response_text = response.text().await?;

    // Try to parse the response, if it fails print the raw response
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
) -> Result<OdosAssembleResponse> {
    let client = reqwest::Client::new();

    let request = OdosAssembleRequest {
        user_addr: format!("0x{:x}", user_address),
        path_id: quote.path_id.clone(),
        simulate: false,
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
