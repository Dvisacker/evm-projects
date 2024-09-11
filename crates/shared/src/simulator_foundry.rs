use alloy::network::Network;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::transports::Transport;
use foundry_config::Config;
use foundry_evm::backend::Backend;
use foundry_evm::executors::Executor;
use foundry_evm::fork::CreateFork;
use foundry_evm::opts::EvmOpts;
use std::marker::PhantomData;
use std::sync::Arc;

// #[derive(Clone)]
pub struct FoundrySimulator<T, N, P>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N> + 'static,
{
    pub provider: Arc<P>,
    pub owner: Address,
    pub executor: Executor,
    pub block_number: u64,

    // pub token: TokenABI,
    // pub v2_pool: V2PoolABI,
    // pub simulator: SimulatorABI,
    pub simulator_address: Address,
    _phantom: PhantomData<(T, N)>,
}

impl<T, N, P> FoundrySimulator<T, N, P>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    pub async fn new(provider: P, owner: Address, block_number: u64, url: String) -> Self
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        // let evm_opts = EvmOpts::default();
        // let env = Env::default();
        let provider = Arc::new(provider);
        let config = Config::figment();
        let mut evm_opts = config.extract::<EvmOpts>().unwrap();
        evm_opts.fork_block_number = Some(block_number);
        let (env, _block) = evm_opts.fork_evm_env(url.clone()).await.unwrap();
        let fork = CreateFork {
            enable_caching: true,
            url: url.clone(),
            env: env.clone(),
            evm_opts,
        };
        let backend = Backend::spawn(Some(fork));
        let executor = Executor::builder().build(env, backend);

        Self {
            provider,
            owner,
            executor,
            block_number,
            simulator_address: Address::ZERO,
            _phantom: PhantomData,
        }
    }

    // pub fn get_block_number(&mut self) -> U256 {
    //     self.evm.context.evm.inner.env.block.number
    // }

    // pub fn get_coinbase(&mut self) -> Address {
    //     self.evm.context.evm.inner.env.block.coinbase
    // }

    // pub fn get_base_fee(&mut self) -> U256 {
    //     self.evm.context.evm.inner.env.block.basefee
    // }
}
