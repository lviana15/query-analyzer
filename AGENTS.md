# AGENTS.md

This file contains guidelines for agentic coding agents working in the `mongo-analyzer` Rust repository.

## Project Overview

`mongo-analyzer` is a CLI tool and library for analyzing MongoDB query usage in TypeScript/NestJS projects.
- **Core Logic**: `src/ast_parser.rs` uses `swc_core` to parse TypeScript ASTs and extract queries.
- **Integration**: `src/lib.rs` coordinates file discovery and configuration.
- **CLI**: `src/main.rs` handles arguments via `clap`.
- **Domain**: Parses `.ts` files to find `find`, `aggregate`, etc., and extracts field usage using AST traversal.

## Build, Test, and Lint Commands

### Essential Commands
```bash
# Build the project
cargo build
cargo build --release

# Check code without building (fast)
cargo check

# Run all tests
cargo test

# Run a specific test by name (IMPORTANT: use this for focused development)
cargo test test_mongo_repository_file_discovery

# Run tests in a specific file
cargo test --test mongo_analyzer_fixes

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
- **Layout**: Standard Cargo layout (`src/` for logic, `tests/` for integration).
- **Modules**: `src/ast_parser.rs` encapsulates all `swc` logic. `src/lib.rs` handles orchestration.
- **Imports**: Group imports: `std` first, then external crates (`serde`, `swc_core`), then `crate::`.

### Naming Conventions
- **Functions/Vars**: `snake_case` (e.g., `find_mongo_queries`)
- **Types**: `PascalCase` (e.g., `ProjectConfig`, `MongoQuery`)
- **Constants**: `SCREAMING_SNAKE_CASE`

### Error Handling
- Use `Result<T, Box<dyn std::error::Error>>` for top-level CLI functions.
- Use `?` operator for propagation.
- For `swc` string atoms, handle `Option<&str>` returns gracefully (e.g., `unwrap_or_default()`).

### Formatting & Types
- **Line Length**: 100 chars (standard rustfmt).
- **Type Safety**: Use strong types. Avoid `String` typing for things that should be Enums if possible.
- **Serde**: Derive `Serialize, Deserialize` for config and output structs.

## Testing Strategy

### Integration Tests (`tests/`)
- Tests in `tests/*.rs` are integration tests.
- **Note**: Some existing tests might reference absolute paths. When writing new tests, **use relative paths** or create temp directories/fixtures.
- **Mocking**: Use `ast_parser::parse_file` with string constants containing TypeScript code in the test function itself.

### Unit Tests (`src/`)
- Place unit tests in a `#[cfg(test)] mod tests { ... }` block at the bottom of the file they test.
- Verify AST extraction logic by creating small TS snippets and asserting on `MongoQuery` fields.

## Development Workflow
1.  **Analyze**: Understand `src/ast_parser.rs` and the `MongoQueryVisitor` struct.
2.  **Plan**: If adding a new MongoDB method, update `is_query_method` and `visit_call_expr`.
3.  **Test**: Write a failing test case in `tests/mongo_analyzer_fixes.rs`.
4.  **Implement**: specific AST visitor changes.
5.  **Verify**: Run `cargo test` and `cargo clippy`.

## Common Patterns

### AST Parsing (SWC)
We use `swc_core` and `swc_ecma_parser` to parse code.
- **Visitor Pattern**: Implement `Visit` trait for `MongoQueryVisitor`.
- **Method Detection**: `visit_call_expr` detects method calls.
- **Mongoose Support**: `visit_constructor` scans for `@InjectModel` to map properties to collections.
- **Field Extraction**: Recursively traverse `ObjectLit` nodes, descending into operators like `$or` or `$set` to find leaf fields.

```rust
// Example Visitor method
fn visit_call_expr(&mut self, n: &CallExpr) {
    n.visit_children_with(self); // Visit args first
    if let Some((coll, method)) = self.analyze_callee(&n.callee) {
        // Extract fields logic...
    }
}
```

### Config Pattern
Configuration uses `ProjectConfig` struct. Default impl provides standard NestJS service patterns.
```rust
impl Default for ProjectConfig {
    fn default() -> Self {
        // ... default services map
    }
}
```
