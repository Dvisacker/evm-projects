use std::fmt;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ValueEnum)]
pub enum ExchangeName {
    UniswapV2,
    SushiswapV2,
    UniswapV3,
    SushiswapV3,
    CamelotV3,
    RamsesV2,
    PancakeswapV3,
    Curve,
    Unknown,
}

impl fmt::Display for ExchangeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ExchangeName {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "uniswapv2" => Ok(ExchangeName::UniswapV2),
            "sushiswapv2" => Ok(ExchangeName::SushiswapV2),
            "uniswapv3" => Ok(ExchangeName::UniswapV3),
            "sushiswapv3" => Ok(ExchangeName::SushiswapV3),
            "unknown" => Ok(ExchangeName::Unknown),
            "camelotv3" => Ok(ExchangeName::CamelotV3),
            "ramsesv2" => Ok(ExchangeName::RamsesV2),
            "pancakeswapv3" => Ok(ExchangeName::PancakeswapV3),
            _ => Err(format!("Invalid exchange name: {}", s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ExchangeName::UniswapV2 => "uniswapv2",
            ExchangeName::SushiswapV2 => "sushiswapv2",
            ExchangeName::UniswapV3 => "uniswapv3",
            ExchangeName::SushiswapV3 => "sushiswapv3",
            ExchangeName::CamelotV3 => "camelotv3",
            ExchangeName::RamsesV2 => "ramsesv2",
            ExchangeName::PancakeswapV3 => "pancakeswapv3",
            ExchangeName::Curve => "curve",
            ExchangeName::Unknown => "unknown",
        }
    }
}

impl Default for ExchangeName {
    fn default() -> Self {
        ExchangeName::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ExchangeType {
    UniV2,
    UniV3,
    CamelotV3,
    Curve,
    ERC4626,
    Unknown,
}

impl ExchangeType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "univ2" => Ok(ExchangeType::UniV2),
            "univ3" => Ok(ExchangeType::UniV3),
            "camelotv3" => Ok(ExchangeType::CamelotV3),
            "erc4626" => Ok(ExchangeType::ERC4626),
            "curve" => Ok(ExchangeType::Curve),
            _ => Err(format!("Invalid exchange type: {}", s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ExchangeType::UniV2 => "univ2",
            ExchangeType::UniV3 => "univ3",
            ExchangeType::CamelotV3 => "camelotv3",
            ExchangeType::ERC4626 => "erc4626",
            ExchangeType::Curve => "curve",
            ExchangeType::Unknown => "unknown",
        }
    }
}

impl Default for ExchangeType {
    fn default() -> Self {
        ExchangeType::Unknown
    }
}
