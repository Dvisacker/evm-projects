## Generalized Arbitrage Cycle Bot

This bot computes cycles on relevant different pools and calculates the profit for each cycle. 

Currently supporting uniswap v2, v3, sushiswap v2, sushiswap v3, camelot v3 etc. 

The engine is a fork of [Artemis](https://github.com/paradigmxyz/artemis) with a uniswap tri-arb strategy.

A fork of [amms-rs](https://github.com/darkforestry/amms-rs) is used for AMM logic (with additional/modified functionality)

## Run tests:
```sh
cargo test --all
```

## Requirements:
- `rustc` >= 1.74
- `foundry` >= 0.2.0
- `sqlite`

## Setup DB: 

```
just setup-db
```

## Run bot (Only arbitrum supported for now)

```
just run-bot-arbitrum
```

## Activate pools

When running the bot, pools will be added to the database. 
Only pools that are activated will be scanned for arbitrage cycles.

To activate pools, use the `activate-pools` command.

```
just activate-pools 42161 uniswap-v3
```

This will activate all pools with at least 100k USD of liquidity.

## TODO
- [ ] Fix MKR token swap events






