# AddressLogger Stylus Contract

Arbitrum Stylus smart contract for logging batches of Ethereum addresses with metadata to the blockchain.

## Overview

The contract emits an AddressLog event containing:

- Operator address (indexed)
- Array of addresses
- Timestamp
- Batch size

## Contract Details

The contract implements an event structure:

```solidity
// Event signature
event AddressLog(
    address indexed operator,
    address[] addresses,
    uint256 timestamp,
    uint256 batch_size
);

// Function signature
function log_addresses(address[] memory addresses) external;
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
const addresses = ["0x123...", "0x456..."];

const tx = await contract.log_addresses(addresses);
const receipt = await tx.wait();

// Get emitted event
const event = receipt.events?.find((e) => e.event === "AddressLog");
```

## Security Notes

- Public logging function - any address can log addresses
- Timestamps are block timestamps
- Gas costs increase with batch size

## Development

The contract is built using:

- Arbitrum Stylus SDK
- Rust programming language
- EVM events for data storage

## License

This project is licensed under the Apache 2.0 License.
