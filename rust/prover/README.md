# zkLeaderboard

A zero-knowledge proof system that verifies if an address is in the top 50% of scores without revealing actual scores.

## Quick Start

Ensure [rustup] is installed.

To run the server:

```bash
cargo run
```

For development mode with logs:

```bash
RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run
```

## Setup Database

1. Set the database URL:

```bash
export DATABASE_URL="sqlite:<database path>"
```

2. If `.sqlx` folder is missing, regenerate SQLx data:

```bash
cargo sqlx prepare
```

This will regenerate necessary SQLx files for compile-time query checking.

Note: `.sqlx` folder contains data for compile-time verification of SQL queries. It must be recreated if deleted or when queries change.

````

## Components

### Guest Program

- Takes addresses and scores as input
- Calculates median score
- Outputs whether each address is in top 50% without revealing scores
- Located in `methods/guest/src/main.rs`

### Host Program

- REST API server handling proof requests
- SQLite database integration for storing addresses and scores
- Endpoints:
  - POST `/check_position` - Submit addresses to check
  - GET `/job/{job_id}` - Get proof status and results
- Located in `host/src/main.rs`

## Database Schema

```sql
CREATE TABLE addresses (
    id INTEGER PRIMARY KEY,
    address TEXT,
    score INTEGER,
    created_at DATETIME
)
````

## Technologies

- RISC0 zkVM for zero-knowledge proofs
- Actix-web for REST API
- SQLx for database operations
- SQLite for data storage

## Current Shortcomings

- No automated testing suite
- Basic error handling needs improvement
- Database queries could be optimized for larger datasets
- No authentication/authorization implemented
- Limited input validation
- Single-threaded proof generation could be a bottleneck
- No rate limiting on API endpoints

## Future Development

- Add comprehensive test suite
- Implement robust error handling and logging
- Optimize database queries with proper indexing
- Add authentication and rate limiting
- Implement parallel proof generation
- Add input validation and sanitization
- Migrate to a more scalable database solution
- Add monitoring and analytics
- Implement caching for frequently requested proofs
- Add documentation for API endpoints

[rustup]: https://rustup.rs
