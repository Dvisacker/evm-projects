// use alloy::network::Network;
// use alloy::primitives::Address;
// use alloy::providers::Provider;
// use alloy::transports::Transport;
// use alloy_rpc_types::BlockId;
// use anyhow::Result;
// use foundry_evm;
// use foundry_evm::revm::{db::AlloyDB, db::CacheDB, Evm};
// use revm::primitives::{AccountInfo, Bytecode, B256, U256};
// use std::marker::PhantomData;
// use std::sync::Arc;

// type AlloyCacheDB<T, N, P> = CacheDB<AlloyDB<T, N, Arc<P>>>;

// // #[derive(Clone)]
// pub struct RevmSimulator<T, N, P>
// where
//     T: Transport + Clone,
//     N: Network,
//     P: Provider<T, N> + 'static,
// {
//     pub provider: Arc<P>,
//     pub owner: Address,
//     pub evm: Evm<'static, (), AlloyCacheDB<T, N, P>>,
//     pub block_number: u64,

//     // pub token: TokenABI,
//     // pub v2_pool: V2PoolABI,
//     // pub simulator: SimulatorABI,
//     pub simulator_address: Address,
//     _phantom: PhantomData<(T, N)>,
// }

// #[derive(Debug, Clone)]
// pub struct Tx {
//     pub caller: Address,
//     pub transact_to: Address,
//     pub data: Vec<u8>,
//     pub value: u128,
//     pub gas_limit: u64,
// }

// #[derive(Debug, Clone)]
// pub struct TxResult {
//     pub output: Vec<u8>,
//     pub gas_used: u64,
//     pub gas_refunded: u64,
// }

// // This simulator might be faster than the foundry simulator
// impl<T, N, P> RevmSimulator<T, N, P>
// where
//     T: Transport + Clone,
//     N: Network,
//     P: Provider<T, N>,
// {
//     pub async fn new(provider: P, owner: Address, block_number: u64) -> Self
//     where
//         T: Transport + Clone,
//         N: Network,
//         P: Provider<T, N>,
//     {
//         let provider = Arc::new(provider);
//         let block_id = BlockId::Number(block_number.into());
//         let chain_id = provider
//             .get_chain_id()
//             .await
//             .expect("Failed to get chain id");
//         let alloy_db = AlloyDB::new(provider.clone(), block_id).unwrap();
//         let db = CacheDB::new(alloy_db);

//         let evm = Evm::builder()
//             .with_db(db)
//             .modify_cfg_env(|c| {
//                 c.chain_id = chain_id;
//             })
//             .build();

//         Self {
//             provider: provider.clone(),
//             owner,
//             evm,
//             block_number,
//             simulator_address: Address::ZERO,
//             _phantom: PhantomData,
//         }
//     }

//     pub fn get_block_number(&mut self) -> U256 {
//         self.evm.context.evm.inner.env.block.number
//     }

//     pub fn get_coinbase(&mut self) -> Address {
//         self.evm.context.evm.inner.env.block.coinbase
//     }

//     pub fn get_base_fee(&mut self) -> U256 {
//         self.evm.context.evm.inner.env.block.basefee
//     }

//     // pub fn _call(&mut self, tx: Tx, commit: bool) -> Result<TxResult> {
//     //     self.evm.context.evm.inner.env.tx.caller = tx.caller.into();
//     //     self.evm.context.evm.inner.env.tx.transact_to = TransactTo::Call(tx.transact_to.into());
//     //     // self.evm.env.tx.data = tx.data;
//     //     // self.evm.env.tx.value = tx.value.into();
//     //     // self.evm.env.tx.gas_price = tx.gas_price.into();
//     //     // self.evm.env.tx.gas_limit = tx.gas_limit;

//     //     // let result;

//     //     // if commit {
//     //     //     result = match self.evm.transact_commit() {
//     //     //         Ok(result) => result,
//     //     //         Err(e) => return Err(anyhow!("EVM call failed: {:?}", e)),
//     //     //     };
//     //     // } else {
//     //     //     let ref_tx = self
//     //     //         .evm
//     //     //         .transact_ref()
//     //     //         .map_err(|e| anyhow!("EVM staticcall failed: {:?}", e))?;
//     //     //     result = ref_tx.result;
//     //     // }

