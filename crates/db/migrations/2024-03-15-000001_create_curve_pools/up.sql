CREATE TABLE curve_pools (
    id SERIAL PRIMARY KEY,
    address VARCHAR NOT NULL,
    chain VARCHAR NOT NULL,
    token_a VARCHAR NOT NULL,
    token_a_decimals INTEGER NOT NULL,
    token_a_symbol VARCHAR NOT NULL,
    token_a_balance VARCHAR NOT NULL,
    token_b VARCHAR NOT NULL,
    token_b_decimals INTEGER NOT NULL,
    token_b_symbol VARCHAR NOT NULL,
    token_b_balance VARCHAR NOT NULL,
    token_c VARCHAR,
    token_c_decimals INTEGER,
    token_c_symbol VARCHAR,
    token_c_balance VARCHAR,
    token_d VARCHAR,
    token_d_decimals INTEGER,
    token_d_symbol VARCHAR,
    token_d_balance VARCHAR,
    exchange_name VARCHAR,
    exchange_type VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_curve_pools_chain_address ON curve_pools (chain, address);

CREATE OR REPLACE FUNCTION update_curve_pools_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_curve_pools_updated_at
BEFORE UPDATE ON curve_pools
FOR EACH ROW
EXECUTE FUNCTION update_curve_pools_updated_at();