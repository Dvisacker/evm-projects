use alloy::signers::local::PrivateKeySigner;
use alloy_chains::{Chain, NamedChain};
use once_cell::sync::Lazy;
use provider::{get_basic_provider, get_chain_rpc_url, BasicProvider};
use std::{collections::HashMap, env, path::PathBuf, sync::Arc};

pub struct ChainConfig {
    pub chain: Chain,
    pub chain_id: u64,
    pub default_rpc_url: String,
    pub explorer_url: String,
    pub explorer_api_key: String,
    pub explorer_api_url: String,
    pub signer: PrivateKeySigner,
    pub provider: Arc<BasicProvider>,
}

// Add this new static mapping
pub static CHAIN_MAP: Lazy<HashMap<String, Chain>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "ethereum".to_string(),
        Chain::from_named(NamedChain::Mainnet),
    );
    m.insert("goerli".to_string(), Chain::from_named(NamedChain::Goerli));
    m.insert(
        "sepolia".to_string(),
        Chain::from_named(NamedChain::Sepolia),
    );
    m.insert(
        "polygon".to_string(),
        Chain::from_named(NamedChain::Polygon),
    );
    m.insert(
        "mumbai".to_string(),
        Chain::from_named(NamedChain::PolygonMumbai),
    );
    m.insert(
        "arbitrum".to_string(),
        Chain::from_named(NamedChain::Arbitrum),
    );
    m.insert(
        "arbitrum_goerli".to_string(),
        Chain::from_named(NamedChain::ArbitrumGoerli),
    );
    m.insert(
        "optimism".to_string(),
        Chain::from_named(NamedChain::Optimism),
    );
    m.insert(
        "optimism_goerli".to_string(),
        Chain::from_named(NamedChain::OptimismGoerli),
    );
    m.insert("base".to_string(), Chain::from_named(NamedChain::Base));
    m
});

pub const DEFAULT_WHITELIST_PATH: &str = "whitelist.json";

pub fn get_whitelist_path() -> PathBuf {
    PathBuf::from(DEFAULT_WHITELIST_PATH)
}

pub async fn get_chain_config(chain: Chain) -> ChainConfig {
    let priv_key = std::env::var("DEV_PRIVATE_KEY").expect("missing PRIVATE_KEY");
    let signer: PrivateKeySigner = priv_key.parse().unwrap();
    let chain_id = chain.id();
    let chain = NamedChain::try_from(chain_id).unwrap();
    let provider = get_basic_provider(Chain::from_named(chain)).await;
    let default_rpc_url = get_chain_rpc_url(chain).await;

    match chain {
        NamedChain::Mainnet => {
            return ChainConfig {
                chain: Chain::from_named(NamedChain::Mainnet),
                chain_id: 1,
                default_rpc_url,
                signer,
                explorer_url: "https://etherscan.io".to_string(),
                explorer_api_key: "TCZS3DYFANPFZRPFY338CCKHTMF5QNMCG9".to_string(),
                explorer_api_url: "https://api.etherscan.io/api".to_string(),
                provider: provider,
            };
        }
        NamedChain::Arbitrum => {
            return ChainConfig {
                chain: Chain::from_named(NamedChain::Arbitrum),
                chain_id: 42161,
                signer,
                default_rpc_url,
                explorer_url: "https://arbiscan.io".to_string(),
                explorer_api_key: "".to_string(),
                explorer_api_url: "https://api.arbiscan.io/api".to_string(),
                provider,
            };
        }
        NamedChain::Base => {
            return ChainConfig {
                chain: Chain::from_named(NamedChain::Base),
                chain_id: 8453,
                signer,
                default_rpc_url,
                explorer_url: "https://basescan.org".to_string(),
                explorer_api_key: "".to_string(),
                explorer_api_url: "https://api.basescan.org/api".to_string(),
                provider,
            };
        }
        NamedChain::Optimism => {
            return ChainConfig {
                chain: Chain::from_named(NamedChain::Optimism),
                chain_id: 10,
                signer,
                default_rpc_url,
                explorer_url: "https://optimistic.etherscan.io".to_string(),
                explorer_api_key: "".to_string(),
                explorer_api_url: "https://api-optimistic.etherscan.io/api".to_string(),
                provider,
            };
        }
        _ => panic!("Chain not supported"),
    }
}

// Add this new function to get Chain from string
pub fn get_chain_from_string(chain_name: &str) -> Option<Chain> {
    CHAIN_MAP.get(chain_name).cloned()
}

pub fn get_chain_id_from_string(chain_name: &str) -> Option<u64> {
    let chain = get_chain_from_string(chain_name);
    match chain {
        Some(chain) => Some(chain.id()),
        None => None,
    }
}
