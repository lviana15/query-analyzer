# MongoDB Query Analyzer

Tool designed to statically analyze MongoDB query usage in TypeScript and NestJS projects.
It parses source code to identify query patterns, extraction fields, and index usage without running the application.

## How to Execute

Prerequisites: Rust and Cargo installed.

```bash
# Build release version
cargo build --release

# Run analysis on the current directory
cargo run -- analyze

# Generate index suggestions based on query usage
cargo run -- indexes

# Run analysis on a specific project directory
cargo run -- -d /path/to/project analyze
```
