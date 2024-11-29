// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script} from "forge-std/Script.sol";
import {BatchExecutor} from "../src/BatchExecutor.sol";
import {HelperConfig} from "./HelperConfig.s.sol";
import {console2} from "forge-std/console2.sol";

contract DeployBatchExecutor is Script {
    address owner;

    function run() external {
        HelperConfig helperConfig = new HelperConfig();
        HelperConfig.NetworkConfig memory networkConfig = helperConfig.getActiveNetworkConfig();

        owner = networkConfig.deployerAddress;
        require(owner != address(0), "Owner address not set");
        console2.log("Owner:", owner);

        vm.startBroadcast();
        BatchExecutor executor = new BatchExecutor(owner);
        console2.log("BatchExecutor deployed at:", address(executor));
        vm.stopBroadcast();
    }
}
