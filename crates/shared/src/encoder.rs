use alloy::network::{Ethereum, Network};
use alloy::providers::Provider;
use alloy::transports::Transport;
use alloy::{hex, sol};
use alloy_chains::NamedChain;
use alloy_primitives::{Address, Bytes, TxHash, U256};
use alloy_sol_types::{abi, SolCall};
use executor_binding::erc20::ERC20;
use executor_binding::executor::Executor::{
    multicallCall, Call, ExecutorCalls, ExecutorInstance, Swap, SwapData,
};
use executor_binding::ipool::IPool as IAaveV3Pool;
use executor_binding::iuniswapv2router::IUniswapV2Router;
use executor_binding::iuniswapv3router::IUniswapV3Router::{
    self, ExactInputSingleParams, ExactOutputSingleParams,
};
use executor_binding::weth::WETH;
use eyre::Result;
use types::exchange::ExchangeName;

use crate::addressbook::Addressbook;

// aave pool

// sol! {
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     interface AaveV3Pool {
//         function flashLoan(address receiverAddress, address asset, uint256 amount, bytes params, uint256 referralCode) public {}
//     }
// }

// a calldata enum that can be either a swap or a multicall
pub enum CallData {
    Swap(SwapData),
    Multicall(Vec<Call>),
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

pub struct ExecutorClient<T, P>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum>,
{
    executor: ExecutorInstance<T, P>,
    calldata: Option<CallData>,
    addresses: ContractAddresses,
}

impl<T, P> ExecutorClient<T, P>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum>,
{
    pub fn new(address: Address, chain: NamedChain, provider: P) -> Self {
        let executor = ExecutorInstance::new(address, provider);
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
            calldata: None,
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

        self.push_multicall_call_data(vec![Call {
            target: token,
            data: encoded.into(),
            value: U256::ZERO,
        }]);

        Ok(())
    }

    pub fn add_wrap_eth(&mut self, weth: Address, amount: U256) -> Result<()> {
        let call = WETH::depositCall {};
        let encoded = call.abi_encode();

        self.push_multicall_call_data(vec![Call {
            target: weth,
            data: encoded.into(),
            value: amount,
        }]);

        Ok(())
    }

    pub fn add_unwrap_eth(&mut self, weth: Address, amount: U256) -> Result<()> {
        let call = WETH::withdrawCall { amount };
        let encoded = call.abi_encode();

        self.push_multicall_call_data(vec![Call {
            target: weth,
            data: encoded.into(),
            value: amount,
        }]);

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

        self.push_multicall_call_data(vec![Call {
            target: token,
            data: encoded.into(),
            value: U256::ZERO,
        }]);

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

        self.push_multicall_call_data(vec![Call {
            target: token,
            data: encoded.into(),
            value: U256::ZERO,
        }]);

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

        self.push_multicall_call_data(vec![Call {
            target: self.addresses.aave_v3_pool_address,
            data: encoded.into(),
            value: U256::ZERO,
        }]);

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

        self.push_multicall_call_data(vec![Call {
            target: self.addresses.aave_v3_pool_address,
            data: encoded.into(),
            value: U256::ZERO,
        }]);

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

        self.push_multicall_call_data(vec![Call {
            target: self.addresses.aave_v3_pool_address,
            data: encoded.into(),
            value: U256::ZERO,
        }]);

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

        self.push_multicall_call_data(vec![Call {
            target: self.addresses.aave_v3_pool_address,
            data: encoded.into(),
            value: U256::ZERO,
        }]);

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

        self.push_multicall_call_data(vec![Call {
            target: self.addresses.aave_v3_pool_address,
            data: encoded.into(),
            value: U256::ZERO,
        }]);

        Ok(())
    }

    // UNISWAP V3

    pub fn add_uniswap_v3_exact_input(&mut self, swap: ExactInputSingleParams) -> Result<()> {
        let call = IUniswapV3Router::exactInputSingleCall { params: swap };
        let encoded = call.abi_encode();

        self.push_multicall_call_data(vec![Call {
            target: self.addresses.uniswap_v3_router_address,
            data: encoded.into(),
            value: U256::ZERO,
        }]);

        Ok(())
    }

    pub fn add_uniswap_v3_exact_output(&mut self, swap: ExactOutputSingleParams) -> Result<()> {
        let call = IUniswapV3Router::exactOutputSingleCall { params: swap };
        let encoded = call.abi_encode();

        self.push_multicall_call_data(vec![Call {
            target: self.addresses.uniswap_v3_router_address,
            data: encoded.into(),
            value: U256::ZERO,
        }]);

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

        self.push_multicall_call_data(vec![Call {
            target: self.addresses.uniswap_v2_router_address,
            data: encoded.into(),
            value: U256::ZERO,
        }]);

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

        self.push_multicall_call_data(vec![Call {
            target: self.addresses.aave_v3_pool_address,
            data: encoded.into(),
            value: U256::ZERO,
        }]);

        Ok(())
    }

    // INTERNAL

    pub fn push_call_data(&mut self, call_data: CallData) {
        match call_data {
            CallData::Swap(calldata) => self.push_swap_call_data(calldata),
            CallData::Multicall(calls) => self.push_multicall_call_data(calls),
        }
    }

    pub fn push_multicall_call_data(&mut self, calldata: Vec<Call>) {
        if let Some(CallData::Multicall(existing)) = self.calldata.as_mut() {
            existing.extend(calldata);
        } else {
            self.calldata = Some(CallData::Multicall(calldata));
        }
    }

    pub fn push_swap_call_data(&mut self, calldata: SwapData) {
        self.calldata = Some(CallData::Swap(calldata));
    }

    pub fn push_swap(&mut self, swap: Swap) {
        if let Some(CallData::Swap(calldata)) = self.calldata.as_mut() {
            calldata.swaps.push(swap);
        }
    }

    pub async fn execute_tx(&self) -> Result<(bool, TxHash)> {
        let calldata = self.calldata.as_ref().unwrap();
        let pending_tx = match calldata {
            CallData::Swap(calldata) => {
                let call = self.executor.swap(calldata.clone());
                let result = call.send().await?;
                result
            }
            CallData::Multicall(calldata) => {
                let call = self.executor.multicall(calldata.clone());
                let result = call.send().await?;
                result
            }
        };
        let receipt = pending_tx.get_receipt().await?;
        let status = receipt.status();
        let tx_hash = receipt.transaction_hash;

        Ok((status, tx_hash))
    }
}
