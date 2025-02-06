# AddressLogger Stylus Contract

Arbitrum Stylus smart contract for logging addresses and managing a betting system based on address performance.

## Overview

The contract provides two main functionalities:

1. Address Logging

- Emits AddressLog events with operator, addresses, timestamp and batch size
- Stores addresses on-chain

2. Betting System

- Users can place bets on specific addresses' future performance
- Position betting (true/false) for performance predictions
- Payable function for ETH-based betting

## Contract Events

```solidity
// Address logging event
event AddressLog(
    address indexed operator,
    address[] addresses,
    uint256 timestamp,
    uint256 batch_size
);

// Betting event
event BetPlaced(
    address indexed bettor,
    address indexed selectedAddress,
    bool position,
    uint256 amount
);
```

## Contract Functions

```solidity
// Log batch of addresses
function log_addresses(address[] memory addresses) external;

// Place bet on address performance
function place_bet(address selected_address, bool position) external payable;

// View functions
function get_bet(uint256 index) external view returns (address, address, bool, uint256);
function get_bet_count() external view returns (uint256);
```

## Prerequisites

- Rust installed (latest stable version)
- Cargo Stylus CLI tool
- An Arbitrum node connection (local or testnet)
- A wallet with test ETH

## Building

1. Clone the repository
2. Install dependencies:

```bash
cargo build
```

3. Build for deployment:

```bash
cargo stylus build
```

## Deployment

Deploy to Arbitrum Stylus network:

```bash
cargo stylus deploy \
  -e <RPC_ENDPOINT> \
  --private-key <YOUR_PRIVATE_KEY>
```

## Example Integration

```typescript
// Logging addresses
const addresses = ["0x123...", "0x456..."];
const tx = await contract.log_addresses(addresses);
const receipt = await tx.wait();

// Get address log event
const logEvent = receipt.events?.find((e) => e.event === "AddressLog");

// Placing a bet
const selectedAddress = "0x789...";
const position = true; // betting on positive performance
const betAmount = ethers.utils.parseEther("0.1");

const betTx = await contract.place_bet(selectedAddress, position, {
  value: betAmount,
});
const betReceipt = await betTx.wait();

// Get bet event
const betEvent = betReceipt.events?.find((e) => e.event === "BetPlaced");

// View functions
const betCount = await contract.get_bet_count();
const betDetails = await contract.get_bet(0); // get first bet
```

## Security Notes

- Public logging function - any address can log addresses
- Payable betting function - users can place bets with ETH
- No withdrawal mechanism implemented yet
- No bet resolution mechanism implemented yet
- Timestamps are block timestamps
- Gas costs increase with batch size
- Users should verify stored bet details after placement

## Development

The contract is built using:

- Arbitrum Stylus SDK
- Rust programming language
- EVM events for data storage
- Storage vectors for persistent data
- Solidity-compatible event emission

## Future Enhancements

- Add bet resolution mechanism
- Implement withdrawal functionality
- Add time windows for betting periods
- Add minimum/maximum bet amounts
- Implement admin controls
- Add bet cancellation functionality

## License

This project is licensed under the Apache 2.0 License.
