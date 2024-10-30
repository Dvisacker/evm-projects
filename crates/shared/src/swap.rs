use std::{
    str::FromStr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use alloy::{
    network::{Ethereum, Network},
    providers::Provider,
    transports::Transport,
};
use alloy_primitives::{aliases::U24, Address, U160, U256};
use alloy_rpc_types::TransactionReceipt;
use amms::{
    amm::{
        uniswap_v2::UniswapV2Pool,
        uniswap_v3::{factory::IUniswapV3Factory, IUniswapV3Pool, UniswapV3Pool},
    },
    errors::AMMError,
};
use eyre::Error;

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

// erc20 contract
sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);
    }
}

// swap router contract
sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract ISwapRouter {
        struct ExactInputSingleParams {
            address tokenIn;
            address tokenOut;
            uint24 fee;
            address recipient;
            uint256 deadline;
            uint256 amountIn;
            uint256 amountOutMinimum;
            uint160 sqrtPriceLimitX96;
        }

        function exactInputSingle(
            ExactInputSingleParams calldata params
        ) external payable returns (uint256 amountOut);
    }
}

// quoter contract
sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IQuoter {
        function quoteExactInputSingle(address tokenIn, address tokenOut, uint24 fee, uint256 amountIn) external view returns (uint256 amountOut);
    }
}

pub fn sqrt_price_x96_to_price(
    sqrt_price_x96: U256,
    decimals_token0: u8,
    decimals_token1: u8,
) -> Result<f64, Error> {
    // Square the price
    let price_x192 = sqrt_price_x96 * sqrt_price_x96;
    // Divide by 2^192 to get the actual price
    let price: U256 = price_x192 >> 192;
    // Convert to f64, adjusting for decimal places
    let decimal_adjustment = 10f64.powi((decimals_token1 as i32) - (decimals_token0 as i32));
    let price_f64 = price.to::<u128>() as f64 * decimal_adjustment / 2f64.powi(192);

    Ok(price_f64)
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

/* This function makes extra calls to get not only the best fee tier but also the corresponding pool address
When calling the router/quoter, the pool address is not needed so in that case, the find_best_v3_fee_tier function is preferred
because it uses less calls. */
pub async fn find_best_v3_pool<T, N, P>(
    provider: Arc<P>,
    token_in: Address,
    token_out: Address,
    amount_in: U256,
) -> Result<(Address, u32, U256), Error>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let factory_address = Address::from_str("0x1F98431c8aD98523631AE4a59f267346ea31F984")
        .expect("Invalid factory address");

    let factory = IUniswapV3Factory::new(factory_address, provider.clone());

    // Standard fee tiers
    let fee_tiers = vec![100, 500, 3000, 10000];
    let mut best_pool = None;
    let mut best_fee = 0u32;
    let mut best_quote = U256::ZERO;

    // Get quote for this pool
    let quoter = IQuoter::new(
        Address::from_str("0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6").unwrap(),
        provider.clone(),
    );

    for fee in fee_tiers {
        // Get pool address for this fee tier
        let pool_address = match factory
            .getPool(token_in, token_out, U24::from(fee))
            .call()
            .await
        {
            Ok(result) => result.pool,
            Err(_) => continue,
        };

        // Check if pool exists by checking its liquidity
        let pool = IUniswapV3Pool::new(pool_address, provider.clone());

        // let sqrt_price_x96 = match pool.slot0().call().await {
        //     Ok(slot0) => slot0._0,
        //     Err(_) => continue,
        // };

        // Try to get pool liquidity - if it fails, pool doesn't exist
        let liquidity = match pool.liquidity().call().await {
            Ok(liq) => liq._0,
            Err(_) => continue,
        };

        // Skip pools with no liquidity
        if liquidity == 0 {
            continue;
        }

        let quote = match quoter
            .quoteExactInputSingle(
                token_in,
                token_out,
                alloy_primitives::Uint::from(fee),
                amount_in,
            )
            .call()
            .await
        {
            Ok(q) => q.amountOut,
            Err(_) => continue,
        };

        // let current_price: U160 = (sqrt_price_x96 * sqrt_price_x96) >> 192;
        // let current_price = U256::from(current_price);
        // let execution_price: U256 = (quote << 256) / amount_in;
        // let price_impact: f64 = current_price.try_into().unwrap();
        // let price_impact = (current_price * U256::from(100) / execution_price) - U256::from(100);

        println!("Pool with fee {}bps:", fee as f64 / 100.0);
        println!("Address: {}", pool_address);
        println!("Liquidity: {}", liquidity);
        println!("Quote for {}: {}", amount_in, quote);

        // Update best pool if this quote is better
        if quote > best_quote {
            best_quote = quote;
            best_pool = Some(pool_address);
            best_fee = fee;
        }
    }

    match best_pool {
        Some(pool) => Ok((pool, best_fee, best_quote)),
        None => Err(eyre::eyre!("No viable pool found")),
    }
}

