#![allow(dead_code)]
use e2e::alloy::sol;

sol!(
#[sol(rpc)]
contract AddressLogger{
    function logAddresses(address[] memory addresses) external returns (uint8[] memory);
    function placeBet(address selected_address, bool position) external payable returns (uint8[] memory);
    function getBet(uint256 index) external view returns (address, address, bool, uint256);
    function getBetCount() external view returns (uint256);
});
