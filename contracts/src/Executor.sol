// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import "../lib/openzeppelin-contracts/contracts/utils/Address.sol";
import "forge-std/console.sol";
import {IAerodromeRouter} from "./interfaces/IAerodromeRouter.sol";
import {IUniswapV3Router} from "./interfaces/IUniswapV3Router.sol";
import {IUniswapV2Router} from "./interfaces/IUniswapV2Router.sol";

uint256 constant FALLBACK_CONTEXT_TLOC = 0;

contract Executor {
    address internal immutable OWNER;
    address internal immutable UNISWAP_V3_ROUTER;
    address internal immutable UNISWAP_V2_ROUTER;
    address internal immutable AERODROME_ROUTER;

    constructor(address _owner, address _uniswapV3Router, address _uniswapV2Router, address _aerodromeRouter) {
        OWNER = _owner;
        UNISWAP_V3_ROUTER = _uniswapV3Router;
        UNISWAP_V2_ROUTER = _uniswapV2Router;
        AERODROME_ROUTER = _aerodromeRouter;
    }

    address public constant AGGREGATION_ROUTER_V5 = 0x1111111254EEB25477B68fb85Ed929f73A960582;

    uint24[] public FEE_TIERS = [100, 500, 3000, 10000];

    struct Swap {
        uint8 swapType; // 0 = v2, 1 = v3, 2 = aerodrome
        address tokenOut;
        uint24 feeTier;
        bool stable; // only used for aerodrome
    }

    struct SwapData {
        address tokenIn;
        uint256 amountIn;
        Swap[] swaps;
    }

    struct Call {
        address target;
        uint256 value;
        bytes data;
    }

    function swap(SwapData calldata data) public returns (uint256) {
        ERC20(data.tokenIn).transferFrom(msg.sender, address(this), data.amountIn);

        uint256 nextAmount = data.amountIn;
        address nextToken = data.tokenIn;
        uint256 amountOut;
        for (uint256 i; i < data.swaps.length; i++) {
            if (data.swaps[i].swapType == 1) {
                amountOut = swapUniswapV3(nextAmount, nextToken, data.swaps[i].tokenOut, data.swaps[i].feeTier);
            } else if (data.swaps[i].swapType == 0) {
                amountOut = swapUniswapV2(nextAmount, nextToken, data.swaps[i].tokenOut);
            } else if (data.swaps[i].swapType == 2) {
                amountOut = swapAerodrome(nextAmount, nextToken, data.swaps[i].tokenOut, data.swaps[i].stable);
            }

            nextAmount = amountOut;
            nextToken = data.swaps[i].tokenOut;
            console.log("output amount: ", nextAmount, " of", nextToken);
        }

        ERC20(nextToken).transfer(msg.sender, nextAmount);
        return nextAmount;
    }

    function swapAll(SwapData calldata swapData) public returns (uint256) {
        ERC20(swapData.tokenIn).transferFrom(msg.sender, address(this), swapData.amountIn);

        uint256 nextAmount = swapData.amountIn;
        address nextToken = swapData.tokenIn;
        for (uint256 i; i < swapData.swaps.length; i++) {
            if (swapData.swaps[i].swapType == 1) {
                swapUniswapV3(nextAmount, nextToken, swapData.swaps[i].tokenOut, swapData.swaps[i].feeTier);
            } else if (swapData.swaps[i].swapType == 0) {
                swapUniswapV2(nextAmount, nextToken, swapData.swaps[i].tokenOut);
            } else if (swapData.swaps[i].swapType == 2) {
                swapAerodrome(nextAmount, nextToken, swapData.swaps[i].tokenOut, swapData.swaps[i].stable);
            }

            nextAmount = ERC20(nextToken).balanceOf(address(this));
            nextToken = swapData.swaps[i].tokenOut;
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
        IUniswapV3Router.ExactInputSingleParams memory params = IUniswapV3Router.ExactInputSingleParams({
            tokenIn: tokenIn,
            tokenOut: tokenOut,
            fee: feeTier,
            recipient: address(this),
            amountIn: amountIn,
            amountOutMinimum: 0,
            sqrtPriceLimitX96: 0
        });

        uint256 amountOut = IUniswapV3Router(UNISWAP_V3_ROUTER).exactInputSingle(params);
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
        uint256[] memory amounts = IUniswapV2Router(UNISWAP_V2_ROUTER).swapExactTokensForTokens(
            amountIn, 1, path, address(this), block.timestamp + 300
        );
        return amounts[amounts.length - 1];
    }

    function swapAerodrome(uint256 amountIn, address tokenIn, address tokenOut, bool stable) public returns (uint256) {
        ERC20(tokenIn).approve(AERODROME_ROUTER, amountIn);
        IAerodromeRouter.Route[] memory routes = new IAerodromeRouter.Route[](1);
        routes[0] = IAerodromeRouter.Route({from: tokenIn, to: tokenOut, stable: stable, factory: address(0)});
        uint256[] memory amountsOut = IAerodromeRouter(AERODROME_ROUTER).swapExactTokensForTokens(
            amountIn, 0, routes, address(this), block.timestamp + 300
        );
        return amountsOut[amountsOut.length - 1];
    }

    function multicall(Call[] memory calls) external payable {
        require(msg.sender == OWNER);

        for (uint256 i; i < calls.length; ++i) {
            (bool success, bytes memory returnData) = calls[i].target.call{value: calls[i].value}(calls[i].data);
            if (!success) _revert(returnData);
        }
    }

    // /// @notice Executes a batch of calls.
    // function multicall(bytes[] memory data) external payable {
    //     require(msg.sender == OWNER);

    //     _multicall(data);
    // }

    // /// @notice Executes a series of calls.
    // function _multicall(bytes[] memory data) internal {
    //     for (uint256 i; i < data.length; ++i) {
    //         (bool success, bytes memory returnData) = address(this).call(data[i]);
    //         if (!success) _revert(returnData);
    //     }
    // }

    receive() external payable {}

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
