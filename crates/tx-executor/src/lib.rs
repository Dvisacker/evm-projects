use std::{env, fmt, str::FromStr};

use alloy::transports::BoxTransport;
use alloy_chains::{Chain, NamedChain};
use alloy_primitives::Address;
use encoder::BatchExecutorClient;
use provider::{get_anvil_signer_provider, get_default_signer_provider, SignerProvider};

pub mod bindings;
pub mod encoder;

pub type BasicEncoder = BatchExecutorClient<
    BoxTransport,
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
            alloy::providers::RootProvider<BoxTransport>,
            BoxTransport,
            alloy::network::Ethereum,
        >,
    >,
>;

impl fmt::Debug for BasicEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BasicEncoder").finish()
    }
}

impl Clone for BasicEncoder {
    fn clone(&self) -> Self {
        self.clone()
    }
}

pub async fn get_default_anvil_encoder(chain: Chain) -> BasicEncoder {
    dotenv::dotenv().ok();
    let provider = get_anvil_signer_provider().await;
    let named_chain = NamedChain::try_from(chain).unwrap();

    let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
    BatchExecutorClient::new(executor_address, named_chain, provider.clone()).await
}

pub async fn get_default_encoder(chain: Chain) -> BasicEncoder {
    let provider = get_default_signer_provider(chain).await;
    let named_chain = NamedChain::try_from(chain).unwrap();
    let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
    BatchExecutorClient::new(executor_address, named_chain, provider.clone()).await
}
