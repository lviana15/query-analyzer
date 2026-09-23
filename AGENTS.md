# AGENTS.md

Guidance for agentic coding assistants working in this repository.

## Project Snapshot

`redshift` is a Rust CLI + library that statically analyzes MongoDB query
usage in TypeScript/NestJS codebases. It parses TS/TSX source files via SWC
AST visitors, extracts query calls/collections/fields, aggregates usage,
emits config-based warnings, and suggests indexes.

High-value files:
- `src/ast_parser.rs` — SWC parsing, `Visit` implementation, field extraction.
- `src/lib.rs` — file traversal, query aggregation, index suggestions, config warnings.
- `src/config.rs` — `redshift.config.json` schema, loading, validation.
- `src/main.rs` — `clap` CLI entrypoint (`analyze`, `indexes` subcommands).

## Build, Check, Lint, and Test Commands

Use Cargo only (no Makefile/Justfile).

```bash
cargo check                        # fast compile check
cargo build                        # debug build
cargo build --release              # release build
cargo fmt                          # format (run before commit)
cargo clippy -- -D warnings        # lint strictly
cargo test                         # run all tests
```

### Running a Single Test

```bash
cargo test <substring>                              # match test name substring
cargo test <substring> -- --nocapture               # with stdout/stderr output
cargo test --lib <substring>                        # library tests only
cargo test -- --exact config::tests::parses_basic_config  # exact full path
```

Test modules live inside the source files under `#[cfg(test)] mod tests`.
Current test locations: `src/lib.rs` (3 tests), `src/config.rs` (2 tests).

### CLI Execution

```bash
cargo run -- analyze               # analyze current directory
cargo run -- analyze -v            # verbose (show per-query lines)
cargo run -- indexes               # suggest indexes
cargo run -- -d /path/to/project analyze  # specific directory
```

## Code Style and Conventions

### Imports

- Group: `std` first, then third-party crates, then `crate::` / `super::`.
- Use explicit imports; avoid globs unless already established (e.g. `swc_core::ecma::ast::*`).
- Remove unused imports.

### Formatting

- Follow `cargo fmt` output; do not hand-format.
- Prefer early returns and `let ... else` over deeply nested `if` blocks.
- Use trailing commas in multiline literals and argument lists.
- Keep functions focused; split when a function handles multiple concerns.

### Naming

- Functions, variables, modules: `snake_case`.
- Structs, enums, traits: `PascalCase`.
- Constants, statics: `SCREAMING_SNAKE_CASE`.
- Names should reflect the MongoDB query analysis domain.

### Types and Modeling

- Prefer structs/enums over loosely-typed string maps.
- Config types use `#[serde(rename_all = "camelCase")]` for JSON field mapping.
- Preserve stable output ordering — sort vectors deterministically before display.
- Keep public types in `src/lib.rs` coherent with CLI display behavior.

### Error Handling

- Top-level APIs return `Result<_, Box<dyn std::error::Error>>`.
- Propagate errors with `?` when the operation should abort.
- For per-file read/parse issues, log with `eprintln!` and continue.
- Avoid panics; avoid `unwrap` unless the invariant is guaranteed.

### Ownership and Cloning

- Avoid unnecessary `.clone()` in hot paths.
- Prefer borrowing (`&str`, `&T`) over owned allocations in helper functions.
- Clone AST/query data only when needed for borrowing/lifetime clarity.

### Collections and Determinism

- Never depend on `HashMap` iteration order in user-facing output.
- Convert grouped data to `Vec`, sort, then print/return.
- Sort keys: file → line → method for raw query lists.
- Deduplicate where repeated AST matches can occur.

## AST Parser Rules (SWC)

- All AST extraction logic belongs in `src/ast_parser.rs`.
- Use the SWC `Visit` trait; always call `visit_children_with` in visitor methods.
- `QUERY_METHODS` is the canonical list of supported MongoDB methods.
- Collection resolution is centralized in `resolve_collection`.
- `InjectModel` decorator mapping drives `this.<prop>` → collection resolution.
- For `$` operators, recurse into nested objects/arrays to extract real fields.
- For array arguments (e.g. aggregation pipeline stages), merge discovered fields.
- Skip unknown AST shapes safely — never panic.

## File Traversal Rules

- Use `walkdir::WalkDir` with `filter_entry`.
- Ignore: `node_modules`, `.git`, `dist`, `target`.
- Process only `.ts` and `.tsx` files.
- Exclude test files (`*.spec.ts`, `*.test.ts`).

## Configuration System

- Config file: `redshift.config.json` (discovered by walking upward from project dir).
- Schema defined in `src/config.rs` with serde deserialization (`camelCase` JSON keys).
- Validation enforces: non-empty collection names, no duplicates, valid index keys.
- `predicateGuidance` supports per-method field recommendations with configurable severity.

## Testing Guidance

- Place unit tests inside source files under `#[cfg(test)]` modules.
- Use inline TypeScript snippets for parser behavior tests.
- Assert full `MongoQuery` semantics: collection, method, fields, line, raw match.
- Add tests for new query methods, resolver changes, and edge AST patterns.
- Keep tests deterministic — sort fields and use order-sensitive assertions.

## CLI and Output

- CLI uses `clap` derive API with `analyze` and `indexes` subcommands.
- `analyze -v` prints per-query line numbers and raw matches.
- Keep output stable and human-readable; avoid accidental reordering.
- Print file names (not full paths) where context allows.

## Performance and Safety

- Avoid expensive rescans or repeated parsing.
- Keep visitor recursion bounded and defensive.
- Prefer linear passes and lightweight allocations in parsing hot paths.
- Handle parse failures gracefully — return empty module/query sets.

## Agent Working Notes

- Preserve existing behavior and output ordering unless asked otherwise.
- Avoid large refactors without explicit user request.
- Keep changes targeted and easy to review.
- If you introduce new behavior, update tests and this guide as needed.
- No Cursor rules or Copilot instructions exist in this repository.
