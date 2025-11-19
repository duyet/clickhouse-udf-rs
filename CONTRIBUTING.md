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

- Rust 1.70+ (stable toolchain)
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

### Performance Considerations

- Avoid unnecessary allocations
- Use `&str` instead of `String` when possible
- Pre-allocate with capacity when building strings
- Use iterators instead of collecting intermediate vectors
- Profile with `cargo bench` before optimizing

## Testing

### Unit Tests

Add tests in the same file or in a `tests` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        assert_eq!(my_function("input"), Some("expected".to_string()));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(my_function(""), None);
        assert_eq!(my_function("special"), Some("result".to_string()));
    }
}
```

### Integration Tests

Place integration tests in `tests/`:

```bash
tests/
  integration_test.rs
```

### Benchmarks

Add benchmarks using Criterion:

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("my_function", |b| {
        b.iter(|| my_function(black_box("input")))
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

### Test Coverage

Aim for:
- 80%+ code coverage
- All public APIs tested
- Edge cases covered
- Error conditions tested

## Submitting Changes

### Pull Request Checklist

Before submitting a PR, ensure:

- [ ] Code builds without errors (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] CLAUDE.md is updated (if architecture changed)
- [ ] Commit messages follow Conventional Commits
- [ ] PR description explains the changes

### PR Description Template

```markdown
## Description
Brief description of changes

## Motivation
Why these changes are needed

## Changes
- Change 1
- Change 2

## Testing
How the changes were tested

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
```

## Release Process

Releases are managed by maintainers:

1. Update version in all `Cargo.toml` files
2. Update CHANGELOG.md
3. Create git tag: `git tag v0.x.x`
4. Push tag: `git push origin v0.x.x`
5. GitHub Actions will build and publish release

## Questions?

- Check existing issues and discussions
- Ask in pull request comments
- Open a new issue for bugs or feature requests

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing! 🎉
