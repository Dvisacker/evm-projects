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
import "../lib/v3-core/contracts/interfaces/IUniswapV3Pool.sol";
import "../lib/aave-v3-core/contracts/interfaces/IPool.sol";
import "../lib/morpho-blue/src/interfaces/IMorpho.sol";

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
    IAerodromeRouter public aerodromeRouter = IAerodromeRouter(0xcF77a3Ba9A5CA399B7c97c74d54e5b1Beb874E43);
    address public aerodromeFactoryAddress = 0x420DD381b31aEf6683db6B902084cB0FFECe40Da;
    // V3 factory address
    address factory = 0x33128a8fC17869897dcE68Ed026d694621f6FDfD;
    // V3 init code hash
    bytes32 initCodeHash = 0xe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54;

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
        // Convert uint32 to bytes32, right-aligned
        bytes32 dataIndexBytes32 = bytes32(uint256(context.dataIndex));

        // Convert address to bytes32, right-aligned
        bytes32 senderBytes32 = bytes32(uint256(uint160(context.sender)));

        bytes32 shiftedDataIndexBytes32 = dataIndexBytes32 << 160;
        // Shift dataIndex left by 160 bits (20 bytes) and combine with sender
        bytes32 result = (shiftedDataIndexBytes32 & 0xffffffffffffffffffffffff0000000000000000000000000000000000000000)
            | (senderBytes32 & 0x000000000000000000000000ffffffffffffffffffffffffffffffffffffffff);

        return result;
    }

    function getCreate2Address(address from, bytes32 salt, bytes32 initCodeHash) public pure returns (address addr) {
        /// @notice Compute CREATE2 address
        /// @dev Follows the same logic as the typescript version but in Solidity
        /// @param from The address deploying the contract
        /// @param salt The salt used in the CREATE2 deployment
        /// @param initCodeHash The hash of the contract's init code

        // Pack and hash according to CREATE2 specification
        bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0xff), // Fixed prefix
                from, // Creator address
                salt, // Randomizing salt
                initCodeHash // Init code hash
            )
        );

        // Convert last 20 bytes of hash to address
        addr = address(uint160(uint256(hash)));
    }

    function computeV3PoolAddress(bytes32 initCodeHash, address factory, address tokenA, address tokenB, uint24 fee)
        public
        pure
        returns (address)
    {
        // Sort tokens
        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);

        // Compute the salt from packed tokens and fee
        bytes32 salt = keccak256(abi.encode(token0, token1, fee));

        return getCreate2Address(factory, salt, initCodeHash);
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

    function buildErc20Approve(address asset, address recipient, uint256 amount) public returns (bytes memory) {
        return this.buildCall(
            asset,
            0,
            abi.encodeWithSelector(ERC20.approve.selector, recipient, amount),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
    }

    function buildErc20Transfer(address asset, address recipient, uint256 amount) public returns (bytes memory) {
        return this.buildCall(
            asset,
            0,
            abi.encodeWithSelector(ERC20.transfer.selector, recipient, amount),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
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

    function testUniswapV2SwapExact() public {
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
            abi.encodeWithSelector(weth.approve.selector, uniswapV2Router, amountIn),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );

        address[] memory path = new address[](2);
        path[0] = address(weth);
        path[1] = address(usdc);

        uint256 deadline = block.timestamp + 1000;

        bytes memory swapCallData = buildCall(
            address(uniswapV2Router),
            0,
            abi.encodeWithSelector(
                uniswapV2Router.swapExactTokensForTokens.selector, amountIn, 0, path, address(executor), deadline
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

    function testAerodromeSwapExact() public {
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
            abi.encodeWithSelector(weth.approve.selector, aerodromeRouter, amountIn),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );

        IAerodromeRouter.Route[] memory routes = new IAerodromeRouter.Route[](1);
        routes[0] = IAerodromeRouter.Route({
            from: address(weth),
            to: address(usdc),
            stable: false,
            factory: address(aerodromeFactoryAddress)
        });

        uint256 deadline = block.timestamp + 1000;

        bytes memory swapCallData = buildCall(
            address(aerodromeRouter),
            0,
            abi.encodeWithSelector(
                aerodromeRouter.swapExactTokensForTokens.selector, amountIn, 0, routes, address(executor), deadline
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

    function testUniswapV3FlashLoan() public {
        address pool_address = computeV3PoolAddress(initCodeHash, factory, address(weth), address(usdc), 3000);

        uint256 amountUsdc = 3000 * 10 ** 6;
        uint256 amountWeth = 1 ether;
        IUniswapV3Pool pool = IUniswapV3Pool(pool_address);

        deal(address(usdc), address(executor), 100000 * 1e6);
        deal(address(weth), address(executor), 100 ether);

        uint256 amountWethWithPremium = amountWeth + (300 * amountWeth / 100000);
        uint256 amountUsdcWithPremium = amountUsdc + (300 * amountUsdc / 100000);

        bytes memory transfer0Calldata = buildCall(
            address(weth),
            0,
            abi.encodeWithSelector(weth.transfer.selector, address(pool), amountWethWithPremium),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
        bytes memory transfer1Calldata = buildCall(
            address(usdc),
            0,
            abi.encodeWithSelector(usdc.transfer.selector, address(pool), amountUsdcWithPremium),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );

        bytes[] memory callbackCalls = new bytes[](2);
        callbackCalls[0] = transfer0Calldata;
        callbackCalls[1] = transfer1Calldata;
        BatchExecutor.FallbackData memory fallbackData =
            BatchExecutor.FallbackData({multicallData: callbackCalls, returnData: abi.encode(true)});
        bytes memory params = abi.encode(fallbackData);

        CallbackContext memory context = CallbackContext({dataIndex: 2, sender: address(pool)});

        bytes memory flashCallData = buildCall(
            address(pool),
            0,
            abi.encodeWithSelector(pool.flash.selector, address(executor), amountWeth, amountUsdc, params),
            context,
            new BatchExecutor.DynamicCall[](0)
        );

        bytes[] memory callDataArray = new bytes[](1);
        callDataArray[0] = flashCallData;

        vm.startPrank(deployer);
        executor.batchCall(callDataArray);
        vm.stopPrank();
    }

    function testMorphoFlashloan() public {
        deal(address(usdc), address(deployer), 100000 ether);
        deal(address(weth), address(deployer), 100 ether);
        IMorpho morpho = IMorpho(0xBBBBBbbBBb9cC5e90e3b3Af64bdAF62C37EEFFCb);

        address asset = address(usdc);
        uint256 amount = 1000 * 1e6;

        bytes[] memory callbackCalls = new bytes[](1);
        callbackCalls[0] = this.buildErc20Approve(address(usdc), address(morpho), amount);
        BatchExecutor.FallbackData memory fallbackData =
            BatchExecutor.FallbackData({multicallData: callbackCalls, returnData: abi.encode(true)});

        CallbackContext memory context = CallbackContext({dataIndex: 1, sender: address(morpho)});
        bytes memory params = abi.encode(fallbackData);

        bytes memory flashCallData = buildCall(
            address(morpho),
            0,
            abi.encodeCall(morpho.flashLoan, (asset, amount, params)),
            context,
            new BatchExecutor.DynamicCall[](0)
        );

        bytes[] memory callDataArray = new bytes[](1);
        callDataArray[0] = flashCallData;

        vm.startPrank(deployer);
        executor.batchCall(callDataArray);
        vm.stopPrank();
    }

    function testAaveV3Flashloan() public {
        deal(address(usdc), address(deployer), 100000 ether);
        deal(address(weth), address(deployer), 100 ether);
        deal(address(usdc), address(executor), 100000 ether);
        deal(address(weth), address(executor), 100 ether);
        uint256 amountUsdc = 3000 * 10 ** 6;
        uint256 amountWeth = 10 ether;
        IPool pool = IPool(0xA238Dd80C259a72e81d7e4664a9801593F98d1c5);

        uint256 aaveV3Premium = pool.FLASHLOAN_PREMIUM_TOTAL();
        uint256 amountUsdcWithPremium = amountUsdc + (aaveV3Premium * amountUsdc / 10000);
        uint256 amountWethWithPremium = amountWeth + (aaveV3Premium * amountWeth / 10000);

        bytes[] memory callbackCalls = new bytes[](2);
        callbackCalls[0] = this.buildErc20Approve(address(usdc), address(pool), amountUsdcWithPremium);
        callbackCalls[1] = this.buildErc20Approve(address(weth), address(pool), amountWethWithPremium);
        BatchExecutor.FallbackData memory params =
            BatchExecutor.FallbackData({multicallData: callbackCalls, returnData: abi.encode(true)});

        CallbackContext memory context = CallbackContext({dataIndex: 4, sender: address(pool)});

        address[] memory assets;
        assets = new address[](2);
        assets[0] = address(usdc);
        assets[1] = address(weth);
        uint256[] memory amounts;
        amounts = new uint256[](2);
        amounts[0] = amountUsdc;
        amounts[1] = amountWeth;
        uint256[] memory modes;
        modes = new uint256[](2);
        modes[0] = 0;
        modes[1] = 0;

        bytes memory encoded = abi.encodeCall(
            pool.flashLoan, (address(executor), assets, amounts, modes, address(executor), abi.encode(params), 0)
        );

        bytes memory flashCallData = buildCall(address(pool), 0, encoded, context, new BatchExecutor.DynamicCall[](0));

        bytes[] memory callDataArray = new bytes[](1);
        callDataArray[0] = flashCallData;
        vm.startPrank(deployer);
        executor.batchCall(callDataArray);
        vm.stopPrank();
    }
}
