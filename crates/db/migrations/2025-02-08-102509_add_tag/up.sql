-- Your SQL goes here

-- Create tags table with name as primary key
CREATE TABLE tags (
    name VARCHAR PRIMARY KEY
);

-- Add tag column to existing pool tables
ALTER TABLE uni_v2_pools
ADD COLUMN tag VARCHAR REFERENCES tags(name) ON DELETE SET NULL;

ALTER TABLE uni_v3_pools
ADD COLUMN tag VARCHAR REFERENCES tags(name) ON DELETE SET NULL;

ALTER TABLE curve_pools
ADD COLUMN tag VARCHAR REFERENCES tags(name) ON DELETE SET NULL;

ALTER TABLE erc4626_vaults
ADD COLUMN tag VARCHAR REFERENCES tags(name) ON DELETE SET NULL;

DROP INDEX IF EXISTS idx_uni_v2_pools_chain_address;
DROP INDEX IF EXISTS idx_uni_v3_pools_chain_address;
DROP INDEX IF EXISTS idx_curve_pools_chain_address;
DROP INDEX IF EXISTS idx_erc4626_vaults_chain_address;

-- Add new unique constraints
ALTER TABLE uni_v2_pools 
ADD CONSTRAINT uni_v2_pools_address_chain_tag_key UNIQUE (address, chain, tag);

ALTER TABLE uni_v3_pools 
ADD CONSTRAINT uni_v3_pools_address_chain_tag_key UNIQUE (address, chain, tag);

ALTER TABLE curve_pools 
ADD CONSTRAINT curve_pools_address_chain_tag_key UNIQUE (address, chain, tag);

ALTER TABLE erc4626_vaults 
ADD CONSTRAINT erc4626_vaults_address_chain_tag_key UNIQUE (address, chain, tag);

-- Create new indices for faster lookups
CREATE UNIQUE INDEX idx_uni_v2_pools_chain_address_tag ON uni_v2_pools (chain, address, tag);
CREATE UNIQUE INDEX idx_uni_v3_pools_chain_address_tag ON uni_v3_pools (chain, address, tag);
CREATE UNIQUE INDEX idx_curve_pools_chain_address_tag ON curve_pools (chain, address, tag);
CREATE UNIQUE INDEX idx_erc4626_vaults_chain_address_tag ON erc4626_vaults (chain, address, tag);