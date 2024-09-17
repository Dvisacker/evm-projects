CREATE TABLE uni_v3_pools (
    id SERIAL PRIMARY KEY,
    address VARCHAR NOT NULL,
    chain VARCHAR NOT NULL,
    token_a VARCHAR NOT NULL,
    token_a_decimals INTEGER NOT NULL,
    token_a_symbol VARCHAR NOT NULL,
    token_b VARCHAR NOT NULL,
    token_b_decimals INTEGER NOT NULL,
    token_b_symbol VARCHAR NOT NULL,
    liquidity NUMERIC NOT NULL,
    sqrt_price NUMERIC NOT NULL,
    fee INTEGER NOT NULL,
    tick INTEGER NOT NULL,
    tick_spacing INTEGER NOT NULL,
    tick_bitmap JSONB NOT NULL,
    ticks JSONB NOT NULL,
    exchange_name VARCHAR NOT NULL,
    exchange_type VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_uni_v3_pools_chain_address ON uni_v3_pools (chain, address);

CREATE OR REPLACE FUNCTION update_uni_v3_pools_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_uni_v3_pools_updated_at
BEFORE UPDATE ON uni_v3_pools
FOR EACH ROW
EXECUTE FUNCTION update_uni_v3_pools_updated_at();