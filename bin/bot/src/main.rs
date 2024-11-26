use std::sync::Arc;

use alloy::{
    eips::BlockNumberOrTag,
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
};
use alloy_chains::Chain;
use bindings::{iuniswapv2pair::IUniswapV2Pair, iuniswapv3pool::IUniswapV3Pool};
use clap::Parser;
use dotenv::dotenv;
use engine::{
    collectors::multi_log_collector::MultiLogCollector,
    engine::Engine,
    executors::mempool_executor::MempoolExecutor,
    types::{CollectorMap, ExecutorMap},
};
use eyre::Result;
use generalized_arb_strategy::{
    strategy::UniTriArb,
    types::{Action, Event},
};
use shared::config::get_chain_config;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
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

    let args = Args::parse();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let chain = Chain::try_from(args.chain_id).expect("Invalid chain ID");
    let chain_config = get_chain_config(chain).await;
    let ws = chain_config.ws;
    let signer = chain_config.signer;
    let provider = ws;
    let mut engine: Engine<Event, Action> = Engine::default();

    // let block_collector = Box::new(BlockCollector::new(provider.clone()));
    // let block_collector = CollectorMap::new(block_collector, |block| Event::NewBlock(block));
    // engine.add_collector(Box::new(block_collector));

    // let uniswap_v2_filter = Filter::new()
    //     .from_block(BlockNumberOrTag::Latest)
    //     .event(IUniswapV2Pair::Swap::SIGNATURE);
    let uniswap_v2_filter = Filter::new()
        .from_block(BlockNumberOrTag::Latest)
        .event(IUniswapV2Pair::Sync::SIGNATURE);
    let uniswap_v3_filter = Filter::new()
        .from_block(BlockNumberOrTag::Latest)
        .event(IUniswapV3Pool::Swap::SIGNATURE);

    let filters = vec![uniswap_v2_filter, uniswap_v3_filter];

    let multi_log_collector = Box::new(MultiLogCollector::new(provider.clone(), filters));
    let multi_log_collector =
        CollectorMap::new(multi_log_collector, |event: Log| Event::Log(event));
    engine.add_collector(Box::new(multi_log_collector));

    info!("Adding strategy...");
    let strategy = UniTriArb::new(chain, Arc::new(provider.clone()), signer, db_url);
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
