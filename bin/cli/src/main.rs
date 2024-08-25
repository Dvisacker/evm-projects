use anyhow::{Error, Result};
use clap::{Args, Parser, Subcommand};
use shared::config::get_chain_config;
use std::str::FromStr;
use std::sync::Arc;
use uni_tri_arb_strategy::addressbook::Addressbook;
// use uni_tri_arb_strategy::uniswapv2::{ExchangeName, UniswapV2Client};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GenerateStrategy,
    // GetPoolAddress(GetPoolAddressArgs),
}

#[derive(Args)]
struct GetPoolAddressArgs {
    #[arg(long)]
    token0: String,
    #[arg(long)]
    token1: String,
    #[arg(short, long)]
    chain_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenerateStrategy => {
            let strategy_parser = generator::parser::StrategyParser::parse();
            strategy_parser.generate()?;
        } // Commands::GetPoolAddress(args) => {
          //     let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
          //     let chain_config = get_chain_config(chain).await;
          //     let ws = chain_config.ws;
          //     let wallet = chain_config.wallet;
          //     let address = wallet.address();
          //     let provider = Arc::new(ws.nonce_manager(address).with_signer(wallet.clone()));

          //     let addressbook = Addressbook::load().unwrap();
          //     let uniswap_client = UniswapV2Client::new(
          //         provider,
          //         &addressbook,
          //         args.chain_id,
          //         ExchangeName::UniswapV2,
          //     );
          //     let address0 = Address::from_str(&args.token0)?;
          //     let address1 = Address::from_str(&args.token1)?;
          //     let pool_address = uniswap_client.get_pool_address(address0, address1).await?;
          //     println!("Pool address: {:?}", pool_address);
          // }
    }

    Ok(())
}
