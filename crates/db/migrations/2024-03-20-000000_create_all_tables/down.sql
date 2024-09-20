-- Drop tables
DROP TABLE IF EXISTS uni_v3_pools;
DROP TABLE IF EXISTS uni_v2_pools;
DROP TABLE IF EXISTS exchanges;
DROP TABLE IF EXISTS erc4626_vaults;
DROP TABLE IF EXISTS curve_pools;

-- Drop trigger
DROP TRIGGER IF EXISTS update_uni_v3_pools_updated_at ON uni_v3_pools;
DROP TRIGGER IF EXISTS update_uni_v2_pools_updated_at ON uni_v2_pools;
DROP TRIGGER IF EXISTS update_curve_pools_updated_at ON curve_pools;
DROP TRIGGER IF EXISTS update_erc4626_vaults_updated_at ON erc4626_vaults;

-- Drop function
DROP FUNCTION IF EXISTS update_updated_at_column();
