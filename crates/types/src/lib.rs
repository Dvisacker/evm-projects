use alloy::primitives::Address;
use alloy_chains::{Chain, NamedChain};
use db::models::NewPool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExchangeName {
    UniswapV2,
    SushiSwapV2,
    UniswapV3,
    SushiSwapV3,
}

impl ExchangeName {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "uniswapv2" => Ok(ExchangeName::UniswapV2),
            "sushiswapv2" => Ok(ExchangeName::SushiSwapV2),
            "uniswapv3" => Ok(ExchangeName::UniswapV3),
            "sushiswapv3" => Ok(ExchangeName::SushiSwapV3),
            _ => Err(format!("Invalid exchange name: {}", s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ExchangeName::UniswapV2 => "uniswapv2",
            ExchangeName::SushiSwapV2 => "sushiswapv2",
            ExchangeName::UniswapV3 => "uniswapv3",
            ExchangeName::SushiSwapV3 => "sushiswapv3",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExchangeType {
    UniV2,
    UniV3,
}

impl ExchangeType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "univ2" => Ok(ExchangeType::UniV2),
            "univ3" => Ok(ExchangeType::UniV3),
            _ => Err(format!("Invalid exchange type: {}", s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ExchangeType::UniV2 => "univ2",
            ExchangeType::UniV3 => "univ3",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DetailedPool {
    pub address: Address,
    pub chain: NamedChain,
    pub exchange_name: String,
    pub exchange_type: ExchangeType,
    pub token_a: Address,
    pub token_a_symbol: String,
    pub token_a_decimals: u8,
    pub token_b: Address,
    pub token_b_symbol: String,
    pub token_b_decimals: u8,
    pub factory_address: Address,
    pub reserve_0: u128,
    pub reserve_1: u128,
    pub fee: u32,
}

impl From<NewPool> for DetailedPool {
    fn from(pool: NewPool) -> Self {
        DetailedPool {
            chain: pool.chain.parse::<NamedChain>().unwrap(),
            exchange_name: pool.exchange_name.to_string(),
            exchange_type: ExchangeType::from_str(&pool.exchange_type).unwrap(),
            address: pool.address.parse().unwrap_or(Address::ZERO),
            token_a: pool.token_a.parse().unwrap_or(Address::ZERO),
            token_a_symbol: pool.token_a_symbol.to_string(),
            token_a_decimals: pool.token_a_decimals as u8,
            token_b: pool.token_b.parse().unwrap_or(Address::ZERO),
            token_b_symbol: pool.token_b_symbol.to_string(),
            token_b_decimals: pool.token_b_decimals as u8,
            factory_address: pool.factory_address.parse().unwrap_or(Address::ZERO),
            reserve_0: pool.reserve_0.parse().unwrap_or(0),
            reserve_1: pool.reserve_1.parse().unwrap_or(0),
            fee: pool.fee as u32,
        }
    }
}

impl DetailedPool {
    pub fn name(&self) -> String {
        format!("{}_{}", self.token_a_symbol, self.token_b_symbol)
    }

    pub fn empty(
        address: Address,
        chain: NamedChain,
        exchange_type: Option<ExchangeType>,
        exchange_name: Option<ExchangeName>,
    ) -> DetailedPool {
        DetailedPool {
            address,
            chain: chain.clone(),
            exchange_name: exchange_name
                .unwrap_or(ExchangeName::UniswapV2)
                .as_str()
                .to_string(),
            exchange_type: exchange_type.unwrap_or(ExchangeType::UniV2),
            token_a: Address::ZERO,
            token_a_symbol: String::new(),
            token_a_decimals: 0,
            token_b: Address::ZERO,
            token_b_symbol: String::new(),
            token_b_decimals: 0,
            factory_address: Address::ZERO,
            reserve_0: 0,
            reserve_1: 0,
            fee: 0,
        }
    }

    pub fn to_new_pool(&self) -> NewPool {
        NewPool {
            address: self.address.to_string(),
            chain: self.chain.as_str().to_string(),
            factory_address: self.factory_address.to_string(),
            exchange_name: self.exchange_name.clone(),
            exchange_type: self.exchange_type.as_str().to_string(),
            token_a: self.token_a.to_string(),
            token_a_symbol: self.token_a_symbol.clone(),
            token_a_decimals: self.token_a_decimals as i32,
            token_b: self.token_b.to_string(),
            token_b_symbol: self.token_b_symbol.clone(),
            token_b_decimals: self.token_b_decimals as i32,
            reserve_0: self.reserve_0.to_string(),
            reserve_1: self.reserve_1.to_string(),
            fee: self.fee as i32,
        }
    }
}
