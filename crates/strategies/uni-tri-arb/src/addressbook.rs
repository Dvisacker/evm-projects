use alloy::primitives::Address;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

// Add this custom deserializer function
fn deserialize_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Address::from_str(&s).map_err(serde::de::Error::custom)
}

// Add this custom type for the HashMap
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddressMap(#[serde(with = "address_map")] pub HashMap<String, Address>);

// Add this module for custom serialization/deserialization of the HashMap
mod address_map {
    use super::*;
    use serde::ser::SerializeMap;

    pub fn serialize<S>(map: &HashMap<String, Address>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map_ser = serializer.serialize_map(Some(map.len()))?;
        for (k, v) in map {
            map_ser.serialize_entry(k, &v.to_string())?;
        }
        map_ser.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Address>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_map = HashMap::<String, String>::deserialize(deserializer)?;
        string_map
            .into_iter()
            .map(|(k, v)| {
                Address::from_str(&v)
                    .map(|addr| (k, addr))
                    .map_err(serde::de::Error::custom)
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniV2Config {
    #[serde(deserialize_with = "deserialize_address")]
    pub factory: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub router: Address,
    pub pools: AddressMap,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChainConfig {
    pub uniswapv2: UniV2Config,
    pub sushiswap: UniV2Config,
    #[serde(deserialize_with = "deserialize_address")]
    pub multicall: Address,
    pub weth: Address,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn get_pool_by_name(
        &self,
        chain: &str,
        exchange_name: &str,
        pool_name: &str,
    ) -> Option<Address> {
        let chain_config = match chain {
            "arbitrum" => &self.arbitrum,
            "optimism" => &self.optimism,
            "mainnet" => &self.mainnet,
            _ => return None,
        };

        let pool_address = match exchange_name {
            "uniswapv2" => chain_config.uniswapv2.pools.0.get(pool_name).cloned(),
            "sushiswap" => chain_config.sushiswap.pools.0.get(pool_name).cloned(),
            _ => None,
        };

        pool_address
    }

    pub fn get_weth(&self, chain: &str) -> Option<Address> {
        let chain_config = match chain {
            "arbitrum" => &self.arbitrum,
            "optimism" => &self.optimism,
            "mainnet" => &self.mainnet,
            _ => return None,
        };

        Some(chain_config.weth)
    }

    pub fn get_multicall(&self, chain: &str) -> Option<Address> {
        let chain_config = match chain {
            "arbitrum" => &self.arbitrum,
            "optimism" => &self.optimism,
            "mainnet" => &self.mainnet,
            _ => return None,
        };

        Some(chain_config.multicall)
    }

    // get all pools from uniswapv2 and sushiswap
    pub fn get_pools_by_chain(&self, chain: &str) -> Vec<Address> {
        let chain_config = match chain {
            "arbitrum" => &self.arbitrum,
            "optimism" => &self.optimism,
            "mainnet" => &self.mainnet,
            _ => return vec![],
        };

        let mut pools: Vec<Address> = chain_config.uniswapv2.pools.0.values().cloned().collect();
        pools.extend(chain_config.sushiswap.pools.0.values().cloned());
        pools
    }

    // pub fn load_pools(
    //     &self,
    //     chain: &str,
    //     exchanges: Vec<&str>,
    // ) -> Option<HashMap<String, Address>> {
    //     let chain_config = match chain {
    //         "arbitrum" => &self.arbitrum,
    //         "optimism" => &self.optimism,
    //         "mainnet" => &self.mainnet,
    //         _ => return None,
    //     };
    // }
}
