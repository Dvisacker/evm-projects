// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import "../lib/openzeppelin-contracts/contracts/utils/Address.sol";
import "forge-std/console.sol";

interface IUniswapV2Router02 {
    function factory() external pure returns (address);
    function WETH() external pure returns (address);
    function quote(uint256 amountA, uint256 reserveA, uint256 reserveB) external pure returns (uint256 amountB);
    function getAmountOut(uint256 amountIn, uint256 reserveIn, uint256 reserveOut)
        external
        pure
        returns (uint256 amountOut);
    function getAmountIn(uint256 amountOut, uint256 reserveIn, uint256 reserveOut)
        external
        pure
        returns (uint256 amountIn);
    function getAmountsOut(uint256 amountIn, address[] calldata path)
        external
        view
        returns (uint256[] memory amounts);
    function getAmountsIn(uint256 amountOut, address[] calldata path)
        external
        view
        returns (uint256[] memory amounts);

    // Swap Functions
    function swapExactTokensForTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);

    function swapTokensForExactTokens(
        uint256 amountOut,
        uint256 amountInMax,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);

    function swapExactETHForTokens(uint256 amountOutMin, address[] calldata path, address to, uint256 deadline)
        external
        payable
        returns (uint256[] memory amounts);

    function swapTokensForExactETH(
        uint256 amountOut,
        uint256 amountInMax,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);

    function swapExactTokensForETH(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);

    function swapETHForExactTokens(uint256 amountOut, address[] calldata path, address to, uint256 deadline)
        external
        payable
        returns (uint256[] memory amounts);
}

interface ISwapRouter {
    struct ExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        uint24 fee;
        address recipient;
        uint256 amountIn;
        uint256 amountOutMinimum;
        uint160 sqrtPriceLimitX96;
    }

    struct ExactOutputSingleParams {
        address tokenIn;
        address tokenOut;
        uint24 fee;
        address recipient;
        uint256 deadline;
        uint256 amountOut;
        uint256 amountInMaximum;
        uint160 sqrtPriceLimitX96;
    }

    function exactInputSingle(ExactInputSingleParams calldata params) external payable returns (uint256 amountOut);
    function exactOutputSingle(ExactOutputSingleParams calldata params) external payable returns (uint256 amountIn);
}

uint256 constant FALLBACK_CONTEXT_TLOC = 0;

contract Executor {
    address internal immutable OWNER;
    address internal immutable UNISWAP_V3_ROUTER;
    address internal immutable UNISWAP_V2_ROUTER;

    constructor(address _owner, address _uniswapV3Router, address _uniswapV2Router) {
        OWNER = _owner;
        UNISWAP_V3_ROUTER = _uniswapV3Router;
        UNISWAP_V2_ROUTER = _uniswapV2Router;
    }

    address public constant AGGREGATION_ROUTER_V5 = 0x1111111254EEB25477B68fb85Ed929f73A960582;

    uint24[] public FEE_TIERS = [100, 500, 3000, 10000];

    struct Swap {
        uint8 swapType; // 0 = v2, 1 = v3
        address tokenOut;
        uint24 feeTier;
    }

    struct SwapData {
        address tokenIn;
        uint256 amountIn;
        Swap[] swaps;
    }

    function swap(bytes calldata swapData) public returns (uint256) {
        SwapData memory data = abi.decode(swapData, (SwapData));

        ERC20(data.tokenIn).transferFrom(msg.sender, address(this), data.amountIn);

        uint256 nextAmount = data.amountIn;
        address nextToken = data.tokenIn;
        uint256 amountOut;
        for (uint256 i; i < data.swaps.length; i++) {
            if (data.swaps[i].swapType == 1) {
                amountOut = swapUniswapV3(nextAmount, nextToken, data.swaps[i].tokenOut, data.swaps[i].feeTier);
            } else if (data.swaps[i].swapType == 0) {
                amountOut = swapUniswapV2(nextAmount, nextToken, data.swaps[i].tokenOut);
            }

            nextAmount = amountOut;
            nextToken = data.swaps[i].tokenOut;
            console.log("output amount: ", nextAmount, " of", nextToken);
        }

        ERC20(nextToken).transfer(msg.sender, nextAmount);
        return nextAmount;
    }

    function swapUniswapV3(uint256 amountIn, address tokenIn, address tokenOut, uint24 feeTier)
        public
        returns (uint256)
    {
        ERC20(tokenIn).approve(UNISWAP_V3_ROUTER, amountIn);
        ISwapRouter.ExactInputSingleParams memory params = ISwapRouter.ExactInputSingleParams({
            tokenIn: tokenIn,
            tokenOut: tokenOut,
            fee: feeTier,
            recipient: address(this),
            amountIn: amountIn,
            amountOutMinimum: 0,
            sqrtPriceLimitX96: 0
        });

        uint256 amountOut = ISwapRouter(UNISWAP_V3_ROUTER).exactInputSingle(params);
        return amountOut;
    }

    function swapUniswapV2(uint256 amountIn, address tokenIn, address tokenOut) public returns (uint256) {
        if (amountIn == 0) {
            amountIn = ERC20(tokenIn).balanceOf(address(this));
        }
        address[] memory path = new address[](2);
        path[0] = tokenIn;
        path[1] = tokenOut;
        ERC20(tokenIn).approve(UNISWAP_V2_ROUTER, amountIn);
        uint256[] memory amounts = IUniswapV2Router02(UNISWAP_V2_ROUTER).swapExactTokensForTokens(
            amountIn, 1, path, address(this), block.timestamp + 300
        );

        console.log("amounts 0", amounts[0]);
        console.log("amounts 1", amounts[1]);
        return amounts[amounts.length - 1];
    }

    // function swap(bytes[] memory data) external payable {
    //     require(msg.sender == OWNER);

    //     for (uint256 i; i < data.length; ++i) {
    //         (bool success, bytes memory returnData) = address(this).call(data[i]);
    //         if (!success) _revert(returnData);
    //     }

    //     // (bool success, bytes memory returnData) = address(this).call(data);
    //     // if (!success) _revert(returnData);
    // }

    /// @notice Executes a batch of calls.
    function multicall(bytes[] memory data) external payable {
        require(msg.sender == OWNER);

        _multicall(data);
    }

    /// @notice Executes a series of calls.
    function _multicall(bytes[] memory data) internal {
        for (uint256 i; i < data.length; ++i) {
            (bool success, bytes memory returnData) = address(this).call(data[i]);
            if (!success) _revert(returnData);
        }
    }

    /// @dev Bubbles up the revert reason / custom error encoded in `returnData`.
    /// @dev Assumes `returnData` is the return data of any kind of failing CALL to a contract.
    function _revert(bytes memory returnData) internal pure {
        uint256 length = returnData.length;
        require(length > 0);

        assembly ("memory-safe") {
            revert(add(32, returnData), length)
        }
    }
}
