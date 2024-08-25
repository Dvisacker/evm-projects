// use crate::addressbook::Addressbook;
// use alloy::primitives::{Address, U256};
// use alloy::providers::Provider;
// use bindings::uniswapv2factory::UniswapV2Factory::UniswapV2FactoryInstance;
// use bindings::uniswapv2pair::UniswapV2Pair;
// use std::collections::HashMap;
// use std::str::FromStr;
// use std::sync::Arc;

// pub enum ExchangeName {
//     UniswapV2,
//     Sushiswap,
// }

// pub struct UniswapV2Client<M: Provider + 'static> {
//     factory: UniswapV2FactoryInstance<M>,
//     chain_id: u64,
//     client: Arc<M>,
// }

// impl<M: Provider + 'static> UniswapV2Client<M> {
//     pub fn new(
//         client: Arc<M>,
//         addressbook: &Addressbook,
//         chain_id: u64,
//         exchange_name: ExchangeName,
//     ) -> Self {
//         let factory_address = match chain_id {
//             42161 => match exchange_name {
//                 ExchangeName::UniswapV2 => {
//                     Address::from_str(&addressbook.arbitrum.uniswapv2.factory).unwrap()
//                 }
//                 ExchangeName::Sushiswap => {
//                     Address::from_str(&addressbook.arbitrum.sushiswap.factory).unwrap()
//                 }
//                 _ => panic!("Unsupported exchange name"),
//             },
//             10 => match exchange_name {
//                 ExchangeName::UniswapV2 => {
//                     Address::from_str(&addressbook.optimism.uniswapv2.factory).unwrap()
//                 }
//                 ExchangeName::Sushiswap => {
//                     Address::from_str(&addressbook.optimism.sushiswap.factory).unwrap()
//                 }
//                 _ => panic!("Unsupported exchange name"),
//             },
//             _ => panic!("Unsupported chain ID"),
//         };

//         let factory = UniswapV2FactoryInstance::new(factory_address, client.clone());

//         Self {
//             factory,
//             chain_id,
//             client,
//         }
//     }

//     pub async fn get_all_pairs(&self) -> Result<Vec<Address>, Box<dyn std::error::Error>> {
//         let pair_count = self.factory.all_pairs_length().call().await?;
//         let mut pairs = Vec::with_capacity(pair_count.as_usize());

//         for i in 0..pair_count.as_u64() {
//             let pair_address = self.factory.all_pairs(i.into()).call().await?;
//             pairs.push(pair_address);
//         }

//         Ok(pairs)
//     }

//     pub async fn get_pool_address(
//         &self,
//         token0: Address,
//         token1: Address,
//     ) -> Result<Address, ContractError<M>> {
//         let (token0, token1) = if token0 < token1 {
//             (token0, token1)
//         } else {
//             (token1, token0)
//         };

//         let pool_address = self.factory.get_pair(token0, token1).call().await?;
//         Ok(pool_address)
//     }

//     pub async fn get_cycle_profit(
//         &self,
//         pools: &[Address],
//     ) -> Result<U256, Box<dyn std::error::Error>> {
//         let reserves = self.update_reserves(pools).await?;
//         let mut profit = U256::from(0);

//         for (pool_address, (reserve0, reserve1)) in reserves {
//             let pair = UniswapV2Pair::new(pool_address, self.client.clone());
//             let (reserve0, reserve1, _) = pair.get_reserves().call().await?;
//         }

//         Ok(profit)
//     }

//     // fn get_pool_address_2(token0: Address, token1: Address) -> Result<Address, Error> {
//     //     // Ensure token0 < token1
//     //     let (token0, token1) = if token0 < token1 {
//     //         (token0, token1)
//     //     } else {
//     //         (token1, token0)
//     //     };

//     //     // Compute the pool address using CREATE2
//     //     let salt = ethers::utils::keccak256(ethers::abi::encode(&[
//     //         ethers::abi::Token::Address(token0),
//     //         ethers::abi::Token::Address(token1),
//     //     ]));

//     //     let init_code_hash = "0x96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f";

//     //     let pool_address =
//     //         ethers::utils::get_create2_address(self.factory.address(), salt, init_code_hash);

//     //     Ok(pool_address)
//     // }
// }
