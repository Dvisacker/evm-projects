use alloy::primitives::Address;
use alloy_chains::NamedChain;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn deserialize_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Address::from_str(&s).map_err(serde::de::Error::custom)
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
pub struct ChainAddressBook {
    pub exchanges: HashMap<String, ExchangeAddressBook>,
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
        chain: &NamedChain,
        exchange_name: &str,
        pool_name: &str,
    ) -> Option<Address> {
        let book = self.get_chain_address_book(chain)?;
        book.exchanges
            .get(exchange_name)?
            .pools
            .0
            .get(pool_name)
            .cloned()
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
            .values()
            .flat_map(|exchange| exchange.pools.0.values().cloned())
            .collect()
    }

    pub fn get_factories_by_chain(&self, chain: &NamedChain) -> Vec<Address> {
        let chain_config = self.get_chain_address_book(chain).unwrap();
        chain_config
            .exchanges
            .values()
            .map(|exchange| exchange.factory)
            .collect()
    }

    fn get_chain_address_book(&self, chain: &NamedChain) -> Option<&ChainAddressBook> {
        match chain {
            NamedChain::Arbitrum => Some(&self.arbitrum),
            NamedChain::Optimism => Some(&self.optimism),
            NamedChain::Mainnet => Some(&self.mainnet),
            _ => None,
        }
    }
}
