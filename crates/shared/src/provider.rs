use alloy::{
    network::{Ethereum, EthereumWallet},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, ProviderBuilder, RootProvider,
    },
    transports::BoxTransport,
};
use alloy_chains::{Chain, NamedChain};
use std::env;
use std::sync::Arc;

pub type SignerProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<BoxTransport>,
    BoxTransport,
    Ethereum,
>;

pub async fn get_provider(chain: Chain, wallet: EthereumWallet) -> Arc<SignerProvider> {
    let chain = NamedChain::try_from(chain.id());

    match chain {
        Ok(NamedChain::Mainnet) => {
            let url = env::var("MAINNET_WS_URL").expect("MAINNET_WS_URL is not set");
            let provider = ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_builtin(url.as_str())
                .await
                .unwrap();
            return Arc::new(provider);
        }
        Ok(NamedChain::Arbitrum) => {
            let url = env::var("ARBITRUM_WS_URL").expect("ARBITRUM_WS_URL is not set");
            let provider = ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_builtin(url.as_str())
                .await
                .unwrap();
            return Arc::new(provider);
        }
        Ok(NamedChain::Optimism) => {
            let url = env::var("OPTIMISM_WS_URL").expect("OPTIMISM_WS_URL is not set");
            let provider = ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_builtin(url.as_str())
                .await
                .unwrap();
            return Arc::new(provider);
        }
        Ok(NamedChain::Base) => {
            let url = env::var("BASE_WS_URL").expect("BASE_WS_URL is not set");
            let provider = ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_builtin(url.as_str())
                .await
                .unwrap();
            return Arc::new(provider);
        }
        _ => panic!("Chain not supported"),
    }
}
