// SPDX-License-Identifier: APACHE-2.0
pragma solidity ^0.8.13;

interface IAddressLogger {
    function processPayouts(bool[] memory winners) external;
}
