# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-02-02

### Added
- Initial release of MongoDB query analyzer
- CLI interface with analyze, fields, methods, and indexes subcommands
- Support for multiple MongoDB query patterns:
  - Direct collection access: `collection.find()`
  - Mongoose models: `User.find()`
  - Connection-based: `connection.useDb().collection().find()`
  - Class properties: `this.collection.find()`
  - Repository patterns: `repository.find()`
- Service-based analysis with configurable detection
- Field extraction from query objects
- Method usage tracking
- Index suggestions based on field usage frequency
- Comprehensive terminal reporting with emoji indicators
- JSON configuration support for service definitions

### Features
- Analyze entire projects or specific services
- Extract field usage patterns across collections
- Generate method usage statistics
- Provide optimization recommendations
- Support for TypeScript/TSX files
- Exclude test files and patterns from analysis