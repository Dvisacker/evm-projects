use alloy::primitives::{Address, U256};
use amms::{
    amm::{AutomatedMarketMaker, AMM},
    errors::AMMError,
};

pub fn simulate_route(
    mut token_in: Address,
    mut amount_in: U256,
    route: &[AMM],
) -> Result<U256, AMMError> {
    let mut amount_out = U256::ZERO;

    for pool in route {
        let tokens = pool.tokens();
        let token_a = tokens[0];
        let token_b = tokens[1];

        let token_out;

        if token_in == token_a {
            token_in = token_b;
            token_out = token_a;
        } else {
            token_in = token_a;
            token_out = token_b;
        }

        amount_out = pool.simulate_swap(token_in, amount_in, token_out).unwrap();

        amount_in = amount_out
    }

    Ok(amount_out)
}
