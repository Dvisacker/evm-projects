-- Add Base exchange records
INSERT INTO exchanges (chain, factory_address, exchange_name, exchange_type) VALUES
-- Base Uniswap V3
('base', '0x33128a8fC17869897dcE68Ed026d694621f6FDfD', 'uniswapv3', 'univ3'),

-- Base PancakeSwap V3
('base', '0x0BFbCF9fa4f9C56B0F40a671Ad40E0805A091865', 'pancakeswapv3', 'univ3'),

-- Base BaseSwap V3
('base', '0x38015D05f4fEC8AFe15D7cc0386a126574e8077B', 'baseswapv3', 'univ3'),

-- Base Aerodrome V2 - TODO: rename aerodrome or simply set it as univ2. 
('base', '0x420DD381b31aEf6683db6B902084cB0FFECe40Da', 'aerodrome', 've33'),

-- Base Alien Base (UniV2 fork)
('base', '0x3E84D913803b02A4a7f027165E8cA42C14C0FdE7', 'alienbase', 'univ2'),

-- Base Slipstream
('base', '0x5e7BB104d84c7CB9B682AaC2F3d509f5F406809A', 'slipstream', 'slipstream'); 

-- Base Uniswap V2
('base', '0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6', 'uniswapv2', 'univ2');

