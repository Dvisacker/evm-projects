use std::ops::Deref;

use amms::amm::uniswap_v2::UniswapV2Pool;

#[derive(Debug, Clone)]
pub struct DetailedPool {
    inner: UniswapV2Pool,
    pub token_a_symbol: String,
    pub token_b_symbol: String,
    pub token_a_total_supply: u128,
    pub token_b_total_supply: u128,
    pub token_a_decimals: u8,
    pub token_b_decimals: u8,
}

// implement deref
impl Deref for DetailedPool {
    type Target = UniswapV2Pool;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DetailedPool {
    pub fn name(&self) -> String {
        format!("{}_{}", self.token_a_symbol, self.token_b_symbol)
    }
}