//     //     // let output = match result {
//     //     //     ExecutionResult::Success {
//     //     //         gas_used,
//     //     //         gas_refunded,
//     //     //         output,
//     //     //         logs,
//     //     //         ..
//     //     //     } => match output {
//     //     //         Output::Call(o) => TxResult {
//     //     //             output: o,
//     //     //             logs: Some(logs),
//     //     //             gas_used,
//     //     //             gas_refunded,
//     //     //         },
//     //     //         Output::Create(o, _) => TxResult {
//     //     //             output: o,
//     //     //             logs: Some(logs),
//     //     //             gas_used,
//     //     //             gas_refunded,
//     //     //         },
//     //     //     },
//     //     //     ExecutionResult::Revert { gas_used, output } => {
//     //     //         return Err(anyhow!(
//     //     //             "EVM REVERT: {:?} / Gas used: {:?}",
//     //     //             output,
//     //     //             gas_used
//     //     //         ))
//     //     //     }
//     //     //     ExecutionResult::Halt { reason, .. } => return Err(anyhow!("EVM HALT: {:?}", reason)),
//     //     };

//     //     Ok(output)
//     // }

//     pub fn insert_account_storage(
//         &mut self,
//         target: Address,
//         slot: U256,
//         value: U256,
//     ) -> Result<()> {
//         self.evm
//             .context
//             .evm
//             .db
//             .insert_account_storage(target.into(), slot, value)?;
//         Ok(())
//     }

//     pub fn insert_account_info(&mut self, target: Address, info: AccountInfo) -> Result<()> {
//         self.evm
//             .context
//             .evm
//             .db
//             .insert_account_info(target.into(), info);
//         Ok(())
//     }

//     pub fn get_eth_balance(&mut self, address: Address) -> Result<U256> {
//         let acc = self.evm.context.evm.db.load_account(address)?;
//         Ok(acc.info.balance)
//     }

//     pub fn set_eth_balance(&mut self, address: Address, balance: U256) -> Result<()> {
//         let user_balance = balance;
//         let user_info = AccountInfo::new(user_balance, 0, B256::ZERO, Bytecode::default());
//         self.insert_account_info(address, user_info)?;
//         Ok(())
//     }

//     pub fn set_v2_pool_reserves(&mut self, pool: Address, reserves: U256) -> Result<()> {
//         let slot = U256::from(8);
//         self.insert_account_storage(pool, slot, reserves)?;
//         Ok(())
//     }

//     // pub fn get_token_balance(&mut self, token: Address, address: Address) -> Result<U256> {
//     //     sol! {
//     //         function balanceOf(address account) public returns (uint256);
//     //     }

//     //     let encoded = balanceOfCall { account: address }.abi_encode();
//     //     let evm = self.evm;
//     // }

//     // fn balance_of(self, token: Address, address: Address) -> Result<U256> {
//     //     sol! {
//     //         function balanceOf(address account) public returns (uint256);
//     //     }

//     //     let encoded = balanceOfCall { account: address }.abi_encode();
//     //     let evm = self.evm;

//     // let evm = self.evm.modify_tx_env(|tx| {
//     //         // 0x1 because calling USDC proxy from zero address fails
//     //         tx.caller = address!("0000000000000000000000000000000000000001");
//     //         tx.transact_to = TransactTo::Call(token);
//     //         tx.data = encoded.into();
//     //         tx.value = U256::from(0);
//     //     })

//     // let ref_tx = evm.transact().unwrap();
//     // let result = ref_tx.result;

//     // let value = match result {
//     //     ExecutionResult::Success {
//     //         output: Output::Call(value),
//     //         ..
//     //     } => value,
//     //     result => return Err(anyhow!("'balanceOf' execution failed: {result:?}")),
//     // };

//     // let balance = <U256>::abi_decode(&value, false)?;

//     // Ok(balance)
// }

// // pub fn run_pending_tx(&mut self, tx: &Transaction) -> Result<TxResult> {
// //     // We simply need to commit changes to the DB
// //     self.evm.env.tx.caller = tx.from.0.into();
// //     self.evm.env.tx.transact_to = TransactTo::Call(tx.to.unwrap_or_default().0.into());
// //     self.evm.env.tx.data = tx.input.0.clone();
// //     self.evm.env.tx.value = tx.value.into();
// //     self.evm.env.tx.chain_id = tx.chain_id.map(|id| id.as_u64());
// //     self.evm.env.tx.gas_limit = tx.gas.as_u64();

