# zkLeaderboard Demo Frontend

A proof-of-concept web3 gaming infrastructure that demonstrates secure leaderboard and betting functionality for blockchain games. Built using zero-knowledge proofs, this system enables trustless score verification and betting on player rankings, making it ideal for competitive web3 games that want to integrate wagering mechanics. The application showcases how game developers can implement verifiable leaderboards and betting systems while maintaining game integrity through zkVerify's proof verification network.

## Features

- Connect wallet functionality using RainbowKit
- Interactive betting interface
- Real-time zero-knowledge proof generation and verification
- Terminal-style instruction display
- Dynamic leaderboard with betting options
- Integration with:
  - zkVerify for proof verification
  - Risc0 for proof generation
  - Arbitrum for smart contract interactions

## Prerequisites

- Node.js (v20 or higher recommended)
- npm
- A web3 wallet (MetaMask recommended)
- Access to Arbitrum Sepolia testnet
- Substrate wallet for zkVerify interaction

## Tech Stack

- Next.js 14.0.3
- React 18
- TypeScript 5
- TailwindCSS 3.3
- RainbowKit 2.2.3
- wagmi 2.14.11
- ethers.js 6.13.5
- zkverifyjs 0.6.0

## Getting Started

1. Clone the repository:

```bash
git clone [your-repo-url]
cd [your-repo-name]
```

2. Install dependencies:

```bash
npm install
```

3. Set up environment variables:
   Copy `.env.example` to `.env.local` and fill in your values:

```env
ZKVERIFY_SEED_PHRASE="YOUR SUBSTRATE WALLET SEED PHRASE"
PRIVATE_KEY="YOUR EVM WALLET PRIVATE KEY"
VERIFICATION_PRIZE_CONTRACT_ADDRESS=0x576995f7a160444bec34d9072e7aada5972bd754
```

4. Run the development server:

```bash
npm run dev
```

5. Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

## Project Structure

```
src/
├── components/         # React components
├── hooks/             # Custom React hooks
├── lib/               # Utilities and constants
│   ├── risc0.ts      # Risc0 integration
│   ├── zkVerify.ts   # zkVerify integration
│   └── types.ts      # TypeScript types
├── app/
│   └── api/          # Next.js API routes
│       ├── verify/   # Main verification endpoint
│       └── test/     # Testing endpoints
└── styles/           # CSS styles
```

## Dependencies

### Main Dependencies

```json
{
  "@rainbow-me/rainbowkit": "^2.2.3",
  "@tanstack/react-query": "^5.66.0",
  "axios": "^1.7.9",
  "ethers": "^6.13.5",
  "next": "14.0.3",
  "react": "^18",
  "react-dom": "^18",
  "viem": "^2.23.1",
  "wagmi": "^2.14.11",
  "zkverifyjs": "^0.6.0"
}
```

### UI Dependencies

```json
{
  "@radix-ui/react-progress": "^1.1.2",
  "@radix-ui/react-slot": "^1.1.2",
  "class-variance-authority": "^0.7.1",
  "clsx": "^2.1.1",
  "lucide-react": "^0.475.0",
  "tailwind-merge": "^3.0.1",
  "tailwindcss-animate": "^1.0.7"
}
```

## API Routes

### Main Verification Route (`/api/verify`)

Handles the complete verification flow:

1. Submits addresses to Risc0 for proof generation
2. Polls for proof completion
3. Submits proof to zkVerify
4. Calls smart contract for winner verification and prize distribution

### Test Route (`/api/test`)

A testing endpoint for verifying proofs directly with zkVerify smart contract.

## Smart Contract Integration

The application interacts with two smart contracts:

- zkVerify Contract (0x82941a739E74eBFaC72D0d0f8E81B1Dac2f586D5)
- Verification Prize Contract (customizable via env)

## Development

### Available Scripts

- `npm run dev`: Runs the development server
- `npm run build`: Builds the application for production
- `npm start`: Starts the production server
- `npm run lint`: Runs the linter

### Testing the API

You can test the verification endpoint using curl or Postman:

```bash
curl -X POST http://localhost:3000/api/verify \
  -H "Content-Type: application/json" \
  -d '{"addresses":["0x123...", "0x456..."]}'
```

## Environment Variables

| Variable                            | Description                                           |
| ----------------------------------- | ----------------------------------------------------- |
| ZKVERIFY_SEED_PHRASE                | Your Substrate wallet seed phrase for zkVerify        |
| PRIVATE_KEY                         | Your EVM wallet private key for contract interactions |
| VERIFICATION_PRIZE_CONTRACT_ADDRESS | Address of the verification prize contract            |

## Browser Support

The application is tested on:

- Chrome (latest)
- Firefox (latest)
- Edge (latest)

Ensure your browser has a Web3 wallet extension installed (like MetaMask) to interact with the dApp.
