# MongoDB Query Analyzer

A Rust CLI tool to analyze MongoDB queries in TypeScript/NestJS projects.

## Features

- 🔍 **Query Pattern Detection**: Detects various MongoDB query patterns including:
  - `collection.find()`, `collection.findOne()`, `collection.aggregate()`
  - `Model.find()`, `Model.findOne()` (Mongoose patterns)
  - `connection.useDb().collection().find()`
  - `this.collection.find()` and repository patterns

- 📊 **Comprehensive Analysis**:
  - Service-based query organization
  - Method usage tracking
  - Field extraction from query objects
  - Index suggestions based on field usage frequency

- 🎯 **CLI Interface**:
  - Analyze entire projects or specific services
  - Extract field usage patterns
  - Get method usage statistics
  - Generate optimization recommendations

## Installation

```bash
cargo build --release
```

## Usage

```bash
# Analyze entire project
cargo run -- analyze

# Analyze specific service
cargo run -- analyze --service user-service

# Analyze specific directory
cargo run -- -d path/to/project analyze

# Get field analysis
cargo run -- -d examples fields --service user-service

# Get method usage
cargo run -- -d examples methods

# Get index suggestions
cargo run -- -d examples indexes
```

## Configuration

The tool supports JSON configuration for service definitions:

```json
{
  "services": {
    "user-service": {
      "name": "user-service",
      "patterns": ["user.*\\.ts", "auth.*\\.ts"],
      "excluded_files": ["*.spec.ts", "*.test.ts"]
    }
  }
}
```

## Example Output

```
🔍 MongoDB Query Analysis Results
=====================================

📦 Service: user-service
   Queries found: 12

   📄 examples/user-service/user.service.ts:10 (users)
      Method: findOne
      Fields: email, isActive
   
   📄 examples/user-service/user.service.ts:17 (users)
      Method: find
      Fields: isActive, lastLogin

📊 Summary:
   Total queries: 19
   Unique collections: 3
   Services using MongoDB: 2
```

## Development

```bash
# Build
cargo build

# Test
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Project Structure

- `src/main.rs` - CLI interface and argument parsing
- `src/lib.rs` - Core MongoDB analysis engine
- `examples/` - Sample TypeScript files for testing

## Supported Query Patterns

1. **Direct Collection Access**: `collection.find({...})`
2. **Mongoose Models**: `User.find({...})`
3. **Connection-based**: `connection.useDb('db').collection('users').find({...})`
4. **Class Properties**: `this.collection.find({...})`
5. **Repository Pattern**: `repository.find({...})`

## Future Enhancements

- Enhanced field extraction with nested object support
- Cross-service query pattern analysis
- Performance recommendations
- Integration with MongoDB Compass
- Export analysis results to JSON/CSV