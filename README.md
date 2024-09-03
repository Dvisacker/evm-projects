## Generalized Arbitrage Cycle Bot

This bot computes cycles on relevant different pools and calculates the profit for each cycle. 

Currently supporting uniswap v2, v3, sushiswap v2, sushiswap v3, camelot v3 etc. 

The engine is a fork of [Artemis](https://github.com/paradigmxyz/artemis) with a uniswap tri-arb strategy.

A fork of [amms-rs](https://github.com/darkforestry/amms-rs) is used for AMM logic (with additional/modified functionality)

## Run tests:
```sh
cargo test --all
```


