use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct LiFiQuoteRequest {
    #[serde(rename = "fromChain")]
    pub from_chain: String,
    #[serde(rename = "toChain")]
    pub to_chain: String,
    #[serde(rename = "fromToken")]
    pub from_token: String,
    #[serde(rename = "toToken")]
    pub to_token: String,
    #[serde(rename = "fromAmount")]
    pub from_amount: String,
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    #[serde(rename = "toAddress")]
    pub to_address: String,
    #[serde(rename = "slippage")]
    pub slippage: String,
    #[serde(rename = "allowBridges")]
    pub allow_bridges: String,
    #[serde(rename = "order")]
    pub order: String,
}

#[derive(Debug, Deserialize)]
pub struct LiFiQuoteResponse {
    // pub id: String,
    // #[serde(rename = "type")]
    // pub quote_type: String,
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
