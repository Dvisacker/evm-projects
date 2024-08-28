use alloy_chains::Chain;
use anyhow::{Error, Result};
use clap::{Args, Parser, Subcommand};
use shared::addressbook::Addressbook;
use shared::amm_utils::{get_filtered_amms, store_uniswap_v3_pools};
use shared::config::get_chain_config;
use shared::token_utils::load_pools_and_fetch_token_data;
use std::sync::Arc;

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
}

#[derive(Args)]
struct FilterArgs {
    #[arg(short, long)]
    chain_id: u64,
    #[arg(short, long)]
    min_usd: f64,
}

#[derive(Args)]
struct GetNamedPoolsArgs {
    #[arg(short, long)]
    chain_id: u64,
}

#[derive(Args)]
struct GetUniswapV3PoolsArgs {
    #[arg(short, long)]
    chain_id: u64,
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
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            let filtered_amms = get_filtered_amms(chain, args.min_usd).await?;
            println!("Filtered AMMs: {:?}", filtered_amms.len());
        }
        Commands::GetNamedPools(args) => {
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            let chain_config = get_chain_config(chain).await;
            let provider = Arc::new(chain_config.ws);

            load_pools_and_fetch_token_data(provider).await?;

            println!("Token data has been fetched and saved to tokens.json");
        }
        Commands::GetUniswapV3Pools(args) => {
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            let chain_config = get_chain_config(chain).await;
            let provider = Arc::new(chain_config.ws);
            let addressbook = Addressbook::load().unwrap();
            let uniswap_v3_factory = addressbook.arbitrum.exchanges.univ3.uniswapv3.factory;

            let from_block = 1090000;
            let to_block = 1100000;
            let step = 10_000;

            // get db_url from env
            let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

            store_uniswap_v3_pools(
                provider,
                uniswap_v3_factory,
                from_block,
                to_block,
                step,
                &db_url,
            )
            .await?;

            println!("Token data has been fetched and saved to tokens.json");
        }
    }

    Ok(())
}
