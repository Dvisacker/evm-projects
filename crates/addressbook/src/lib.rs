use alloy::primitives::Address;
use alloy_chains::NamedChain;
use serde::{Deserialize, Deserializer, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::{collections::HashMap, env};
use types::exchange::{ExchangeName, ExchangeType};
use types::token::NamedToken;

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
pub struct Ve33Addresses {
    #[serde(deserialize_with = "deserialize_address")]
    pub factory: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub router: Address,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniV3Addresses {
    #[serde(deserialize_with = "deserialize_address")]
    pub factory: Address,
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

pub type UniV3Exchanges = HashMap<ExchangeName, UniV3Addresses>;
pub type UniV2Exchanges = HashMap<ExchangeName, UniV2Addresses>;
pub type Ve33Exchanges = HashMap<ExchangeName, Ve33Addresses>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Exchanges {
    pub univ2: UniV2Exchanges,
    pub univ3: UniV3Exchanges,
    pub ve33: Option<Ve33Exchanges>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AaveV3Addresses {
    pub pool: Address,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MorphoAddresses {
    pub pool: Address,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lending {
    pub aave_v3: AaveV3Addresses,
    pub morpho: Option<MorphoAddresses>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tokens {
    #[serde(deserialize_with = "deserialize_address")]
    pub weth: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub usdc: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub usdt: Address,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChainAddressBook {
    pub exchanges: Exchanges,
    pub lending: Lending,
    pub tokens: Tokens,
    #[serde(deserialize_with = "deserialize_address")]
    pub multicall: Address,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Addressbook {
    pub arbitrum: ChainAddressBook,
    pub optimism: ChainAddressBook,
    pub base: ChainAddressBook,
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
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let current_file = std::path::Path::new(file!());
        let parent_dir = current_file.parent().unwrap();
        let filepath = parent_dir.join("addressbook.json");
        println!("Loading addressbook from: {:?}", filepath);
        let mut file = File::open(filepath)?;
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
                    .get(&exchange_name)
                    .unwrap()
                    .pools
                    .0
                    .get(pool_name)
                    .cloned(),
                ExchangeName::SushiswapV2 => book
                    .exchanges
                    .univ2
                    .get(&exchange_name)
                    .unwrap()
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
                    .get(&exchange_name)
                    .unwrap()
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
        self.get_chain_address_book(chain)
            .map(|config| config.tokens.weth)
    }

    pub fn get_usdc(&self, chain: &NamedChain) -> Option<Address> {
        self.get_chain_address_book(chain)
            .map(|config| config.tokens.usdc)
    }

    pub fn get_usdt(&self, chain: &NamedChain) -> Option<Address> {
        self.get_chain_address_book(chain)
            .map(|config| config.tokens.usdt)
    }

    pub fn get_token(&self, chain: &NamedChain, token_name: &NamedToken) -> Option<Address> {
        let config = self.get_chain_address_book(chain).unwrap();
        match token_name {
            NamedToken::USDC => Some(config.tokens.usdc),
            NamedToken::WETH => Some(config.tokens.weth),
            NamedToken::USDT => Some(config.tokens.usdt),
            _ => None,
        }
    }

    pub fn get_multicall(&self, chain: &NamedChain) -> Option<Address> {
        self.get_chain_address_book(chain)
            .map(|config| config.multicall)
    }

    pub fn get_uni_v3_quoter(
        &self,
        chain: &NamedChain,
        exchange_name: ExchangeName,
    ) -> Option<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        let quoter_address = match exchange_name {
            ExchangeName::UniswapV3 => {
                chain_config
                    .exchanges
                    .univ3
                    .get(&exchange_name)
                    .unwrap()
                    .quoter
            }
            _ => None,
        };

        return quoter_address;
    }

    pub fn get_uni_v3_universal_router(
        &self,
        chain: &NamedChain,
        exchange_name: ExchangeName,
    ) -> Option<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        let universal_router_address = match exchange_name {
            ExchangeName::UniswapV3 => {
                chain_config
                    .exchanges
                    .univ3
                    .get(&exchange_name)
                    .unwrap()
                    .universal_router
            }
            _ => None,
        };

        return universal_router_address;
    }

    pub fn get_uni_v3_swap_router(
        &self,
        chain: &NamedChain,
        exchange_name: ExchangeName,
    ) -> Option<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        let swap_router_address = match exchange_name {
            ExchangeName::UniswapV3 => {
                chain_config
                    .exchanges
                    .univ3
                    .get(&exchange_name)
                    .unwrap()
                    .swap_router_02
            }
            _ => None,
        };

        return swap_router_address;
    }

    pub fn get_uni_v2_swap_router(
        &self,
        chain: &NamedChain,
        exchange_name: ExchangeName,
    ) -> Option<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        let swap_router_address: Option<Address> = match exchange_name {
            ExchangeName::UniswapV2 => Some(
                chain_config
                    .exchanges
                    .univ2
                    .get(&exchange_name)
                    .unwrap()
                    .router,
            ),
            _ => None,
        };

        return swap_router_address;
    }

    pub fn get_ve33_router(
        &self,
        chain: &NamedChain,
        exchange_name: ExchangeName,
    ) -> Option<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        chain_config
            .exchanges
            .ve33
            .as_ref()
            .unwrap()
            .get(&exchange_name)
            .map(|config| config.router)
    }

    pub fn get_ve33_factory(
        &self,
        chain: &NamedChain,
        exchange_name: ExchangeName,
    ) -> Option<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        chain_config
            .exchanges
            .ve33
            .as_ref()
            .unwrap()
            .get(&exchange_name)
            .map(|config| config.factory)
    }

    pub fn get_factory(&self, chain: &NamedChain, exchange_name: ExchangeName) -> Option<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        match exchange_name {
            ExchangeName::UniswapV2 => Some(
                chain_config
                    .exchanges
                    .univ2
                    .get(&exchange_name)
                    .unwrap()
                    .factory,
            ),
            ExchangeName::UniswapV3 => Some(
                chain_config
                    .exchanges
                    .univ3
                    .get(&exchange_name)
                    .unwrap()
                    .factory,
            ),
            ExchangeName::Aerodrome => Some(
                chain_config
                    .exchanges
                    .ve33
                    .as_ref()
                    .unwrap()
                    .get(&exchange_name)
                    .unwrap()
                    .factory,
            ),
            _ => None,
        }
    }

    pub fn get_v2_factories(&self, chain: &NamedChain) -> Vec<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        vec![
            chain_config
                .exchanges
                .univ2
                .get(&ExchangeName::UniswapV2)
                .unwrap()
                .factory,
            chain_config
                .exchanges
                .univ2
                .get(&ExchangeName::SushiswapV2)
                .unwrap()
                .factory,
        ]
    }

    pub fn get_v3_factories(&self, chain: &NamedChain) -> Vec<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        let exchanges = chain_config.exchanges.univ3.clone();

        let factories: Vec<Address> = exchanges
            .into_iter()
            .map(|e| e.1.factory)
            .filter(|addr| !addr.is_zero())
            .collect();

        factories
    }

    fn get_chain_address_book(&self, chain: &NamedChain) -> Option<&ChainAddressBook> {
        match chain {
            NamedChain::Arbitrum => Some(&self.arbitrum),
            NamedChain::Optimism => Some(&self.optimism),
            NamedChain::Mainnet => Some(&self.mainnet),
            NamedChain::Base => Some(&self.base),
            _ => None,
        }
    }

    pub fn get_lending_pool(
        &self,
        chain: &NamedChain,
        protocol: &str,
    ) -> Result<Address, eyre::Error> {
        match chain {
            NamedChain::Mainnet => match protocol {
                "aave_v3" => Ok(self.mainnet.lending.aave_v3.pool),
                "morpho" => Ok(self.mainnet.lending.morpho.as_ref().unwrap().pool),
                _ => Err(eyre::eyre!("Unsupported lending protocol")),
            },
            NamedChain::Optimism => match protocol {
                "aave_v3" => Ok(self.optimism.lending.aave_v3.pool),
                _ => Err(eyre::eyre!("Unsupported lending protocol")),
            },
            NamedChain::Base => match protocol {
                "aave_v3" => Ok(self.base.lending.aave_v3.pool),
                "morpho" => Ok(self.base.lending.morpho.as_ref().unwrap().pool),
                _ => Err(eyre::eyre!("Unsupported lending protocol")),
            },
            NamedChain::Arbitrum => match protocol {
                "aave_v3" => Ok(self.arbitrum.lending.aave_v3.pool),
                _ => Err(eyre::eyre!("Unsupported lending protocol")),
            },
            _ => Err(eyre::eyre!("Unsupported chain for lending")),
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
        let addressbook = Addressbook::load().expect("Failed to load addressbook");

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
        let addressbook = Addressbook::load().expect("Failed to load addressbook");

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
        let addressbook = Addressbook::load().expect("Failed to load addressbook");

        let arb_config = addressbook
            .get_chain_address_book(&NamedChain::Arbitrum)
            .expect("Failed to get Arbitrum config");

        let univ3_config = arb_config
            .exchanges
            .univ3
            .get(&ExchangeName::UniswapV3)
            .unwrap();

        // Check factory address
        assert!(!univ3_config.factory.is_zero());

        // Check router addresses
        if let Some(router02) = univ3_config.swap_router_02 {
            assert!(!router02.is_zero());
        }
    }

    #[test]
    fn test_get_pool_by_name() {
        let addressbook = Addressbook::load().expect("Failed to load addressbook");

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
        let addressbook = Addressbook::load().expect("Failed to load addressbook");

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
        let addressbook = Addressbook::load().expect("Failed to load addressbook");

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
}
