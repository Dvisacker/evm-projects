use alloy::network::{Ethereum, Network};
use alloy::providers::Provider;
use alloy::transports::Transport;
use alloy_chains::NamedChain;
use alloy_primitives::{Address, Bytes, FixedBytes, TxHash, U256};
use alloy_sol_types::SolCall;
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
    ) -> Result<()> {
        let call = ERC20::approveCall { spender, value };
        let encoded = call.abi_encode();

        self.add_call(token, U256::ZERO, Bytes::from(encoded), None);

        Ok(())
    }

    pub fn add_wrap_eth(&mut self, weth: Address, amount: U256) -> Result<()> {
        let call = WETH::depositCall {};
        let encoded = call.abi_encode();

        self.add_call(weth, amount, Bytes::from(encoded), None);

        Ok(())
    }

    pub fn add_unwrap_eth(&mut self, weth: Address, amount: U256) -> Result<()> {
        let call = WETH::withdrawCall { amount };
        let encoded = call.abi_encode();

        self.add_call(weth, amount, Bytes::from(encoded), None);

        Ok(())
    }

    pub fn add_transfer_erc20(
        &mut self,
        token: Address,
        recipient: Address,
        value: U256,
    ) -> Result<()> {
        let call = ERC20::transferCall {
            to: recipient,
            value,
        };
        let encoded = call.abi_encode();

        self.add_call(token, U256::ZERO, Bytes::from(encoded), None);

        Ok(())
    }

    pub fn add_transfer_from_erc20(
        &mut self,
        token: Address,
        owner: Address,
        recipient: Address,
        value: U256,
    ) -> Result<()> {
        let call = ERC20::transferFromCall {
            from: owner,
            to: recipient,
            value,
        };
        let encoded = call.abi_encode();

        self.add_call(token, U256::ZERO, Bytes::from(encoded), None);

        Ok(())
    }

    // AAVE V3

    pub fn add_aave_v3_supply(
        &mut self,
        asset: Address,
        amount: U256,
        on_behalf_of: Address,
    ) -> Result<()> {
        let call = IAaveV3Pool::supplyCall {
            asset,
            amount,
            onBehalfOf: on_behalf_of,
            referralCode: 0,
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        Ok(())
    }

    pub fn add_aave_v3_borrow(
        &mut self,
        asset: Address,
        amount: U256,
        on_behalf_of: Address,
    ) -> Result<()> {
        let call = IAaveV3Pool::borrowCall {
            asset,
            amount,
            onBehalfOf: on_behalf_of,
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

        Ok(())
    }

    pub fn add_aave_v3_repay(
        &mut self,
        asset: Address,
        amount: U256,
        on_behalf_of: Address,
    ) -> Result<()> {
        let call = IAaveV3Pool::repayCall {
            asset,
            amount,
            interestRateMode: U256::from(1),
            onBehalfOf: on_behalf_of,
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        Ok(())
    }

    pub fn add_aave_v3_withdraw(
        &mut self,
        asset: Address,
        amount: U256,
        to: Address,
    ) -> Result<()> {
        let call = IAaveV3Pool::withdrawCall { asset, amount, to };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        Ok(())
    }

    pub fn add_aave_v3_liquidate(
        &mut self,
        collateral: Address,
        debt: Address,
        user: Address,
        amount: U256,
    ) -> Result<()> {
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

        Ok(())
    }

    // UNISWAP V3

    pub fn add_uniswap_v3_exact_input(&mut self, swap: ExactInputSingleParams) -> Result<()> {
        let call = IUniswapV3Router::exactInputSingleCall { params: swap };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.uniswap_v3_router_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        Ok(())
    }

    pub fn add_uniswap_v3_exact_output(&mut self, swap: ExactOutputSingleParams) -> Result<()> {
        let call = IUniswapV3Router::exactOutputSingleCall { params: swap };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.uniswap_v3_router_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        Ok(())
    }

    // UNISWAP V2

    pub fn add_uniswap_v2_swap(
        &mut self,
        amount_in: U256,
        token_in: Address,
        token_out: Address,
        deadline: U256,
    ) -> Result<()> {
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

        Ok(())
    }

    // FLASH LOANS

    pub fn add_aave_v3_flash_loan(
        &mut self,
        receiver_address: Address,
        asset: Address,
        amount: U256,
        params: Bytes,
        referral_code: u64,
    ) -> Result<()> {
        let flash_loan_call = IAaveV3Pool::flashLoanCall {
            receiverAddress: receiver_address,
            assets: vec![asset],
            amounts: vec![amount],
            interestRateModes: vec![U256::from(1)],
            onBehalfOf: receiver_address,
            params,
            referralCode: referral_code.try_into()?,
        };
        let encoded = flash_loan_call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
        );

        Ok(())
    }

    // INTERNAL

    pub fn encoded_context(&self, context: CallbackContext) -> Result<FixedBytes<32>> {
        let data_index = context.data_index.to_be_bytes();
        let padded_data_index = FixedBytes::<12>::left_padding_from(&data_index);
        let sender = FixedBytes::<20>::from(context.sender);
        let context = padded_data_index.concat_const(sender);

        Ok(context)
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

    pub async fn exec(&self, calldata: Vec<Bytes>) -> Result<(bool, TxHash)> {
        let call = self.executor.batchCall(calldata);
        let pending_tx = call.send().await?;
        let receipt = pending_tx.get_receipt().await?;
        let status = receipt.status();
        let tx_hash = receipt.transaction_hash;

        Ok((status, tx_hash))
    }
}
