use alloy_chains::Chain;
use anyhow::{Error, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use shared::addressbook::Addressbook;
use shared::amm_utils::{filter_amms, store_uniswap_v2_pools, store_uniswap_v3_pools};
use shared::config::get_chain_config;
use shared::token_utils::load_pools_and_fetch_token_data;
use std::sync::Arc;
use tracing::{error, info};

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

#[derive(Clone, ValueEnum)]
enum ExchangeName {
    UniswapV3,
    SushiswapV3,
    UniswapV2,
    SushiswapV2,
}

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    dotenv::dotenv().ok();

    match &cli.command {
        Commands::GenerateStrategy => {
            let strategy_parser = generator::parser::StrategyParser::parse();
            strategy_parser.generate()?;
        }
        Commands::Filter(args) => {
            let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            let filtered_amms = filter_amms(chain, args.min_usd, &db_url).await?;
            info!("Filtered AMMs: {:?}", filtered_amms.len());
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

            let factory_address = match args.exchange {
                ExchangeName::UniswapV3 => addressbook.arbitrum.exchanges.univ3.uniswapv3.factory,
                ExchangeName::SushiswapV3 => {
                    addressbook.arbitrum.exchanges.univ3.sushiswapv3.factory
                }
                _ => panic!("Choose a uniswap v3 exchange"),
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

            // info!(
            //     "{:?} pools have been fetched and stored in the database.",
            //     args.exchange
            // );
        }
        Commands::GetUniswapV2Pools(args) => {
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            let chain_config = get_chain_config(chain).await;
            let provider = Arc::new(chain_config.ws);
            let addressbook = Addressbook::load().unwrap();

            let factory_address = match args.exchange {
                ExchangeName::UniswapV2 => addressbook.arbitrum.exchanges.univ2.uniswapv2.factory,
                ExchangeName::SushiswapV2 => {
                    addressbook.arbitrum.exchanges.univ2.sushiswapv2.factory
                }
                _ => panic!("Choose a uniswap v2 exchange"),
            };

            let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

            store_uniswap_v2_pools(provider.clone(), chain, factory_address, &db_url).await?;
        }
    }

    Ok(())
}
