use alloy::network::{Ethereum, EthereumWallet};
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{Identity, RootProvider};
use alloy::transports::BoxTransport;
use alloy_primitives::Address;
use bindings::ierc20::IERC20::IERC20Instance;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

pub type SignerProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<BoxTransport>,
    BoxTransport,
>;

pub type Token = IERC20Instance<BoxTransport, Arc<SignerProvider>>;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ValueEnum, PartialEq, Eq, Hash)]
pub enum NamedToken {
    USDC,
    USDT,
    WETH,
}

impl fmt::Display for NamedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NamedToken::USDC => "USDC",
                NamedToken::USDT => "USDT",
                NamedToken::WETH => "WETH",
            }
        )
    }
}

impl NamedToken {
    /// Returns all available tokens
    pub fn all() -> Vec<NamedToken> {
        vec![NamedToken::USDC, NamedToken::USDT, NamedToken::WETH]
    }

    /// Returns true if the token is a stablecoin
    pub fn is_stable(&self) -> bool {
        matches!(self, NamedToken::USDC | NamedToken::USDT)
    }
}

#[derive(Clone)]
pub enum TokenIsh {
    Named(NamedToken),
    Address(Address),
    Token(Token),
}

// Implement FromStr to allow parsing from command line
impl FromStr for TokenIsh {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First try to parse as a named token
        if let Ok(named) = NamedToken::from_str(s, true) {
            return Ok(TokenIsh::Named(named));
        }

        // If that fails, try to parse as an address
        match Address::from_str(s) {
            Ok(addr) => Ok(TokenIsh::Address(addr)),
            Err(_) => Err(format!(
                "Invalid token: must be either a known token name or a valid Ethereum address: {}",
                s
            )),
        }
    }
}

impl From<TokenIsh> for NamedToken {
    fn from(token: TokenIsh) -> Self {
        match token {
            TokenIsh::Named(named_token) => named_token,
            TokenIsh::Address(_) => panic!("Cannot convert address to NamedToken"),
            TokenIsh::Token(_) => panic!("Cannot convert token to NamedToken"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(NamedToken::USDC.to_string(), "USDC");
        assert_eq!(NamedToken::USDT.to_string(), "USDT");
        assert_eq!(NamedToken::WETH.to_string(), "WETH");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            NamedToken::from_str("USDC", true).unwrap(),
            NamedToken::USDC
        );
        assert_eq!(
            NamedToken::from_str("usdc", true).unwrap(),
            NamedToken::USDC
        );
        assert_eq!(
            NamedToken::from_str("USDT", true).unwrap(),
            NamedToken::USDT
        );
        assert_eq!(
            NamedToken::from_str("WETH", true).unwrap(),
            NamedToken::WETH
        );
        assert!(NamedToken::from_str("INVALID", false).is_err());
    }

    #[test]
    fn test_is_stable() {
        assert!(NamedToken::USDC.is_stable());
        assert!(NamedToken::USDT.is_stable());
        assert!(!NamedToken::WETH.is_stable());
    }
}
