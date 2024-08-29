use std::str::FromStr;

use alloy_chains::NamedChain;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S>(chain: &NamedChain, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    chain.to_string().serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NamedChain, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NamedChain::from_str(&s).map_err(serde::de::Error::custom)
}
