# EVM crates 

Various crates and binaries to trade and interact with EVM chains.

The pub/sub engine is originally forked from [Artemis](https://github.com/paradigmxyz/artemis).

## Project Structure

```
/
├── bin/                    # Binary crates (bot, cli, swap)
├── crates/                 # Core library crates
│   ├── addressbook/       # Address book to easily fetch known addresses
│   ├── amms/              # Fork of amms-rs using alloy and with added support for curve, ramses, etc.  
│   ├── engine/            # Pub/sub engine. Based on artemis.
│   ├── db/                # Database models and queries
│   ├── pool-manager/      # Pool storage manager for example for tagging and saving liquidity pools to db
│   ├── tx-executor/       # Bundled tx encoder/executor
│   ├── tx-simulator/      # Swap simulator
│   ├── odos-client/       # Odos aggregator client
│   ├── codex-client/      # Codex API client
│   ├── lifi-client/       # Lifi bridge client
│   ├── metadata/          # Functions to fetch aggregated chain data
│   ├── provider/          # Provider utils
│   ├── shared/            # Shared utils
│   └── types/             # Shared types
└── docker/               # Docker configuration
```

## Getting Started

### Prerequisites

- Rust 1.82 or higher
- Docker (optional)
- Access to Ethereum nodes or providers

### Installation

2. Copy the example environment file:
```bash
cp .env.example .env
```

3. Configure your environment variables in `.env`

4. Build the project:
```bash
cargo build --release
```

### Running the Bot

1. Using the CLI:
```bash
cargo run --bin cli -- --help
```

2. Using Docker:
```bash
docker-compose up -d
```

## Configuration

The bot can be configured through:
- Environment variables (`.env` file)
- Command-line arguments
- Configuration files

See `.env.example` for available configuration options.

## Development

### Adding New Strategies

1. Create a new crate in `crates/strategies/`
2. Implement the strategy traits from `engine`


## Pool Storage Example CLI commands:

### Get all Uniswap V3 pools on Base
```bash
cargo run --bin cli get-uniswap-v3-pools --chain-id 8453 --exchange uniswap-v3 --from-block 0 --step 5000 --tag univ3-base
```

### Get the most traded Uniswap V3 pools on Base
```bash
cargo run --bin cli get-most-traded-pools --chain-id 8453 --exchange uniswap-v3 --limit 100 --min-volume 100000 --tag univ3-base-most-traded
```
