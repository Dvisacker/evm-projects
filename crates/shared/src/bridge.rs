use alloy::providers::{DynProvider, Provider};
use alloy::{hex, sol};
use alloy_chains::NamedChain;
use alloy_primitives::{Address, U256};
use alloy_rpc_types::TransactionRequest;
use eyre::{eyre, Context, Result};

use reqwest;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use types::bridge::BridgeName;

sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);
        function balanceOf(address owner) external view returns (uint256);
    }
}

#[derive(Debug, Serialize)]
pub struct LiFiQuoteRequest {
    #[serde(rename = "chainId")]
    pub chain_id: u64,
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
    pub slippage: String,
    #[serde(rename = "allowBridges")]
    pub allow_bridges: String,
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

pub async fn bridge_lifi<P>(
    origin_chain_provider: Arc<P>,
    destination_chain_provider: Arc<P>,
    from_chain: &NamedChain,
    to_chain: &NamedChain,
    from_token_address: Address,
    to_token_address: Address,
    amonut_in: U256,
    from_address: Address,
    to_address: Address,
    bridge_name: BridgeName,
) -> Result<U256>
where
    P: Provider,
{
    let client = reqwest::Client::new();
    let from_token = IERC20::new(from_token_address, origin_chain_provider.clone());
    let to_token = IERC20::new(to_token_address, destination_chain_provider.clone());
    let from_chain_id: u64 = (*from_chain).into();
    let to_chain_id: u64 = (*to_chain).into();

    let quote_request = LiFiQuoteRequest {
        chain_id: from_chain_id,
        from_chain: from_chain.to_string(),
        to_chain: to_chain.to_string(),
        from_token: from_token_address.to_string(),
        to_token: to_token_address.to_string(),
        from_amount: amonut_in.to_string(),
        from_address: from_address.to_string(),
        to_address: to_address.to_string(),
        slippage: "0.10".to_string(),
        order: "FASTEST".to_string(),
        allow_bridges: bridge_name.to_string(),
    };

    let response = client
        .get("https://li.quest/v1/quote")
        .query(&quote_request)
        .send()
        .await
        .wrap_err("Failed to get quote from Li.Fi API")?;

    println!("Response: {:?}", response);

    let quote_response: LiFiQuoteResponse = response.json().await?;
    let bridge_address = Address::from_str(&quote_response.transaction_request.to)?;

    // 2. Approve tokens to bridge
    let approve_tx = from_token.approve(bridge_address, amonut_in).send().await?;
    let _approve_receipt = approve_tx.get_receipt().await.unwrap();

    // 3. Execute the bridge transaction
    let data = hex::decode(&quote_response.transaction_request.data[2..])
        .wrap_err("Failed to decode transaction data")?;
    let contract_address = Address::from_str(&quote_response.transaction_request.to)
        .wrap_err("Failed to parse 'to' address")?;
    let value = U256::from_str(&quote_response.transaction_request.value)
        .wrap_err("Failed to parse transaction value")?;
    let gas_limit = U256::from_str(&quote_response.transaction_request.gas_limit)
        .wrap_err("Failed to parse gas limit")?;
    let gas_price = U256::from_str(&quote_response.transaction_request.gas_price)
        .wrap_err("Failed to parse gas price")?;

    let to_token_balance_before = match to_token.balanceOf(to_address).call().await {
        Ok(balance) => balance._0,
        Err(e) => return Err(eyre!("Error getting balance before: {:?}", e)),
    };

    let tx_request = TransactionRequest::default()
        .to(contract_address)
        .input(data.into())
        .value(value)
        .gas_limit(gas_limit.try_into().unwrap())
        .max_fee_per_gas(gas_price.try_into().unwrap())
        .max_priority_fee_per_gas(gas_price.try_into().unwrap());

    let pending_tx = origin_chain_provider.send_transaction(tx_request).await?;
    let receipt = pending_tx.get_receipt().await?;

    println!("Receipt: {:?}", receipt.transaction_hash);

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // 4. Monitor bridge status
    let mut status = "PENDING".to_string();
    while status == "PENDING" || status == "UNKNOWN" {
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

        println!("Status response: {:?}", status_response);

        status = status_response["status"]
            .as_str()
            .unwrap_or("UNKNOWN")
            .to_string();

        println!("Status: {:?}", status);

        if status == "FAILED" {
            return Err(eyre!("Bridge transaction failed"));
        }

        if status == "PENDING" || status == "UNKNOWN" {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }

    println!("Sleeping for 5 seconds");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let to_token_balance_after = match to_token.balanceOf(to_address).call().await {
        Ok(balance) => balance._0,
        Err(e) => return Err(eyre!("Error getting balance after: {:?}", e)),
    };
    let amount_out = to_token_balance_after - to_token_balance_before;

    Ok(amount_out)
}

#[cfg(test)]
mod tests {
    use crate::token_helpers::parse_token_units;

    use super::*;
    use addressbook::Addressbook;
    use alloy::{network::EthereumWallet, signers::local::PrivateKeySigner};
    use alloy_chains::{Chain, NamedChain};
    use provider::{get_default_signer, get_provider_map, get_signer_provider};
    use types::token::{NamedToken, TokenIsh};

    #[tokio::test]
    async fn test_bridge_usdc_arbitrum_to_base() -> Result<()> {
        dotenv::dotenv().ok();

        let addressbook = Addressbook::load().unwrap();
        let signer: PrivateKeySigner = get_default_signer();
        let wallet_address = signer.address();
        let provider_map = get_provider_map().await;
        let origin_provider = provider_map.get(&NamedChain::Arbitrum).unwrap();
        let destination_provider = provider_map.get(&NamedChain::Base).unwrap();
        let from_address = wallet_address;
        let to_address = wallet_address;
        let usdc_arb = addressbook
            .get_token(&NamedChain::Arbitrum, &NamedToken::USDC)
            .unwrap();
        let usdc_base = addressbook
            .get_token(&NamedChain::Base, &NamedToken::USDC)
            .unwrap();
        let from_chain = NamedChain::Arbitrum;
        let to_chain = NamedChain::Base;
        let bridge_name = BridgeName::Accross;

        // Amount to bridge (e.g., 1 USDC = 1_000_000 because USDC has 6 decimals)
        let amount = U256::from(1_000_000u64);

        let result = bridge_lifi(
            Arc::new(origin_provider.clone()),
            Arc::new(destination_provider.clone()),
            &from_chain,
            &to_chain,
            usdc_arb,
            usdc_base,
            amount,
            from_address,
            to_address,
            bridge_name,
        )
        .await?;

        println!(
            "Bridge transaction completed. Expected output amount: {}",
            result
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_bridge_weth_arbitrum_to_base() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load().unwrap();
        let signer: PrivateKeySigner = get_default_signer();
        let wallet_address = signer.address();
        let wallet = EthereumWallet::new(signer);
        let origin_provider =
            get_signer_provider(Chain::from_named(NamedChain::Arbitrum), wallet.clone()).await;
        let destination_provider =
            get_signer_provider(Chain::from_named(NamedChain::Base), wallet.clone()).await;
        let from_address = wallet_address;
        let to_address = wallet_address;
        let weth_arb = addressbook.get_weth(&NamedChain::Arbitrum).unwrap();
        let weth_base = addressbook.get_weth(&NamedChain::Base).unwrap();
        let from_chain = NamedChain::Arbitrum;
        let to_chain = NamedChain::Base;
        let bridge_name = BridgeName::StargateV2;
        let amount = parse_token_units(&from_chain, &TokenIsh::Address(weth_arb), "0.0004").await?;

        let result = bridge_lifi(
            Arc::new(origin_provider),
            Arc::new(destination_provider),
            &from_chain,
            &to_chain,
            weth_arb,
            weth_base,
            amount,
            from_address,
            to_address,
            bridge_name,
        )
        .await?;

        println!(
            "Bridge transaction completed. Expected output amount: {}",
            result
        );
        Ok(())
    }
}
