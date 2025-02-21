// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {Groth16Verifier} from "src/Verifier_sha256_32.sol";

contract Verifier_sha256_32Script is Script {
    Groth16Verifier public verifier;

    function run() public {
        vm.startBroadcast();

        verifier = new Groth16Verifier();

        vm.stopBroadcast();
    }
}
