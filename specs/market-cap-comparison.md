# Market Cap Comparison Feature Specification

## Overview
Add functionality to compare market cap data between two dates, calculating differences and percentage changes, with additional analytics on rank changes and market share.

## Requirements

### Functional Requirements
1. Compare market cap data between two arbitrary dates
2. Calculate absolute and percentage changes in USD
3. Track ranking changes between dates
4. Generate comprehensive comparison reports
5. Handle missing data gracefully (companies not present on both dates)

### Non-Functional Requirements
- Performance: Should handle 200+ companies efficiently
- Usability: Clear CLI interface with progress indicators
- Reliability: Robust error handling for missing files or invalid dates

## CLI Interface
```bash
cargo run -- compare-market-caps --from 2025-07-01 --to 2025-08-01
```

## Implementation Architecture

### Module Structure
- New module: `src/compare_marketcaps.rs`
- Integration with existing CLI in `main.rs`

### Core Data Structures

```rust
struct MarketCapComparison {
    ticker: String,
    name: String,
    market_cap_from: Option<f64>,  // USD value on from_date
    market_cap_to: Option<f64>,    // USD value on to_date
    absolute_change: Option<f64>,
    percentage_change: Option<f64>,
    rank_from: Option<usize>,
    rank_to: Option<usize>,
    rank_change: Option<i32>,
    market_share_from: Option<f64>,
    market_share_to: Option<f64>,
}
```

### Processing Pipeline

1. **Data Loading**
   - Locate CSV files for specified dates
   - Parse CSV files and extract USD market cap values
   - Build ticker-indexed maps for efficient lookup

2. **Comparison Analysis**
   - Match companies by ticker
   - Calculate absolute changes: `to_value - from_value`
   - Calculate percentage changes: `(to_value - from_value) / from_value * 100`
   - Determine rankings based on market cap
   - Calculate rank changes

3. **Market Share Analysis**
   - Calculate total market cap for each date
   - Compute individual company market shares
   - Track market share changes

4. **Output Generation**
   - Main CSV with all comparison data
   - Summary markdown report with insights

## Output Specifications

### Main Comparison CSV
**Filename**: `output/comparison_YYYY-MM-DD_to_YYYY-MM-DD_YYYYMMDD_HHMMSS.csv`

**Columns**:
- Ticker
- Name
- Market Cap From (USD)
- Market Cap To (USD)
- Absolute Change (USD)
- Percentage Change
- Rank From
- Rank To
- Rank Change
- Market Share From (%)
- Market Share To (%)

**Sorting**: By percentage change (descending)

### Summary Report
**Filename**: `output/comparison_YYYY-MM-DD_to_YYYY-MM-DD_summary_YYYYMMDD_HHMMSS.md`

**Sections**:
1. Overview statistics
2. Top 10 gainers by percentage
3. Top 10 losers by percentage
4. Top 10 by absolute gain
5. Top 10 by absolute loss
6. Biggest rank improvements
7. Biggest rank declines
8. Market concentration analysis

## Implementation Checklist

- [x] Create specs directory and documentation
- [ ] Add new CLI subcommand `CompareMarketCaps` with from/to date parameters
- [ ] Create `src/compare_marketcaps.rs` module
- [ ] Implement CSV reading and parsing functions
- [ ] Implement comparison logic with percentage/absolute calculations
- [ ] Implement ranking calculation and rank change detection
- [ ] Implement market share calculation
- [ ] Create main comparison CSV export
- [ ] Create summary markdown report with top gainers/losers
- [ ] Add error handling for missing files and invalid dates
- [ ] Add progress indicators for better UX
- [ ] Format numbers appropriately (2 decimal places for percentages)
- [ ] Add module to main.rs and wire up CLI command
- [ ] Test with July 1 and August 1 data
- [ ] Verify edge cases (NA values, new companies, delisted companies)

## Error Handling

### Expected Errors
- CSV file not found for specified date
- Invalid date format
- Malformed CSV data
- Division by zero in percentage calculations

### Error Responses
- Clear error messages indicating the problem
- Suggestions for resolution (e.g., "Run fetch-specific-date-market-caps first")

## Testing Strategy

### Test Cases
1. Normal comparison between two dates with complete data
2. Comparison with companies missing on one date
3. Comparison with zero or negative market caps
4. Same date comparison (should show 0% change)
5. Invalid date formats
6. Missing CSV files

### Validation
- Verify calculations manually for a subset of companies
- Check sort order is correct
- Ensure NA values are handled properly
- Validate markdown formatting

## Future Enhancements
- Support for comparing multiple dates (time series)
- Graphical visualization of changes
- Sector-wise analysis
- Export to different formats (JSON, Excel)
- Web dashboard for interactive exploration