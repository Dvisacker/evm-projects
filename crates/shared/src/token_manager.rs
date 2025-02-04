use addressbook::Addressbook;
use alloy::sol;
use alloy_chains::NamedChain;
use alloy_primitives::Address;
use lazy_static::lazy_static;
use provider::get_signer_provider_map;
use provider::SignerProviderMap;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use types::token::{Token, TokenIsh};

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

lazy_static! {
    static ref TOKEN_MANAGER: Mutex<Option<TokenManager>> = Mutex::new(None);
}

pub struct TokenManager {
    tokens: Mutex<HashMap<Address, Token>>,
    addressbook: Arc<Addressbook>,
    providers: Arc<SignerProviderMap>,
}

impl TokenManager {
    pub async fn instance() -> Arc<TokenManager> {
        if let Ok(manager) = TOKEN_MANAGER.lock() {
            if let Some(manager) = manager.as_ref() {
                return Arc::new(manager.clone());
            }
        }

        let new_instance = Self::new().await;

        if let Ok(mut manager) = TOKEN_MANAGER.lock() {
            *manager = Some(new_instance);
            return Arc::new(manager.as_ref().unwrap().clone());
        }

        panic!("Failed to initialize TokenManager");
    }

    async fn new() -> Self {
        let providers = get_signer_provider_map().await;
        let addressbook = Addressbook::load().unwrap();

        TokenManager {
            tokens: Mutex::new(HashMap::new()),
            addressbook: Arc::new(addressbook),
            providers,
        }
    }

    pub fn clone(&self) -> Self {
        TokenManager {
            tokens: Mutex::new(HashMap::new()),
            addressbook: self.addressbook.clone(),
            providers: self.providers.clone(),
        }
    }

    pub fn get_by_address(
        &self,
        address: Address,
        chain: &NamedChain,
    ) -> Result<Token, Box<dyn Error>> {
        let mut tokens = self
            .tokens
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        if let Some(token) = tokens.get(&address) {
            Ok(token.clone())
        } else {
            let provider = (*(*self.providers.get(chain).unwrap()).clone()).clone();
            let new_token = Token::new(address, provider);

            tokens.insert(address, new_token.clone());
            Ok(new_token)
        }
    }

    pub fn get(&self, chain: &NamedChain, token: &TokenIsh) -> Result<Token, Box<dyn Error>> {
        match token {
            TokenIsh::Named(named) => {
                let address = self.addressbook.get_token(chain, named).unwrap();
                self.get_by_address(address, chain)
            }
            TokenIsh::Address(address) => self.get_by_address(*address, chain),
            TokenIsh::Token(contract) => Ok(contract.clone()),
        }
    }
}
