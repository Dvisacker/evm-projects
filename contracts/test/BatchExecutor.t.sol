// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {WETH} from "../lib/solmate/src/tokens/WETH.sol";
import "../src/BatchExecutor.sol";
import "forge-std/console.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

contract BatchExecutorTest is Test {
    address deployer = makeAddr("deployer");

    WETH weth = WETH(payable(0x4200000000000000000000000000000000000006));
    ERC20 dai = ERC20(0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb);
    ERC20 usdc = ERC20(0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913);
    ERC20 usdt = ERC20(0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2);

    BatchExecutor public executor;
    address owner = address(this);
    address uniswapV3Router = address(0x2626664c2603336E57B271c5C0b26F421741e481);
    address uniswapV2Router = address(0x4752ba5DBc23f44D87826276BF6Fd6b1C372aD24);
    address aerodromeRouter = address(0xcF77a3Ba9A5CA399B7c97c74d54e5b1Beb874E43);

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

    function buildCall(address target, uint256 value, bytes memory callData, CallbackContext memory context)
        public
        returns (bytes memory)
    {
        bytes32 contextBytes = encodedContext(context);
        return abi.encodeWithSelector(executor.singlecall.selector, target, value, contextBytes, callData);
    }

    function testWrapEth() public {
        uint256 amountIn = 1 ether;
        bytes memory depositWethCallData = abi.encodeWithSelector(weth.deposit.selector);
        bytes memory callData = buildCall(
            address(weth), amountIn, depositWethCallData, CallbackContext({dataIndex: 0, sender: address(this)})
        );
        bytes[] memory callDataArray = new bytes[](1);
        callDataArray[0] = callData;

        vm.startPrank(deployer);
        executor.batchCall{value: amountIn}(callDataArray);
        vm.stopPrank();
    }
}
