# MongoDB Query Syntax Reference

This guide provides common patterns and syntax for MongoDB queries, useful for analyzing ASTs and generating code.

## Comparison Operators

| Operator | Description | Example |
|---|---|---|
| `$eq` | Matches values that are equal to a specified value. | `{ qty: { $eq: 20 } }` |
| `$gt` | Matches values that are greater than a specified value. | `{ qty: { $gt: 20 } }` |
| `$gte` | Matches values that are greater than or equal to a specified value. | `{ qty: { $gte: 20 } }` |
| `$lt` | Matches values that are less than a specified value. | `{ qty: { $lt: 20 } }` |
| `$lte` | Matches values that are less than or equal to a specified value. | `{ qty: { $lte: 20 } }` |
| `$ne` | Matches all values that are not equal to a specified value. | `{ qty: { $ne: 20 } }` |
| `$in` | Matches any of the values specified in an array. | `{ qty: { $in: [ 5, 15 ] } }` |
| `$nin` | Matches none of the values specified in an array. | `{ qty: { $nin: [ 5, 15 ] } }` |

## Logical Operators

| Operator | Description | Example |
|---|---|---|
| `$and` | Joins query clauses with a logical AND returns all documents that match the conditions of both clauses. | `{ $and: [ { price: { $ne: 1.99 } }, { price: { $exists: true } } ] }` |
| `$not` | Inverts the effect of a query expression and returns documents that do not match the query expression. | `{ price: { $not: { $gt: 1.99 } } }` |
| `$nor` | Joins query clauses with a logical NOR returns all documents that fail to match both clauses. | `{ $nor: [ { price: 1.99 }, { sale: true } ] }` |
| `$or` | Joins query clauses with a logical OR returns all documents that match the conditions of either clause. | `{ $or: [ { quantity: { $lt: 20 } }, { price: 10 } ] }` |

## Element Operators

| Operator | Description | Example |
|---|---|---|
| `$exists` | Matches documents that have the specified field. | `{ qty: { $exists: true, $nin: [ 5, 15 ] } }` |
| `$type` | Selects documents if a field is of the specified type. | `{ zipCode: { $type: "string" } }` |

## Array Operators

| Operator | Description | Example |
|---|---|---|
| `$all` | Matches arrays that contain all elements specified in the query. | `{ tags: { $all: [ "ssl", "security" ] } }` |
| `$elemMatch` | Selects documents if element in the array field matches all the specified $elemMatch conditions. | `{ results: { $elemMatch: { product: "xyz", score: { $gte: 8 } } } }` |
| `$size` | Selects documents if the array field is a specified size. | `{ tags: { $size: 3 } }` |

## Projection

Projections determine which fields are returned in the matching documents.

- `1` or `true`: Include field.
- `0` or `false`: Exclude field.

**Example:**
```javascript
db.inventory.find(
    { status: "A" },
    { item: 1, status: 1, _id: 0 } // Return item and status, exclude _id
)
```
