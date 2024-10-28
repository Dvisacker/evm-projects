use std::{
    str::FromStr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use alloy::{network::Network, providers::Provider, transports::Transport};
use alloy_primitives::{Address, U256};
use amms::{
    amm::{uniswap_v2::UniswapV2Pool, uniswap_v3::UniswapV3Pool},
    errors::AMMError,
};

use crate::helpers::get_contract_creation_block;
use alloy::sol;

sol! {
    /// Interface of the UniswapV2Pair
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IUniswapV2Pair {
        event Sync(uint112 reserve0, uint112 reserve1);
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
        function token0() external view returns (address);
        function token1() external view returns (address);
        function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data);
        function factory() external view returns (address);
    }
}

sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IUniswapV2Router02 {
        function factory() external pure returns (address);
        function WETH() external pure returns (address);
        function quote(uint amountA, uint reserveA, uint reserveB) external pure returns (uint amountB);
        function getAmountOut(uint amountIn, uint reserveIn, uint reserveOut) external pure returns (uint amountOut);
        function getAmountIn(uint amountOut, uint reserveIn, uint reserveOut) external pure returns (uint amountIn);
        function getAmountsOut(uint amountIn, address[] calldata path) external view returns (uint[] memory amounts);
        function getAmountsIn(uint amountOut, address[] calldata path) external view returns (uint[] memory amounts);

        // Swap Functions
        function swapExactTokensForTokens(
            uint amountIn,
            uint amountOutMin,
            address[] calldata path,
            address to,
            uint deadline
        ) external returns (uint[] memory amounts);

        function swapTokensForExactTokens(
            uint amountOut,
            uint amountInMax,
            address[] calldata path,
            address to,
            uint deadline
        ) external returns (uint[] memory amounts);

        function swapExactETHForTokens(
            uint amountOutMin,
            address[] calldata path,
            address to,
            uint deadline
        ) external payable returns (uint[] memory amounts);

        function swapTokensForExactETH(
            uint amountOut,
            uint amountInMax,
            address[] calldata path,
            address to,
            uint deadline
        ) external returns (uint[] memory amounts);

        function swapExactTokensForETH(
            uint amountIn,
            uint amountOutMin,
            address[] calldata path,
            address to,
            uint deadline
        ) external returns (uint[] memory amounts);

        function swapETHForExactTokens(
            uint amountOut,
            address[] calldata path,
            address to,
            uint deadline
        ) external payable returns (uint[] memory amounts);
    }
}

sol! {
    /// Interface of the ERC20 token
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);
    }
}

pub async fn load_uni_v2_pool<T, N, P>(
    pool_address: Address,
    provider: Arc<P>,
) -> Result<UniswapV2Pool, AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let pool = UniswapV2Pool::new_from_address(pool_address, 300, provider).await?;
    Ok(pool)
}

pub async fn load_uni_v3_pool<T, N, P>(
    pool_address: Address,
    provider: Arc<P>,
) -> Result<UniswapV3Pool, AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let end_block = provider.get_block_number().await.unwrap();
    let contract_creation_block =
        get_contract_creation_block(provider.clone(), pool_address, 0, end_block)
            .await
            .unwrap();
    let pool =
        UniswapV3Pool::new_from_address(pool_address, contract_creation_block, provider).await?;
    Ok(pool)
}

pub async fn swap_v2_pool<T, N, P>(
    provider: Arc<P>,
    recipient: Address,
    pool_address: Address,
    token_in_address: Address,
    amount_in: U256,
) -> Result<U256, AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let router_address = Address::from_str("0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506")
        .expect("Invalid router address");
    let router = IUniswapV2Router02::new(router_address, provider.clone());

    let pool = load_uni_v2_pool(pool_address, provider.clone()).await?;
    let token_in = IERC20::new(token_in_address, provider.clone());
    let zero_for_one = token_in_address == pool.token_a;
    let token_out_address = if zero_for_one {
        pool.token_b
    } else {
        pool.token_a
    };

    let approve_tx = token_in.approve(router_address, amount_in).send().await?;
    // Wait for approval
    let receipt = approve_tx.get_receipt().await.unwrap();
    println!("approve receipt: {:?}", receipt);

    // 3. Calculate minimum amount out (to protect against slippage)
    let return_data = router
        .getAmountsOut(amount_in, vec![token_in_address, token_out_address])
        .call()
        .await?;

    let amount_out = return_data.amounts[1];

    let amount_out_min = amount_out * U256::from(995) / U256::from(1000);

    println!("Swapping {} tokens", amount_in);
    println!("Minimum output: {}", amount_out_min);

    let deadline = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 1200; // 20 minutes

    // 5. Execute swap
    let swap_tx = router
        .swapExactTokensForTokens(
            amount_in,
            amount_out_min,
            vec![token_in_address, token_out_address],
            recipient,
            U256::from(deadline),
        )
        .send()
        .await?;

    let receipt = swap_tx.get_receipt().await.unwrap();
    println!("Swap completed in tx: {:?}", receipt);

    Ok(U256::ZERO)
}
