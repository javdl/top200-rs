# top200-rs Development Guide

## Build & Test Commands
- Build: `cargo build`
- Run: `cargo run` or with commands: `cargo run -- ExportCombined`
- Test all: `cargo test`
- Test specific: `cargo test test_get_last_day_of_month`
- Lint: `cargo clippy`
- Format: `cargo fmt`

## Code Style Guidelines
- Follow Rust snake_case for functions/variables, CamelCase for types
- Use Result<T, E> with anyhow for error handling
- Import order: std first, then external crates alphabetically, then internal modules
- Document public functions with /// comments explaining purpose
- Include tests in `#[cfg(test)]` modules within source files
- Use `async/await` consistently for asynchronous code
- Maintain SPDX license headers on all source files
- Keep functions focused on single responsibility
- Structure project with modular components per README.md
- Include GitHub workflow badges in documentation

## Directory Structure
- `/src`: Core implementation files
- `/migrations`: Database schema and updates
- `/output`: Generated reports and analysis
- `/tests`: SQL validation queries