use alloy::network::{Ethereum, Network};
use alloy::providers::Provider;
use alloy::sol;
use alloy::transports::Transport;
use alloy_chains::NamedChain;
use alloy_primitives::{Address, Bytes, FixedBytes, TxHash, U256};
use alloy_sol_types::sol_data::Bytes as SolBytes;
use alloy_sol_types::{SolCall, SolValue};
use bindings::iuniswapv3pool::IUniswapV3Pool;
use executor_binding::batchexecutor::BatchExecutor::{
    singlecallCall, BatchExecutorCalls, BatchExecutorInstance,
};
use executor_binding::erc20::ERC20;
use executor_binding::ipool::IPool as IAaveV3Pool;
use executor_binding::iuniswapv2router::IUniswapV2Router;
use executor_binding::iuniswapv3router::IUniswapV3Router::{
    self, ExactInputSingleParams, ExactOutputSingleParams,
};
use executor_binding::weth::WETH;
use eyre::Result;
use types::exchange::ExchangeName;

use crate::addressbook::Addressbook;

sol! {
    #[allow(missing_docs)]
    #[derive(Debug, PartialEq, Eq)]
    struct FallbackData {
        bytes[] callback_params;
        bytes return_data;
    }
}

// a calldata enum that can be either a swap or a multicall
pub struct CallbackContext {
    sender: Address,
    data_index: u64,
}

pub struct AaveV3FlashLoanParams {
    pub receiver_address: Address,
    pub asset: Address,
    pub amount: U256,
    pub params: Bytes,
    pub referral_code: u64,
}

pub struct ContractAddresses {
    pub aave_v3_pool_address: Address,
    pub uniswap_v3_router_address: Address,
    pub uniswap_v2_router_address: Address,
}

pub struct BatchExecutorClient<T, P>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum>,
{
    executor: BatchExecutorInstance<T, P>,
    calldata: Vec<Bytes>,
    addresses: ContractAddresses,
}

impl<T, P> BatchExecutorClient<T, P>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum>,
{
    pub fn new(address: Address, chain: NamedChain, provider: P) -> Self {
        let executor = BatchExecutorInstance::new(address, provider);
        let addressbook = Addressbook::load(None).unwrap();

        let aave_v3_pool_address = addressbook
            .get_lending_pool(&chain, "aave_v3")
            .expect("Aave v3 pool not found");

        let uniswap_v3_router_address = addressbook
            .get_uni_v3_swap_router(&chain, ExchangeName::UniswapV3)
            .expect("Uniswap v3 router not found");

        let uniswap_v2_router_address = addressbook
            .get_uni_v2_swap_router(&chain, ExchangeName::UniswapV2)
            .expect("Uniswap v2 router not found");

        Self {
            executor,
            calldata: Vec::new(),
            addresses: ContractAddresses {
                aave_v3_pool_address,
                uniswap_v3_router_address,
                uniswap_v2_router_address,
            },
        }
    }

    // BASIC CALLS

    pub fn add_approve_erc20(
        &mut self,
        token: Address,
        spender: Address,
        value: U256,
    ) -> &mut Self {
        let call = ERC20::approveCall { spender, value };
        let encoded = call.abi_encode();

        self.add_call(token, U256::ZERO, Bytes::from(encoded), None);

        self
    }

    pub fn add_wrap_eth(&mut self, weth: Address, amount: U256) -> &mut Self {
        let call = WETH::depositCall {};
        let encoded = call.abi_encode();

        self.add_call(weth, amount, Bytes::from(encoded), None);

        self
    }

    pub fn add_unwrap_eth(&mut self, weth: Address, amount: U256) -> &mut Self {
        let call = WETH::withdrawCall { amount };
        let encoded = call.abi_encode();

        self.add_call(weth, amount, Bytes::from(encoded), None);

        self
    }

