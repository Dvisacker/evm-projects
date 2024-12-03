use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum StrategyType {
    GeneralizedArb,
    BaseArb,
}

impl FromStr for StrategyType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "generalized-arb" => Ok(StrategyType::GeneralizedArb),
            "base-arb" => Ok(StrategyType::BaseArb),
            _ => Err(format!("Unknown strategy type: {}", s)),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub chain_id: u64,
    #[arg(long)]
    pub checkpoint_path: Option<String>,
    #[arg(short, long, default_value = "unitriarb")]
    pub strategy: StrategyType,
}
