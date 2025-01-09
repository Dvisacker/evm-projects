// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {WETH} from "../lib/solmate/src/tokens/WETH.sol";
import "../src/TxSimulator.sol";
import "forge-std/console.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "../src/interfaces/IUniswapV3Router.sol";
import "../src/interfaces/IUniswapV2Router.sol";
import "../src/interfaces/IAerodromeRouter.sol";

contract TxSimulatorTest is Test {
    address deployer = makeAddr("deployer");
    address owner = address(this);

    WETH weth = WETH(payable(0x4200000000000000000000000000000000000006));
    ERC20 dai = ERC20(0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb);
    ERC20 usdc = ERC20(0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913);
    ERC20 usdt = ERC20(0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2);

    TxSimulator public simulator;
    IUniswapV3Router public uniswapV3Router = IUniswapV3Router(0x2626664c2603336E57B271c5C0b26F421741e481);
    IUniswapV2Router public uniswapV2Router = IUniswapV2Router(0x4752ba5DBc23f44D87826276BF6Fd6b1C372aD24);
    IAerodromeRouter public aerodromeRouter = IAerodromeRouter(payable(0xcF77a3Ba9A5CA399B7c97c74d54e5b1Beb874E43));
    address public aerodromeFactoryAddress = 0x420DD381b31aEf6683db6B902084cB0FFECe40Da;
    // V3 factory address
    address uniswapV3Factory = 0x33128a8fC17869897dcE68Ed026d694621f6FDfD;
    address uniswapV3Quoter = 0x3d4e44Eb1374240CE5F1B871ab261CD16335B76a;
    address uniswapV2Factory = 0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6;
    // V3 init code hash
    bytes32 initCodeHash = 0xe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54;

    function setUp() public {
        vm.createSelectFork((vm.envString("BASE_RPC_URL")));
        simulator = new TxSimulator();
        vm.deal(deployer, 10 ether);
    }

    function testUniswapV3Swap() public {
        uint256 amountIn = 1 ether;

        TxSimulator.SwapParams[] memory paramsArray = new TxSimulator.SwapParams[](1);
        paramsArray[0] = TxSimulator.SwapParams({
            protocol: 1,
            handler: address(uniswapV3Quoter),
            tokenIn: address(weth),
            tokenOut: address(usdc),
            fee: 3000,
            amount: amountIn,
            stable: false
        });

        vm.startPrank(deployer);
        simulator.simulateSwapIn(paramsArray);
        vm.stopPrank();
    }

    function testUniswapV2Swap() public {
        uint256 amountIn = 1 ether;

        TxSimulator.SwapParams[] memory paramsArray = new TxSimulator.SwapParams[](1);
        paramsArray[0] = TxSimulator.SwapParams({
            protocol: 0,
            handler: address(uniswapV2Factory),
            tokenIn: address(weth),
            tokenOut: address(usdc),
            fee: 3000,
            amount: amountIn,
            stable: false
        });

        vm.startPrank(deployer);
        simulator.simulateSwapIn(paramsArray);
        vm.stopPrank();
    }

    function testCurveSwap() public {
        uint256 amountIn = 1 ether;

        TxSimulator.SwapParams[] memory paramsArray = new TxSimulator.SwapParams[](1);
        paramsArray[0] = TxSimulator.SwapParams({
            protocol: 2,
            handler: address(aerodromeRouter),
            tokenIn: address(weth),
            tokenOut: address(usdc),
            fee: 0,
            amount: amountIn,
            stable: true
        });

        vm.startPrank(deployer);
        simulator.simulateSwapIn(paramsArray);
        vm.stopPrank();
    }
}
