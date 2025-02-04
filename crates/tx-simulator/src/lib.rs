use std::{env, str::FromStr};

use alloy::primitives::Address;
use alloy_chains::NamedChain;
use provider::get_default_signer_provider;
use simulator::TxSimulatorClient;

pub mod bindings;
pub mod simulator;

pub type SimulatorClient = TxSimulatorClient<
    std::sync::Arc<
        alloy::providers::fillers::FillProvider<
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::JoinFill<
                    alloy::providers::Identity,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::GasFiller,
                        alloy::providers::fillers::JoinFill<
                            alloy::providers::fillers::BlobGasFiller,
                            alloy::providers::fillers::JoinFill<
                                alloy::providers::fillers::NonceFiller,
                                alloy::providers::fillers::ChainIdFiller,
                            >,
                        >,
                    >,
                >,
                alloy::providers::fillers::WalletFiller<alloy::network::EthereumWallet>,
            >,
            alloy::providers::RootProvider,
            alloy::network::Ethereum,
        >,
    >,
>;

pub async fn get_default_simulator(chain: NamedChain) -> SimulatorClient {
    let provider = get_default_signer_provider(chain.into()).await;
    let simulator_address = Address::from_str(&env::var("SIMULATOR_ADDRESS").unwrap()).unwrap();
    let simulator = TxSimulatorClient::new(simulator_address, provider.clone()).await;
    simulator
}
