use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    rpc::types::{Filter, Log},
    signers::local::PrivateKeySigner,
    sol_types::SolEvent,
};
use alloy_chains::{Chain, NamedChain};
use clap::Parser;
use dotenv::dotenv;
use eyre::Result;
use provider::get_provider;
use shared::{config::get_chain_config, swap::swap_v2_pool};
use std::str::FromStr;

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

    let signer: PrivateKeySigner = std::env::var("DEV_PRIVATE_KEY")
        .expect("PRIVATE_KEY must be set")
        .parse()
        .expect("should parse private key");

    let wallet_address = signer.address();
    let wallet = EthereumWallet::new(signer);
    let provider = get_provider(Chain::from_named(NamedChain::Arbitrum), wallet).await;

    let weth_usdc_v2 = Address::from_str("0xf64dfe17c8b87f012fcf50fbda1d62bfa148366a").unwrap();
    let token_in = Address::from_str("0xaf88d065e77c8cC2239327C5EDb3A432268e5831").unwrap();
    let amount_in = U256::from(1000000);
    let amount_out_min = U256::from(0u128);

    println!("wallet_address: {:?}", wallet_address);

    // let tx = swap_v2_pool(
    //     provider,
    //     wallet_address,
    //     weth_usdc_v2,
    //     token_in,
    //     amount_in, // amount_out_min,
    // )
    // .await?;

    Ok(())
}
