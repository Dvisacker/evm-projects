-- Add active column back to uni_v2_pools
ALTER TABLE uni_v2_pools ADD COLUMN active BOOLEAN;

-- Add active column back to uni_v3_pools
ALTER TABLE uni_v3_pools ADD COLUMN active BOOLEAN;

-- Add active column back to curve_pools
ALTER TABLE curve_pools ADD COLUMN active BOOLEAN;

-- Add active column back to erc4626_vaults
ALTER TABLE erc4626_vaults ADD COLUMN active BOOLEAN; 