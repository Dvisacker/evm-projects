use std::sync::Arc;

use anyhow::Result;
use artemis_core::{
    collectors::{
        block_collector::BlockCollector, log_collector::LogCollector,
        mempool_collector::MempoolCollector,
    },
    engine::Engine,
    executors::mempool_executor::MempoolExecutor,
    types::{CollectorMap, ExecutorMap},
};
use clap::Parser;
use dotenv::dotenv;
use ethers::{
    abi::Abi,
    prelude::MiddlewareBuilder,
    providers::{Provider, Ws},
    signers::Signer,
    types::{Address, Chain, Filter, H256},
};
use ethers::{providers::Middleware, types::U64};
use shared::config::get_chain_config;
use std::str::FromStr;
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};
use uni_tri_arb_strategy::{
    strategy::UniTriArb,
    types::{Action, Event},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    chain_id: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let filter = filter::Targets::new()
        .with_target("uni-tri-arb", Level::INFO)
        .with_target("artemis_core", Level::INFO);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let args = Args::parse();
    let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");

    let chain_config = get_chain_config(chain).await;
    let ws = chain_config.ws;
    let wallet = chain_config.wallet;
    let address = wallet.address();
    let provider = Arc::new(ws.nonce_manager(address).with_signer(wallet.clone()));

    let mut engine: Engine<Event, Action> = Engine::default();

    let block_collector = Box::new(BlockCollector::new(provider.clone()));
    let block_collector = CollectorMap::new(block_collector, Event::NewBlock);
    engine.add_collector(Box::new(block_collector));

    let block_number = provider.get_block_number().await?;

    let filter = Filter::new()
        .from_block(block_number.saturating_sub(U64::from(100)))
        .event("Swap(address,uint256,uint256,uint256,uint256,address)");

    let log_collector = Box::new(LogCollector::new(provider.clone(), filter));
    let log_collector = CollectorMap::new(log_collector, |log| Event::UniswapV2Swap(log));
    engine.add_collector(Box::new(log_collector));

    let strategy = UniTriArb::new(Arc::new(provider.clone()), wallet);
    engine.add_strategy(Box::new(strategy));

    let mempool_executor = Box::new(MempoolExecutor::new(provider.clone()));
    let mempool_executor = ExecutorMap::new(mempool_executor, |action: Action| match action {
        Action::SubmitTx(tx) => Some(tx),
        _ => None,
    });
    engine.add_executor(Box::new(mempool_executor));

    // Start engine.
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("res: {:?}", res);
        }
    }

    Ok(())
}
