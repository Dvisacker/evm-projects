use std::fmt;

use crate::route::simulate_route;
use alloy::primitives::{keccak256, Address, U256};
use amms::amm::{AutomatedMarketMaker, AMM};

#[derive(Debug, Clone)]
pub struct Cycle {
    pub amms: Vec<AMM>,
    pub id: String,
}

impl fmt::Display for Cycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let profit_perc = self.get_profit_perc();
        for (i, pool) in self.amms.iter().enumerate() {
            if i > 0 {
                write!(f, " -> ")?;
            }

            let token_symbols = pool.token_symbols();
            let [token_a, token_b] = token_symbols.as_slice() else {
                todo!()
            };

            write!(f, "{}:{}-{}", pool.exchange_name(), token_a, token_b)?;
        }
        write!(f, "] ")?;
        write!(f, "Profit: {}", profit_perc)
    }
}

impl Cycle {
    pub fn new(amms: Vec<AMM>) -> Self {
        let id = Self::compute_id(&amms);
        Self { amms, id }
    }

    pub fn compute_id(amms: &Vec<AMM>) -> String {
        let tokens: Vec<Address> = amms.iter().map(|pool| pool.tokens()).flatten().collect();
        let token_strings = tokens
            .iter()
            .map(|token| token.to_string())
            .collect::<Vec<String>>();
        let concatenated_string = token_strings.join("");
        let hash = keccak256(concatenated_string);
        hash.to_string()
    }

    pub fn get_entry_token(&self) -> Address {
        // the entry token is the token that is in the first pool and the last pool
        let first_pool_tokens = self.amms.first().unwrap().tokens();
        let last_pool_tokens = self.amms.last().unwrap().tokens();

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

    pub fn get_profit(&self, amount_in: U256) -> i128 {
        let token_in = self.get_entry_token();
        let amount_out = simulate_route(token_in, amount_in, &self.amms).unwrap();
        let profit = amount_out.to::<i128>() - amount_in.to::<i128>();

        profit
    }

    pub fn get_profit_no_slippage(&self, amount_in: U256) -> i128 {
        let token_in = self.get_entry_token();
        let amount_out = simulate_route(token_in, amount_in, &self.amms).unwrap();
        let profit = amount_out.to::<i128>() - amount_in.to::<i128>();

        profit
    }

    pub fn get_profit_perc(&self) -> f64 {
        let amount_in = U256::from(10u64.pow(18));
        let profit = self.get_profit(amount_in);
        let profit_perc = profit as f64 / (amount_in.to::<i128>() as f64);
        profit_perc
    }
}

pub fn sort_cycle_by_profit(cycles: Vec<Cycle>) -> Vec<Cycle> {
    let mut cycles = cycles;
    cycles.sort_by(|a, b| {
        b.get_profit_perc()
            .partial_cmp(&a.get_profit_perc())
            .unwrap()
    });
    cycles
}

pub fn get_most_profitable_cycle(cycles: Vec<Cycle>) -> Option<Cycle> {
    sort_cycle_by_profit(cycles).first().cloned()
}

pub fn get_most_profitable_cycles(cycles: Vec<Cycle>, n: usize) -> Vec<Cycle> {
    sort_cycle_by_profit(cycles).into_iter().take(n).collect()
}
