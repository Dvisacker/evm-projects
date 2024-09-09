DELETE FROM exchanges WHERE 
    chain IN ('arbitrum', 'optimism', 'mainnet') AND 
    exchange_name IN ('uniswapv2', 'sushiswapv2', 'uniswapv3', 'sushiswapv3', 'camelotv3', 'ramsesv2', 'pancakeswapv3');