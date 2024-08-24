use ethers::{
    providers::{Provider, Ws},
    signers::LocalWallet,
    types::Chain,
};
use once_cell::sync::Lazy;
use std::{collections::HashMap, env, path::PathBuf, sync::Arc};

pub struct ChainConfig {
    pub chain: Chain,
    pub chain_id: u64,
    pub explorer_url: String,
    pub explorer_api_key: String,
    pub explorer_api_url: String,
    pub wallet: LocalWallet,
    // pub http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub ws: Arc<Provider<Ws>>,
}

// Add this new static mapping
pub static CHAIN_MAP: Lazy<HashMap<String, Chain>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("ethereum".to_string(), Chain::Mainnet);
    m.insert("goerli".to_string(), Chain::Goerli);
    m.insert("sepolia".to_string(), Chain::Sepolia);
    m.insert("polygon".to_string(), Chain::Polygon);
    m.insert("mumbai".to_string(), Chain::PolygonMumbai);
    m.insert("arbitrum".to_string(), Chain::Arbitrum);
    m.insert("arbitrum_goerli".to_string(), Chain::ArbitrumGoerli);
    m.insert("optimism".to_string(), Chain::Optimism);
    m.insert("optimism_goerli".to_string(), Chain::OptimismGoerli);
    m
});

pub const DEFAULT_WHITELIST_PATH: &str = "whitelist.json";

pub fn get_whitelist_path() -> PathBuf {
    PathBuf::from(DEFAULT_WHITELIST_PATH)
}

pub async fn get_chain_config(chain: Chain) -> ChainConfig {
    let priv_key = std::env::var("DEV_PRIVATE_KEY").expect("missing PRIVATE_KEY");
    let wallet: LocalWallet = priv_key.parse().unwrap();

    match chain {
        Chain::Mainnet => {
            let url = env::var("MAINNET_RPC_URL").expect("MAINNET_RPC_URL is not set");
            let ws_url = env::var("MAINNET_WS_URL").expect("MAINNET_WS_URL is not set");
            let ws_provider = Provider::<Ws>::connect(ws_url).await.unwrap();
            return ChainConfig {
                chain,
                chain_id: 1,
                wallet,
                explorer_url: "https://etherscan.io".to_string(),
                explorer_api_key: "TCZS3DYFANPFZRPFY338CCKHTMF5QNMCG9".to_string(),
                explorer_api_url: "https://api.etherscan.io/api".to_string(),
                ws: Arc::new(ws_provider),
            };
        }
        Chain::Arbitrum => {
            let url = env::var("ARBITRUM_RPC_URL").expect("ARBITRUM_RPC_URL is not set");
            let ws_url = env::var("ARBITRUM_WS_URL").expect("ARBITRUM_WS_URL is not set");
            let ws_provider = Provider::<Ws>::connect(ws_url).await.unwrap();
            return ChainConfig {
                chain,
                chain_id: 42161,
                wallet,
                explorer_url: "https://arbiscan.io".to_string(),
                explorer_api_key: "".to_string(),
                explorer_api_url: "https://api.arbiscan.io/api".to_string(),
                ws: Arc::new(ws_provider),
            };
        }
        Chain::Optimism => {
            let url = env::var("OPTIMISM_RPC_URL").expect("OPTIMISM_RPC_URL is not set");
            let ws_url = env::var("OPTIMISM_WS_URL").expect("OPTIMISM_WS_URL is not set");
            let ws_provider = Provider::<Ws>::connect(ws_url).await.unwrap();
            return ChainConfig {
                chain,
                chain_id: 10,
                wallet,
                explorer_url: "https://optimistic.etherscan.io".to_string(),
                explorer_api_key: "".to_string(),
                explorer_api_url: "https://api-optimistic.etherscan.io/api".to_string(),
                ws: Arc::new(ws_provider),
            };
        }
        _ => panic!("Chain not supported"),
    }
}

// Add this new function to get Chain from string
pub fn get_chain_from_string(chain_name: &str) -> Option<Chain> {
    println!("Chain name: {}", chain_name);
    CHAIN_MAP.get(chain_name).cloned()
}

pub fn get_chain_id_from_string(chain_name: &str) -> Option<u64> {
    let chain = get_chain_from_string(chain_name);
    match chain {
        Some(chain) => Some(chain as u64),
        None => None,
    }
}
