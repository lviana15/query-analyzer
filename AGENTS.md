# AGENTS.md

Guidance for agentic coding assistants working in this repository.

## Project Snapshot
`redshift` is a Rust CLI + library that statically analyzes MongoDB query
usage in TypeScript/NestJS codebases.
Core responsibilities:
- Parse TS/TSX source with SWC AST visitors.
- Extract query method calls, collections, fields, and source line numbers.
- Aggregate usage and suggest indexes.
High-value files:
- `src/ast_parser.rs`: SWC parsing and AST traversal (`Visit` implementation).
- `src/lib.rs`: file traversal, query aggregation, index suggestions.
- `src/main.rs`: `clap` CLI entrypoint and output rendering.
- `README.md`: quick run examples.

## Build, Check, Lint, and Test Commands
Use Cargo only (no Makefile/Justfile tasks).
```bash
# Fast compile checks
cargo check

# Build debug/release binaries
cargo build
cargo build --release

# Format code (run before commit)
cargo fmt

# Lint strictly (warnings denied)
cargo clippy -- -D warnings

# Run all tests
cargo test
```
Single-test workflows (important):
```bash
# Run tests matching a substring
cargo test <test_name_substring>

# Run an exact module::path style test
cargo test ast_parser::tests::parses_find_query

# Run a single test and print output
cargo test <test_name_substring> -- --nocapture

# Restrict to library tests only
cargo test --lib <test_name_substring>

# Run one exact test by full name without fuzzy matching
cargo test -- --exact ast_parser::tests::parses_find_query
```
CLI execution examples:
```bash
# Analyze current directory
cargo run -- analyze

# Suggest indexes for current directory
cargo run -- indexes

# Analyze a specific project directory
cargo run -- -d /path/to/project analyze
```

## Code Style and Conventions
### Imports
- Group imports in this order: `std`, third-party crates, then `crate::...`.
- Prefer explicit imports; avoid glob imports unless already established.
- Keep imports minimal and remove unused entries.
### Formatting
- Follow `cargo fmt` output; do not hand-format against formatter style.
- Keep functions focused; split when a function handles mixed concerns.
- Prefer early returns/`let ... else` to reduce nested `if` blocks.
- Use trailing commas in multiline literals and argument lists.
### Naming
- Functions/variables/modules: `snake_case`.
- Structs/enums/traits: `PascalCase`.
- Constants/statics: `SCREAMING_SNAKE_CASE`.
- Choose descriptive names tied to Mongo query analysis domain.
### Types and Modeling
- Prefer structs/enums over loosely typed string maps where practical.
- Preserve stable output ordering by sorting vectors deterministically.
- Use `Vec<T>` for ordered output and document ordering assumptions.
- Keep public types in `src/lib.rs` coherent with CLI display behavior.
### Error Handling
- Top-level APIs should return `Result<_, Box<dyn std::error::Error>>`.
- Propagate failures with `?` when the operation should abort.
- For per-file read/parse issues, log warnings (`eprintln!`) and continue.
- Avoid panics in normal flow; avoid `unwrap` unless invariant is guaranteed.
### Ownership and Cloning
- Avoid unnecessary cloning in hot paths.
- Clone AST/query data only when required for borrowing/lifetime clarity.
- Prefer borrowing (`&str`, `&T`) over owned allocations in helper APIs.
### Collections and Determinism
- Avoid depending on `HashMap` iteration order in user-facing output.
- Convert map/grouped data into vectors and sort before printing/returning.
- Keep sort keys stable (currently file, line, method for raw query list).
- Deduplicate where repeated AST matches can occur.

## AST Parser Rules (SWC)
- Keep AST extraction logic in `src/ast_parser.rs`.
- Use SWC visitor pattern (`Visit`) and call `visit_children_with`.
- Maintain `QUERY_METHODS` as the canonical supported method list.
- Keep collection resolution logic centralized in `resolve_collection`.
- Preserve decorator-based model mapping (`InjectModel`) behavior.
- Continue supporting identifier-backed local object predicates.
- For `$` operators, recurse into nested objects/arrays to find real fields.
- For array query args (e.g., aggregation stages), merge discovered fields.
- Skip unknown AST shapes safely instead of panicking.

## File Traversal Rules
- Traverse with `walkdir::WalkDir` + `filter_entry`.
- Ignore heavy/unrelated directories: `node_modules`, `.git`, `dist`, `target`.
- Only process `.ts` and `.tsx` files.
- Exclude test files (`*.spec.ts`, `*.test.ts`) from analysis.
- Keep traversal efficient; avoid extra filesystem passes when possible.

## Testing Guidance
- Place unit tests near implementation under `#[cfg(test)]` modules.
- Prefer inline TypeScript snippets for parser behavior tests.
- Assert full `MongoQuery` semantics: collection, method, fields, line, raw match.
- Add tests for new query methods, resolver changes, and edge AST patterns.
- Keep tests deterministic (sorted fields/order-sensitive expectations).

## CLI and Output Guidelines
- CLI is built with `clap` and exposes `analyze` and `indexes` subcommands.
- Keep output stable and human-readable; avoid accidental reordering.
- When printing files, prefer file names where context allows.
- Preserve current command semantics unless change is explicitly requested.

## Performance and Safety
- Avoid expensive rescans or repeated parsing of identical inputs.
- Keep visitor recursion bounded and defensive.
- Prefer linear passes and lightweight allocations in parsing hot paths.
- Handle parse failures gracefully by returning empty module/query sets.

## Repository Rule Files (Cursor/Copilot)
Checked locations:
- `.cursor/rules/`
- `.cursorrules`
- `.github/copilot-instructions.md`
Result: none of these files exist in this repository at time of writing.

## Reference Docs
See `docs/` for MongoDB syntax/domain references:
- `docs/QUERY_SYNTAX.md`
- `docs/AGGREGATION_SYNTAX.md`
- `docs/CRUD.md`
- `docs/PERFORMANCE.md`
- `docs/GLOSSARY.md`

## Agent Working Notes
- Preserve existing behavior and output ordering unless asked otherwise.
- Avoid large refactors without explicit user request.
- Keep changes targeted and easy to review.
- If you introduce new behavior, update tests and this guide as needed.
