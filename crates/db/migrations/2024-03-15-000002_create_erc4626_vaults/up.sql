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
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_erc4626_vaults_chain_address ON erc4626_vaults (chain, address);

CREATE OR REPLACE FUNCTION update_erc4626_vaults_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_erc4626_vaults_updated_at
BEFORE UPDATE ON erc4626_vaults
FOR EACH ROW
EXECUTE FUNCTION update_erc4626_vaults_updated_at();