use alloy_chains::NamedChain;
use clap::Args;
use types::exchange::ExchangeName;
use types::token::TokenIsh;

// Common argument structures
#[derive(Args)]
pub struct ChainArgs {
    #[arg(short, long)]
    pub chain_id: u64,
}

#[derive(Args)]
pub struct TagArgs {
    #[arg(long)]
    pub tag: Option<String>,
}

#[derive(Args)]
pub struct BlockRangeArgs {
    #[arg(long, default_value = "0")]
    pub from_block: u64,
    #[arg(long, default_value = "1000000")]
    pub to_block: u64,
    #[arg(long, default_value = "10000")]
    pub step: u64,
}

#[derive(Args)]
pub struct ExchangeArgs {
    #[arg(long, value_enum)]
    pub exchange: ExchangeName,
}

#[derive(Args)]
pub struct BridgeArgs {
    #[arg(short, long)]
    pub from_chain: NamedChain,
    #[arg(short, long)]
    pub to_chain: NamedChain,
    #[arg(short, long)]
    pub token: TokenIsh,
    #[arg(short, long)]
    pub amount_in: String,
}

#[derive(Args)]
pub struct CrossChainSwapArgs {
    #[arg(short, long)]
    pub origin_chain: NamedChain,
    #[arg(short, long)]
    pub destination_chain: NamedChain,
    #[arg(short, long)]
    pub origin_token: TokenIsh,
    #[arg(short, long)]
    pub bridge_token: TokenIsh,
    #[arg(short, long)]
    pub destination_token: TokenIsh,
    #[arg(short, long)]
    pub amount_in: String,
}

#[derive(Args)]
pub struct GetNamedPoolsArgs {
    #[command(flatten)]
    pub chain: ChainArgs,
}

#[derive(Args)]
pub struct GetUniswapV3PoolsArgs {
    #[command(flatten)]
    pub chain: ChainArgs,
    #[command(flatten)]
    pub block_range: BlockRangeArgs,
    #[command(flatten)]
    pub exchange: ExchangeArgs,
    #[command(flatten)]
    pub tag: TagArgs,
}

#[derive(Args)]
pub struct GetUniswapV2PoolsArgs {
    #[command(flatten)]
    pub chain: ChainArgs,
    #[command(flatten)]
    pub exchange: ExchangeArgs,
    #[command(flatten)]
    pub tag: TagArgs,
}

#[derive(Args)]
pub struct GetMostTradedUniswapV3PoolsArgs {
    #[command(flatten)]
    pub common: GetUniswapV3PoolsArgs,
    #[arg(short, long)]
    pub limit: u64,
    #[arg(short, long)]
    pub min_volume: f64,
}

#[derive(Args)]
pub struct GetAerodromePoolsArgs {
    #[command(flatten)]
    pub tag: TagArgs,
}

#[derive(Args)]
pub struct ActivatePoolsArgs {
    #[command(flatten)]
    pub chain: ChainArgs,
    #[arg(short, long)]
    pub min_usd: f64,
    #[command(flatten)]
    pub exchange: ExchangeArgs,
}

#[derive(Args)]
pub struct GetAMMValueArgs {
    #[command(flatten)]
    pub chain: ChainArgs,
    #[arg(short, long)]
    pub pool_address: String,
}

#[derive(Args)]
pub struct GetContractCreationBlockArgs {
    #[command(flatten)]
    pub chain: ChainArgs,
    #[arg(long)]
    pub contract_address: String,
    #[arg(long)]
    pub start_block: Option<u64>,
    #[arg(long)]
    pub end_block: Option<u64>,
}

#[derive(Args)]
pub struct WrapEthArgs {
    #[command(flatten)]
    pub chain: ChainArgs,
    #[arg(short, long)]
    pub amount: String,
}

#[derive(Args)]
pub struct UnwrapEthArgs {
    #[command(flatten)]
    pub chain: ChainArgs,
    #[arg(short, long)]
    pub amount: String,
}

#[derive(Args)]
pub struct WithdrawArgs {
    #[command(flatten)]
    pub chain: ChainArgs,
}
