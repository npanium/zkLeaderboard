# VerificationAndPrize Contract

Smart contract for verifying zkVerify proofs and processing payouts.

## Setup

```bash
npm install
```

## Deployment

### Using Hardhat Ignition

1. Configure network in hardhat.config.js:

```javascript
require("@nomicfoundation/hardhat-ignition");
require("dotenv").config();

module.exports = {
  solidity: "0.8.19",
  networks: {
    arbitrumSepolia: {
      url: process.env.RPC_URL,
      accounts: [process.env.PRIVATE_KEY],
    },
  },
};
```

2. Deploy:

```bash
npx hardhat ignition deploy ignition/modules/VerificationAndPrize.js --network arbitrumSepolia
```

3. To clean deployment state:

```bash
npx hardhat ignition clean --network arbitrumSepolia
```

## Contract Functions

### verifyWinnersAndProcess

Verifies zkVerify proof and processes payouts for winners.

Parameters:

- \_leaf: bytes32
- \_attestationId: uint256
- \_merklePath: bytes32[]
- \_leafCount: uint256
- \_index: uint256
- winners: bool[]

## Contract Events

- ProofVerified(bytes32 indexed leaf, uint256 indexed attestationId, uint256 index)
- PayoutsProcessed(uint256 winnersCount)
- VerificationFailed(bytes32 indexed leaf, uint256 indexed attestationId, string reason)

## Dependencies

- zkVerify Contract: 0x82941a739E74eBFaC72D0d0f8E81B1Dac2f586D5
- RISC0 Proving System
