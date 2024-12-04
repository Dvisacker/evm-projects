pragma solidity ^0.8.4;

library IRouter {
    struct Route {
        address from;
        address to;
        bool stable;
        address factory;
    }

    struct Zap {
        address tokenA;
        address tokenB;
        bool stable;
        address factory;
        uint256 amountOutMinA;
        uint256 amountOutMinB;
        uint256 amountAMin;
        uint256 amountBMin;
    }
}

interface IAerodromeRouter {
    error ETHTransferFailed();
    error Expired();
    error InsufficientAmount();
    error InsufficientAmountA();
    error InsufficientAmountADesired();
    error InsufficientAmountAOptimal();
    error InsufficientAmountB();
    error InsufficientAmountBDesired();
    error InsufficientLiquidity();
    error InsufficientOutputAmount();
    error InvalidAmountInForETHDeposit();
    error InvalidPath();
    error InvalidRouteA();
    error InvalidRouteB();
    error InvalidTokenInForETHDeposit();
    error OnlyWETH();
    error PoolDoesNotExist();
    error PoolFactoryDoesNotExist();
    error SameAddresses();
    error ZeroAddress();

    receive() external payable;

    function ETHER() external view returns (address);
    function UNSAFE_swapExactTokensForTokens(
        uint256[] memory amounts,
        IRouter.Route[] memory routes,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory);
    function addLiquidity(
        address tokenA,
        address tokenB,
        bool stable,
        uint256 amountADesired,
        uint256 amountBDesired,
        uint256 amountAMin,
        uint256 amountBMin,
        address to,
        uint256 deadline
    ) external returns (uint256 amountA, uint256 amountB, uint256 liquidity);
    function addLiquidityETH(
        address token,
        bool stable,
        uint256 amountTokenDesired,
        uint256 amountTokenMin,
        uint256 amountETHMin,
        address to,
        uint256 deadline
    ) external payable returns (uint256 amountToken, uint256 amountETH, uint256 liquidity);
    function defaultFactory() external view returns (address);
    function factoryRegistry() external view returns (address);
    function generateZapInParams(
        address tokenA,
        address tokenB,
        bool stable,
        address _factory,
        uint256 amountInA,
        uint256 amountInB,
        IRouter.Route[] memory routesA,
        IRouter.Route[] memory routesB
    ) external view returns (uint256 amountOutMinA, uint256 amountOutMinB, uint256 amountAMin, uint256 amountBMin);
    function generateZapOutParams(
        address tokenA,
        address tokenB,
        bool stable,
        address _factory,
        uint256 liquidity,
        IRouter.Route[] memory routesA,
        IRouter.Route[] memory routesB
    ) external view returns (uint256 amountOutMinA, uint256 amountOutMinB, uint256 amountAMin, uint256 amountBMin);
    function getAmountsOut(uint256 amountIn, IRouter.Route[] memory routes)
        external
        view
        returns (uint256[] memory amounts);
    function getReserves(address tokenA, address tokenB, bool stable, address _factory)
        external
        view
        returns (uint256 reserveA, uint256 reserveB);
    function isTrustedForwarder(address forwarder) external view returns (bool);
    function poolFor(address tokenA, address tokenB, bool stable, address _factory)
        external
        view
        returns (address pool);
    function quoteAddLiquidity(
        address tokenA,
        address tokenB,
        bool stable,
        address _factory,
        uint256 amountADesired,
        uint256 amountBDesired
    ) external view returns (uint256 amountA, uint256 amountB, uint256 liquidity);
    function quoteRemoveLiquidity(address tokenA, address tokenB, bool stable, address _factory, uint256 liquidity)
        external
        view
        returns (uint256 amountA, uint256 amountB);
    function quoteStableLiquidityRatio(address tokenA, address tokenB, address _factory)
        external
        view
        returns (uint256 ratio);
    function removeLiquidity(
        address tokenA,
        address tokenB,
        bool stable,
        uint256 liquidity,
        uint256 amountAMin,
        uint256 amountBMin,
        address to,
        uint256 deadline
    ) external returns (uint256 amountA, uint256 amountB);
    function removeLiquidityETH(
        address token,
        bool stable,
        uint256 liquidity,
        uint256 amountTokenMin,
        uint256 amountETHMin,
        address to,
        uint256 deadline
    ) external returns (uint256 amountToken, uint256 amountETH);
    function removeLiquidityETHSupportingFeeOnTransferTokens(
        address token,
        bool stable,
        uint256 liquidity,
        uint256 amountTokenMin,
        uint256 amountETHMin,
        address to,
        uint256 deadline
    ) external returns (uint256 amountETH);
    function sortTokens(address tokenA, address tokenB) external pure returns (address token0, address token1);
    function swapExactETHForTokens(uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline)
        external
        payable
        returns (uint256[] memory amounts);
    function swapExactETHForTokensSupportingFeeOnTransferTokens(
        uint256 amountOutMin,
        IRouter.Route[] memory routes,
        address to,
        uint256 deadline
    ) external payable;
    function swapExactTokensForETH(
        uint256 amountIn,
        uint256 amountOutMin,
        IRouter.Route[] memory routes,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);
    function swapExactTokensForETHSupportingFeeOnTransferTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        IRouter.Route[] memory routes,
        address to,
        uint256 deadline
    ) external;
    function swapExactTokensForTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        IRouter.Route[] memory routes,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);
    function swapExactTokensForTokensSupportingFeeOnTransferTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        IRouter.Route[] memory routes,
        address to,
        uint256 deadline
    ) external;
    function voter() external view returns (address);
    function weth() external view returns (address);
    function zapIn(
        address tokenIn,
        uint256 amountInA,
        uint256 amountInB,
        IRouter.Zap memory zapInPool,
        IRouter.Route[] memory routesA,
        IRouter.Route[] memory routesB,
        address to,
        bool stake
    ) external payable returns (uint256 liquidity);
    function zapOut(
        address tokenOut,
        uint256 liquidity,
        IRouter.Zap memory zapOutPool,
        IRouter.Route[] memory routesA,
        IRouter.Route[] memory routesB
    ) external;
}
