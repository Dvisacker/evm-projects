-- Remove active column from uni_v2_pools
ALTER TABLE uni_v2_pools DROP COLUMN active;

-- Remove active column from uni_v3_pools
ALTER TABLE uni_v3_pools DROP COLUMN active;

-- Remove active column from curve_pools
ALTER TABLE curve_pools DROP COLUMN active;

-- Remove active column from erc4626_vaults
ALTER TABLE erc4626_vaults DROP COLUMN active; 