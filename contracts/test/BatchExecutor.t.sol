// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {WETH} from "../lib/solmate/src/tokens/WETH.sol";
import "../src/BatchExecutor.sol";
import "forge-std/console.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "../src/interfaces/IUniswapV3Router.sol";
import "../src/interfaces/IUniswapV2Router.sol";
import "../src/interfaces/IAerodromeRouter.sol";

contract BatchExecutorTest is Test {
    address deployer = makeAddr("deployer");

    WETH weth = WETH(payable(0x4200000000000000000000000000000000000006));
    ERC20 dai = ERC20(0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb);
    ERC20 usdc = ERC20(0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913);
    ERC20 usdt = ERC20(0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2);

    BatchExecutor public executor;
    address owner = address(this);
    IUniswapV3Router public uniswapV3Router = IUniswapV3Router(0x2626664c2603336E57B271c5C0b26F421741e481);
    IUniswapV2Router public uniswapV2Router = IUniswapV2Router(0x4752ba5DBc23f44D87826276BF6Fd6b1C372aD24);
    IAerodromeRouter aerodromeRouter = IAerodromeRouter(0xcF77a3Ba9A5CA399B7c97c74d54e5b1Beb874E43);

    function setUp() public {
        vm.createSelectFork((vm.envString("BASE_RPC_URL")));
        executor = new BatchExecutor(deployer);
        vm.deal(deployer, 10 ether);
    }

    struct CallbackContext {
        uint32 dataIndex;
        address sender;
    }

    function encodedContext(CallbackContext memory context) public pure returns (bytes32) {
        // Convert dataIndex to bytes and left pad to 12 bytes
        bytes memory paddedDataIndex = new bytes(12);

        // Convert uint32 to big-endian bytes
        bytes4 dataIndexBytes = bytes4(uint32(context.dataIndex));

        // Copy the 4 bytes to the end of the 12 byte array (left padding with zeros)
        assembly {
            mstore(add(paddedDataIndex, 12), dataIndexBytes)
        }

        // Concatenate with the address (20 bytes)
        bytes memory result = bytes.concat(paddedDataIndex, abi.encodePacked(context.sender));

        // Convert to bytes32
        bytes32 finalResult;
        assembly {
            finalResult := mload(add(result, 32))
        }

        return finalResult;
    }

    function buildCall(
        address target,
        uint256 value,
        bytes memory callData,
        CallbackContext memory context,
        BatchExecutor.DynamicCall[] memory dynamicCalls
    ) public returns (bytes memory) {
        bytes32 contextBytes = encodedContext(context);
        return abi.encodeWithSelector(executor.singlecall.selector, target, value, contextBytes, callData, dynamicCalls);
    }

    function testWrapEth() public {
        uint256 amountIn = 1 ether;
        bytes memory depositWethCallData = abi.encodeWithSelector(weth.deposit.selector);
        bytes memory callData = buildCall(
            address(weth),
            amountIn,
            depositWethCallData,
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
        bytes[] memory callDataArray = new bytes[](1);
        callDataArray[0] = callData;

        vm.startPrank(deployer);
        executor.batchCall{value: amountIn}(callDataArray);
        vm.stopPrank();
    }

    function testWrapUnwrapEth() public {
        uint256 amountIn = 1 ether;
        bytes memory depositWethCallData = buildCall(
            address(weth),
            amountIn,
            abi.encodeWithSelector(weth.deposit.selector),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
        bytes memory withdrawWethCallData = buildCall(
            address(weth),
            0,
            abi.encodeWithSelector(weth.withdraw.selector, amountIn),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
        bytes[] memory callDataArray = new bytes[](2);
        callDataArray[0] = depositWethCallData;
        callDataArray[1] = withdrawWethCallData;

        vm.startPrank(deployer);
        executor.batchCall{value: amountIn}(callDataArray);
        vm.stopPrank();
    }

    function testUniswapV3SwapExact() public {
        uint256 amountIn = 1 ether;
        bytes memory depositWethCallData = buildCall(
            address(weth),
            amountIn,
            abi.encodeWithSelector(weth.deposit.selector),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
        bytes memory approveErc20CallData = buildCall(
            address(weth),
            0,
            abi.encodeWithSelector(weth.approve.selector, uniswapV3Router, amountIn),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );

        bytes memory swapCallData = buildCall(
            address(uniswapV3Router),
            0,
            abi.encodeWithSelector(
                uniswapV3Router.exactInputSingle.selector,
                IUniswapV3Router.ExactInputSingleParams({
                    tokenIn: address(weth),
                    tokenOut: address(usdc),
                    fee: 3000,
                    recipient: address(executor),
                    amountIn: amountIn,
                    amountOutMinimum: 0,
                    sqrtPriceLimitX96: 0
                })
            ),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );

        bytes[] memory callDataArray = new bytes[](3);
        callDataArray[0] = depositWethCallData;
        callDataArray[1] = approveErc20CallData;
        callDataArray[2] = swapCallData;

        vm.startPrank(deployer);
        executor.batchCall{value: amountIn}(callDataArray);
        vm.stopPrank();
    }

    function testUniswapV3SwapAll() public {
        uint256 amountIn = 1 ether;

        bytes memory depositWethCallData = buildCall(
            address(weth),
            amountIn,
            abi.encodeWithSelector(weth.deposit.selector),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
        bytes memory approveErc20CallData = buildCall(
            address(weth),
            0,
            abi.encodeWithSelector(weth.approve.selector, uniswapV3Router, amountIn),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );

        BatchExecutor.DynamicCall memory balanceOf = BatchExecutor.DynamicCall({
            to: address(weth),
            data: abi.encodeWithSelector(weth.balanceOf.selector, address(executor)),
            offset: 4 + 32 * 4,
            length: 32,
            resOffset: 0
        });

        BatchExecutor.DynamicCall[] memory dynamicCalls = new BatchExecutor.DynamicCall[](1);
        dynamicCalls[0] = balanceOf;

        bytes memory swapCallData = buildCall(
            address(uniswapV3Router),
            0,
            abi.encodeWithSelector(
                uniswapV3Router.exactInputSingle.selector,
                IUniswapV3Router.ExactInputSingleParams({
                    tokenIn: address(weth),
                    tokenOut: address(usdc),
                    fee: 3000,
                    recipient: address(executor),
                    amountIn: 0, // replaced by dynamic call
                    amountOutMinimum: 0,
                    sqrtPriceLimitX96: 0
                })
            ),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            dynamicCalls
        );

        bytes[] memory callDataArray = new bytes[](3);
        callDataArray[0] = depositWethCallData;
        callDataArray[1] = approveErc20CallData;
        callDataArray[2] = swapCallData;

        vm.startPrank(deployer);
        executor.batchCall{value: amountIn}(callDataArray);
        vm.stopPrank();
    }
}
