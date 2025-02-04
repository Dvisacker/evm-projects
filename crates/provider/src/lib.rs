use alloy::{
    network::{Ethereum, EthereumWallet},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
};
use alloy_chains::{Chain, NamedChain};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, env};

// read/write provider with wallet and all recommended fillers
pub type SignerProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
    Ethereum,
>;

pub type BasicProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
    Ethereum,
>;

pub type SignerProviderMap = HashMap<NamedChain, Arc<SignerProvider>>;
pub type BasicProviderMap = HashMap<NamedChain, Arc<BasicProvider>>;

static SIGNER_PROVIDER_MAP: Lazy<Mutex<Option<SignerProviderMap>>> = Lazy::new(|| Mutex::new(None));
static BASIC_PROVIDER_MAP: Lazy<Mutex<Option<BasicProviderMap>>> = Lazy::new(|| Mutex::new(None));

pub fn get_default_signer() -> PrivateKeySigner {
    std::env::var("DEV_PRIVATE_KEY")
        .expect("PRIVATE_KEY must be set")
        .parse()
        .expect("should parse private key")
}

pub fn get_default_wallet() -> EthereumWallet {
    let signer: PrivateKeySigner = get_default_signer();
    let wallet = EthereumWallet::new(signer);
    wallet
}

pub fn get_anvil_signer() -> PrivateKeySigner {
    String::from("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
        .parse()
        .unwrap()
}

pub async fn get_anvil_signer_provider() -> Arc<SignerProvider> {
    let signer: PrivateKeySigner = get_anvil_signer();
    let wallet = EthereumWallet::new(signer);
    let url = "http://localhost:8545";
    let provider: SignerProvider = ProviderBuilder::new()
        .wallet(wallet)
        .on_builtin(url)
        .await
        .unwrap();
    return Arc::new(provider);
}

// read provider without wallet and all recommended fillers
pub async fn get_anvil_basic_provider() -> Arc<BasicProvider> {
    let url = "http://localhost:8545";
    let provider: BasicProvider = ProviderBuilder::new().on_builtin(url).await.unwrap();
    return Arc::new(provider);
}

pub async fn get_chain_rpc_url(chain: NamedChain) -> String {
    match chain {
        NamedChain::Mainnet => env::var("MAINNET_WS_URL").expect("MAINNET_WS_URL is not set"),
        NamedChain::Arbitrum => env::var("ARBITRUM_WS_URL").expect("ARBITRUM_WS_URL is not set"),
        NamedChain::Optimism => env::var("OPTIMISM_WS_URL").expect("OPTIMISM_WS_URL is not set"),
        NamedChain::Base => env::var("BASE_WS_URL").expect("BASE_WS_URL is not set"),
        _ => panic!("Chain not supported"),
    }
}

pub async fn get_basic_provider(chain: Chain) -> Arc<BasicProvider> {
    let chain = NamedChain::try_from(chain.id()).unwrap();
    let rpc_url = get_chain_rpc_url(chain).await;

    let provider = ProviderBuilder::new()
        .on_builtin(rpc_url.as_str())
        .await
        .unwrap();

    return Arc::new(provider);
}

pub async fn get_signer_provider(chain: Chain, wallet: EthereumWallet) -> Arc<SignerProvider> {
    let chain = NamedChain::try_from(chain.id()).unwrap();
    let rpc_url = get_chain_rpc_url(chain).await;

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_builtin(rpc_url.as_str())
        .await
        .unwrap();

    return Arc::new(provider);
}

pub async fn get_default_signer_provider(chain: Chain) -> Arc<SignerProvider> {
    let wallet = get_default_wallet();
    get_signer_provider(chain, wallet).await
}

pub async fn get_signer_provider_map() -> Arc<SignerProviderMap> {
    let mut provider_guard = SIGNER_PROVIDER_MAP.lock().unwrap();

    if provider_guard.is_none() {
        let wallet = get_default_wallet();
        let mut providers = SignerProviderMap::new();

        for chain in [
            NamedChain::Mainnet,
            NamedChain::Arbitrum,
            NamedChain::Optimism,
            NamedChain::Base,
        ] {
            providers.insert(
                chain,
                get_signer_provider(Chain::from_named(chain), wallet.clone()).await,
            );
        }

        *provider_guard = Some(providers);
    }

    Arc::new(provider_guard.as_ref().unwrap().clone())
}

pub async fn get_basic_provider_map() -> Arc<BasicProviderMap> {
    let mut provider_guard = BASIC_PROVIDER_MAP.lock().unwrap();

    if provider_guard.is_none() {
        let mut providers = BasicProviderMap::new();

        for chain in [
            NamedChain::Mainnet,
            NamedChain::Arbitrum,
            NamedChain::Optimism,
            NamedChain::Base,
        ] {
            providers.insert(chain, get_basic_provider(Chain::from_named(chain)).await);
        }

        *provider_guard = Some(providers);
    }

    Arc::new(provider_guard.as_ref().unwrap().clone())
}

pub fn is_signer_provider_map_initialized() -> bool {
    SIGNER_PROVIDER_MAP.lock().unwrap().is_some()
}
