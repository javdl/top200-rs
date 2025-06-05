# Build and Deployment Issues

## GitHub Actions Daily Data Collection Failure

### Issue
The daily data collection GitHub Actions workflow was failing when executing `./target/release/top200-rs export-combined`.

### Root Cause
The `export-combined` command tries to create CSV files in the `output/` directory without ensuring the directory exists first. The functions `export_market_caps()` and `export_top_100_active()` in `src/marketcaps.rs` would call `std::fs::File::create()` on paths like:
- `output/combined_marketcaps_{timestamp}.csv`
- `output/top_100_active_{timestamp}.csv`

When the `output/` directory doesn't exist (as in a fresh GitHub Actions environment), these calls would fail.

### Solution
Added `std::fs::create_dir_all("output")?;` before file creation in both export functions to ensure the output directory exists.

### Files Modified
- `src/marketcaps.rs`: Lines 210 and 260 - Added directory creation before CSV file exports

### Testing
The fix ensures the application can run in any environment where the output directory may not exist, including:
- Fresh GitHub Actions runners
- Clean development environments
- Docker containers
- New clones of the repository