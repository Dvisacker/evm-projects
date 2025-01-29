use alloy_primitives::{utils::parse_units, I256, U256};

#[derive(Debug)]
pub enum ComparisonError {
    DivisionByZero,
    Overflow,
}

// Check if (amount2 - amount1) / amount1 is within the tolerance.
// Tolerance is a percentage, so it should be a string like "0.01" for 1%.
pub fn within_tolerance(amount1: U256, amount2: U256, tolerance: String) -> bool {
    let scale_decimals = 5;
    let scale_multiplier: U256 = parse_units("1", scale_decimals).unwrap().into();
    let difference = (amount2 - amount1) * scale_multiplier / amount1;
    let scaled_tolerance: I256 = parse_units(&tolerance, scale_decimals).unwrap().into();
    let within_tolerance = I256::try_from(difference).unwrap() >= scaled_tolerance;
    return within_tolerance;
}
