use std::fmt;

use crate::route::simulate_route;
use alloy::primitives::{Address, U256};
use amms::amm::uniswap_v2::UniswapV2Pool;
use amms::amm::{AutomatedMarketMaker, AMM};

#[derive(Debug, Clone)]
pub struct Cycle(pub Vec<AMM>);

impl fmt::Display for Cycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cycle[")?;
        for (i, pool) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " -> ")?;
            }
            let address = pool.address();
            write!(f, "{}", address)?;
        }
        write!(f, "]")
    }
}

impl Cycle {
    pub fn get_entry_token(&self) -> Address {
        // the entry token is the token that is in the first pool and the last pool
        let first_pool_tokens = self.0.first().unwrap().tokens();
        let last_pool_tokens = self.0.last().unwrap().tokens();

        let [first_pool_token_a, first_pool_token_b] = first_pool_tokens.as_slice() else {
            todo!()
        };
        let [last_pool_token_a, last_pool_token_b] = last_pool_tokens.as_slice() else {
            todo!()
        };

        if first_pool_token_a == last_pool_token_a || first_pool_token_a == last_pool_token_b {
            *first_pool_token_a
        } else {
            *first_pool_token_b
        }
    }

    pub fn get_profit(&self) -> i128 {
        let token_in = self.get_entry_token();
        let input_amount = U256::from(10u64.pow(18));
        let output_amount = simulate_route(token_in, input_amount, &self.0).unwrap();
        let profit = output_amount.to::<i128>() - input_amount.to::<i128>();

        profit
    }
}
