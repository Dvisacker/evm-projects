// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {WETH} from "../lib/solmate/src/tokens/WETH.sol";
import "../src/AtomicExecutor.sol";
import "forge-std/console.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) {}

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract ExecutorTest is Test {
    address deployer = makeAddr("deployer");
    address swapper = makeAddr("swapper");

    WETH weth = WETH(payable(0x4200000000000000000000000000000000000006));
    ERC20 dai = ERC20(0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb);
    ERC20 usdc = ERC20(0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913);
    ERC20 usdt = ERC20(0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2);

    AtomicExecutor executor;
    address owner = address(this);
    address uniswapV3Router = address(0x2626664c2603336E57B271c5C0b26F421741e481);
    address uniswapV2Router = address(0x4752ba5DBc23f44D87826276BF6Fd6b1C372aD24);
    address aerodromeRouter = address(0xcF77a3Ba9A5CA399B7c97c74d54e5b1Beb874E43);

    function setUp() public {
        vm.createSelectFork((vm.envString("BASE_RPC_URL")));
        executor = new AtomicExecutor(deployer, uniswapV3Router, uniswapV2Router, aerodromeRouter);

        vm.deal(swapper, 10 ether);
        vm.prank(swapper);
        weth.deposit{value: 10 ether}();
    }

    function testSwapUniswapV3() public {
        uint256 amountIn = 1 ether;

        AtomicExecutor.Swap[] memory swaps = new AtomicExecutor.Swap[](1);
        swaps[0] = AtomicExecutor.Swap(1, address(usdc), 500, false);

        AtomicExecutor.SwapData memory swapData =
            AtomicExecutor.SwapData({tokenIn: address(weth), amountIn: amountIn, swaps: swaps});

        vm.startPrank(swapper);
        weth.approve(address(executor), amountIn);
        uint256 amountOut = executor.swap(swapData);
        vm.stopPrank();
        assertGt(amountOut, 0, "Swap did not produce any output");
    }

    function testSwapUniswapV2() public {
        uint256 amountIn = 1 ether;

        AtomicExecutor.Swap[] memory swaps = new AtomicExecutor.Swap[](1);
        swaps[0] = AtomicExecutor.Swap(0, address(usdc), 0, false);

        AtomicExecutor.SwapData memory swapData =
            AtomicExecutor.SwapData({tokenIn: address(weth), amountIn: amountIn, swaps: swaps});

        vm.startPrank(swapper);
        weth.approve(address(executor), amountIn);
        uint256 amountOut = executor.swap(swapData);
        vm.stopPrank();
        assertGt(amountOut, 0, "Swap did not produce any output");
    }

    function testSwapAerodromeVolatile() public {
        uint256 amountIn = 1 ether;

        AtomicExecutor.Swap[] memory swaps = new AtomicExecutor.Swap[](1);
        swaps[0] = AtomicExecutor.Swap(2, address(usdc), 0, false);

        AtomicExecutor.SwapData memory swapData =
            AtomicExecutor.SwapData({tokenIn: address(weth), amountIn: amountIn, swaps: swaps});

        vm.startPrank(swapper);
        weth.approve(address(executor), amountIn);
        uint256 amountOut = executor.swap(swapData);
        vm.stopPrank();
        assertGt(amountOut, 0, "Swap did not produce any output");
    }

    function testMultipleSwap() public {
        uint256 amountIn = 1 ether;

        AtomicExecutor.Swap[] memory swaps = new AtomicExecutor.Swap[](2);
        swaps[0] = AtomicExecutor.Swap(1, address(usdc), 500, false);
        swaps[1] = AtomicExecutor.Swap(0, address(weth), 0, false);

        AtomicExecutor.SwapData memory swapData =
            AtomicExecutor.SwapData({tokenIn: address(weth), amountIn: amountIn, swaps: swaps});

        vm.startPrank(swapper);
        weth.approve(address(executor), amountIn);
        uint256 amountOut = executor.swap(swapData);
        vm.stopPrank();

        assertGt(amountOut, 0, "Swap did not produce any output");
    }
}
