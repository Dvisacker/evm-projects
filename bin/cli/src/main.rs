use alloy_chains::Chain;
use anyhow::{Error, Result};
use clap::{Args, Parser, Subcommand};
use shared::amm_utils::get_filtered_amms;

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
}

#[derive(Args)]
struct FilterArgs {
    #[arg(short, long)]
    chain_id: u64,
    #[arg(short, long)]
    min_usd: f64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenerateStrategy => {
            let strategy_parser = generator::parser::StrategyParser::parse();
            strategy_parser.generate()?;
        }
        Commands::Filter(args) => {
            let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
            let filtered_amms = get_filtered_amms(chain, args.min_usd).await?;

            println!("Filtered AMMs above {} USD threshold:", args.min_usd);
            for amm in filtered_amms {
                println!("{:?}", amm);
            }
        }
    }

    Ok(())
}
