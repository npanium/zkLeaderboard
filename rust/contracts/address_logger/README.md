# AddressLogger Stylus Contract

Arbitrum Stylus smart contract for managing a betting system with ERC20 token integration.

## Overview

The contract manages a betting system where:

1. An operator controls betting windows
2. Users can place ERC20 token-based bets on addresses
3. Includes fee collection and treasury management
4. Supports position betting (up/down) on addresses
5. Automated payout processing based on winners

## Contract Events

```solidity
event WindowStarted(
    address indexed operator,
    address[] validAddresses,
    uint256 timestamp
);

event WindowClosed(
    address indexed operator,
    uint256 timestamp
);

event BetPlaced(
    address indexed bettor,
    address indexed selectedAddress,
    bool position,
    uint256 amount
);

event PayoutProcessed(
    address indexed bettor,
    uint256 amount,
    bool isWinner
);
```

## Key Features

- ERC20 token-based betting system
- Operator-controlled betting windows
- 10% fee on all bets sent to treasury
- Up/Down position betting
- Automatic payout calculation and distribution
- Comprehensive betting amount tracking
- Multi-address betting support

## Contract Functions

### Admin Functions

```solidity
// Initialize contract
function init(address operator, address authorizedContract, address treasury, address token) external;

// Start betting window
function start_betting_window(address[] memory addresses) external;

// Close betting window
function close_betting_window() external;

// Process payouts
function process_payouts(bool[] memory winners) external;
```

### Betting Functions

```solidity
// Place bet with tokens
function place_bet(address bettor, address selectedAddress, bool position, uint256 amount) external;

// View functions
function get_bet(uint256 index) external view returns (address, address, bool, uint256);
function get_bet_count() external view returns (uint256);
function get_window_active() external view returns (bool);
function get_up_amount(uint256 addrIndex) external view returns (uint256);
function get_down_amount(uint256 addrIndex) external view returns (uint256);
```

### Helper Functions

```solidity
function is_valid_address(address addr) external view returns (bool);
function get_operator() external view returns (address);
function get_treasury() external view returns (address);
function get_token() external view returns (address);
```

## Prerequisites

- Rust (latest stable version)
- Cargo Stylus CLI tool
- Arbitrum node connection
- ERC20 token contract deployed
- Wallet with tokens for betting

## Building and Deployment

1. Build the contract:

```bash
cargo stylus build
```

2. Deploy to Arbitrum Stylus network:

```bash
cargo stylus deploy \
  -e <RPC_ENDPOINT> \
  --private-key <YOUR_PRIVATE_KEY>
```

## Integration Example

```typescript
// Initialize contract
const tx = await contract.init(
  operator.address,
  authorizedContract.address,
  treasury.address,
  token.address
);

// Start betting window
const addresses = ["0x123...", "0x456..."];
await contract.start_betting_window(addresses);

// Place bet (requires token approval first)
const selectedAddress = "0x789...";
const position = true; // betting up
const amount = ethers.utils.parseEther("100");

// Approve tokens first
await token.approve(contract.address, amount);

// Place bet
const betTx = await contract.place_bet(
  bettor.address,
  selectedAddress,
  position,
  amount
);

// Close window
await contract.close_betting_window();

// Process payouts
const winners = [true, false]; // results for each address
await contract.process_payouts(winners);
```

## Payout Mechanism

The contract implements a payout system where:

1. Winning pool gets their initial bet back plus a proportion of the losing pool
2. Proportion is based on bet size relative to total winning pool
3. If either side has no bets, all funds go to treasury
4. 10% fee is taken from all bets and sent to treasury

## Security Features

- Operator-controlled windows
- ERC20 allowance checks
- Fee collection system
- Treasury management
- Automatic pool calculations
- Input validation
- Access control on critical functions

## Contract States

The contract can be in two states:

1. Window Active: Betting is allowed
2. Window Closed: No betting allowed, payouts can be processed

## Storage Layout

- Window status (active/inactive)
- Valid addresses for current window
- Bets with bettor, address, position, amount
- Up/Down amounts per address
- Operator address
- Treasury address
- Token address

## License

This project is licensed under the Apache 2.0 License.
