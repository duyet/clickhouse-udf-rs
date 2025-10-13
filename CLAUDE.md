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
