// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {IUniswapV3QuoterV2Lib} from "./interfaces/IUniswapV3QuoterV2.sol";
import {IUniswapV3QuoterV2} from "./interfaces/IUniswapV3QuoterV2.sol";
import {ICurvePoolV2} from "./interfaces/ICurvePoolV2.sol";
import {UniswapV2Library} from "./libraries/UniswapV2Library.sol";
import {SafeMath} from "./libraries/SafeMath.sol";
import {IRouter, IAerodromeRouter} from "./interfaces/IAerodromeRouter.sol";
import "forge-std/console.sol";

// Credits to https://github.com/solidquant/whack-a-mole.
// This is a modified version of the original code to support Aerodrome.

contract TxSimulator {
    using SafeMath for uint256;

    struct SwapParams {
        uint8 protocol; // 0 (UniswapV2), 1 (UniswapV3), 2 (Curve Finance) // 3 (Aerodrome)
        address handler; // UniswapV2: Factory, UniswapV3: Quoter, Curve: Pool, Aerodrome: Router
        address tokenIn;
        address tokenOut;
        uint24 fee; // only used in Uniswap V3
        uint256 amount; // amount in (1 USDC = 1,000,000 / 1 MATIC = 1 * 10 ** 18)
        bool stable; // only used in Aerodrome
        address factory; // only used in Aerodrome
    }

    constructor() {}

    function simulateSwapIn(SwapParams[] calldata paramsArray) public returns (uint256) {
        uint256 amountOut;
        uint256 paramsArrayLength = paramsArray.length;

        for (uint256 i; i < paramsArrayLength;) {
            SwapParams memory params = paramsArray[i];

            if (amountOut != 0) {
                params.amount = amountOut;
            } 
            
            if (params.protocol == 0) {
                amountOut = simulateUniswapV2SwapIn(params);
            } else if (params.protocol == 1) {
                amountOut = simulateUniswapV3SwapIn(params);
            } else if (params.protocol == 2) {
                console.log("simulateCurveSwapIn");
                amountOut = simulateCurveSwapIn(params);
            } else if (params.protocol == 3) {
                amountOut = simulateAeroSwapIn(params);
            }

            unchecked {
                ++i;
            }
        }

        return amountOut;
    }

    function simulateUniswapV2SwapIn(SwapParams memory params) public view returns (uint256 amountOut) {
        (uint256 reserveIn, uint256 reserveOut) =
            UniswapV2Library.getReserves(params.handler, params.tokenIn, params.tokenOut);
        amountOut = UniswapV2Library.getAmountOut(params.amount, reserveIn, reserveOut);
    }

    function simulateUniswapV3SwapIn(SwapParams memory params) public returns (uint256 amountOut) {
        IUniswapV3QuoterV2 quoter = IUniswapV3QuoterV2(params.handler);
        IUniswapV3QuoterV2Lib.QuoteExactInputSingleParams memory quoterParams;
        quoterParams.tokenIn = params.tokenIn;
        quoterParams.tokenOut = params.tokenOut;
        quoterParams.amountIn = params.amount;
        quoterParams.fee = params.fee;
        quoterParams.sqrtPriceLimitX96 = 0;
        (amountOut,,,) = quoter.quoteExactInputSingle(quoterParams);
    }

    // TODO: Replace the pool with the curve router.
    // NOTE: This uses the curve pool v2 interface (the one deployed on base)
    function simulateCurveSwapIn(SwapParams memory params) public returns (uint256 amountOut) {
        ICurvePoolV2 pool = ICurvePoolV2(params.handler);

        uint256 i;
        uint256 j;

        uint256 coinIdx = 0;

        while (i == j) {
            address coin = pool.coins(coinIdx);

            if (coin == params.tokenIn) {
                i = coinIdx;
            } else if (coin == params.tokenOut) {
                j = coinIdx;
            }

            if (i != j) {
                break;
            }

            unchecked {
                ++coinIdx;
            }
        }

        amountOut = pool.get_dy(i, j, params.amount);
    }

    function simulateAeroSwapIn(SwapParams memory params) public returns (uint256 amountOut) {
        IAerodromeRouter router = IAerodromeRouter(payable(params.handler));
        IRouter.Route[] memory route = new IRouter.Route[](1);

        route[0].from = params.tokenIn;
        route[0].to = params.tokenOut;
        route[0].stable = params.stable;
        route[0].factory = params.factory;

        amountOut = router.getAmountsOut(params.amount, route)[1];
    }
}
