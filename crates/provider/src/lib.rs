use alloy::{
    network::EthereumWallet,
    providers::{DynProvider, Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
};
use alloy_chains::{Chain, NamedChain};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, env};

pub type ProviderMap = HashMap<NamedChain, DynProvider>;

static PROVIDER_MAP: Lazy<Mutex<Option<ProviderMap>>> = Lazy::new(|| Mutex::new(None));

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

pub async fn get_anvil_provider() -> DynProvider {
    let signer: PrivateKeySigner = get_anvil_signer();
    let wallet = EthereumWallet::new(signer);
    let url = "http://localhost:8545";
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_builtin(url)
        .await
        .unwrap()
        .erased();

    return provider;
}

pub async fn get_anvil_provider_arc() -> Arc<DynProvider> {
    Arc::new(get_anvil_provider().await)
}

pub fn get_chain_rpc_url(chain: NamedChain) -> String {
    match chain {
        NamedChain::Mainnet => env::var("MAINNET_WS_URL").expect("MAINNET_WS_URL is not set"),
        NamedChain::Arbitrum => env::var("ARBITRUM_WS_URL").expect("ARBITRUM_WS_URL is not set"),
        NamedChain::Optimism => env::var("OPTIMISM_WS_URL").expect("OPTIMISM_WS_URL is not set"),
        NamedChain::Base => env::var("BASE_WS_URL").expect("BASE_WS_URL is not set"),
        _ => panic!("Chain not supported"),
    }
}

pub async fn get_basic_provider(chain: Chain) -> DynProvider {
    let chain = NamedChain::try_from(chain.id()).unwrap();
    let rpc_url = get_chain_rpc_url(chain);

    let provider = ProviderBuilder::new()
        .on_builtin(rpc_url.as_str())
        .await
        .unwrap()
        .erased();

    return provider;
}

pub async fn get_basic_provider_arc(chain: Chain) -> Arc<DynProvider> {
    Arc::new(get_basic_provider(chain).await)
}

pub async fn get_signer_provider(chain: Chain, wallet: EthereumWallet) -> DynProvider {
    let chain = NamedChain::try_from(chain.id()).unwrap();
    let rpc_url = get_chain_rpc_url(chain);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_builtin(rpc_url.as_str())
        .await
        .unwrap()
        .erased();

    return provider;
}

pub async fn get_signer_provider_arc(chain: Chain, wallet: EthereumWallet) -> Arc<DynProvider> {
    Arc::new(get_signer_provider(chain, wallet).await)
}

pub async fn get_default_signer_provider(chain: Chain) -> DynProvider {
    let wallet = get_default_wallet();
    get_signer_provider(chain, wallet).await
}

pub async fn get_default_signer_provider_arc(chain: Chain) -> Arc<DynProvider> {
    Arc::new(get_default_signer_provider(chain).await)
}

pub async fn get_provider_map() -> Arc<ProviderMap> {
    let mut provider_guard = PROVIDER_MAP.lock().unwrap();

    if provider_guard.is_none() {
        let wallet = get_default_wallet();
        let mut providers = ProviderMap::new();

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
