# Contributing to ClickHouse UDF Rust

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to this project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

## Code of Conduct

This project adheres to a code of conduct based on respect and professionalism. Please be kind and courteous to others.

## Getting Started

### Prerequisites

- Rust 1.73+ (stable toolchain)
- cargo
- git

### Setup Development Environment

```bash
# Clone the repository
git clone https://github.com/duyet/clickhouse-udf-rs.git
cd clickhouse-udf-rs

# Build the project
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 2. Make Your Changes

Follow the [Coding Standards](#coding-standards) below.

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run tests for a specific package
cargo test -p vin

# Run with output
cargo test -- --nocapture

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy --all-targets --all-features
```

### 4. Commit Your Changes

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
# Format: <type>(<scope>): <description>
git commit -m "feat(vin): add VIN checksum validation"
git commit -m "fix(url): handle URLs with ports correctly"
git commit -m "docs: update README with new examples"
git commit -m "test(array): add edge case tests for topk"
```

**Commit Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style/formatting
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## Coding Standards

### Rust Style

We use `rustfmt` and `clippy` to enforce code quality:

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy --all-targets --all-features -- -D warnings
```

Configuration files:
- `rustfmt.toml` - Code formatting rules
- `clippy.toml` - Linting configuration

### Documentation

- Add doc comments (`///`) to all public items
- Include examples in doc comments when helpful
- Update `CLAUDE.md` for significant architectural changes

Example:

```rust
/// Extracts the first URL from the input string.
///
/// # Arguments
///
/// * `s` - The input string to search for URLs
///
/// # Returns
///
/// An `Option<String>` containing the first URL found, or `None` if no URL exists
///
/// # Examples
///
/// ```
/// use url::url::extract_url;
///
/// let text = "Visit https://example.com today";
/// assert_eq!(extract_url(text), Some("https://example.com".to_string()));
/// ```
pub fn extract_url(s: &str) -> Option<String> {
    // implementation
}
```

### Code Organization

#### Binary Structure

Place binaries in `src/bin/`:

```rust
// src/bin/my-udf.rs
use anyhow::Result;
use shared::io::{process_stdin, ProcessFn};

fn my_processing_fn(input: &str) -> Option<String> {
    // Your logic here
    Some(input.to_uppercase())
}

fn main() -> Result<()> {
    process_stdin(Box::new(my_processing_fn));
    Ok(())
}
```

#### Library Structure

Keep business logic in `src/lib.rs` or separate modules:

```rust
// src/lib.rs
//! Brief description of the package

pub mod my_module;

// Business logic here
pub fn process_data(input: &str) -> Option<String> {
    // implementation
}
```

### Error Handling

- Use `Result<T, E>` for operations that can fail
- Use `Option<T>` for values that may or may not exist
- Log errors to stderr with context
- Never use `unwrap()` or `expect()` in production code without justification

## Testing

### Unit Tests

Place tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        assert_eq!(process_data("input"), Some("output".to_string()));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(process_data(""), None);
    }
}
```

### Integration Tests

Integration tests are in `.github/workflows/clickhouse-integration.yaml` and test against a live ClickHouse server.

### Pre-commit Hooks

Install pre-commit hooks for automatic checks:

```bash
pip install pre-commit
pre-commit install
```

The hooks run:
- `cargo fmt` - Code formatting
- `cargo clippy` - Linting
- `cargo test` - Tests (on push)

## Submitting Changes

### Pull Request Checklist

- [ ] Code follows the style guide
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] All tests pass (`cargo test`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)

### Pull Request Template

```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Performance improvement
- [ ] Documentation update
- [ ] Refactoring

## Testing
Describe testing performed

## Checklist
- [ ] Tests pass locally
- [ ] New tests added
- [ ] Documentation updated
```

## Release Process

Releases are automated through GitHub Actions. To create a release:

1. Update version in affected `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
4. Push tag: `git push origin v0.1.0`
5. GitHub Actions will build and publish releases

## Additional Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [ClickHouse UDF Documentation](https://clickhouse.com/docs/en/sql-reference/functions/external-user-defined-functions)
