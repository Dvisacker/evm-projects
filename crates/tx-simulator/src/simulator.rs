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
use types::exchange::ExchangeName;

#[derive(Clone)]
pub struct TxSimulatorClient<T, P>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Clone,
{
    #[allow(unused)]
    address: Address,
    simulator: TxSimulatorInstance<T, P>,
    chain: NamedChain,
    addressbook: Addressbook,
    #[allow(unused)]
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
                token_in = token_a;
                token_out = token_b;
            } else {
                token_in = token_b;
                token_out = token_a;
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

            // The token_in for the next swap is the token_out of the current swap
            token_in = token_out;
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

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use addressbook::utils::get_workspace_dir;
    use alloy_chains::{Chain, NamedChain};
    use amms::amm::{uniswap_v2::UniswapV2Pool, uniswap_v3::UniswapV3Pool, ve33::Ve33Pool};
    use provider::{get_anvil_signer_provider, get_default_signer_provider};
    use shared::utils::get_most_recent_deployment;
    use std::str::FromStr;

    use super::*;

    async fn get_broadcast_dir_path() -> PathBuf {
        let workspace_dir = get_workspace_dir().unwrap();
        let current_file = std::path::Path::new(file!());
        let parent_dir = current_file.parent().unwrap().parent().unwrap();
        workspace_dir.join(parent_dir).join("contracts/broadcast/")
    }

    #[tokio::test]
    async fn test_simulate_route_uniswap_v3() -> Result<(), Error> {
        dotenv::dotenv().ok();
        let chain = Chain::try_from(8453).unwrap();
        let provider = get_default_signer_provider(chain).await;
        let broadcast_dir_path = get_broadcast_dir_path().await;
        let simulator_address =
            get_most_recent_deployment("TxSimulator", 8453, Some(broadcast_dir_path)).unwrap();
        let simulator = TxSimulatorClient::new(simulator_address, provider.clone()).await;
        let weth = Address::from_str("0x4200000000000000000000000000000000000006").unwrap();

        let mut weth_usdc_pool = UniswapV3Pool::new_empty(
            Address::from_str("0xd0b53d9277642d899df5c87a3966a349a798f224").unwrap(),
            NamedChain::Base,
        )
        .await
        .unwrap();

        weth_usdc_pool.populate_data(None, provider).await.unwrap();

        let result = simulator
            .simulate_route(
                weth,
                U256::from(100000000000000u128),
                &[AMM::UniswapV3Pool(weth_usdc_pool)],
            )
            .await;

        assert!(result.unwrap() > U256::from(0));
        Ok(())
    }

    #[tokio::test]
    async fn test_simulate_route_uniswap_v2() -> Result<(), Error> {
        dotenv::dotenv().ok();
        let provider = get_anvil_signer_provider().await;
        let simulator = TxSimulatorClient::new(
            Address::from_str(&env::var("SIMULATOR_ADDRESS").unwrap()).unwrap(),
            provider.clone(),
        )
        .await;

        let mut weth_usdc_pool = UniswapV2Pool::new_from_address(
            Address::from_str("0x88A43bbDF9D098eEC7bCEda4e2494615dfD9bB9C").unwrap(),
            300,
            provider.clone(),
        )
        .await
        .unwrap();

        let weth = Address::from_str("0x4200000000000000000000000000000000000006").unwrap();

        weth_usdc_pool.populate_data(None, provider).await.unwrap();

        let result = simulator
            .simulate_route(
                weth,
                U256::from(100000000000000u128),
                &[AMM::UniswapV2Pool(weth_usdc_pool)],
            )
            .await;

        assert!(result.unwrap() > U256::from(0));
        Ok(())
    }

    #[tokio::test]
    async fn test_simulate_route_aerodrome() -> Result<(), Error> {
        dotenv::dotenv().ok();
        let provider = get_anvil_signer_provider().await;
        let broadcast_dir_path = get_broadcast_dir_path().await;
        let simulator_address =
            get_most_recent_deployment("TxSimulator", 8453, Some(broadcast_dir_path)).unwrap();
        let simulator = TxSimulatorClient::new(simulator_address, provider.clone()).await;

        let mut weth_usdc_pool = Ve33Pool::new_from_address(
            Address::from_str("0xcdac0d6c6c59727a65f871236188350531885c43").unwrap(),
            30, // not used i think
            provider.clone(),
        )
        .await
        .unwrap();

        let weth = Address::from_str("0x4200000000000000000000000000000000000006").unwrap();

        weth_usdc_pool.populate_data(None, provider).await.unwrap();

        let result = simulator
            .simulate_route(
                weth,
                U256::from(100000000000000u128),
                &[AMM::Ve33Pool(weth_usdc_pool)],
            )
            .await;

        assert!(result.unwrap() > U256::from(0));
        Ok(())
    }

    #[tokio::test]
    async fn test_simulate_route_aerodrome_2() -> Result<(), Error> {
        dotenv::dotenv().ok();
        let provider = get_anvil_signer_provider().await;
        let broadcast_dir_path = get_broadcast_dir_path().await;
        let simulator_address =
            get_most_recent_deployment("TxSimulator", 8453, Some(broadcast_dir_path)).unwrap();
        let simulator = TxSimulatorClient::new(simulator_address, provider.clone()).await;

        let mut weth_wseth_pool = Ve33Pool::new_from_address(
            Address::from_str("0x29BBb5F85F01702Ec85D217CEEb2d9657700cF04").unwrap(),
            30, // not used i think
            provider.clone(),
        )
        .await
        .unwrap();

        let mut fbomb_wseth_pool = Ve33Pool::new_from_address(
            Address::from_str("0xBd1F3d188de7eE07B1b323C0D26D6720CAfB8780").unwrap(),
            30, // not used i think
            provider.clone(),
        )
        .await
        .unwrap();

        let mut weth_fbomb_pool = Ve33Pool::new_from_address(
            Address::from_str("0x4F9Dc2229f2357B27C22db56cB39582c854Ad6d5").unwrap(),
            30, // not used i think
            provider.clone(),
        )
        .await
        .unwrap();

        let weth = Address::from_str("0x4200000000000000000000000000000000000006").unwrap();

        weth_wseth_pool
            .populate_data(None, provider.clone())
            .await
            .unwrap();
        fbomb_wseth_pool
            .populate_data(None, provider.clone())
            .await
            .unwrap();
        weth_fbomb_pool
            .populate_data(None, provider.clone())
            .await
            .unwrap();

        let result = simulator
            .simulate_route(
                weth,
                U256::from(100000000000000u128),
                &[
                    AMM::Ve33Pool(weth_wseth_pool.clone()),
                    AMM::Ve33Pool(fbomb_wseth_pool.clone()),
                    AMM::Ve33Pool(weth_fbomb_pool.clone()),
                ],
            )
            .await;

        assert!(result.unwrap() > U256::from(0));
        Ok(())
    }
}