/* This function makes extra calls to get not only the best fee tier but also the corresponding pool address
When calling the router/quoter, the pool address is not needed so in that case, the find_best_v3_fee_tier function is preferred
because it uses less calls. */
pub async fn find_best_v3_fee_tier<T, N, P>(
    provider: Arc<P>,
    token_in: Address,
    token_out: Address,
    amount_in: U256,
) -> Result<(u32, U256), Error>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    // Standard fee tiers
    let fee_tiers = vec![100, 500, 3000, 10000];
    let mut best_fee = 0u32;
    let mut best_quote = U256::ZERO;

    // Get quote for this pool
    let quoter = IQuoter::new(
        Address::from_str("0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6").unwrap(),
        provider.clone(),
    );

    for fee in fee_tiers {
        // Get pool address for this fee tier
        let quote = match quoter
            .quoteExactInputSingle(
                token_in,
                token_out,
                alloy_primitives::Uint::from(fee),
                amount_in,
            )
            .call()
            .await
        {
            Ok(q) => q.amountOut,
            Err(_) => continue,
        };

        println!("Pool with fee {}bps:", fee as f64 / 100.0);
        println!("Quote for {}: {}", amount_in, quote);

        // Update best pool if this quote is better
        if quote > best_quote {
            best_quote = quote;
            best_fee = fee;
        }
    }

    if best_fee == 0 {
        return Err(eyre::eyre!("No viable pool found"));
    }

    Ok((best_fee, best_quote))
}

pub async fn swap_v3_pool<T, P>(
    provider: Arc<P>,
    recipient: Address,
    token_in_address: Address,
    token_out_address: Address,
    amount_in: U256,
) -> Result<U256, AMMError>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum>,
{
    // Uniswap V3 Router address
    let router_address = Address::from_str("0xE592427A0AEce92De3Edee1F18E0157C05861564")
        .expect("Invalid router address");

    let router = ISwapRouter::new(router_address, provider.clone());
    let token_in = IERC20::new(token_in_address, provider.clone());

    let (fee, quote) =
        find_best_v3_fee_tier(provider, token_in_address, token_out_address, amount_in).await?;

    // 1. Approve the router to spend our tokens
    let approve_tx = token_in.approve(router_address, amount_in).send().await?;
    let receipt = approve_tx.get_receipt().await.unwrap();
    println!("Approve transaction hash: {:?}", receipt);

    // 2. Calculate minimum amount out (0.5% slippage)
    let amount_out_min = quote * U256::from(995) / U256::from(1000);

    println!("Amount in: {}", amount_in);
    println!("Minimum amount out: {}", amount_out_min);

    // 3. Deadline to 20 minutes
    let deadline = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 1200;

    let params = ISwapRouter::ExactInputSingleParams {
        tokenIn: token_in_address,
        tokenOut: token_out_address,
        fee: alloy_primitives::Uint::from(fee),
        recipient,
        deadline: U256::from(deadline),
        amountIn: amount_in,
        amountOutMinimum: amount_out_min,
        sqrtPriceLimitX96: U160::ZERO, // No price limit
    };

    // 5. Execute the swap
    let swap_tx = router.exactInputSingle(params).send().await?;
    let receipt: TransactionReceipt = swap_tx.get_receipt().await.unwrap();

    println!("Swap transaction hash: {:?}", receipt);

    // Return the actual amount out
    Ok(amount_out_min)
}
