use crate::bindings::txsimulator::TxSimulator::{SwapParams, TxSimulatorInstance};
use alloy::{
    network::Ethereum,
    primitives::{aliases::U24, Address, U256},
    providers::Provider,
    transports::Transport,
};
use amms::amm::{AutomatedMarketMaker, AMM};
use eyre::Error;

pub struct TxSimulatorClient<T, P>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Clone,
{
    address: Address,
    simulator: TxSimulatorInstance<T, P>,
    provider: P,
}

impl<T, P> TxSimulatorClient<T, P>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Clone,
{
    pub async fn new(address: Address, provider: P) -> Self {
        Self {
            address,
            simulator: TxSimulatorInstance::new(address, provider.clone()),
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
                    params.push(SwapParams {
                        protocol: 0,
                        handler: self.address,
                        tokenIn: token_in,
                        tokenOut: token_out,
                        amount: amount,
                        fee: U24::from(0),
                        stable: false,
                    });
                }
                AMM::UniswapV3Pool(pool) => {
                    params.push(SwapParams {
                        protocol: 1,
                        handler: self.address,
                        tokenIn: token_in,
                        tokenOut: token_out,
                        amount: amount,
                        fee: U24::from(pool.fee),
                        stable: false,
                    });
                }
                AMM::CurvePool(_pool) => {
                    params.push(SwapParams {
                        protocol: 2,
                        handler: self.address,
                        tokenIn: token_in,
                        tokenOut: token_out,
                        amount: amount,
                        fee: U24::from(0),
                        stable: false,
                    });
                }
                AMM::Ve33Pool(pool) => {
                    params.push(SwapParams {
                        protocol: 3,
                        handler: pool.factory,
                        tokenIn: token_in,
                        tokenOut: token_out,
                        amount: amount,
                        fee: U24::from(0),
                        stable: pool.stable,
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
