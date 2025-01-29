# EVM projects 

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
│   ├── bindings/          # Contract bindings (to be removed)
│   ├── db/                # Database models and queries
│   ├── pool-manager/      # Pool storage manager for example for fetching/flagging pools in the db
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
