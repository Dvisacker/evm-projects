# Artemis MEV Framework

Artemis is a high-performance MEV (Maximal Extractable Value) bot framework written in Rust. It provides a modular and extensible architecture for building, testing, and deploying MEV strategies across different blockchain networks.

## Features

- 🚀 High-performance Rust implementation
- 🔌 Modular architecture with pluggable components
- 💱 Support for multiple AMMs and DEX protocols
- 🔄 Customizable execution strategies
- 📊 Database integration for transaction tracking
- 🔐 Secure provider management
- 🤖 CLI tools for bot management and testing

## Project Structure

```
artemis/
├── bin/                    # Binary crates (bot, cli, swap)
├── crates/                 # Core library crates
│   ├── addressbook/       # Address management
│   ├── amms/              # AMM integrations
│   ├── artemis-core/      # Core framework
│   ├── bindings/          # Contract bindings
│   ├── db/                # Database interactions
│   ├── encoder-client/    # Transaction encoding
│   ├── executor-binding/  # Execution handlers
│   ├── odos-client/       # Odos protocol integration
│   ├── provider/          # Blockchain provider
│   ├── shared/            # Shared utilities
│   ├── strategies/        # MEV strategies
│   └── types/             # Common types
├── contracts/             # Smart contracts
└── docker/               # Docker configuration
```

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Docker (optional)
- Access to Ethereum node(s)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/your-username/artemis.git
cd artemis
```

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
2. Implement the strategy traits from `artemis-core`
3. Add tests and documentation
4. Register the strategy in the bot configuration

### Testing

Run the test suite:
```bash
cargo test --all
```

## License

This project is dual-licensed under:
- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## Disclaimer

This software is for educational purposes only. Do not use it to exploit blockchain networks or engage in harmful MEV practices. Users are responsible for ensuring compliance with all applicable laws and regulations. 