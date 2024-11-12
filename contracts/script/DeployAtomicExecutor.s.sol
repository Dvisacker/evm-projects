// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script} from "forge-std/Script.sol";
import {HelperConfig} from "./HelperConfig.s.sol";
import {AtomicExecutor} from "../src/AtomicExecutor.sol";
import {console2} from "forge-std/console2.sol";

contract DeployAtomicExecutor is Script {
    address owner;
    address uniswapV3Router;
    address uniswapV2Router;
    address aerodromeRouter;

    function run() external {
        HelperConfig helperConfig = new HelperConfig();
        HelperConfig.NetworkConfig memory networkConfig = helperConfig.getActiveNetworkConfig();

        owner = networkConfig.deployerAddress;
        uniswapV3Router = networkConfig.uniswapV3Router;
        uniswapV2Router = networkConfig.uniswapV2Router;
        aerodromeRouter = networkConfig.aerodromeRouter;
        require(owner != address(0), "Owner address not set");
        require(uniswapV3Router != address(0), "Uniswap V3 Router address not set");
        require(uniswapV2Router != address(0), "Uniswap V2 Router address not set");
        require(aerodromeRouter != address(0), "Aerodrome Router address not set");

        vm.startBroadcast();

        AtomicExecutor executor = new AtomicExecutor(owner, uniswapV3Router, uniswapV2Router, aerodromeRouter);

        console2.log("AtomicExecutor deployed at:", address(executor));

        vm.stopBroadcast();
    }
}
