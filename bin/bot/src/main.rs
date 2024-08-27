use std::sync::Arc;

use alloy::{eips::BlockNumberOrTag, rpc::types::Filter, sol_types::SolEvent};
use alloy_chains::Chain;
use anyhow::Result;
use artemis_core::{
    collectors::{block_collector::BlockCollector, event_collector::EventCollector},
    engine::Engine,
    executors::mempool_executor::MempoolExecutor,
    types::{CollectorMap, ExecutorMap},
};
use bindings::iuniswapv2pair::IUniswapV2Pair;
use clap::Parser;
use dotenv::dotenv;
use shared::config::get_chain_config;
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};
use uni_tri_arb_strategy::{
    strategy::UniTriArb,
    types::{Action, Event},
};
// use bindings::uniswap::UniswapV2Factory;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    chain_id: u64,
    #[arg(long)]
    checkpoint_path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    // env_logger::init();
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
    let signer = chain_config.signer;
    let provider = ws;
    let mut engine: Engine<Event, Action> = Engine::default();
    let checkpoint_path: &str = "./checkpoints/filtered-pools.json";

    let block_collector = Box::new(BlockCollector::new(provider.clone()));
    let block_collector = CollectorMap::new(block_collector, |block| Event::NewBlock(block));
    engine.add_collector(Box::new(block_collector));

    let swap_filter = Filter::new()
        .from_block(BlockNumberOrTag::Latest)
        .event(IUniswapV2Pair::Swap::SIGNATURE);

    let swap_collector = Box::new(EventCollector::<_, IUniswapV2Pair::Swap>::new(
        provider.clone(),
        swap_filter,
    ));
    let swap_collector = CollectorMap::new(swap_collector, |event: IUniswapV2Pair::Swap| {
        Event::UniswapV2Swap(event)
    });
    engine.add_collector(Box::new(swap_collector));

    info!("Adding strategy...");
    let strategy = UniTriArb::new(Arc::new(provider.clone()), signer, Some(checkpoint_path));
    engine.add_strategy(Box::new(strategy));

    info!("Adding executor...");
    let mempool_executor = Box::new(MempoolExecutor::new(provider.clone()));
    let mempool_executor = ExecutorMap::new(mempool_executor, |action: Action| match action {
        Action::SubmitTx(tx) => Some(tx),
    });
    engine.add_executor(Box::new(mempool_executor));

    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("res: {:?}", res);
        }
    }

    Ok(())
}
