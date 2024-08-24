use std::sync::Arc;

use anyhow::Result;
use artemis_core::{
    collectors::{block_collector::BlockCollector, mempool_collector::MempoolCollector},
    engine::Engine,
    executors::mempool_executor::MempoolExecutor,
    types::{CollectorMap, ExecutorMap},
};
use dotenv::dotenv;
use ethers::{prelude::MiddlewareBuilder, signers::Signer, types::Chain};
use shared::config::get_chain_config;
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};
use uni_tri_arb_strategy::{
    strategy::UniTriArb,
    types::{Action, Event},
};

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

    let chain_config = get_chain_config(Chain::Arbitrum).await;
    let ws = chain_config.ws;
    let wallet = chain_config.wallet;
    let address = wallet.address();
    let provider = Arc::new(ws.nonce_manager(address).with_signer(wallet.clone()));

    let mut engine: Engine<Event, Action> = Engine::default();

    let block_collector = Box::new(BlockCollector::new(provider.clone()));
    let block_collector = CollectorMap::new(block_collector, Event::NewBlock);
    engine.add_collector(Box::new(block_collector));

    let mempool_collector = Box::new(MempoolCollector::new(provider.clone()));
    let mempool_collector = CollectorMap::new(mempool_collector, |tx| Event::UniswapOrder(tx));
    engine.add_collector(Box::new(mempool_collector));

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