    pub fn add_transfer_erc20(
        &mut self,
        token: Address,
        recipient: Address,
        value: U256,
    ) -> &mut Self {
        let call = ERC20::transferCall {
            to: recipient,
            value,
        };
        let encoded = call.abi_encode();

        self.add_call(token, U256::ZERO, Bytes::from(encoded), None);

        self
    }

    pub fn add_transfer_from_erc20(
        &mut self,
        token: Address,
        owner: Address,
        recipient: Address,
        value: U256,
    ) -> &mut Self {
        let call = ERC20::transferFromCall {
            from: owner,
            to: recipient,
            value,
        };
        let encoded = call.abi_encode();

        self.add_call(token, U256::ZERO, Bytes::from(encoded), None);

        self
    }

    // AAVE V3

    pub fn add_aave_v3_supply(&mut self, asset: Address, amount: U256) -> &mut Self {
        let call = IAaveV3Pool::supplyCall {
            asset,
            amount,
            onBehalfOf: *self.executor.address(),
            referralCode: 0,
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        self
    }

    pub fn add_aave_v3_borrow(&mut self, asset: Address, amount: U256) -> &mut Self {
        let call = IAaveV3Pool::borrowCall {
            asset,
            amount,
            onBehalfOf: *self.executor.address(),
            interestRateMode: U256::from(1),
            referralCode: 0,
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        self
    }

    pub fn add_aave_v3_repay(&mut self, asset: Address, amount: U256) -> &mut Self {
        let call = IAaveV3Pool::repayCall {
            asset,
            amount,
            interestRateMode: U256::from(1),
            onBehalfOf: *self.executor.address(),
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        self
    }

    pub fn add_aave_v3_withdraw(&mut self, asset: Address, amount: U256) -> &mut Self {
        let call = IAaveV3Pool::withdrawCall {
            asset,
            amount,
            to: *self.executor.address(),
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        self
    }

    pub fn add_aave_v3_liquidate(
        &mut self,
        collateral: Address,
        debt: Address,
        user: Address,
        amount: U256,
    ) -> &mut Self {
        let call = IAaveV3Pool::liquidationCallCall {
            collateralAsset: collateral,
            debtAsset: debt,
            user,
            debtToCover: amount,
            receiveAToken: false,
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        self
    }

    // UNISWAP V3

    pub fn add_uniswap_v3_exact_input(&mut self, swap: ExactInputSingleParams) -> &mut Self {
        let call = IUniswapV3Router::exactInputSingleCall { params: swap };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.uniswap_v3_router_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        self
    }

    pub fn add_uniswap_v3_exact_output(&mut self, swap: ExactOutputSingleParams) -> &mut Self {
        let call = IUniswapV3Router::exactOutputSingleCall { params: swap };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.uniswap_v3_router_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        self
    }

    // UNISWAP V2

    pub fn add_uniswap_v2_swap(
        &mut self,
        amount_in: U256,
        token_in: Address,
        token_out: Address,
        deadline: U256,
    ) -> &mut Self {
        let call = IUniswapV2Router::swapExactTokensForTokensCall {
            amountIn: amount_in,
            amountOutMin: U256::ZERO,
            path: vec![token_in, token_out],
            to: *self.executor.address(),
            deadline,
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.uniswap_v2_router_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        self
    }

    // FLASH LOANS
    pub fn add_aave_v3_flash_loan(
        &mut self,
        receiver_address: Address,
        asset: Address,
        amount: U256,
        premium: U256,
        callbacks: Vec<Bytes>,
    ) -> &mut Self {
        // This consists of 1) The array of callback calls calldata and 2) The callback return value (specific to each flashloan)
        let approve_premium_call_data =
            self.build_approve_erc20(asset, self.addresses.aave_v3_pool_address, premium);

        let callback_params = [&callbacks[..], &[approve_premium_call_data]].concat();
        let return_data = Bytes::from(FixedBytes::<32>::left_padding_from(&[1u8]));

        let params = FallbackData {
            callback_params,
            return_data,
        };

        let flash_loan_call = IAaveV3Pool::flashLoanCall {
            receiverAddress: receiver_address,
            assets: vec![asset],
            amounts: vec![amount],
            interestRateModes: vec![U256::from(1)],
            onBehalfOf: receiver_address,
            params: Bytes::from(params.abi_encode()),
            referralCode: 0,
        };
        let encoded = flash_loan_call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        self
    }

    pub fn add_uniswap_v2_flash_swap(
        &mut self,
        pool: Address,
        assets: [Address; 2],
        amounts: [U256; 2],
        callbacks: Vec<Bytes>,
    ) -> &mut Self {
        // Unpack assets and amounts
        let [asset0, asset1] = assets;
        let [amount0, amount1] = amounts;

        // This consists of callback calls and the callback return value
        let approve0 = self.build_approve_erc20(asset0, pool, amount0);
        let approve1 = self.build_approve_erc20(asset1, pool, amount1);

        let callback_params = [&callbacks[..], &[approve0, approve1]].concat();
        let return_data = Bytes::default(); // Empty bytes - "0x"

        let params = FallbackData {
            callback_params,
            return_data,
        };

        // Create the flash call
        let flash_call = IUniswapV3Pool::flashCall {
            recipient: *self.executor.address(),
            amount0: amount0,
            amount1: amount1,
            data: Bytes::from(params.abi_encode()),
        };
        let encoded = flash_call.abi_encode();

        // Add the call with the appropriate context
        self.add_call(
            pool,
            U256::ZERO,
            Bytes::from(encoded),
            Some(CallbackContext {
                sender: pool,
                data_index: 3, // uniswapV2Call(address,uint256,uint256,bytes)
            }),
        );

        self
    }

    pub fn add_uniswap_v3_flash_loan(
        &mut self,
        pool: Address,
        assets: [Address; 2],
        amounts: [U256; 2],
        fee: U256,
        callbacks: Vec<Bytes>,
    ) -> &mut Self {
        // Unpack assets and amounts
        let [asset0, asset1] = assets;
        let [amount0, amount1] = amounts;

        // (fee in basis points, e.g. 3000 = 0.3%)
        // TODO
        let fee0 = U256::from(0);
        let fee1 = U256::from(0);
        // let fee0 = amount0.mul(fee).div(U256::from(1_000_000));
        // let fee1 = amount1.mul(fee).div(U256::from(1_000_000));

        // Transfer calls for repayment
        let transfer0 = self.build_transfer_erc20(asset0, pool, amount0 + fee0);
        let transfer1 = self.build_transfer_erc20(asset1, pool, amount1 + fee1);

        let callback_params = [&callbacks[..], &[transfer0, transfer1]].concat();
        let return_data = Bytes::default();

        let params = FallbackData {
            callback_params,
            return_data,
        };

        let flash_call = IUniswapV3Pool::flashCall {
            recipient: *self.executor.address(),
            amount0: amount0,
            amount1: amount1,
            data: Bytes::from(params.abi_encode()),
        };
        let encoded = flash_call.abi_encode();

        // Add the call with the appropriate context
        self.add_call(
            pool,
            U256::ZERO,
            Bytes::from(encoded),
            Some(CallbackContext {
                sender: pool,
                data_index: 2, // uniswapV3FlashCallback(uint256,uint256,bytes)
            }),
        );

        self
    }

    // INTERNAL

    pub fn encoded_context(&self, context: CallbackContext) -> Result<FixedBytes<32>> {
        let data_index = context.data_index.to_be_bytes();
        let padded_data_index = FixedBytes::<12>::left_padding_from(&data_index);
        let sender = FixedBytes::<20>::from(context.sender);
        let context = padded_data_index.concat_const(sender);

        Ok(context)
    }

    pub fn build_call(
        &mut self,
        target: Address,
        value: U256,
        calldata: Bytes,
        context: Option<CallbackContext>,
    ) -> Bytes {
        let context = self.encoded_context(context.unwrap()).unwrap();
        let single_call = singlecallCall {
            target,
            value,
            context,
            callData: calldata,
        };

        return Bytes::from(single_call.abi_encode());
    }

    pub fn build_approve_erc20(
        &mut self,
        asset: Address,
        recipient: Address,
        amount: U256,
    ) -> Bytes {
        let call = ERC20::approveCall {
            spender: recipient,
            value: amount,
        };
        return self.build_call(asset, U256::ZERO, Bytes::from(call.abi_encode()), None);
    }

    pub fn build_transfer_erc20(
        &mut self,
        asset: Address,
        recipient: Address,
        amount: U256,
    ) -> Bytes {
        let call = ERC20::transferCall {
            to: recipient,
            value: amount,
        };
        return self.build_call(asset, U256::ZERO, Bytes::from(call.abi_encode()), None);
    }

    pub fn add_call(
        &mut self,
        target: Address,
        value: U256,
        calldata: Bytes,
        context: Option<CallbackContext>,
    ) {
        let context = self.encoded_context(context.unwrap()).unwrap();

        let single_call = singlecallCall {
            target,
            value,
            context,
            callData: calldata,
        };

        let encoded = Bytes::from(single_call.abi_encode());
        self.calldata.push(encoded);
    }

    pub fn get(&mut self) -> Vec<Bytes> {
        let calldata = self.calldata.clone();
        self.calldata.clear();
        calldata
    }

    pub async fn exec(&mut self) -> Result<(bool, TxHash)> {
        let calldata = self.get();
        let call = self.executor.batchCall(calldata);
        let pending_tx = call.send().await?;
        let receipt = pending_tx.get_receipt().await?;
        let status = receipt.status();
        let tx_hash = receipt.transaction_hash;

        Ok((status, tx_hash))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        helpers::{compute_v2_pool_address, compute_v3_pool_address, parse_token_units},
        provider::{
            get_default_anvil_provider, get_default_anvil_signer, get_default_signer, get_provider,
        },
    };

    use super::*;
    use alloy::{network::EthereumWallet, signers::local::PrivateKeySigner};
    use alloy_chains::Chain;
    use alloy_primitives::{aliases::U24, U160, U256};
    use std::{env, str::FromStr};
    use types::token::TokenIsh;
    use IUniswapV3Router::{exactInputSingleCall, IUniswapV3RouterCalls};

    const CHAIN: NamedChain = NamedChain::Base;

    #[tokio::test]
    async fn test_swap_usdc_for_dai_via_uniswap_v3() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load(None).unwrap();
        let weth = addressbook.get_weth(&CHAIN).unwrap();
        let usdc = addressbook.get_usdc(&CHAIN).unwrap();
        let uni_v3_router = addressbook
            .get_uni_v3_universal_router(&CHAIN, ExchangeName::UniswapV3)
            .unwrap();
        let provider = get_default_anvil_provider().await;

        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider);
        println!("HERE");

        // Amount of USDC to swap (1,000 USDC = 1_000_000_000 considering 6 decimals)
        let amount = parse_token_units(&CHAIN, &TokenIsh::Address(usdc), "1")
            .await
            .unwrap();

        encoder
            .add_approve_erc20(usdc, uni_v3_router, U256::MAX)
            .add_uniswap_v3_exact_input(ExactInputSingleParams {
                tokenIn: usdc,
                tokenOut: weth,
                fee: U24::from(500u32),
                recipient: executor_address,
                amountIn: amount,
                amountOutMinimum: U256::ZERO,
                sqrtPriceLimitX96: U160::ZERO,
            })
            .exec()
            .await?;

        let (success, _tx_hash) = encoder.exec().await?;
        assert!(success, "Transaction failed");

        Ok(())
    }

    #[tokio::test]
    async fn test_aave_v3_flash_loan() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load(None).unwrap();
        let aave_v3_pool = addressbook.get_lending_pool(&CHAIN, "aave_v3").unwrap();
        let provider = get_default_anvil_provider().await;

        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone());

        let usdc = addressbook.get_usdc(&CHAIN).unwrap();
        let aave_pool_address = addressbook.get_lending_pool(&CHAIN, "aave_v3").unwrap();
        let amount = parse_token_units(&CHAIN, &TokenIsh::Address(usdc), "1")
            .await
            .unwrap();
        let aave_pool = IAaveV3Pool::new(aave_pool_address, provider.clone());
        let premium = aave_pool.FLASHLOAN_PREMIUM_TOTAL().call().await.unwrap()._0;

        let callbacks = vec![];

        encoder
            .add_aave_v3_flash_loan(
                executor_address,
                aave_v3_pool,
                amount,
                U256::from(premium),
                callbacks,
            )
            .exec()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_uniswap_v3_flash_loan() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load(None).unwrap();
        let provider = get_default_anvil_provider().await;

        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone());

        let usdc = addressbook.get_usdc(&CHAIN).unwrap();
        let weth = addressbook.get_weth(&CHAIN).unwrap();
        let amount = parse_token_units(&CHAIN, &TokenIsh::Address(usdc), "1")
            .await
            .unwrap();

        let pool_address =
            compute_v3_pool_address(&CHAIN, ExchangeName::UniswapV3, usdc, weth, 500)?;

        let assets = [usdc, weth];
        let amounts = [amount, U256::ZERO];
        let fee = U256::from(500u32);
        let callbacks = vec![];

        encoder
            .add_uniswap_v3_flash_loan(pool_address, assets, amounts, fee, callbacks)
            .exec()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_aave_v3_flashloan_with_callbacks() -> Result<()> {
        let addressbook = Addressbook::load(None).unwrap();
        let aave_v3_pool = addressbook.get_lending_pool(&CHAIN, "aave_v3").unwrap();
        let signer: PrivateKeySigner = get_default_signer();
        let wallet = EthereumWallet::new(signer);
        let provider = get_provider(Chain::from_named(CHAIN), wallet.clone()).await;

        let executor_address = Address::from_str("EXECUTOR_ADDRESS").unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone());

        let usdc = addressbook.get_usdc(&CHAIN).unwrap();
        let weth = addressbook.get_weth(&CHAIN).unwrap();
        let aave_pool_address = addressbook.get_lending_pool(&CHAIN, "aave_v3").unwrap();
        let usdc_amount = parse_token_units(&CHAIN, &TokenIsh::Address(usdc), "1000")
            .await
            .unwrap();

        let borrowed_weth_amount = parse_token_units(&CHAIN, &TokenIsh::Address(weth), "0.01")
            .await
            .unwrap();
        let aave_pool = IAaveV3Pool::new(aave_pool_address, provider.clone());
        let premium = aave_pool.FLASHLOAN_PREMIUM_TOTAL().call().await.unwrap()._0;

        let callbacks = encoder
            .add_approve_erc20(usdc, executor_address, usdc_amount)
            .add_aave_v3_supply(usdc, usdc_amount)
            .add_aave_v3_borrow(weth, borrowed_weth_amount)
            .add_unwrap_eth(weth, borrowed_weth_amount)
            .add_wrap_eth(weth, borrowed_weth_amount)
            .add_approve_erc20(weth, aave_pool_address, borrowed_weth_amount)
            .add_aave_v3_repay(weth, borrowed_weth_amount)
            .add_aave_v3_withdraw(usdc, usdc_amount)
            .get();

        encoder
            .add_aave_v3_flash_loan(
                executor_address,
                aave_v3_pool,
                usdc_amount,
                U256::from(premium),
                callbacks,
            )
            .exec()
            .await?;

        Ok(())
    }
}
