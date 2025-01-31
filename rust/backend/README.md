# Address Generator Backend

A Rust-based backend service that generates Ethereum-style addresses with random scores and stores them in SQLite.

## Features

- Generate random Ethereum-style addresses
- Assign random scores (100-1000)
- SQLite storage
- RESTful API endpoints
- Pagination support for retrieving addresses

## Prerequisites

- Rust (latest stable version)
- SQLite
- Cargo SQLx CLI (optional - only needed if setting up database from scratch)

## Setup

1. Build the project:

```bash
cargo build
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

The server will start at `http://localhost:3000`

## API Endpoints

### Generate Addresses (In-Memory)

```
GET /api/addresses?count=500
```

Generates addresses without storing them.

- `count` (optional): Number of addresses to generate (default: 1000)

### Generate and Store Addresses

```
POST /api/addresses/generate?count=500
```

Generates addresses and stores them in the database.

- `count` (optional): Number of addresses to generate (default: 1000)

### Retrieve Stored Addresses

```
GET /api/addresses/stored?page=1&per_page=100
```

Retrieves stored addresses with pagination.

- `page` (optional): Page number (default: 1)
- `per_page` (optional): Items per page (default: 100)

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
│   ├── main.rs          # Application entry point
│   ├── models.rs        # Data models
│   ├── handlers.rs      # Request handlers
│   ├── db.rs           # Database setup
│   └── services/
│       └── address_service.rs  # Address generation logic
├── data/               # Database directory
│   └── addresses.db    # SQLite database (included)
└── Cargo.toml         # Project dependencies
```

## Error Handling

The API uses standard HTTP status codes:

- 200: Success
- 400: Bad Request
- 500: Internal Server Error

All error responses include a JSON body with an error message.

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
