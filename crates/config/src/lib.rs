use alloy_chains::{Chain, NamedChain};
use once_cell::sync::Lazy;
use provider::get_chain_rpc_url;
use std::{collections::HashMap, path::PathBuf};

pub struct ChainConfig {
    pub chain: Chain,
    pub default_rpc_url: String,
    pub explorer_url: String,
    pub explorer_api_key: String,
    pub explorer_api_url: String,
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

pub async fn get_chain_config(chain: Chain) -> ChainConfig {
    let chain = NamedChain::try_from(chain.id()).unwrap();
    let default_rpc_url = get_chain_rpc_url(chain).await;

    match chain {
        NamedChain::Mainnet => {
            return ChainConfig {
                chain: Chain::from_named(NamedChain::Mainnet),
                default_rpc_url,
                explorer_url: "https://etherscan.io".to_string(),
                explorer_api_key: "TCZS3DYFANPFZRPFY338CCKHTMF5QNMCG9".to_string(),
                explorer_api_url: "https://api.etherscan.io/api".to_string(),
            };
        }
        NamedChain::Arbitrum => {
            return ChainConfig {
                chain: Chain::from_named(NamedChain::Arbitrum),
                default_rpc_url,
                explorer_url: "https://arbiscan.io".to_string(),
                explorer_api_key: "".to_string(),
                explorer_api_url: "https://api.arbiscan.io/api".to_string(),
            };
        }
        NamedChain::Base => {
            return ChainConfig {
                chain: Chain::from_named(NamedChain::Base),
                default_rpc_url,
                explorer_url: "https://basescan.org".to_string(),
                explorer_api_key: "".to_string(),
                explorer_api_url: "https://api.basescan.org/api".to_string(),
            };
        }
        NamedChain::Optimism => {
            return ChainConfig {
                chain: Chain::from_named(NamedChain::Optimism),
                default_rpc_url,
                explorer_url: "https://optimistic.etherscan.io".to_string(),
                explorer_api_key: "".to_string(),
                explorer_api_url: "https://api-optimistic.etherscan.io/api".to_string(),
            };
        }
        _ => panic!("Chain not supported"),
    }
}
