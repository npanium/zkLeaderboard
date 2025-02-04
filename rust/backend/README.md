# Address Generator Backend

A Rust-based backend service that generates Ethereum-style addresses with random scores, stores them in SQLite, and provides secure hashing functionality for on-chain verification via Stylus smart contracts.

## Features

- Generate random Ethereum-style addresses
- Assign random scores (100-1000)
- SQLite storage
- RESTful API endpoints
- Pagination support for retrieving addresses
- Secure Keccak-256 hashing of address data
- Smart contract integrations with Stylus for both hashing and address logging

## Prerequisites

- Rust (latest stable version)
- SQLite
- Cargo SQLx CLI (optional - only needed if setting up database from scratch)
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
```

### Database Setup

The SQLite database is included in the repository at `data/addresses.db`.

If you need to set up the database from scratch (only if database doesn't exist):

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

The server will start at `http://localhost:3001`

## API Endpoints

All endpoints are prefixed with `/api/v0/addresses`

### Generate Addresses (In-Memory)

```
GET /api/v0/addresses?count=500
```

Generates addresses without storing them.

- `count` (optional): Number of addresses to generate (default: 1000)

### Generate and Store Addresses

```
POST /api/v0/addresses/generate?count=500
```

Generates addresses and stores them in the database.

- `count` (optional): Number of addresses to generate (default: 1000)

### Retrieve Stored Addresses

```
GET /api/v0/addresses/stored?page=1&per_page=100
```

Retrieves stored addresses with pagination.

- `page` (optional): Page number (default: 1)
- `per_page` (optional): Items per page (default: 100)

### Get Hash of Stored Addresses (Paginated)

```
GET /api/v0/addresses/hash?page=1&per_page=100
```

Generates a deterministic hash of the specified page of addresses.

### Get Hash of All Addresses

```
GET /api/v0/addresses/hash/all
```

Generates a deterministic hash of all stored addresses.

### Store Hash On-Chain

```
POST /api/v0/addresses/hash/store
```

Generates a hash of all addresses and stores it in the hash storage contract.

Returns:

```json
{
  "hash": "0x...",
  "timestamp": 1234567890,
  "record_count": 1000,
  "transaction_result": "0x..."
}
```

### Log Random Addresses On-Chain

```
POST /api/v0/addresses/random-log?count=5
```

Selects random addresses from the database and logs them to the address logger contract.

- `count` (optional): Number of random addresses to log (default: 5)

Returns:

```json
{
  "count": 5,
  "transaction_hash": "0x..."
}
```

## Database Schema

The addresses are stored in an SQLite database with the following schema:

```sql
CREATE TABLE addresses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT NOT NULL,
    score INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

## Project Structure

```
backend/
├── src/
│   ├── main.rs              # Application entry point
│   ├── models.rs            # Data models
│   ├── handlers.rs          # Request handlers
│   ├── db.rs               # Database setup
│   └── services/
│       ├── mod.rs          # Service module declarations
│       ├── address_service.rs       # Address generation logic
│       ├── hash_service.rs         # Hashing functionality
│       ├── hash_contract_service.rs # Hash storage contract interactions
│       └── addr_logger_contract_service.rs # Address logging contract interactions
├── data/                   # Database directory
│   └── addresses.db        # SQLite database (included)
└── Cargo.toml             # Project dependencies
```

## Error Handling

The API uses standard HTTP status codes:

- 200: Success
- 400: Bad Request
- 404: Not Found
- 500: Internal Server Error

All error responses include a JSON body with an error message.

## Security Features

- Deterministic address sorting before hashing
- Keccak-256 hashing (Ethereum standard)
- Secure contract interactions with two separate contracts
- Environment variable configuration
- Type-safe database operations
- Comprehensive error handling and logging
- Not found handler for undefined routes

## Development

To run tests:

```bash
cargo test
```

To check code formatting:

```bash
cargo fmt -- --check
```

To run linter:

```bash
cargo clippy
```
