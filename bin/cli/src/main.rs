use alloy::primitives::Address;
use alloy_chains::{Chain, ChainKind, NamedChain};
use anyhow::{Error, Result};
use clap::{Args, Parser, Subcommand};
use shared::addressbook::Addressbook;
use shared::amm_utils::{
    activate_pools, get_amm_value, store_uniswap_v2_pools, store_uniswap_v3_pools,
};
use shared::config::get_chain_config;
use shared::token_utils::load_pools_and_fetch_token_data;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use types::exchange::ExchangeName;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GenerateStrategy,
    Filter(FilterArgs),
    GetNamedPools(GetNamedPoolsArgs),
    GetUniswapV3Pools(GetUniswapV3PoolsArgs),
    GetUniswapV2Pools(GetUniswapV2PoolsArgs),
    GetAMMValue(GetAMMValueArgs),
    ActivatePools(ActivatePoolsArgs),
}

#[derive(Args)]
struct FilterArgs {
    #[arg(short, long)]
    chain_id: u64,
    #[arg(short, long, default_value = "10000")]
    min_usd: f64,
}

#[derive(Args)]
struct GetNamedPoolsArgs {
    #[arg(short, long)]
    chain_id: u64,
}

// #[derive(Clone, ValueEnum)]
// enum ExchangeName {
//     UniswapV2,
//     SushiswapV2,
//     UniswapV3,
//     SushiswapV3,
//     CamelotV3,
//     RamsesV2,
//     PancakeswapV3,
//     Unknown,
// }

#[derive(Args)]
struct GetUniswapV3PoolsArgs {
    #[arg(short, long)]
    chain_id: u64,
    #[arg(long, default_value = "0")]
    from_block: u64,
    #[arg(long, default_value = "1000000")]
    to_block: u64,
    #[arg(long, default_value = "10000")]
    step: u64,
    #[arg(long, value_enum)]
    exchange: ExchangeName,
}

#[derive(Args)]
struct GetUniswapV2PoolsArgs {
    #[arg(short, long)]
    chain_id: u64,
    #[arg(long, value_enum)]
    exchange: ExchangeName,
}

#[derive(Args)]
struct ActivatePoolsArgs {
    #[arg(short, long)]
    chain_id: u64,
    #[arg(short, long)]
    min_usd: f64,
    #[arg(short, long)]
    exchange: ExchangeName,
}

#[derive(Args)]
struct GetAMMValueArgs {
    #[arg(short, long)]
    chain_id: u64,
    #[arg(short, long)]
    pool_address: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    dotenv::dotenv().ok();

    // Updated tracing configuration
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"))
        .add_directive("uni_tri_arb_strategy=info".parse().unwrap())
        .add_directive("artemis_core=info".parse().unwrap())
        .add_directive("shared=info".parse().unwrap())
        .add_directive("amms_rs=info".parse().unwrap());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(env_filter)
        .init();

    match &cli.command {
        Commands::GenerateStrategy => {
            let strategy_parser = generator::parser::StrategyParser::parse();
            strategy_parser.generate()?;
        }
        Commands::Filter(args) => {
            // let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
            // let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            // let filtered_amms = filter_amms(chain, args.min_usd, &db_url).await?;
            // info!("Filtered AMMs: {:?}", filtered_amms.len());
        }
        Commands::GetNamedPools(args) => {
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            let chain_config = get_chain_config(chain).await;
            let provider = Arc::new(chain_config.ws);

            load_pools_and_fetch_token_data(provider).await?;

            info!("Token data has been fetched and saved to tokens.json");
        }
        Commands::GetUniswapV3Pools(args) => {
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            let chain_config = get_chain_config(chain).await;
            let provider = Arc::new(chain_config.ws);
            let addressbook = Addressbook::load().unwrap();

            let factory_address = match chain.kind() {
                ChainKind::Named(NamedChain::Arbitrum) => match args.exchange {
                    ExchangeName::UniswapV3 => {
                        addressbook.arbitrum.exchanges.univ3.uniswapv3.factory
                    }
                    ExchangeName::SushiswapV3 => {
                        addressbook.arbitrum.exchanges.univ3.sushiswapv3.factory
                    }
                    ExchangeName::CamelotV3 => {
                        addressbook.arbitrum.exchanges.univ3.camelotv3.factory
                    }
                    ExchangeName::RamsesV2 => addressbook.arbitrum.exchanges.univ3.ramsesv2.factory,
                    ExchangeName::PancakeswapV3 => {
                        addressbook.arbitrum.exchanges.univ3.pancakeswapv3.factory
                    }
                    _ => panic!("Choose a uniswap v3 exchange"),
                },
                ChainKind::Named(NamedChain::Mainnet) => match args.exchange {
                    ExchangeName::UniswapV3 => {
                        addressbook.mainnet.exchanges.univ3.uniswapv3.factory
                    }
                    ExchangeName::SushiswapV3 => {
                        addressbook.mainnet.exchanges.univ3.sushiswapv3.factory
                    }
                    ExchangeName::PancakeswapV3 => {
                        addressbook.mainnet.exchanges.univ3.pancakeswapv3.factory
                    }
                    _ => panic!("Choose a uniswap v3 exchange"),
                },
                _ => panic!("Unsupported chain"),
            };

            let from_block = args.from_block;
            let to_block = args.to_block;
            let step = args.step;
            let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

            for block in (from_block..=to_block).step_by(step as usize) {
                info!(
                    "Fetching pools from block {:?} to {:?}",
                    block,
                    block + step - 1
                );
                store_uniswap_v3_pools(
                    provider.clone(),
                    chain,
                    factory_address,
                    block,
                    block + step - 1,
                    step,
                    &db_url,
                )
                .await?;
            }
        }
        Commands::GetUniswapV2Pools(args) => {
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            let chain_config = get_chain_config(chain).await;
            let provider = Arc::new(chain_config.ws);
            let addressbook = Addressbook::load().unwrap();

            let factory_address = match chain.kind() {
                ChainKind::Named(NamedChain::Arbitrum) => match args.exchange {
                    ExchangeName::UniswapV2 => {
                        addressbook.arbitrum.exchanges.univ2.uniswapv2.factory
                    }
                    ExchangeName::SushiswapV2 => {
                        addressbook.arbitrum.exchanges.univ2.sushiswapv2.factory
                    }
                    _ => panic!("Choose a uniswap v2 type exchange"),
                },
                ChainKind::Named(NamedChain::Mainnet) => match args.exchange {
                    ExchangeName::UniswapV2 => {
                        addressbook.mainnet.exchanges.univ2.uniswapv2.factory
                    }
                    ExchangeName::SushiswapV2 => {
                        addressbook.mainnet.exchanges.univ2.sushiswapv2.factory
                    }
                    _ => panic!("Choose a uniswap v2 type exchange"),
                },
                _ => panic!("Unsupported chain"),
            };

            info!("Downloading pools from {:?}", factory_address);
            let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
            store_uniswap_v2_pools(
                provider.clone(),
                chain,
                args.exchange,
                factory_address,
                &db_url,
            )
            .await?;
        }
        Commands::ActivatePools(args) => {
            let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            activate_pools(chain, args.exchange, args.min_usd, &db_url).await?;
        }
        Commands::GetAMMValue(args) => {
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            let pool_address = Address::from_str(&args.pool_address).expect("Invalid pool address");
            let amm_value = get_amm_value(chain, pool_address).await?;
            // info!("AMM value: {:?}", amm_value);
        }
    }

    Ok(())
}
