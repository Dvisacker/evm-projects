CREATE TABLE exchanges (
    id SERIAL PRIMARY KEY,
    chain VARCHAR NOT NULL,
    factory_address VARCHAR,
    exchange_name VARCHAR NOT NULL,
    exchange_type VARCHAR NOT NULL
);

CREATE TABLE pools (
    id SERIAL PRIMARY KEY,
    address VARCHAR NOT NULL,
    chain VARCHAR NOT NULL,
    factory_address VARCHAR NOT NULL,
    exchange_name VARCHAR NOT NULL,
    exchange_type VARCHAR NOT NULL,
    token_a VARCHAR NOT NULL,
    token_a_symbol VARCHAR NOT NULL,
    token_a_decimals INTEGER NOT NULL,
    token_b VARCHAR NOT NULL,
    token_b_symbol VARCHAR NOT NULL,
    token_b_decimals INTEGER NOT NULL,
    reserve_0 VARCHAR NOT NULL,
    reserve_1 VARCHAR NOT NULL,
    fee INTEGER NOT NULL,
    filtered BOOLEAN
);

CREATE UNIQUE INDEX idx_pools_chain_address ON pools (chain, address);