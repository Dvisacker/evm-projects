use alloy::{
    eips::BlockNumberOrTag,
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
};
use alloy_chains::Chain;
use base_arb_strategy::{
    strategy::BaseArb,
    types::{Action as BaseArbAction, Event as BaseArbEvent},
};
use bindings::{iuniswapv2pair::IUniswapV2Pair, iuniswapv3pool::IUniswapV3Pool};
use engine::{
    collectors::multi_log_collector::{self, MultiLogCollector},
    engine::Engine,
    executors::mempool_executor::MempoolExecutor,
    types::{CollectorMap, ExecutorMap},
};
use generalized_arb_strategy::{
    strategy::GeneralizedArb,
    types::{Action as GeneralizedArbAction, Event as GeneralizedArbEvent},
};
use provider::SignerProvider;
use std::sync::Arc;

pub fn init_generalized_arbitrage_bot(
    chain: Chain,
    provider: Arc<SignerProvider>,
    db_url: String,
) -> Engine<GeneralizedArbEvent, GeneralizedArbAction> {
    let mut engine: Engine<GeneralizedArbEvent, GeneralizedArbAction> = Engine::default();

    let uniswap_v2_filter = Filter::new()
        .from_block(BlockNumberOrTag::Latest)
        .event(IUniswapV2Pair::Sync::SIGNATURE);
    let uniswap_v3_filter = Filter::new()
        .from_block(BlockNumberOrTag::Latest)
        .event(IUniswapV3Pool::Swap::SIGNATURE);

    let filters = vec![uniswap_v2_filter, uniswap_v3_filter];

    let strategy = GeneralizedArb::new(chain, provider.clone(), db_url);
    engine.add_strategy(Box::new(strategy));

    let mempool_executor = Box::new(MempoolExecutor::new(provider.clone()));
    let mempool_executor =
        ExecutorMap::new(
            mempool_executor,
            |action: GeneralizedArbAction| match action {
                GeneralizedArbAction::SubmitTx(tx) => Some(tx),
            },
        );
    engine.add_executor(Box::new(mempool_executor));

    let collector = Box::new(MultiLogCollector::new(provider, filters));
    let collector = CollectorMap::new(collector, |event: Log| GeneralizedArbEvent::Log(event));
    engine.add_collector(Box::new(collector));
    engine
}

pub fn init_base_arbitrage_bot(
    chain: Chain,
    provider: Arc<SignerProvider>,
    db_url: String,
) -> Engine<BaseArbEvent, BaseArbAction> {
    let mut engine: Engine<BaseArbEvent, BaseArbAction> = Engine::default();

    let uniswap_v2_filter = Filter::new()
        .from_block(BlockNumberOrTag::Latest)
        .event(IUniswapV2Pair::Sync::SIGNATURE);

    let filters = vec![uniswap_v2_filter];

    let strategy = BaseArb::new(chain, provider.clone(), db_url);
    engine.add_strategy(Box::new(strategy));

    let mempool_executor = Box::new(MempoolExecutor::new(provider.clone()));
    let mempool_executor =
        ExecutorMap::new(mempool_executor, |action: BaseArbAction| match action {
            BaseArbAction::SubmitTx(tx) => Some(tx),
        });
    engine.add_executor(Box::new(mempool_executor));

    let collector = Box::new(MultiLogCollector::new(provider, filters));
    let collector = CollectorMap::new(collector, |event: Log| BaseArbEvent::Log(event));
    engine.add_collector(Box::new(collector));
    engine
}
