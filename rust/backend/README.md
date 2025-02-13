# Address Generator Backend

A Rust-based backend service that generates Ethereum-style addresses with random scores, stores them in SQLite, and provides secure hashing functionality for on-chain verification via Stylus smart contracts. Includes betting and token management features.

## Features

- Generate random Ethereum-style addresses with scores
- SQLite database storage
- RESTful API endpoints with CORS support
- Smart contract integrations with Arbitrum Stylus for:
  - Hash storage and verification
  - Address logging
  - Betting functionality
  - Token management
- Complete betting window management
- Token minting and burning operations
- Comprehensive error handling and logging

## Prerequisites

- Rust (latest stable version)
- SQLite
- Cargo SQLx CLI (optional - for database setup)
- Ethereum wallet with private key for contract interactions
- Access to Arbitrum Stylus RPC endpoint

## Setup

1. Build the project:

```bash
cargo build
```

2. Configure environment variables in `.env`:

```env
DATABASE_URL="sqlite:data/addresses.db"
RPC_URL="https://stylus-testnet.arbitrum.io/rpc"
PRIVATE_KEY="your-private-key-here"
HASH_CONTRACT_ADDRESS="your-hash-contract-address-here"
ADDR_LOGGER_CONTRACT_ADDRESS="your-address-logger-contract-address-here"
TOKEN_CONTRACT_ADDRESS="your-token-contract-address-here"
```

### Database Setup

The SQLite database is included in the repository at `data/addresses.db`.

For fresh database setup:

```bash
mkdir -p data
export DATABASE_URL="sqlite:data/addresses.db"
cargo sqlx prepare -- --lib
```

## Run the Server

Development mode:

```bash
cargo run
```

Production mode:

```bash
cargo run --release
```

Server starts at `http://localhost:3001`

## API Endpoints

All endpoints are prefixed with `/api/v0`

### Addresses

```
GET /addresses
```

Retrieves all stored addresses.

```
GET /addresses/stored?page=1&per_page=100
```

Retrieves stored addresses with pagination.

### Hashing

```
GET /addresses/hash?page=1&per_page=100
```

Generates hash for paginated addresses.

```
GET /addresses/hash/all
```

Generates hash of all addresses.

```
POST /addresses/hash/store
```

Stores hash of all addresses on-chain.

### Betting Window Management

```
POST /addresses/init
```

Initializes contract with operator, treasury and token addresses.

```json
{
  "operator": "0x...",
  "authorized_contract": "0x...",
  "treasury": "0x..."
}
```

```
POST /addresses/window/start?count=3
```

Starts betting window with random addresses.

- `count` (optional): Number of addresses (default: 3)

```
POST /addresses/window/close
```

Closes current betting window.

```
GET /addresses/window/status
```

Returns current betting window status.

### Betting Operations

```
POST /addresses/bets
```

Places a new bet.

```json
{
  "bettor": "0x...",
  "selected_address": "0x...",
  "position": true,
  "amount": "0.1"
}
```

```
GET /addresses/bets/count
```

Returns total number of bets placed.

```
GET /addresses/bets/{index}
```

Returns details of specific bet.

```
GET /addresses/bets/amounts/{index}
```

Returns up/down betting amounts for address index.

### Token Operations

```
POST /token/mint
```

Mints new tokens.

```json
{
  "amount": "1000"
}
```

```
POST /token/mint-to
```

Mints tokens to specific address.

```json
{
  "address": "0x...",
  "amount": "1000"
}
```

```
POST /token/burn
```

Burns tokens.

```json
{
  "amount": "1000"
}
```

```
GET /token/balance/{address}
```

Returns token balance for address.

## Error Handling

The API uses standard HTTP status codes:

- 200: Success
- 400: Bad Request (invalid input)
- 403: Forbidden (e.g., betting window already active)
- 404: Not Found
- 500: Internal Server Error

All error responses include a JSON body with an error message.

## Project Structure

```
backend/
├── src/
│   ├── main.rs              # Application entry point and server setup
│   ├── models.rs            # Data models and types
│   ├── handlers.rs          # API endpoint handlers
│   ├── db.rs               # Database setup and operations
│   └── services/
│       ├── mod.rs          # Service module declarations
│       ├── address_service.rs       # Address generation
│       ├── hash_service.rs         # Hashing functionality
│       ├── hash_contract_service.rs # Hash storage contract
│       ├── addr_logger_contract_service.rs # Address logging contract
│       └── betting_token_service.rs # Token management
├── data/
│   └── addresses.db        # SQLite database
└── Cargo.toml             # Project dependencies
```

## Security Features

- CORS support (configurable)
- Environment variable configuration
- Type-safe database operations
- Input validation for all endpoints
- Secure contract interactions
- Comprehensive error handling and logging
- Default route handler for undefined paths

## Environment Variables

Required environment variables:

- `DATABASE_URL`: SQLite database URL
- `RPC_URL`: Arbitrum Stylus RPC endpoint
- `PRIVATE_KEY`: Private key for contract interactions
- `HASH_CONTRACT_ADDRESS`: Address of hash storage contract
- `ADDR_LOGGER_CONTRACT_ADDRESS`: Address of address logger contract
- `TOKEN_CONTRACT_ADDRESS`: Address of betting token contract

## Development

Run tests:

```bash
cargo test
```

Check code formatting:

```bash
cargo fmt -- --check
```

Run linter:

```bash
cargo clippy
```

## Database Schema

```sql
CREATE TABLE addresses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT NOT NULL,
    score INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```
