pub mod args;
pub mod cmd;

use alloy_chains::Chain;
use clap::{Parser, Subcommand};
use eyre::Error;
use provider::get_basic_provider;
use shared::token_helpers::load_pools_and_fetch_token_data;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::args::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GetNamedPools(GetNamedPoolsArgs),
    GetAerodromePools(GetAerodromePoolsArgs),
    GetUniswapV3Pools(GetUniswapV3PoolsArgs),
    GetUniswapV2Pools(GetUniswapV2PoolsArgs),
    GetAMMValue(GetAMMValueArgs),
    ActivatePools(ActivatePoolsArgs),
    GetContractCreationBlock(GetContractCreationBlockArgs),
    Bridge(BridgeArgs),
    CrossChainSwap(CrossChainSwapArgs),
    WrapEth(WrapEthArgs),
    UnwrapEth(UnwrapEthArgs),
    Withdraw(WithdrawArgs),
    GetMostTradedPools(GetMostTradedPoolsArgs),
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
        Commands::GetAerodromePools(args) => {
            cmd::get_aerodrome_pools_command(args.tag.tag.clone()).await?;
        }
        Commands::GetNamedPools(args) => {
            let chain = Chain::try_from(args.chain.chain_id).expect("Invalid chain ID");
            let provider = get_basic_provider(chain).await;
            load_pools_and_fetch_token_data(provider).await?;
            info!("Token data has been fetched and saved to tokens.json");
        }
        Commands::GetUniswapV3Pools(args) => {
            cmd::get_uniswap_v3_pools_command(
                args.chain.chain_id,
                args.exchange.exchange,
                args.block_range.from_block,
                args.block_range.step,
                args.tag.tag.clone(),
            )
            .await?;
        }
        Commands::GetUniswapV2Pools(args) => {
            cmd::get_uniswap_v2_pools_command(
                args.chain.chain_id,
                args.exchange.exchange,
                args.tag.tag.clone(),
            )
            .await?;
        }
        Commands::GetMostTradedPools(args) => {
            cmd::get_most_traded_pools_command(
                args.chain.chain_id,
                args.exchange.exchange,
                args.limit,
                args.min_volume,
                args.tag.tag.clone(),
            )
            .await?;
        }
        Commands::ActivatePools(args) => {
            cmd::activate_pools_command(args.chain.chain_id, args.exchange.exchange, args.min_usd)
                .await?;
        }
        Commands::GetAMMValue(args) => {
            cmd::get_amm_value_command(args.chain.chain_id, &args.pool_address).await?;
        }
        Commands::GetContractCreationBlock(args) => {
            cmd::get_contract_creation_block_command(
                args.chain.chain_id,
                &args.contract_address,
                args.start_block,
                args.end_block,
            )
            .await?;
        }
        Commands::Bridge(args) => {
            cmd::bridge_command(
                &args.from_chain,
                &args.to_chain,
                &args.token,
                &args.amount_in,
            )
            .await?;
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
        Commands::WrapEth(args) => {
            cmd::wrap_eth_command(args.chain.chain_id, &args.amount).await?;
        }
        Commands::UnwrapEth(args) => {
            cmd::unwrap_eth_command(args.chain.chain_id, &args.amount).await?;
        }
        Commands::Withdraw(args) => {
            cmd::withdraw_command(args.chain.chain_id).await?;
        }
    }

    Ok(())
}
