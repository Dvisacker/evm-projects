use std::fmt;

use crate::route::simulate_route;
use alloy::primitives::{Address, I256, U256};
use amms::amm::uniswap_v2::UniswapV2Pool;
use amms::amm::AMM;

#[derive(Debug, Clone)]
pub struct Cycle(pub Vec<UniswapV2Pool>);

impl fmt::Display for Cycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cycle[")?;
        for (i, pool) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " -> ")?;
            }
            write!(f, "{}", pool.address)?;
        }
        write!(f, "]")
    }
}

impl Cycle {
    pub fn get_entry_token(&self) -> Address {
        // the entry token is the token that is in the first pool and the last pool
        let first_pool = self.0.first().unwrap();
        let last_pool = self.0.last().unwrap();

        if first_pool.token_a == last_pool.token_a || first_pool.token_a == last_pool.token_b {
            first_pool.token_a
        } else {
            first_pool.token_b
        }
    }

    pub fn get_profit(&self) -> i128 {
        let amms: Vec<AMM> = self
            .0
            .iter()
            .map(|pool| AMM::UniswapV2Pool(pool.clone()))
            .collect();

        let token_in = self.get_entry_token();
        let input_amount = U256::from(10u64.pow(18));
        let output_amount = simulate_route(token_in, input_amount, &amms).unwrap();
        let profit = output_amount.to::<i128>() - input_amount.to::<i128>();

        profit
    }
}
