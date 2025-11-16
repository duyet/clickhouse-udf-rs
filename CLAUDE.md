# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a collection of ClickHouse User-Defined Functions (UDFs) written in Rust. The project is structured as a Cargo workspace with multiple binary crates that compile into executables for ClickHouse integration.

## Build Commands

```bash
# Build all packages in release mode
cargo build --release

# Build specific package
cargo build --release -p vin
cargo build --release -p wkt
cargo build --release -p url
cargo build --release -p array
cargo build --release -p string

# Build in debug mode
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## Testing

```bash
# Run all tests
cargo test

# Run tests for specific package
cargo test -p vin
cargo test -p array

# Run specific test
cargo test test_topk_3
```

## Architecture

### Workspace Structure

This is a Cargo workspace with the following packages:

- **shared**: Common utilities used across all UDF packages
  - `io.rs`: Core I/O processing functions for stdin/stdout handling
  - `process_stdin()`: Standard line-by-line processing
  - `process_stdin_send_chunk_header()`: ClickHouse chunk header protocol support

- **vin**: Vehicle Identification Number processing functions
  - Generates 6 binaries (cleaner, year, manuf × [standard, chunk-header])

- **wkt**: Well-Known Text geometry format parsing
  - LineString parsing functionality

- **url**: URL extraction and detection functions
  - extract-url, has-url binaries

- **array**: Array manipulation functions
  - array-topk binary using FilteredSpaceSaving algorithm

- **string**: String processing functions

### Binary Structure Pattern

Each package follows a consistent pattern:

1. **Cargo.toml** defines multiple `[[bin]]` entries for different UDF functions
2. **src/bin/*.rs** contains minimal main functions that:
   - Import processing logic from the package's lib.rs
   - Call `shared::io::process_stdin()` with a closure
   - Handle command-line arguments via `shared::io::args()`
3. **src/lib.rs** contains the core business logic and tests

Example flow:
```
stdin → process_stdin() → transformation function → stdout
```

### ClickHouse Integration

The binaries are designed to be called by ClickHouse as executable UDFs:

- Standard mode: Line-by-line stdin/stdout processing
- Chunk header mode: Reads chunk length, processes batch, flushes stdout
- Both modes defined in `shared/src/io.rs`

UDF binaries must be placed in `/var/lib/clickhouse/user_scripts/` and configured via XML in `/etc/clickhouse-server/*_function.xml`.

### Shared I/O Module

The `shared::io` module provides the foundation for all UDFs:

- **ProcessFn type**: `Box<dyn Fn(&str) -> Option<String>>` - standardized processing function signature
- **args()**: Command-line argument parsing
- **process_stdin()**: Standard processing loop
- **process_stdin_send_chunk_header()**: Batch processing with ClickHouse chunk protocol

All UDF binaries follow this pattern to maintain consistency and reduce code duplication.

## Development Notes

- Each package can have benchmarks defined in `[[bench]]` sections using Criterion
- The workspace uses Rust edition 2021
- Release binaries are approximately 434KB each (from README example)
- CI runs on both Ubuntu and macOS in debug and release modes

## Code Quality Tools

### Formatting

The project uses `rustfmt` for consistent code formatting:

```bash
cargo fmt --all
```

Configuration is in `rustfmt.toml` using only stable features.

### Linting

The project uses `clippy` for catching common mistakes:

```bash
cargo clippy --all-targets --all-features
```

Configuration is in `clippy.toml` with reasonable thresholds for complexity and other metrics.

### Security Auditing

The project uses `cargo-deny` for security and license checking:

```bash
cargo install cargo-deny
cargo deny check
```

Configuration is in `deny.toml`.

## CI/CD

The project has the following GitHub Actions workflows:

1. **build-test.yaml**: Builds and tests on Ubuntu and macOS (both debug and release)
2. **cargo-clippy.yaml**: Runs clippy with SARIF output for GitHub security alerts
3. **cargo-fmt.yaml**: Checks code formatting
4. **security-audit.yaml**: Runs daily security audits using cargo-deny and cargo-audit

All workflows use:
- `dtolnay/rust-toolchain` for Rust installation (replaces deprecated `actions-rs/toolchain`)
- `Swatinem/rust-cache@v2` for dependency caching to speed up builds
- `actions/checkout@v4` (latest version)

## Project Structure Best Practices

- Each package has a `lib.rs` with crate-level documentation
- All public functions have doc comments
- Binaries are kept minimal, with logic in library modules
- Tests are co-located with the code they test
- Shared functionality is centralized in the `shared` crate

## Recent Improvements

The following improvements were made to enhance code quality:

- Fixed Rust edition from invalid "2024" to "2021" in all Cargo.toml files
- Removed placeholder string/src/main.rs and created proper lib.rs
- Added comprehensive crate-level documentation to all packages
- Enhanced shared/io.rs with detailed doc comments and examples
- Added `.editorconfig` for consistent editor settings
- Improved `.gitignore` with comprehensive Rust and IDE exclusions
- Added `rustfmt.toml` and `clippy.toml` for code quality enforcement
- Updated GitHub Actions to use non-deprecated actions:
  - Replaced `actions-rs/toolchain` with `dtolnay/rust-toolchain`
  - Added Rust cache with `Swatinem/rust-cache@v2`
  - Updated to `actions/checkout@v4`
- Added security audit workflow with cargo-deny and cargo-audit
- Added `deny.toml` for security and license compliance
- Enhanced test coverage with additional edge cases
