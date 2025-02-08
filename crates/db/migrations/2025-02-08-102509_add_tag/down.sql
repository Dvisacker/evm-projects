-- Drop indexes first
DROP INDEX idx_uni_v2_pools_tag;
DROP INDEX idx_uni_v3_pools_tag;
DROP INDEX idx_curve_pools_tag;

-- Drop tag columns from pool tables
ALTER TABLE uni_v2_pools DROP COLUMN tag;
ALTER TABLE uni_v3_pools DROP COLUMN tag;
ALTER TABLE curve_pools DROP COLUMN tag;

-- Drop tags table last (since other tables reference it)
DROP TABLE tags;