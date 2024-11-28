pub mod cmd;
use alloy::primitives::Address;
use alloy_chains::{Chain, NamedChain};
use clap::{Args, Parser, Subcommand};
use config::get_chain_config;
use eyre::Error;
use shared::amm_utils::{activate_pools, get_amm_value};
use shared::token_helpers::load_pools_and_fetch_token_data;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use types::exchange::ExchangeName;
use types::token::TokenIsh;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GetNamedPools(GetNamedPoolsArgs),
    GetAerodromePools,
    GetUniswapV3Pools(GetUniswapV3PoolsArgs),
    GetUniswapV2Pools(GetUniswapV2PoolsArgs),
    GetAMMValue(GetAMMValueArgs),
    ActivatePools(ActivatePoolsArgs),
    GetContractCreationBlock(GetContractCreationBlockArgs),
    Bridge {
        #[arg(short, long)]
        from_chain: NamedChain,
        #[arg(short, long)]
        to_chain: NamedChain,
        #[arg(short, long)]
        token: TokenIsh,
        #[arg(short, long)]
        amount_in: String,
    },
    CrossChainSwap(CrossChainSwapArgs),
}

#[derive(Args)]
struct CrossChainSwapArgs {
    #[arg(short, long)]
    origin_chain: NamedChain,
    #[arg(short, long)]
    destination_chain: NamedChain,
    #[arg(short, long)]
    origin_token: TokenIsh,
    #[arg(short, long)]
    bridge_token: TokenIsh,
    #[arg(short, long)]
    destination_token: TokenIsh,
    #[arg(short, long)]
    amount_in: String,
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
struct GetUniswapV3Pools2Args {
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

#[derive(Args)]
struct GetContractCreationBlockArgs {
    #[arg(short, long)]
    chain_id: u64,
    #[arg(long)]
    contract_address: String,
    #[arg(long)]
    start_block: Option<u64>,
    #[arg(long)]
    end_block: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    dotenv::dotenv().ok();

    // Updated tracing configuration
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"))
        .add_directive("generalized_arb_strategy=info".parse().unwrap())
        .add_directive("engine=info".parse().unwrap())
        .add_directive("shared=info".parse().unwrap())
        .add_directive("amms_rs=info".parse().unwrap());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(env_filter)
        .init();

    match &cli.command {
        Commands::GetAerodromePools => {
            cmd::get_aerodrome_pools_command().await?;
        }
        Commands::GetNamedPools(args) => {
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            let chain_config = get_chain_config(chain).await;
            let provider = Arc::new(chain_config.ws);
            load_pools_and_fetch_token_data(provider).await?;
            info!("Token data has been fetched and saved to tokens.json");
        }
        Commands::GetUniswapV3Pools(args) => {
            cmd::get_uniswap_v3_pools_command(
                args.chain_id,
                args.exchange,
                args.from_block,
                args.step,
            )
            .await?;
        }
        Commands::GetUniswapV2Pools(args) => {
            cmd::get_uniswap_v2_pools_command(args.chain_id, args.exchange).await?;
        }
        Commands::ActivatePools(args) => {
            let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            activate_pools(chain, args.exchange, args.min_usd, &db_url).await?;
        }
        Commands::GetAMMValue(args) => {
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            let pool_address = Address::from_str(&args.pool_address).expect("Invalid pool address");
            let _amm_value = get_amm_value(chain, pool_address).await?;
            // info!("AMM value: {:?}", amm_value);
        }
        Commands::GetContractCreationBlock(args) => {
            cmd::get_contract_creation_block_command(
                args.chain_id,
                &args.contract_address,
                args.start_block,
                args.end_block,
            )
            .await?;
        }
        Commands::Bridge {
            from_chain,
            to_chain,
            token,
            amount_in,
        } => {
            cmd::bridge_command(from_chain, to_chain, token, amount_in).await?;
        }
        Commands::CrossChainSwap(args) => {
            cmd::cross_chain_swap_command(
                args.origin_chain,
                args.destination_chain,
                args.origin_token.clone(),
                args.bridge_token.clone(),
                args.destination_token.clone(),
                &args.amount_in,
            )
            .await?;
        }
    }

    Ok(())
}
