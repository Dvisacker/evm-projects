-- Drop new unique indices first
DROP INDEX IF EXISTS idx_uni_v2_pools_chain_address_tag;
DROP INDEX IF EXISTS idx_uni_v3_pools_chain_address_tag;
DROP INDEX IF EXISTS idx_curve_pools_chain_address_tag;
DROP INDEX IF EXISTS idx_erc4626_vaults_chain_address_tag;

-- Drop new unique constraints
ALTER TABLE uni_v2_pools DROP CONSTRAINT uni_v2_pools_address_chain_tag_key;
ALTER TABLE uni_v3_pools DROP CONSTRAINT uni_v3_pools_address_chain_tag_key;
ALTER TABLE curve_pools DROP CONSTRAINT curve_pools_address_chain_tag_key;
ALTER TABLE erc4626_vaults DROP CONSTRAINT erc4626_vaults_address_chain_tag_key;

-- Drop tag columns from pool tables
ALTER TABLE uni_v2_pools DROP COLUMN tag;
ALTER TABLE uni_v3_pools DROP COLUMN tag;
ALTER TABLE curve_pools DROP COLUMN tag;
ALTER TABLE erc4626_vaults DROP COLUMN tag;

CREATE UNIQUE INDEX idx_uni_v2_pools_chain_address ON uni_v2_pools (chain, address);
CREATE UNIQUE INDEX idx_uni_v3_pools_chain_address ON uni_v3_pools (chain, address);
CREATE UNIQUE INDEX idx_curve_pools_chain_address ON curve_pools (chain, address);
CREATE UNIQUE INDEX idx_erc4626_vaults_chain_address ON erc4626_vaults (chain, address);

-- Drop tags table last (since other tables reference it)
DROP TABLE tags;