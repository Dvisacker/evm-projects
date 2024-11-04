use crate::addressbook::Addressbook;
use crate::provider::{get_provider_map, ProviderMap, SignerProvider};
use alloy::network::EthereumWallet;
use alloy::sol;
use alloy::transports::BoxTransport;
use alloy_chains::NamedChain;
use alloy_primitives::Address;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use IERC20::IERC20Instance;

sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IERC20 {
        function decimals() external view returns (uint8);
        function approve(address spender, uint256 amount) external returns (bool);
        function balanceOf(address account) external view returns (uint256);
        function transfer(address recipient, uint256 amount) external returns (bool);
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
    }
}

type Token = IERC20Instance<BoxTransport, Arc<SignerProvider>>;

pub struct TokenManager {
    tokens: Mutex<HashMap<Address, Token>>,
    addressbook: Arc<Addressbook>,
    providers: Arc<ProviderMap>,
}

impl TokenManager {
    pub async fn new(wallet: EthereumWallet) -> Self {
        let providers = get_provider_map(wallet).await;
        let addressbook = Addressbook::load(None).unwrap();

        TokenManager {
            tokens: Mutex::new(HashMap::new()),
            addressbook: Arc::new(addressbook),
            providers: providers,
        }
    }

    pub fn get(&self, address: Address, chain: &NamedChain) -> Result<Token, Box<dyn Error>> {
        let mut tokens = self
            .tokens
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        if let Some(token) = tokens.get(&address) {
            Ok(token.clone())
        } else {
            let provider = self.providers.get(chain).unwrap();
            let new_token = IERC20::new(address, provider.clone());
            tokens.insert(address, new_token.clone());
            Ok(new_token)
        }
    }

    pub fn get_by_name(&self, chain: &NamedChain, name: &str) -> Result<Token, Box<dyn Error>> {
        let address = self.addressbook.get_token(chain, name).unwrap();
        self.get(address, chain)
    }
}
