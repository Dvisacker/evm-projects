use alloy::primitives::Address;
use alloy_chains::NamedChain;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use types::exchange::{ExchangeName, ExchangeType};

fn deserialize_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Address::from_str(&s).map_err(serde::de::Error::custom)
}

fn deserialize_address_option<'de, D>(deserializer: D) -> Result<Option<Address>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if s.is_empty() {
        Ok(None)
    } else {
        Address::from_str(&s)
            .map_err(serde::de::Error::custom)
            .map(Some)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddressMap(#[serde(with = "address_map")] pub HashMap<String, Address>);

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
pub struct ExchangeAddressBook {
    #[serde(deserialize_with = "deserialize_address")]
    pub factory: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub router: Address,
    pub pools: AddressMap,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniV2Addresses {
    #[serde(deserialize_with = "deserialize_address")]
    pub factory: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub router: Address,
    pub pools: AddressMap,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniV3Addresses {
    #[serde(deserialize_with = "deserialize_address")]
    pub factory: Address,
    #[serde(
        default,
        deserialize_with = "deserialize_address_option",
        rename = "swapRouter01"
    )]
    pub swap_router_01: Option<Address>,
    #[serde(
        default,
        deserialize_with = "deserialize_address_option",
        rename = "swapRouter02"
    )]
    pub swap_router_02: Option<Address>,
    #[serde(
        default,
        deserialize_with = "deserialize_address_option",
        rename = "universalRouter"
    )]
    pub universal_router: Option<Address>,
    #[serde(default, deserialize_with = "deserialize_address_option")]
    pub quoter: Option<Address>,
    pub pools: AddressMap,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniV3Exchanges {
    pub uniswapv3: UniV3Addresses,
    pub sushiswapv3: UniV3Addresses,
    pub camelotv3: UniV3Addresses,
    pub ramsesv2: UniV3Addresses,
    pub pancakeswapv3: UniV3Addresses,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniV2Exchanges {
    pub uniswapv2: UniV2Addresses,
    pub sushiswapv2: UniV2Addresses,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Exchanges {
    pub univ2: UniV2Exchanges,
    pub univ3: UniV3Exchanges,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChainAddressBook {
    pub exchanges: Exchanges,
    #[serde(deserialize_with = "deserialize_address")]
    pub multicall: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub weth: Address,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Addressbook {
    pub arbitrum: ChainAddressBook,
    pub optimism: ChainAddressBook,
    pub mainnet: ChainAddressBook,
}

pub fn get_exchange_type(exchange_name: ExchangeName) -> ExchangeType {
    match exchange_name {
        ExchangeName::UniswapV2 => ExchangeType::UniV2,
        ExchangeName::UniswapV3 => ExchangeType::UniV3,
        ExchangeName::SushiswapV2 => ExchangeType::UniV2,
        ExchangeName::SushiswapV3 => ExchangeType::UniV3,
        ExchangeName::CamelotV3 => ExchangeType::UniV3,
        ExchangeName::RamsesV2 => ExchangeType::UniV3,
        ExchangeName::PancakeswapV3 => ExchangeType::UniV3,
        _ => panic!("Invalid exchange name"),
    }
}

impl Addressbook {
    pub fn load(filepath: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(filepath.unwrap_or("./addressbook.json"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let addressbook: Addressbook = serde_json::from_str(&contents)?;
        Ok(addressbook)
    }

    pub fn get_pool_by_name(
        &self,
        chain: &NamedChain,
        exchange_name: ExchangeName,
        pool_name: &str,
    ) -> Option<Address> {
        let book = self.get_chain_address_book(chain)?;
        let exchange_type = get_exchange_type(exchange_name.clone());
        match exchange_type {
            ExchangeType::UniV2 => match exchange_name {
                ExchangeName::UniswapV2 => book
                    .exchanges
                    .univ2
                    .uniswapv2
                    .pools
                    .0
                    .get(pool_name)
                    .cloned(),
                ExchangeName::SushiswapV2 => book
                    .exchanges
                    .univ2
                    .sushiswapv2
                    .pools
                    .0
                    .get(pool_name)
                    .cloned(),
                _ => return None,
            },
            ExchangeType::UniV3 => match exchange_name {
                ExchangeName::UniswapV3 => book
                    .exchanges
                    .univ3
                    .uniswapv3
                    .pools
                    .0
                    .get(pool_name)
                    .cloned(),
                _ => return None,
            },
            _ => return None,
        }
    }

    pub fn get_weth(&self, chain: &NamedChain) -> Option<Address> {
        self.get_chain_address_book(chain).map(|config| config.weth)
    }

    pub fn get_multicall(&self, chain: &NamedChain) -> Option<Address> {
        self.get_chain_address_book(chain)
            .map(|config| config.multicall)
    }

    pub fn get_pools_by_chain(&self, chain: &NamedChain) -> Vec<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        chain_config
            .exchanges
            .univ2
            .uniswapv2
            .pools
            .0
            .values()
            .cloned()
            .collect()
    }

    pub fn get_factories_by_chain(&self, chain: &NamedChain) -> Vec<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        vec![
            chain_config.exchanges.univ2.uniswapv2.factory,
            chain_config.exchanges.univ2.sushiswapv2.factory,
            chain_config.exchanges.univ3.uniswapv3.factory,
            chain_config.exchanges.univ3.sushiswapv3.factory,
            chain_config.exchanges.univ3.camelotv3.factory,
            chain_config.exchanges.univ3.ramsesv2.factory,
            chain_config.exchanges.univ3.pancakeswapv3.factory,
        ]
    }

    pub fn get_v2_factories(&self, chain: &NamedChain) -> Vec<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        vec![
            chain_config.exchanges.univ2.uniswapv2.factory,
            chain_config.exchanges.univ2.sushiswapv2.factory,
        ]
    }

    pub fn get_v3_factories(&self, chain: &NamedChain) -> Vec<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        let factories = vec![
            chain_config.exchanges.univ3.uniswapv3.factory,
            chain_config.exchanges.univ3.sushiswapv3.factory,
            chain_config.exchanges.univ3.camelotv3.factory,
            chain_config.exchanges.univ3.ramsesv2.factory,
            chain_config.exchanges.univ3.pancakeswapv3.factory,
        ];

        // Filter out empty addresses
        factories
            .into_iter()
            .filter(|addr| !addr.is_zero())
            .collect()
    }

    // pub fn get_exchange_name(&self, chain: &NamedChain, factory: Address) -> Option<ExchangeName> {
    //     let addressbook = self.get_chain_address_book(chain).unwrap();
    //     let factory = addressbook.exchanges.univ2.uniswapv2.factory;

    //     match factory {
    //         addressbook.exchanges.univ2.uniswapv2.factory => Some(ExchangeName::UniswapV2),
    //         addressbook.exchanges.univ2.sushiswapv2.factory => Some(ExchangeName::SushiSwapV2),
    //         addressbook.exchanges.univ3.uniswapv3.factory => Some(ExchangeName::UniswapV3),
    //         addressbook.exchanges.univ3.sushiswapv3.factory => Some(ExchangeName::SushiSwapV3),
    //         addressbook.exchanges.univ3.camelotv3.factory => Some(ExchangeName::CamelotV3),
    //         addressbook.exchanges.univ3.ramsesv2.factory => Some(ExchangeName::RamsesV2),
    //         addressbook.exchanges.univ3.pancakeswapv3.factory => Some(ExchangeName::PancakeswapV3),
    //         _ => None,
    //     }
    // }

    fn get_chain_address_book(&self, chain: &NamedChain) -> Option<&ChainAddressBook> {
        match chain {
            NamedChain::Arbitrum => Some(&self.arbitrum),
            NamedChain::Optimism => Some(&self.optimism),
            NamedChain::Mainnet => Some(&self.mainnet),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_chains::NamedChain;

    #[test]
    fn test_load_addressbook() {
        let current_dir = std::env::current_dir().unwrap();
        println!("Current directory: {:?}", current_dir);
        let addressbook =
            Addressbook::load(Some("./src/addressbook.json")).expect("Failed to load addressbook");

        // Test that we can load basic chain configs
        assert!(addressbook
            .get_chain_address_book(&NamedChain::Arbitrum)
            .is_some());
        assert!(addressbook
            .get_chain_address_book(&NamedChain::Optimism)
            .is_some());
        assert!(addressbook
            .get_chain_address_book(&NamedChain::Mainnet)
            .is_some());
        assert!(addressbook
            .get_chain_address_book(&NamedChain::Base)
            .is_none());
    }

    #[test]
    fn test_get_weth_addresses() {
        let addressbook =
            Addressbook::load(Some("./src/addressbook.json")).expect("Failed to load addressbook");

        // Get WETH addresses for each chain
        let arb_weth = addressbook
            .get_weth(&NamedChain::Arbitrum)
            .expect("Failed to get Arbitrum WETH");
        let op_weth = addressbook
            .get_weth(&NamedChain::Optimism)
            .expect("Failed to get Optimism WETH");
        let mainnet_weth = addressbook
            .get_weth(&NamedChain::Mainnet)
            .expect("Failed to get Mainnet WETH");

        // Verify they're not zero addresses
        assert!(!arb_weth.is_zero());
        assert!(!op_weth.is_zero());
        assert!(!mainnet_weth.is_zero());
    }

    #[test]
    fn test_get_uniswap_v3_config() {
        let addressbook =
            Addressbook::load(Some("./src/addressbook.json")).expect("Failed to load addressbook");

        let arb_config = addressbook
            .get_chain_address_book(&NamedChain::Arbitrum)
            .expect("Failed to get Arbitrum config");

        let univ3_config = &arb_config.exchanges.univ3.uniswapv3;

        // Check factory address
        assert!(!univ3_config.factory.is_zero());

        // Check router addresses
        if let Some(router01) = univ3_config.swap_router_01 {
            assert!(!router01.is_zero());
        }
        if let Some(router02) = univ3_config.swap_router_02 {
            assert!(!router02.is_zero());
        }
    }

    #[test]
    fn test_get_pool_by_name() {
        let addressbook =
            Addressbook::load(Some("./src/addressbook.json")).expect("Failed to load addressbook");

        // Test getting a known pool
        let pool = addressbook.get_pool_by_name(
            &NamedChain::Arbitrum,
            ExchangeName::UniswapV3,
            "USDC/WETH",
        );

        // The pool should exist and not be a zero address
        assert!(pool.is_some());
        if let Some(addr) = pool {
            assert!(!addr.is_zero());
        }
    }

    #[test]
    fn test_get_factories() {
        let addressbook =
            Addressbook::load(Some("./src/addressbook.json")).expect("Failed to load addressbook");

        // Get V2 and V3 factories for Arbitrum
        let v2_factories = addressbook.get_v2_factories(&NamedChain::Arbitrum);
        let v3_factories = addressbook.get_v3_factories(&NamedChain::Arbitrum);

        // Check that we got some factories
        assert!(!v2_factories.is_empty());
        assert!(!v3_factories.is_empty());

        // Verify none are zero addresses
        for factory in v2_factories {
            assert!(!factory.is_zero());
        }
        for factory in v3_factories {
            assert!(!factory.is_zero());
        }
    }

    #[test]
    fn test_get_multicall() {
        let addressbook =
            Addressbook::load(Some("./src/addressbook.json")).expect("Failed to load addressbook");

        // Test multicall addresses for each chain
        let chains = vec![
            NamedChain::Arbitrum,
            NamedChain::Optimism,
            NamedChain::Mainnet,
        ];

        for chain in chains {
            let multicall = addressbook
                .get_multicall(&chain)
                .expect(&format!("Failed to get multicall for {:?}", chain));
            assert!(!multicall.is_zero());
        }
    }

    #[test]
    fn test_get_pools_by_chain() {
        let addressbook =
            Addressbook::load(Some("./src/addressbook.json")).expect("Failed to load addressbook");

        // Get pools for Arbitrum
        let pools = addressbook.get_pools_by_chain(&NamedChain::Arbitrum);

        // Verify we got some pools
        assert!(!pools.is_empty());

        // Check that none are zero addresses
        for pool in pools {
            assert!(!pool.is_zero());
        }
    }
}
