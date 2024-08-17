// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {GirlfriendBank} from "../src/girlfriendBank.sol";

contract DeployZK is Script {
    function run() public {
        vm.startBroadcast();
        GirlfriendBank temp = new GirlfriendBank(1e18, 10, 0x3B6041173B80E77f038f3F2C0f9744f04837185e, 0x002e3cd0431437b0747be8442b444d9bad9303730a343819cd927d4613d3d213);
        vm.stopBroadcast();
    }
}