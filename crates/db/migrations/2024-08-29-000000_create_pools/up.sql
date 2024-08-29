CREATE TABLE pools (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    address TEXT NOT NULL,
    chain TEXT NOT NULL,
    factory_address TEXT NOT NULL,
    exchange_name TEXT NOT NULL,
    exchange_type TEXT NOT NULL,
    token_a TEXT NOT NULL,
    token_a_symbol TEXT NOT NULL,
    token_a_decimals INTEGER NOT NULL,
    token_b TEXT NOT NULL,
    token_b_symbol TEXT NOT NULL,
    token_b_decimals INTEGER NOT NULL,
    reserve_0 TEXT NOT NULL,
    reserve_1 TEXT NOT NULL,
    fee INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_pools_address_chain ON pools (address, chain);