// //     match tx.transaction_type {
// //         Some(U64([0])) => self.evm.env.tx.gas_price = tx.gas_price.unwrap_or_default().into(),
// //         Some(_) => {
// //             self.evm.env.tx.gas_priority_fee =
// //                 tx.max_priority_fee_per_gas.map(|mpf| mpf.into());
// //             self.evm.env.tx.gas_price = tx.max_fee_per_gas.unwrap_or_default().into();
// //         }
// //         None => self.evm.env.tx.gas_price = tx.gas_price.unwrap_or_default().into(),
// //     }

// //     let result = match self.evm.transact_commit() {
// //         Ok(result) => result,
// //         Err(e) => return Err(anyhow!("EVM call failed: {}", e)),
// //     };

// //     let output = match result {
// //         ExecutionResult::Success {
// //             gas_used,
// //             gas_refunded,
// //             output,
// //             ..
// //         } => match output {
// //             Output::Call(o) => TxResult {
// //                 output: o,
// //                 gas_used,
// //                 gas_refunded,
// //             },
// //             Output::Create(o, _) => TxResult {
// //                 output: o,
// //                 gas_used,
// //                 gas_refunded,
// //             },
// //         },
// //         ExecutionResult::Revert { gas_used, output } => {
// //             return Err(anyhow!(
// //                 "EVM REVERT: {:?} / Gas used: {:?}",
// //                 output,
// //                 gas_used
// //             ))
// //         }
// //         ExecutionResult::Halt { reason, .. } => return Err(anyhow!("EVM HALT: {:?}", reason)),
// //     };

// //     Ok(output)
// // }

// //     pub fn _call(&mut self, tx: Tx, commit: bool) -> Result<TxResult> {
// //         self.evm.env.tx.caller = tx.caller.into();
// //         self.evm.env.tx.transact_to = TransactTo::Call(tx.transact_to.into());
// //         self.evm.env.tx.data = tx.data;
// //         self.evm.env.tx.value = tx.value.into();
// //         self.evm.env.tx.gas_limit = 5000000;

// //         let result;

// //         if commit {
// //             result = match self.evm.transact_commit() {
// //                 Ok(result) => result,
// //                 Err(e) => return Err(anyhow!("EVM call failed: {:?}", e)),
// //             };
// //         } else {
// //             let ref_tx = self
// //                 .evm
// //                 .transact_ref()
// //                 .map_err(|e| anyhow!("EVM staticcall failed: {:?}", e))?;
// //             result = ref_tx.result;
// //         }

// //         let output = match result {
// //             ExecutionResult::Success {
// //                 gas_used,
// //                 gas_refunded,
// //                 output,
// //                 ..
// //             } => match output {
// //                 Output::Call(o) => TxResult {
// //                     output: o,
// //                     gas_used,
// //                     gas_refunded,
// //                 },
// //                 Output::Create(o, _) => TxResult {
// //                     output: o,
// //                     gas_used,
// //                     gas_refunded,
// //                 },
// //             },
// //             ExecutionResult::Revert { gas_used, output } => {
// //                 return Err(anyhow!(
// //                     "EVM REVERT: {:?} / Gas used: {:?}",
// //                     output,
// //                     gas_used
// //                 ))
// //             }
// //             ExecutionResult::Halt { reason, .. } => return Err(anyhow!("EVM HALT: {:?}", reason)),
// //         };

// //         Ok(output)
// //     }

// //     pub fn staticcall(&mut self, tx: Tx) -> Result<TxResult> {
// //         self._call(tx, false)
// //     }

// //     pub fn call(&mut self, tx: Tx) -> Result<TxResult> {
// //         self._call(tx, true)
// //     }

// //     pub fn get_eth_balance(&mut self) -> U256 {
// //         let acc = self
// //             .evm
// //             .db
// //             .as_mut()
// //             .unwrap()
// //             .basic(self.owner.into())
// //             .unwrap()
// //             .unwrap();
// //         acc.balance.into()
// //     }

// //     pub fn set_eth_balance(&mut self, balance: u32) {
// //         let user_balance = rU256::from(balance)
// //             .checked_mul(rU256::from(10).pow(rU256::from(18)))
// //             .unwrap();
// //         let user_info = AccountInfo::new(user_balance, 0, Bytecode::default());
// //         self.evm
// //             .db
// //             .as_mut()
// //             .unwrap()
// //             .insert_account_info(self.owner.into(), user_info);
// //     }

