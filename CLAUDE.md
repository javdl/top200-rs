# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a Rust application that tracks and analyzes market capitalization data for top companies (Top200-rs). It fetches data from financial APIs, stores it in a PostgreSQL database, and provides various commands for analysis and export.

## Building and Running

This project use a Nix development environment.
Prefix commands with `nix develop --command` to run them in the Nix environment. In the docs we put regular commands without the prefix to be concise.

### Development Environment Setup

```bash
# Clone and enter the repository
git clone https://github.com/javdl/top200-rs.git
cd top200-rs

# Set up environment using Nix
nix develop

# Or run directly with Nix
nix develop --command cargo run
```

### Environment Variables

Create a `.env` file in the project root with:

```env
FMP_API_KEY=your_api_key_here
FINANCIALMODELINGPREP_API_KEY=your_api_key_here
POSTGRES_URL_NON_POOLING="postgres://user:password@host:port/dbname" # For PostgreSQL connection
# Example: POSTGRES_URL_NON_POOLING="postgres://postgres:mysecretpassword@localhost:5432/top200_dev"
```

### Build Commands

```bash
# Build the project
cargo build

# Build for release
cargo build --release
```

### Run Commands

```bash
# Run without arguments (defaults to marketcaps subcommand)
cargo run

# Run with help to see all commands
cargo run -- --help

# Run a specific subcommand
cargo run -- ExportCombined
cargo run -- ListCurrencies
cargo run -- FetchHistoricalMarketCaps 2022 2025
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_details_serialization
```

## Database Operations

The application uses PostgreSQL with the `tokio-postgres` crate for database operations. Schema migrations are managed by `refinery` and are located in the `migrations/` directory.

Ensure your `POSTGRES_URL_NON_POOLING` environment variable is set before running database commands.

```bash
# Apply database migrations
# (Run from within nix develop if refinery-cli is only in the dev shell,
#  otherwise use `nix develop --command refinery ...`)
refinery migrate -e POSTGRES_URL_NON_POOLING -p ./migrations

# Inspect database (using psql CLI)
# (Run from within nix develop if psql is only in the dev shell)
psql $POSTGRES_URL_NON_POOLING 
# Or: psql "postgres://user:password@host:port/dbname"
```

## Code Architecture

### Core Components

1. **API Clients**: Abstraction layer for external APIs
   - Financial Modeling Prep (FMP) API client
   - Polygon.io API client

2. **Data Models**: Defined in `src/models.rs`
   - Company details
   - Financial data
   - Exchange rates

3. **Database Layer**: Handles PostgreSQL operations using `tokio-postgres` and schema migrations using `refinery`.

4. **Commands**: CLI interface using clap for parsing arguments

### Data Flow

1. Fetch exchange rates for currency conversion
2. Retrieve market cap data from various sources
3. Store in PostgreSQL database
4. Generate reports (CSV exports, charts)

### Key Modules

- `marketcaps.rs`: Core functionality for market cap data
- `exchange_rates.rs`: Currency exchange rate handling
- `details_*.rs`: Company details from different sources
- `historical_marketcaps.rs`: Historical data retrieval
- `utils.rs`: Common utilities and helpers
- `db.rs` (or similar): Will contain database connection logic and `refinery` integration.

## Common Tasks

### Adding New Tickers

Edit the `config.toml` file to add new tickers to either the `us_tickers` or `non_us_tickers` arrays.

### Updating Exchange Rates

```bash
cargo run -- ExportRates
```

### Generating Combined Market Cap Reports

```bash
cargo run -- ExportCombined
```

### Working with Historical Data

```bash
# Fetch historical market caps for a range of years
cargo run -- FetchHistoricalMarketCaps 2023 2025

# Fetch monthly historical market caps
cargo run -- FetchMonthlyHistoricalMarketCaps 2023 2025
```

### Code Formatting

After making code changes, always run the Rust formatter to ensure code style consistency:

```bash
# Format all code in the project (run from within nix develop)
nix develop --command cargo fmt --all
```

### Dependency and License Checks

After making changes, especially to dependencies, run `cargo-deny` to check for issues:

```bash
# Run cargo-deny checks (run from within nix develop)
nix develop --command cargo deny check
```