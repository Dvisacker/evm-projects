use std::cell::RefCell;

use alloy::primitives::{Address, I256, U256};
use amms::amm::uniswap_v2::UniswapV2Pool;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NetPositiveCycle {
    pub profit: I256,
    pub optimal_in: U256,
    pub swap_amounts: Vec<U256>,
    pub cycle_addresses: Vec<Address>,
}

pub fn maximize_profit(
    mut domain_min: U256,
    mut domain_max: U256,
    lowest_delta: U256,
    f: impl Fn(U256) -> I256,
) -> U256 {
    loop {
        if domain_max > domain_min {
            if (domain_max - domain_min) > lowest_delta {
                let mid = (domain_min + domain_max) / U256::from(2);

                let lower_mid = (mid + domain_min) / U256::from(2);
                let upper_mid = (mid + domain_max) / U256::from(2);

                let f_output_lower = f(lower_mid);
                let f_output_upper = f(upper_mid);

                if f_output_lower > f_output_upper {
                    domain_max = mid;
                } else {
                    domain_min = mid;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    (domain_max + domain_min) / U256::from(2)
}

pub fn get_profit(token_in: Address, amount_in: U256, pairs: &Vec<UniswapV2Pool>) -> I256 {
    let mut amount_out: U256 = amount_in;
    let mut token_in = token_in;
    for pair in pairs {
        let (reserve0, reserve1) = if pair.token_a == token_in {
            (pair.reserve_0, pair.reserve_1)
        } else {
            (pair.reserve_1, pair.reserve_0)
        };
        amount_out = pair.get_amount_out(
            U256::from(amount_out),
            U256::from(reserve0),
            U256::from(reserve1),
        );
        token_in = if pair.token_a == token_in {
            pair.token_b
        } else {
            pair.token_a
        };
    }

    I256::from_raw(amount_out) - I256::from_raw(amount_in)
}

pub fn get_profit_with_amount(
    token_in: Address,
    amount_in: U256,
    pairs: &Vec<&RefCell<UniswapV2Pool>>,
) -> (I256, Vec<U256>) {
    let mut amount_out: U256 = amount_in;
    let mut token_in = token_in;
    let mut amounts = Vec::with_capacity(pairs.len() + 1);
    amounts.push(amount_in);
    for pair in pairs {
        let pair = pair.borrow();
        let (reserve0, reserve1) = if pair.token_a == token_in {
            (pair.reserve_0, pair.reserve_1)
        } else {
            (pair.reserve_1, pair.reserve_0)
        };
        amount_out = pair.get_amount_out(
            U256::from(amount_out),
            U256::from(reserve0),
            U256::from(reserve1),
        );
        amounts.push(amount_out);
        token_in = if pair.token_a == token_in {
            pair.token_b
        } else {
            pair.token_a
        };
    }

    (
        I256::from_raw(amount_out) - I256::from_raw(amount_in),
        amounts,
    )
}