// //     // ERC-20 Token functions
// //     pub fn set_token_balance(
// //         &mut self,
// //         account: H160,
// //         token: H160,
// //         decimals: u8,
// //         slot: u32,
// //         balance: u32,
// //     ) {
// //         let slot = keccak256(&abi::encode(&[
// //             abi::Token::Address(account.into()),
// //             abi::Token::Uint(U256::from(slot)),
// //         ]));
// //         let target_balance = rU256::from(balance)
// //             .checked_mul(rU256::from(10).pow(rU256::from(decimals)))
// //             .unwrap();
// //         self.evm
// //             .db
// //             .as_mut()
// //             .unwrap()
// //             .insert_account_storage(token.into(), slot.into(), target_balance)
// //             .unwrap();
// //     }

// //     pub fn token_balance_of(&mut self, token: H160, account: H160) -> Result<U256> {
// //         let calldata = self.token.balance_of_input(account)?;
// //         let value = self.staticcall(Tx {
// //             caller: self.owner.into(),
// //             transact_to: token,
// //             data: calldata.0,
// //             value: U256::zero(),
// //             gas_limit: 0,
// //         })?;
// //         let out = self.token.balance_of_output(value.output)?;
// //         Ok(out)
// //     }

// //     // V2 Pool functions
// //     pub fn set_v2_pool_reserves(&mut self, pool: H160, reserves: rU256) {
// //         let slot = rU256::from(8);
// //         self.evm
// //             .db
// //             .as_mut()
// //             .unwrap()
// //             .insert_account_storage(pool.into(), slot.into(), reserves)
// //             .unwrap();
// //     }

// //     pub fn v2_pool_get_reserves(&mut self, pool: H160) -> Result<(u128, u128, u32)> {
// //         let calldata = self.v2_pool.get_reserves_input()?;
// //         let value = self.staticcall(Tx {
// //             caller: self.owner,
// //             transact_to: pool,
// //             data: calldata.0,
// //             value: U256::zero(),
// //             gas_limit: 0,
// //         })?;
// //         let out = self.v2_pool.get_reserves_output(value.output)?;
// //         Ok(out)
// //     }

// //     // Simulator functions
// //     pub fn deploy_simulator(&mut self) {
// //         let contract_info = AccountInfo::new(
// //             rU256::ZERO,
// //             0,
// //             Bytecode::new_raw((*SIMULATOR_CODE.0).into()),
// //         );
// //         self.evm
// //             .db
// //             .as_mut()
// //             .unwrap()
// //             .insert_account_info(self.simulator_address.into(), contract_info);
// //     }

// //     pub fn v2_simulate_swap(
// //         &mut self,
// //         amount_in: U256,
// //         target_pool: H160,
// //         input_token: H160,
// //         output_token: H160,
// //         commit: bool,
// //     ) -> Result<(U256, U256)> {
// //         let calldata = self.simulator.v2_simulate_swap_input(
// //             amount_in,
// //             target_pool,
// //             input_token,
// //             output_token,
// //         )?;
// //         let tx = Tx {
// //             caller: self.owner,
// //             transact_to: self.simulator_address,
// //             data: calldata.0,
// //             value: U256::zero(),
// //             gas_limit: 5000000,
// //         };
// //         let value = if commit {
// //             self.call(tx)?
// //         } else {
// //             self.staticcall(tx)?
// //         };
// //         let out = self.simulator.v2_simulate_swap_output(value.output)?;
// //         Ok(out)
// //     }

// //     pub fn get_amount_out(
// //         &mut self,
// //         amount_in: U256,
// //         reserve_in: U256,
// //         reserve_out: U256,
// //     ) -> Result<U256> {
// //         let calldata = self
// //             .simulator
// //             .get_amount_out_input(amount_in, reserve_in, reserve_out)?;
// //         let value = self.staticcall(Tx {
// //             caller: self.owner,
// //             transact_to: self.simulator_address,
// //             data: calldata.0,
// //             value: U256::zero(),
// //             gas_limit: 5000000,
// //         })?;
// //         let out = self.simulator.get_amount_out_output(value.output)?;
// //         Ok(out)
// //     }
// // }
