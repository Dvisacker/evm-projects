// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script} from "forge-std/Script.sol";
import {TxSimulator} from "../src/TxSimulator.sol";
import {HelperConfig} from "./HelperConfig.s.sol";
import {console2} from "forge-std/console2.sol";

contract DeployTxSimulator is Script {
    address owner;

    function run() external {
        HelperConfig helperConfig = new HelperConfig();
        HelperConfig.NetworkConfig memory networkConfig = helperConfig.getActiveNetworkConfig();

        owner = networkConfig.deployerAddress;
        require(owner != address(0), "Owner address not set");
        console2.log("Owner:", owner);

        vm.startBroadcast();
        TxSimulator simulator = new TxSimulator();
        console2.log("TxSimulator deployed at:", address(simulator));
        vm.stopBroadcast();
    }
}
