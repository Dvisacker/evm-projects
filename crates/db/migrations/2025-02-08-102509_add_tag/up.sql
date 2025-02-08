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

-- Create indexes for faster lookups
CREATE INDEX idx_uni_v2_pools_tag ON uni_v2_pools(tag);
CREATE INDEX idx_uni_v3_pools_tag ON uni_v3_pools(tag);
CREATE INDEX idx_curve_pools_tag ON curve_pools(tag);