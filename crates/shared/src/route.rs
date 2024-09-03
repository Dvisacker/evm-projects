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
        amount_out = pool.simulate_swap(token_in, amount_in).unwrap();
        let tokens = pool.tokens();
        let [token_a, token_b] = tokens.as_slice() else {
            todo!()
        };

        if token_in == *token_a {
            token_in = *token_b;
        } else {
            token_in = *token_a;
        }

        amount_in = amount_out
    }

    Ok(amount_out)
}
