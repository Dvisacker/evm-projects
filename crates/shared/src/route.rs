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
        let [token_a, token_b] = tokens.as_slice() else {
            todo!()
        };

        let token_out;

        if token_in == *token_a {
            token_in = *token_b;
            token_out = *token_a;
        } else {
            token_in = *token_a;
            token_out = *token_b;
        }

        amount_out = pool.simulate_swap(token_in, amount_in, token_out).unwrap();

        amount_in = amount_out
    }

    Ok(amount_out)
}

pub fn simulate_route_2(
    mut token_in: Address,
    mut amount_in: U256,
    route: &[AMM],
) -> Result<U256, AMMError> {
    todo!()
}

// pub fn simulate_route_without_slippage(
//     mut token_in: Address,
//     mut amount_in: U256,
//     route: &[AMM],
// ) -> Result<U256, AMMError> {
//     let mut amount_out = amount_in;

//     for pool in route {
//         let tokens = pool.tokens();
//         let [token_a, token_b] = tokens.as_slice() else {
//             return Err(AMMError::SimulationError);
//         };

//         let (base_token, quote_token) = if token_in == *token_a {
//             (token_a, token_b)
//         } else {
//             (token_b, token_a)
//         };

//         let price = pool.calculate_price(*base_token)?;

//         // Convert amount_in to f64 for calculation
//         let amount_in_f64 = amount_out.to_f64().ok_or(AMMError::SimulationError)?;

//         // Calculate amount_out based on the price
//         let amount_out_f64 = if token_in == *base_token {
//             amount_in_f64 * price
//         } else {
//             amount_in_f64 / price
//         };

//         // Convert amount_out back to U256
//         amount_out = U256::from_f64_lossy(amount_out_f64);

//         // Prepare for the next iteration
//         token_in = *quote_token;
//     }

//     Ok(amount_out)
// }
