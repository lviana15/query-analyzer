# AGENTS.md

Guidelines for agentic coding agents working in the `mongo-analyzer` Rust repo.

## Project Overview

`mongo-analyzer` is a CLI tool and library for analyzing MongoDB query usage in
TypeScript/NestJS projects. It parses TypeScript ASTs with SWC to extract query
methods, fields, and usage patterns.

Key locations:
- `src/ast_parser.rs` parses TypeScript ASTs (SWC) and collects `MongoQuery`.
- `src/lib.rs` orchestrates file traversal and aggregation helpers.
- `src/main.rs` implements the CLI using `clap`.

## Build, Lint, and Test Commands

Essential commands:
```bash
# Build
cargo build
cargo build --release

# Fast type-check
cargo check

# Format
cargo fmt

# Lint (treat warnings as errors)
cargo clippy -- -D warnings

# Run all tests
cargo test
```

Run a single test:
```bash
# By test name substring (unit tests)
cargo test <test_name>

# More precise: module path (example)
cargo test ast_parser::tests::parses_find_query

# Show stdout/stderr from tests
cargo test <test_name> -- --nocapture

# Only library tests
cargo test --lib <test_name>
```

Notes:
- There are no custom Makefile/Justfile tasks.
- Use `cargo fmt` before committing changes.

## Code Style Guidelines

### Imports
- Group imports in this order: `std`, external crates, then `crate::`.
- Prefer explicit imports over glob re-exports to keep call sites clear.

### Formatting
- Default Rust style (`cargo fmt`) with ~100 char line width.
- Use trailing commas in multi-line structs, enums, and function args.
- Prefer early returns to reduce nesting.

### Naming Conventions
- Functions/vars: `snake_case` (e.g., `find_mongo_queries`).
- Types/traits: `PascalCase` (e.g., `MongoQuery`, `IndexSuggestion`).
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `QUERY_METHODS`).

### Types and Data Modeling
- Prefer strong types and enums over stringly-typed maps.
- Use `Vec<T>` for ordered output and sort deterministically before printing.
- Implement `Display` for user-facing output (see `IndexSuggestion`).

### Error Handling
- Top-level functions return `Result<T, Box<dyn std::error::Error>>`.
- Use `?` for propagation when failing should stop the command.
- For per-file processing, log with `eprintln!` and continue.
- Avoid panics; only `unwrap` when invariants are guaranteed.

### Collections and Sorting
- Deduplicate and sort results for stable output.
- In `src/lib.rs`, sort by file, line, and method.
- When aggregating, sort maps into vectors before returning or printing.

### Control Flow
- Prefer `match` / `if let` over nested `if` chains.
- Keep AST visitor logic in `MongoQueryVisitor` and delegate helpers.

## AST Parsing Conventions (SWC)

- Use the SWC visitor pattern (`Visit`) and call `visit_children_with`.
- Keep AST extraction and analysis in `src/ast_parser.rs`.
- Track local object literals to resolve identifier-based queries.
- Treat `$`-prefixed keys as operators and drill into nested objects.
- When a query arg is an array of objects, merge extracted fields.

Relevant helpers:
- `QUERY_METHODS` lists supported MongoDB methods.
- `get_injected_model_name` handles NestJS `@InjectModel`.
- `resolve_collection` follows `collection()`, `useDb()`, or model naming.

## File Traversal

- Use `walkdir::WalkDir` with `filter_entry` to avoid large directories.
- Ignore `node_modules`, `.git`, `dist`, and `target`.
- Only scan `.ts` and `.tsx` files.
- Exclude test files (`*.spec.ts`, `*.test.ts`).

## Testing Strategy

- Unit tests live in the same module under `#[cfg(test)]`.
- Use inline TypeScript snippets to test AST parsing.
- Assert on `MongoQuery` fields: collection, method, fields, line.
- Add tests when introducing new query methods or collection resolvers.

## CLI Conventions

- CLI uses `clap` with subcommands (`analyze`, `indexes`).
- Keep output readable and stable; avoid reordering without intent.
- For printing paths, prefer the file name when possible (see `main.rs`).

CLI usage examples:
```bash
# Analyze queries in current directory
mongo-analyzer analyze

# Analyze queries in a specific project
mongo-analyzer --directory /path/to/project analyze
```

## Output and Determinism

- Keep ordering stable to make diffs easy to review.
- Prefer deterministic sorting over HashMap iteration order.
- Avoid adding noisy debug output to CLI paths.
- Use clear, human-readable wording in `Display` implementations.

## Performance and Safety

- Avoid loading large directories by extending `IGNORED_DIRS` only when needed.
- Reuse parsed data rather than re-walking the filesystem.
- Keep recursion in AST visitors bounded and defensive.
- When in doubt, skip unrecognized AST shapes instead of panicking.

## Reference Documentation

For detailed MongoDB syntax and concepts, refer to the following helper files in the `docs/` directory:

- `docs/QUERY_SYNTAX.md`: Common query operators ($eq, $gt, $in) and logical operators.
- `docs/AGGREGATION_SYNTAX.md`: Aggregation pipeline stages ($match, $group, $lookup) and accumulators.
- `docs/CRUD.md`: High-level overview of Create, Read, Update, Delete operations.
- `docs/PERFORMANCE.md`: Performance tuning, indexes, and metrics.
- `docs/GLOSSARY.md`: Definitions of MongoDB terms.

## Cursor / Copilot Rules

- No `.cursor/rules/`, `.cursorrules`, or `.github/copilot-instructions.md`
  were found in the repo at the time of writing.

## Agent Notes

- Preserve existing logic and ordering when making changes.
- Avoid large refactors unless requested.
- Keep functions small and focused, especially in the AST visitor.
