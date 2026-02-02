# AGENTS.md

This file contains guidelines and commands for agentic coding agents working in this Rust repository.

## Project Overview

This is a Rust project using Cargo as the build system. The project is currently minimal with a basic "Hello, world!" application in `src/main.rs`.

## Build, Test, and Lint Commands

### Essential Commands
```bash
# Build the project
cargo build

# Build with optimizations (release mode)
cargo build --release

# Run the application
cargo run

# Check code without building (faster than build)
cargo check

# Run all tests
cargo test

# Run a specific test by name
cargo test test_name

# Run tests in a specific module
cargo test module_name

# Run tests with specific filter
cargo test -- filter_pattern

# Run tests with verbose output
cargo test -- --verbose

# Run tests without capturing output
cargo test -- --nocapture
```

### Code Quality and Formatting
```bash
# Format code according to Rust standards
cargo fmt

# Check formatting without applying changes
cargo fmt -- --check

# Run Clippy lints
cargo clippy

# Run Clippy with all pedantic checks
cargo clippy -- -D warnings -W clippy::pedantic

# Check code and run lints together
cargo clippy --all-targets --all-features
```

### Documentation
```bash
# Generate and view documentation
cargo doc --open

# Generate documentation for dependencies too
cargo doc --document-private-items
```

## Code Style Guidelines

### General Structure
- Use standard Rust project layout: `src/` for source code, `tests/` for integration tests
- Keep `main.rs` minimal for binary projects
- Organize code into modules within `src/` directory

### Naming Conventions
- Functions and variables: `snake_case`
- Types (structs, enums, traits): `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case` matching filename

### Import Organization
```rust
// Standard library imports first
use std::collections::HashMap;
use std::fs::File;

// External crate imports second
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

// Local module imports last
use crate::config::Config;
use crate::utils::helper_function;
```

### Code Formatting
- Use `rustfmt` for consistent formatting (run `cargo fmt` before commits)
- Maximum line length: 100 characters (default rustfmt setting)
- Use trailing commas in multi-line arrays/structs
- Prefer block comments (`/* */`) for documentation, line comments (`//`) for implementation notes

### Error Handling
- Use `Result<T, E>` for functions that can fail
- Prefer the `?` operator for error propagation
- Create custom error types using `thiserror` crate when needed
- Use `Option<T>` for values that may be absent

### Type Safety
- Enable `#![deny(missing_docs)]` for public APIs
- Use type annotations for function signatures
- Prefer explicit types over type inference in public APIs
- Use `#[derive(Debug)]` for types that need debugging

### Testing Guidelines
- Unit tests go in the same module using `#[cfg(test)]`
- Integration tests go in `tests/` directory
- Use descriptive test names with `test_` prefix
- Use `assert_eq!`, `assert_ne!`, `assert!` for assertions
- Test both success and error cases

### Performance Considerations
- Use `cargo clippy` to catch performance anti-patterns
- Prefer `Vec::with_capacity()` when size is known
- Use `&str` over `String` for function parameters when possible
- Consider using `#[inline]` for small, hot functions

### Dependencies Management
- Add dependencies with `cargo add crate_name`
- Specify versions precisely in Cargo.toml
- Keep dependencies minimal and well-maintained
- Check for security updates with `cargo audit`

## Git Workflow

### Before Committing
1. Run `cargo fmt` to format code
2. Run `cargo clippy` to check for lint issues
3. Run `cargo test` to ensure tests pass
4. Run `cargo check` for quick syntax validation

### Commit Message Style
- Use conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`, etc.
- Keep first line under 50 characters
- Provide detailed description when necessary

## Development Tools

### IDE Integration
- VS Code with rust-analyzer extension recommended
- Configure rust-analyzer to use workspace features
- Enable clippy and rustfmt integration

### Debugging
- Use `cargo run` for basic debugging
- Use `println!` for quick debugging
- Consider using `dbg!` macro for structured debugging output
- For complex debugging, use IDE integration or `gdb`/`lldb`

## Environment Setup

### Required Tools
- Rust 1.90.0+ (current: 1.90.0)
- Cargo 1.90.0+ (included with Rust)
- rustfmt 1.8.0+ (included with Rust)
- clippy 0.1.90+ (included with Rust)

### Installation
```bash
# Install Rust (includes Cargo, rustfmt, clippy)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update to latest stable
rustup update stable
```

## Testing Strategy

### Test Organization
```
src/
├── main.rs          # Main binary
├── lib.rs           # Library root (if applicable)
├── config.rs        # Config module with unit tests
└── utils.rs         # Utility functions with tests

tests/
├── integration_tests.rs  # Integration tests
└── fixtures/        # Test data if needed
```

### Running Specific Tests
```bash
# Run all tests in a specific file
cargo test --lib config

# Run a single test function
cargo test test_config_loading

# Run tests matching a pattern
cargo test config

# Run tests with specific features
cargo test --features "feature1,feature2"
```

## Common Patterns

### Result Error Handling Pattern
```rust
fn parse_config(path: &str) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)
        .map_err(ConfigError::IoError)?;
    
    toml::from_str(&content)
        .map_err(ConfigError::ParseError)
}
```

### Module Organization Pattern
```rust
// lib.rs
pub mod config;
pub mod utils;
pub mod errors;

pub use config::Config;
pub use errors::{Result, AppError};
```

This AGENTS.md file should be kept updated as the project evolves and new patterns are established.