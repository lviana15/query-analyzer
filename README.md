# Redshift

Tool designed to statically analyze MongoDB query usage in TypeScript and NestJS projects.
It parses source code to identify query patterns, extracted fields, and index usage without running the application.

## How to Execute

Prerequisites: Rust and Cargo installed.

```bash
# Build release version
cargo build --release

# Run analysis on the current directory
cargo run -- analyze

# Or use the binary directly
redshift analyze

# Generate index suggestions based on query usage
cargo run -- indexes

# Run analysis on a specific project directory
cargo run -- -d /path/to/project analyze
```

## Configuration

Create a `redshift.config.json` file in your project root.
The analyzer resolves configuration by walking upward from the analyzed directory
until it finds `redshift.config.json`.

Example:

```json
{
  "defaults": {
    "unknownCollectionSeverity": "warning",
    "recommendedPredicateMissSeverity": "warning"
  },
  "collections": [
    {
      "name": "users",
      "indexes": [
        {
          "keys": [{ "field": "email", "order": 1 }],
          "unique": true
        }
      ],
      "predicateGuidance": {
        "recommendedFields": ["organizationId", "tenantId"],
        "methods": {
          "find": ["organizationId", "tenantId"]
        }
      }
    }
  ]
}
```
