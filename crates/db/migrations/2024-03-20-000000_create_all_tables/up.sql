-- Create trigger for updating 'updated_at' column
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create exchanges table
CREATE TABLE exchanges (
    id SERIAL PRIMARY KEY,
    chain VARCHAR NOT NULL,
    factory_address VARCHAR,
    exchange_name VARCHAR NOT NULL,
    exchange_type VARCHAR NOT NULL
);



-- Create uni_v2_pools table
CREATE TABLE uni_v2_pools (
    id SERIAL PRIMARY KEY,
    address VARCHAR NOT NULL,
    chain VARCHAR NOT NULL,
    token_a VARCHAR NOT NULL,
    token_a_symbol VARCHAR NOT NULL,
    token_a_decimals INTEGER NOT NULL,
    token_b VARCHAR NOT NULL,
    token_b_symbol VARCHAR NOT NULL,
    token_b_decimals INTEGER NOT NULL,
    reserve_0 VARCHAR NOT NULL,
    reserve_1 VARCHAR NOT NULL,
    fee INTEGER NOT NULL,
    exchange_name VARCHAR,
    exchange_type VARCHAR,
    factory_address VARCHAR,
    active BOOLEAN,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (address, chain)
);

CREATE UNIQUE INDEX idx_uni_v2_pools_chain_address ON uni_v2_pools (chain, address);

CREATE TRIGGER update_uni_v2_pools_updated_at
BEFORE UPDATE ON uni_v2_pools
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Create uni_v3_pools table
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
    liquidity VARCHAR,
    sqrt_price VARCHAR,
    fee INTEGER,
    tick INTEGER,
    tick_spacing INTEGER,
    tick_bitmap JSONB,
    ticks JSONB,
    exchange_name VARCHAR,
    exchange_type VARCHAR,
    factory_address VARCHAR,
    active BOOLEAN,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (address, chain)
);

CREATE TRIGGER update_uni_v3_pools_updated_at
BEFORE UPDATE ON uni_v3_pools
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE UNIQUE INDEX idx_uni_v3_pools_chain_address ON uni_v3_pools (chain, address);

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
    active BOOLEAN,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_curve_pools_chain_address ON curve_pools (chain, address);

CREATE TRIGGER trigger_update_curve_pools_updated_at
BEFORE UPDATE ON curve_pools
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE erc4626_vaults (
    id SERIAL PRIMARY KEY,
    address VARCHAR NOT NULL,
    chain VARCHAR NOT NULL,
    vault_token VARCHAR NOT NULL,
    vault_token_decimals INTEGER NOT NULL,
    vault_token_symbol VARCHAR NOT NULL,
    asset_token VARCHAR NOT NULL,
    asset_token_decimals INTEGER NOT NULL,
    asset_token_symbol VARCHAR NOT NULL,
    vault_reserve VARCHAR NOT NULL,
    asset_reserve VARCHAR NOT NULL,
    deposit_fee INTEGER NOT NULL,
    withdraw_fee INTEGER NOT NULL,
    exchange_name VARCHAR,
    exchange_type VARCHAR,
    active BOOLEAN,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_erc4626_vaults_chain_address ON erc4626_vaults (chain, address);


CREATE TRIGGER trigger_update_erc4626_vaults_updated_at
BEFORE UPDATE ON erc4626_vaults
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();


-- Seed exchanges table
INSERT INTO exchanges (chain, factory_address, exchange_name, exchange_type) VALUES
-- Arbitrum
('arbitrum', '0xf1D7CC64Fb4452F05c498126312eBE29f30Fbcf9', 'uniswapv2', 'univ2'),
('arbitrum', '0xc35DADB65012eC5796536bD9864eD8773aBc74C4', 'sushiswapv2', 'univ2'),
('arbitrum', '0x1F98431c8aD98523631AE4a59f267346ea31F984', 'uniswapv3', 'univ3'),
('arbitrum', '0x1af415a1EbA07a4986a52B6f2e7dE7003D82231e', 'sushiswapv3', 'univ3'),
('arbitrum', '0x1a3c9B1d2F0529D97f2afC5136Cc23e58f1FD35B', 'camelotv3', 'univ3'),
('arbitrum', '0xAA2cd7477c451E703f3B9Ba5663334914763edF8', 'ramsesv2', 'univ3'),
('arbitrum', '0x0BFbCF9fa4f9C56B0F40a671Ad40E0805A091865', 'pancakeswapv3', 'univ3'),

-- Optimism
('optimism', '0x0c3c1c532F1e39EdF36BE9Fe0bE1410313E074Bf', 'uniswapv2', 'univ2'),
('optimism', '0xFbc12984689e5f15626Bad03Ad60160Fe98B303C', 'sushiswapv2', 'univ2'),
('optimism', '0x1F98431c8aD98523631AE4a59f267346ea31F984', 'uniswapv3', 'univ3'),
('optimism', '0xFbc12984689e5f15626Bad03Ad60160Fe98B303C', 'sushiswapv3', 'univ3'),

-- Mainnet
('mainnet', '0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f', 'uniswapv2', 'univ2'),
('mainnet', '0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac', 'sushiswapv2', 'univ2'),
('mainnet', '0x1F98431c8aD98523631AE4a59f267346ea31F984', 'uniswapv3', 'univ3'),
('mainnet', '0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac', 'sushiswapv3', 'univ3');

