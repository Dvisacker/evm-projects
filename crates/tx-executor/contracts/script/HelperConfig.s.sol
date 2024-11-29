// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";

contract HelperConfig is Script {
    NetworkConfig public activeNetworkConfig;

    struct NetworkConfig {
        uint256 deployerKey;
        address deployerAddress;
        address aaveLendingPool;
        address uniswapV2Router;
        address uniswapV3Factory;
        address uniswapV3Router;
        address aerodromeRouter;
        address usdc;
        address usdt;
        address weth;
    }

    constructor() {
        console2.log(block.chainid);
        uint256 deployerKey;
        address deployerAddress;

        bool isAnvil = vm.envBool("USE_ANVIL");
        if (isAnvil) {
            console2.log("Anvil detected");
            deployerKey = vm.envUint("ANVIL_DEV_PRIVATE_KEY");
            deployerAddress = vm.envAddress("ANVIL_DEV_ADDRESS");
        } else {
            deployerKey = vm.envUint("DEV_PRIVATE_KEY");
            deployerAddress = vm.envAddress("DEV_ADDRESS");
        }
        if (block.chainid == 42161 || block.chainid == 31337) {
            activeNetworkConfig = NetworkConfig({
                deployerKey: deployerKey,
                deployerAddress: vm.addr(deployerKey),
                aaveLendingPool: 0x794a61358D6845594F94dc1DB02A252b5b4814aD,
                uniswapV2Router: 0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506,
                uniswapV3Factory: 0x1F98431c8aD98523631AE4a59f267346ea31F984,
                uniswapV3Router: 0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45,
                aerodromeRouter: 0x0000000000000000000000000000000000000000,
                usdc: 0xaf88d065e77c8cC2239327C5EDb3A432268e5831,
                usdt: 0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9,
                weth: 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1
            });
        } else if (block.chainid == 8453) {
            activeNetworkConfig = NetworkConfig({
                deployerKey: deployerKey,
                deployerAddress: vm.addr(deployerKey),
                aaveLendingPool: 0x794a61358D6845594F94dc1DB02A252b5b4814aD,
                uniswapV2Router: 0x4752ba5DBc23f44D87826276BF6Fd6b1C372aD24,
                uniswapV3Factory: 0x33128a8fC17869897dcE68Ed026d694621f6FDfD,
                uniswapV3Router: 0x2626664c2603336E57B271c5C0b26F421741e481,
                aerodromeRouter: 0xcF77a3Ba9A5CA399B7c97c74d54e5b1Beb874E43,
                usdc: 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913,
                usdt: 0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2,
                weth: 0x4200000000000000000000000000000000000006
            });
            // sepolia
        } else if (block.chainid == 11155111) {
            activeNetworkConfig = NetworkConfig({
                deployerKey: deployerKey,
                deployerAddress: vm.addr(deployerKey),
                aaveLendingPool: 0x6Ae43d3271ff6888e7Fc43Fd7321a503ff738951,
                uniswapV2Router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D,
                uniswapV3Factory: 0x1F98431c8aD98523631AE4a59f267346ea31F984,
                aerodromeRouter: 0x0000000000000000000000000000000000000000,
                uniswapV3Router: 0xE592427A0AEce92De3Edee1F18E0157C05861564,
                usdc: 0x94a9D9AC8a22534E3FaCa9F4e7F2E2cf85d5E4C8,
                usdt: 0xaA8E23Fb1079EA71e0a56F48a2aA51851D8433D0,
                weth: 0xC558DBdd856501FCd9aaF1E62eae57A9F0629a3c
            });
        } else {
            revert("Unsupported network");
        }
    }

    function getActiveNetworkConfig() public view returns (NetworkConfig memory) {
        return activeNetworkConfig;
    }
}
