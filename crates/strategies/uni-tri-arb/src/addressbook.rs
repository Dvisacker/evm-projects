use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct UniV2Config {
    pub factory: String,
    pub router: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainConfig {
    pub uniswapv2: UniV2Config,
    pub sushiswap: UniV2Config,
    pub multicall: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Addressbook {
    pub arbitrum: ChainConfig,
    pub optimism: ChainConfig,
    pub mainnet: ChainConfig,
}

impl Addressbook {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open("addressbook.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let addressbook: Addressbook = serde_json::from_str(&contents)?;
        Ok(addressbook)
    }
}
