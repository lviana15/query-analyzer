# Roadmap: Static Analysis Extensions for redshift

This document lists candidate features that are possible using static analysis
of server-side code only, plus explicit limitations. It also includes relevant
MongoDB guidance that informs lint rules and heuristics.

## Goals

- Provide deeper insight into query usage without runtime data.
- Enforce organization-specific query patterns via configuration.
- Surface likely performance risks using documented MongoDB guidance.

## Static Analysis Capabilities (Today)

- Extract MongoDB query method usage and fields per collection.
- Aggregate query field usage and suggest indexes heuristically.
- Provide file/line references for query sites.

## Roadmap: Static-Only Features

### Phase 1 (High value, low risk)

- Query shape inventory: normalize filter/sort/projection into stable
  identifiers per collection to dedupe patterns and track drift.
- Index advisor v2: suggest compound index order using equality/sort/range
  heuristics and warn when sort keys do not align with filters.
  https://www.mongodb.com/docs/manual/core/query-optimization/
- Aggregation lint rules:
  - `$match` should appear early when it does not depend on computed fields.
  - `$project` should appear late; early `$project` rarely improves performance.
  - `$sort` followed by `$match` can be re-ordered by the optimizer; warn when
    large sort appears before filtering.
  - `$sort` + `$limit` coalescing opportunities (top-k pipelines).
  - `$lookup` + `$unwind` + `$match` on the lookup result can be merged.
  https://www.mongodb.com/docs/manual/core/aggregation-pipeline-optimization/
- Anti-pattern detection:
  - `$where` usage (deprecated server-side JS in MongoDB 8.0; no index use).
    https://www.mongodb.com/docs/manual/reference/operator/query/where/
  - `$regex` without anchors or case-insensitive regex on indexed fields.
    https://www.mongodb.com/docs/manual/reference/operator/query/regex/
  - Unbounded list queries (no `limit()` or no stable sort for pagination).
- Projection hygiene:
  - Find queries that return full documents where a projection could be used.
  - Highlight candidates for covered queries (projection + index overlap).
  https://www.mongodb.com/docs/manual/core/query-optimization/

### Phase 2 (Configuration-driven validation)

- Query contract checking (config file):
  - Allowed filter fields per collection.
  - Allowed sort keys and max sort depth.
  - Required projection rules for sensitive collections.
  - Allowed aggregation stages/order per pipeline family.
- Policy checks:
  - Enforce `limit()` for public list endpoints.
  - Enforce stable sorts (e.g., include `_id` as a tie-breaker).
- Mongoose/NestJS conventions:
  - Validate model naming -> collection mapping consistency.
  - Detect `collection()`/`useDb()` patterns that bypass models.

### Phase 3 (Nice-to-have)

- Explain plan helpers:
  - Generate `explain()` commands per query shape to guide runtime analysis.
  - Highlight which queries should be checked for COLLSCAN vs IXSCAN.
  https://www.mongodb.com/docs/manual/reference/explain-results/
- Index impact simulation:
  - Heuristics for when single vs compound indexes likely help.
  - Flag sorts without supporting indexes (based on query shape).
  https://www.mongodb.com/docs/manual/indexes/

## Optional Inputs to Improve Precision

- Collection schema (JSON schema or Mongoose model metadata).
- Existing index definitions (from code or exported metadata).
- Expected cardinality or selectivity hints (config-provided).
- Route metadata (to detect list endpoints and pagination behavior).

## Static Analysis Limitations (Explicitly Not Possible)

- Cannot know actual query runtime or COLLSCAN/IXSCAN outcomes without
  executing `explain()` on a live dataset. Explain results also ignore the
  plan cache, so they differ from steady-state behavior.
  https://www.mongodb.com/docs/manual/reference/explain-results/
- Cannot infer data distribution, cardinality, or selectivity from code alone.
- Cannot detect runtime-built queries that are highly dynamic or string-built
  beyond conservative heuristics.
- Cannot validate sharding behavior, cluster topology, or resource limits.
- Cannot measure real performance impacts of indexes on write-heavy workloads.
  https://www.mongodb.com/docs/manual/indexes/

## Suggested Config Schema (Draft)

```yaml
collections:
  users:
    allowed_filters: ["email", "status", "orgId", "createdAt"]
    allowed_sorts: ["createdAt", "_id"]
    require_projection: true
    forbid_regex: true
  orders:
    allowed_filters: ["status", "customerId", "createdAt"]
    allowed_sorts: ["createdAt", "_id"]
rules:
  require_limit_for_lists: true
  require_stable_sort: true
  forbid_where: true
  max_in_list_items: 200
```

## References

- Query optimization guidance and covered queries.
  https://www.mongodb.com/docs/manual/core/query-optimization/
- Index concepts and trade-offs.
  https://www.mongodb.com/docs/manual/indexes/
- Aggregation pipeline optimization rules.
  https://www.mongodb.com/docs/manual/core/aggregation-pipeline-optimization/
- Explain plan fields and limitations.
  https://www.mongodb.com/docs/manual/reference/explain-results/
- `$regex` performance behavior.
  https://www.mongodb.com/docs/manual/reference/operator/query/regex/
- `$where` limitations and deprecation note.
  https://www.mongodb.com/docs/manual/reference/operator/query/where/
