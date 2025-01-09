use crate::bindings::txsimulator::TxSimulator::{SwapParams, TxSimulatorInstance};
use addressbook::Addressbook;
use alloy::{
    network::Ethereum,
    primitives::{aliases::U24, Address, U256},
    providers::Provider,
    transports::Transport,
};
use alloy_chains::NamedChain;
use amms::amm::{AutomatedMarketMaker, AMM};
use eyre::Error;
use std::str::FromStr;
use types::exchange::ExchangeName;

pub struct TxSimulatorClient<T, P>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Clone,
{
    address: Address,
    simulator: TxSimulatorInstance<T, P>,
    chain: NamedChain,
    addressbook: Addressbook,
    provider: P,
}

impl<T, P> TxSimulatorClient<T, P>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Clone,
{
    pub async fn new(address: Address, provider: P) -> Self {
        let addressbook = Addressbook::load().expect("Failed to load addressbook");
        let chain_id = provider
            .get_chain_id()
            .await
            .expect("Failed to get chain id");
        let chain = NamedChain::try_from(chain_id).expect("Unknown chain");
        Self {
            address,
            simulator: TxSimulatorInstance::new(address, provider.clone()),
            addressbook,
            chain,
            provider,
        }
    }

    pub fn build_swap_params(
        &self,
        token_0: Address,
        amount: U256,
        route: &[AMM],
    ) -> Result<Vec<SwapParams>, Error> {
        let mut token_in = token_0;
        let mut params = Vec::new();

        for amm in route {
            let tokens = amm.tokens();
            let token_a = tokens[0];
            let token_b = tokens[1];
            let token_out;

            if token_in == token_a {
                token_in = token_b;
                token_out = token_a;
            } else {
                token_in = token_a;
                token_out = token_b;
            }

            match amm {
                AMM::UniswapV2Pool(_pool) => {
                    let uniswap_v2_factory = self
                        .addressbook
                        .get_factory(&self.chain, ExchangeName::UniswapV2)
                        .expect("Failed to get uniswap v2 factory");

                    params.push(SwapParams {
                        protocol: 0,
                        handler: uniswap_v2_factory,
                        tokenIn: token_in,
                        tokenOut: token_out,
                        amount: amount,
                        fee: U24::from(0),
                        stable: false,
                        factory: Address::ZERO,
                    });
                }
                AMM::UniswapV3Pool(pool) => {
                    let uniswap_v3_quoter = self
                        .addressbook
                        .get_uni_v3_quoter(&self.chain, ExchangeName::UniswapV3)
                        .expect("Failed to get uniswap v3 quoter");

                    params.push(SwapParams {
                        protocol: 1,
                        handler: uniswap_v3_quoter,
                        tokenIn: token_in,
                        tokenOut: token_out,
                        amount: amount,
                        fee: U24::from(pool.fee),
                        stable: false,
                        factory: Address::ZERO,
                    });
                }
                AMM::CurvePool(pool) => {
                    params.push(SwapParams {
                        protocol: 2,
                        handler: pool.address,
                        tokenIn: token_in,
                        tokenOut: token_out,
                        amount: amount,
                        fee: U24::from(0),
                        stable: true,
                        factory: Address::ZERO,
                    });
                }
                AMM::Ve33Pool(pool) => {
                    let ve33_router = self
                        .addressbook
                        .get_ve33_router(&self.chain, ExchangeName::Aerodrome)
                        .expect("Failed to get ve33 router");

                    let ve33_factory = self
                        .addressbook
                        .get_ve33_factory(&self.chain, ExchangeName::Aerodrome)
                        .expect("Failed to get ve33 factory");

                    params.push(SwapParams {
                        protocol: 3,
                        handler: ve33_router,
                        tokenIn: token_in,
                        tokenOut: token_out,
                        amount: amount,
                        fee: U24::from(30), // Not used by aerodrome
                        stable: pool.stable,
                        factory: ve33_factory,
                    });
                }
                _ => {
                    return Err(eyre::eyre!("Unsupported AMM: {:?}", amm));
                }
            }
        }
        Ok(params)
    }

    pub async fn simulate_route(
        &self,
        token_in: Address,
        amount_in: U256,
        route: &[AMM],
    ) -> Result<U256, Error> {
        let params = self
            .build_swap_params(token_in, amount_in, route)
            .expect("Failed to build swap params");

        let call_builder = self.simulator.simulateSwapIn(params);
        let result = call_builder.call().await?;
        Ok(result._0)
    }
}

mod tests {
    use std::env;

    use alloy_chains::NamedChain;
    use amms::amm::{uniswap_v2::UniswapV2Pool, uniswap_v3::UniswapV3Pool, ve33::Ve33Pool};
    use provider::get_anvil_signer_provider;

    use super::*;

    #[tokio::test]
    async fn test_simulate_route_uniswap_v3() -> Result<(), Error> {
        dotenv::dotenv().ok();
        let provider = get_anvil_signer_provider().await;
        let simulator = TxSimulatorClient::new(
            Address::from_str(&env::var("SIMULATOR_ADDRESS").unwrap()).unwrap(),
            provider.clone(),
        )
        .await;

        let mut pool = UniswapV3Pool::new_empty(
            Address::from_str("0xd0b53d9277642d899df5c87a3966a349a798f224").unwrap(),
            NamedChain::Base,
        )
        .await
        .unwrap();

        pool.populate_data(None, provider).await.unwrap();

        let result = simulator
            .simulate_route(
                Address::from_str("0x4200000000000000000000000000000000000006").unwrap(),
                U256::from(100000000000000u128),
                &[AMM::UniswapV3Pool(pool)],
            )
            .await;

        println!("Result: {:?}", result);

        Ok(())
    }
}
