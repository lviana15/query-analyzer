# MongoDB Aggregation Syntax Reference

This guide provides common patterns and syntax for MongoDB aggregation pipelines.

## Pipeline Concept

Aggregations process documents through a multi-stage pipeline. Each stage transforms the documents as they pass through.

```javascript
db.collection.aggregate([
    { $stage1: { ... } },
    { $stage2: { ... } },
    ...
])
```

## Common Stages

### `$match`
Filters the documents to pass only the documents that match the specified condition(s) to the next pipeline stage.

**Syntax:** `{ $match: { <query> } }`
**Example:**
```javascript
{ $match: { status: "A" } }
```

### `$group`
Groups input documents by the specified `_id` expression and for each distinct grouping, outputs a document. The `_id` field of each output document contains the unique group by value.

**Syntax:** `{ $group: { _id: <expression>, <field1>: { <accumulator1> : <expression1> }, ... } }`
**Example:**
```javascript
{
  $group: {
    _id: "$custId",
    totalAmount: { $sum: { $multiply: [ "$price", "$quantity" ] } },
    count: { $sum: 1 }
  }
}
```

### `$project`
Passes along the documents with the requested fields to the next stage in the pipeline. The specified fields can be existing fields from the input documents or newly computed fields.

**Syntax:** `{ $project: { <specification(s)> } }`
**Example:**
```javascript
{
  $project: {
    _id: 0,
    title: 1,
    author: 1,
    fullName: { $concat: [ "$firstName", " ", "$lastName" ] }
  }
}
```

### `$lookup`
Performs a left outer join to an unsharded collection in the same database to filter in documents from the "joined" collection for processing.

**Syntax:**
```javascript
{
   $lookup:
     {
       from: <collection to join>,
       localField: <field from the input documents>,
       foreignField: <field from the documents of the "from" collection>,
       as: <output array field>
     }
}
```
**Example:**
```javascript
{
  $lookup: {
    from: "inventory",
    localField: "item",
    foreignField: "sku",
    as: "inventory_docs"
  }
}
```

### `$unwind`
Deconstructs an array field from the input documents to output a document for each element. Each output document is the input document with the value of the array field replaced by the element.

**Syntax:** `{ $unwind: <field path> }`
**Example:**
```javascript
{ $unwind: "$sizes" }
```

### `$sort`
Sorts all input documents and returns them to the pipeline in sorted order.

**Syntax:** `{ $sort: { <field1>: <sort order>, <field2>: <sort order> ... } }`
**Example:**
```javascript
{ $sort: { age: -1, posts: 1 } } // -1 for descending, 1 for ascending
```

### `$limit`
Limits the number of documents passed to the next stage in the pipeline.

**Syntax:** `{ $limit: <positive integer> }`
**Example:**
```javascript
{ $limit: 5 }
```

### `$skip`
Skips over the specified number of documents that pass into the stage and passes the remaining documents to the next stage in the pipeline.

**Syntax:** `{ $skip: <positive integer> }`
**Example:**
```javascript
{ $skip: 10 }
```

## Common Accumulators (for `$group`)

- `$sum`: Calculates the sum.
- `$avg`: Calculates the average.
- `$min`: Returns the minimum value.
- `$max`: Returns the maximum value.
- `$push`: Returns an array of all values that result from applying an expression to each document.
- `$addToSet`: Returns an array of all unique values that result from applying an expression to each document.
- `$first`: Returns the value that results from applying an expression to the first document in a group of documents.
- `$last`: Returns the value that results from applying an expression to the last document in a group of documents.

## Date Operators

- `$year`, `$month`, `$dayOfMonth`, `$hour`, `$minute`, `$second`
- `$dateToString`: Converts a date object to a string according to a user-specified format.

**Example:**
```javascript
{
  $project: {
    year: { $year: "$date" },
    month: { $month: "$date" },
    day: { $dayOfMonth: "$date" },
    formattedDate: { $dateToString: { format: "%Y-%m-%d", date: "$date" } }
  }
}
```
