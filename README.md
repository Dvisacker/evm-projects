## Uni Tri-Arb
A fork of [Artemis](https://github.com/paradigmxyz/artemis) with a uniswap tri-arb strategy.

## Run tests:
```sh
cargo test --all
```

## Run the strategy:
```sh
cargo run --bin artemis -- --chain-id <CHAIN_ID>
```

## Filter AMMs:
```sh
cargo run --bin cli filter --chain-id <CHAIN_ID> --min-usd <MIN_USD>
```