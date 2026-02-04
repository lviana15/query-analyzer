# AGENTS.md

This file contains guidelines for agentic coding agents working in the `mongo-analyzer` Rust repository.

## Project Overview

`mongo-analyzer` is a CLI tool and library for analyzing MongoDB query usage in TypeScript/NestJS projects.
- **Core Logic**: `src/ast_parser.rs` uses `swc_core` to parse TypeScript ASTs and extract queries.
- **Analysis**: `src/lib.rs` traverses files and aggregates usage statistics (indexes, fields).
- **CLI**: `src/main.rs` handles arguments via `clap`.
- **Domain**: Parses `.ts` files to find `find`, `aggregate`, etc., and extracts field usage.

## Build, Test, and Lint Commands

### Essential Commands
```bash
# Build the project
cargo build
cargo build --release

# Check code without building (fast)
cargo check

# Run all tests (Unit tests in src/)
cargo test

# Run a specific test by name
# Example: cargo test tests::test_ast_parsing
cargo test <test_name>

# Run tests with output (don't capture stdout)
cargo test -- --nocapture
```

### Code Quality
```bash
# Format code (run before commit)
cargo fmt

# Run Clippy lints (fix warnings)
cargo clippy -- -D warnings
```

## Code Style Guidelines

### General Structure
- **Layout**: Standard Cargo layout. Logic in `src/lib.rs`, parsing in `src/ast_parser.rs`, CLI in `src/main.rs`.
- **Modules**: Keep `ast_parser` focused on SWC logic. `lib.rs` handles file traversal and high-level analysis.
- **Imports**: Group imports: `std` first, then external crates (`walkdir`, `swc_core`, `clap`), then `crate::`.

### Naming Conventions
- **Functions/Vars**: `snake_case` (e.g., `find_mongo_queries`)
- **Types**: `PascalCase` (e.g., `MongoQuery`, `IndexSuggestion`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `IGNORED_DIRS`)

### Error Handling
- Use `Result<T, Box<dyn std::error::Error>>` for top-level functions (`analyze_project`).
- Use `?` operator for propagation.
- **Resilience**: For file operations in loops (e.g., reading multiple files), handle errors gracefully (log via `eprintln!` and continue) instead of crashing the entire process.
- **SWC**: Handle `Option<&str>` returns gracefully (e.g., `unwrap_or_default()`).

### Formatting & Types
- **Line Length**: 100 chars (standard rustfmt).
- **Type Safety**: Use strong types. Prefer Enums (like `IndexSuggestion`) over "stringly typed" Maps for logic. Implement `Display` for user-facing output.
- **Performance**: Use `WalkDir` with `filter_entry` to skip huge directories (`node_modules`) efficiently.

## Testing Strategy

### Unit Tests (`src/`)
- Place unit tests in a `#[cfg(test)] mod tests { ... }` block at the bottom of the file they test.
- **Mocking**: Verify AST extraction logic by calling `ast_parser::parse_file` with string constants containing TypeScript code.
- **Assertions**: Assert on `MongoQuery` fields (collection, method, fields).

## Development Workflow
1.  **Analyze**: Understand `src/ast_parser.rs` and the `MongoQueryVisitor` struct.
2.  **Plan**: If adding a new MongoDB method, update `is_query_method` and `visit_call_expr`.
3.  **Test**: Write a failing unit test case with a sample TS code snippet.
4.  **Implement**: specific AST visitor changes.
5.  **Verify**: Run `cargo test` and `cargo clippy`.

## Common Patterns

### AST Parsing (SWC)
We use `swc_core` and `swc_ecma_parser`.
- **Visitor Pattern**: Implement `Visit` trait for `MongoQueryVisitor`.
- **Method Detection**: `visit_call_expr` detects method calls.
- **Mongoose Support**: `visit_constructor` scans for `@InjectModel`.
- **Field Extraction**: Recursively traverse `ObjectLit` nodes.

### File Traversal (WalkDir)
Use `walkdir::WalkDir` with `filter_entry` to explicitly ignore `node_modules` before recursion to avoid performance bottlenecks.
```rust
WalkDir::new(dir).into_iter().filter_entry(|e| !is_ignored(e))
```
