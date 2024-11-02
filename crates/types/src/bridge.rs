use std::fmt;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ValueEnum)]
pub enum BridgeName {
    Accross,
    #[serde(rename = "stargateV2")]
    StargateV2,
}

impl fmt::Display for BridgeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl BridgeName {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "across" => Ok(BridgeName::Accross),
            "stargateV2" => Ok(BridgeName::StargateV2),
            _ => Err(format!("Invalid bridge name: {}", s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            BridgeName::Accross => "across",
            BridgeName::StargateV2 => "stargateV2",
        }
    }
}
