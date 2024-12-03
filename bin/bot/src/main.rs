mod cli;
mod strategies;

use alloy_chains::Chain;
use clap::Parser;
use cli::{Args, StrategyType};
use dotenv::dotenv;
use eyre::Result;
use provider::get_default_signer_provider;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"))
        .add_directive("generalized_arb_strategy=info".parse().unwrap())
        .add_directive("base_arb_strategy=info".parse().unwrap())
        .add_directive("engine=info".parse().unwrap())
        .add_directive("shared=info".parse().unwrap())
        .add_directive("amms_rs=info".parse().unwrap());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(env_filter)
        .init();

    let args = Args::parse();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
    let provider = get_default_signer_provider(chain).await;

    match args.strategy {
        StrategyType::GeneralizedArb => {
            info!("Initializing GeneralizedArb strategy...");
            let engine =
                strategies::init_generalized_arbitrage_bot(chain, provider.clone(), db_url);

            if let Ok(mut set) = engine.run().await {
                while let Some(res) = set.join_next().await {
                    info!("res: {:?}", res);
                }
            }
        }
        StrategyType::BaseArb => {
            info!("Initializing BaseArb strategy...");
            let engine = strategies::init_base_arbitrage_bot(chain, provider.clone(), db_url);

            if let Ok(mut set) = engine.run().await {
                while let Some(res) = set.join_next().await {
                    info!("res: {:?}", res);
                }
            }
        }
    }

    Ok(())
}
