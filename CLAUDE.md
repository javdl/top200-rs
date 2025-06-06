# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a Rust application that tracks and analyzes market capitalization data for top companies (Top200-rs). It fetches data from financial APIs, stores it in SQLite, and provides various commands for analysis and export.

## Building and Running

This project uses a Nix development environment.
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
DATABASE_URL=sqlite:data.db  # Optional, defaults to sqlite:data.db
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

# Run tests with coverage
cargo tarpaulin --out lcov --output-dir coverage
```

## Linting and Formatting

```bash
# Format code
cargo fmt --all

# Run clippy linter
cargo clippy

# Check license compliance
reuse lint
```

## Database Operations

The application uses SQLite with SQLx for database operations. Migrations are located in the `migrations/` directory.

```bash
# Inspect database (using sqlite3 CLI)
sqlite3 data.db

# Run a specific SQL query from tests
sqlite3 data.db < tests/market_caps_totals_per_year.sql
```

## Code Architecture

### Core Components

1. **API Clients**: Abstraction layer for external APIs
   - Financial Modeling Prep (FMP) API client in `src/api.rs`
   - Polygon.io API client in `src/api.rs`
   - Rate limiting with tokio semaphore (300 req/min for FMP)

2. **Data Models**: Defined in `src/models.rs`
   - Company details
   - Financial data
   - Exchange rates

3. **Database Layer**: Handles SQLite operations and migrations
   - Connection pooling with SQLx
   - Automatic migrations on startup
   - Tables: `currencies`, `forex_rates`, `market_caps`, `ticker_details`

4. **Commands**: CLI interface using clap for parsing arguments

### Data Flow

1. Fetch exchange rates for currency conversion
2. Retrieve market cap data from various sources
3. Store in SQLite database
4. Generate reports (CSV exports, charts)

### Key Modules

- `marketcaps.rs`: Core functionality for market cap data
- `exchange_rates.rs`: Currency exchange rate handling
- `details_*.rs`: Company details from different sources
- `historical_marketcaps.rs`: Historical data retrieval
- `monthly_historical_marketcaps.rs`: Monthly historical data
- `ticker_details.rs`: Company details management
- `utils.rs`: Common utilities and helpers

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

## API Rate Limits and Error Handling

- **FMP API**: 300 requests per minute (enforced via semaphore)
- **Polygon API**: Standard rate limits apply
- Automatic retry logic for transient failures
- Progress bars for long-running operations
- Comprehensive error messages with anyhow

## CLI Commands

The application supports these main commands:

- `MarketCaps` (default) - Fetch and update market cap data
- `ExportCombined` - Export combined market cap report to CSV
- `ExportRates` - Export exchange rates to CSV
- `FetchHistoricalMarketCaps` - Fetch historical yearly data
- `FetchMonthlyHistoricalMarketCaps` - Fetch historical monthly data
- `ListCurrencies` - List all available currencies
- `Details` - Fetch company details
- `ReadConfig` - Display configuration
