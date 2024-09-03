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

        token_in = match pool {
            AMM::UniswapV2Pool(pool) => {
                if token_in == pool.token_a {
                    pool.token_b
                } else {
                    pool.token_a
                }
            }

            AMM::UniswapV3Pool(pool) => {
                if token_in == pool.token_a {
                    pool.token_b
                } else {
                    pool.token_a
                }
            }

            AMM::CamelotV3Pool(pool) => {
                if token_in == pool.token_a {
                    pool.token_b
                } else {
                    pool.token_a
                }
            }

            _ => unreachable!(),
        };

        amount_in = amount_out
    }

    Ok(amount_out)
}
