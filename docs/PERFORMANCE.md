# Performance Tuning

MongoDB deployments can support large-scale databases with high transaction volumes, making performance tuning essential. Regular tuning helps identify issues within the cluster early, allowing you to address them before they impact system responsiveness or stability.

This document addresses some common methods to optimize your deployment performance by using performance tuning and helpful metrics. These methods apply to both MongoDB Atlas clusters and self-managed deployments. However, the tuning process is significantly easier with [MongoDB Atlas](https://www.mongodb.com/docs/atlas/getting-started/#std-label-atlas-getting-started), which automates many tasks and streamlines for efficiency. For more information on performance, see [MongoDB Performance](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/administration/analyzing-mongodb-performance/#std-label-performance).

## Run Your Queries at Top Speed

To ensure optimal query performance, you can use metrics that reveal query performance problems and tell you what to do if you find slow queries.

MongoDB log files record the execution time and method for each query, allowing you to search for slow queries. The [database profiler](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/manage-the-database-profiler/#std-label-database-profiler) logs queries exceeding a specified threshold.

If a query is slow, first access your query plans. For more information on finding query plan data, see [Explain Results](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/explain-results/#std-label-explain-results).

- Ensure that your query performed an index scan, rather than a collection scan.

  An index scan limits the number of documents that MongoDB inspects, while a collection scan requires that MongoDB reads all documents in a collection. To learn more about how to interpret plan results, see [Interpret Explain Plan Results](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/analyze-query-plan/#std-label-interpret-explain-plan).

- If you see a lot of collection scans in your explain plan results, consider adding an [index](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/indexes/#std-label-indexes).

  Indexes can slow down writes and updates, so having too many underutilized indexes may hinder document modifications or insertions, depending on your workload.

### Query Metrics

You can also use the following query metrics to ensure your query is running at top speed:

- [`metrics.queryExecutor.scanned`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-serverstatus-serverstatus.metrics.queryExecutor.scanned) tells you how many documents were scanned to return your query results.

  - Ideally, the ratio of scanned documents to returned documents is 1:1, which means MongoDB returns all documents. Typically, the ratio is greater than 1, indicating MongoDB does not return some scanned documents.

  - The ratio can be less than 1 or even 0, indicating a covered query where the index contains all necessary data.

  - If MongoDB is scanning large numbers of documents to respond to your query, you may be missing indexes or need to optimize your query.

- [`metrics.operation.scanAndOrder`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-serverstatus-serverstatus.metrics.operation.scanAndOrder) indicates the server's effort to sort query results.

  - A high Scan and Order number, such as 20 or more, indicates that the server is having to sort results, increasing query result time and server memory load.

  - To fix a high Scan and Order number, sort your indexes according to query requirements, or add any missing indexes. Generally, sort b-tree indexes in ascending order from the leading field in the index, if it's a compound index.

- The [WiredTiger Ticket Number](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/wiredtiger/#std-label-storage-wiredtiger) metric reflects the performance of the WiredTiger storage engine.

  - WiredTiger read and write tickets are the WiredTiger storage engine's concurrency control mechanism to manage the number of concurrent transactions. Starting in version 7.0, MongoDB uses a dynamic algorithm to adjust the maximum number of concurrent storage engine transactions, optimizing database throughput during cluster overload.

  - The read and write tickets control the maximum number of concurrent transactions. The WiredTiger ticket number should always be at 128. Sustained values below 128 indicates a server delay and consequential potential issues.

  - You can use the [`serverStatus`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-dbcommand-dbcmd.serverStatus) command to check the current number of read and write tickets and their usage. Look at the [`queues.execution`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-serverstatus-serverstatus.queues.execution) section to understand the current load and ticket availability.

  - To remedy a low WiredTiger ticket number:

    - Ensure that the Dynamic Adjustment feature is enabled to manage ticket allocation automatically.

    - Ensure that your cluster has sufficient resources, such as CPU and memory, to handle the workload.

    - If you are using MongoDB 3.2 or earlier, upgrade to a later version that uses WiredTiger.

    - If you need to manually adjust the maximum number of concurrent transactions, you can modify the [`storageEngineConcurrentReadTransactions`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/parameters/#mongodb-parameter-param.storageEngineConcurrentReadTransactions) and [`storageEngineConcurrentWriteTransactions`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/parameters/#mongodb-parameter-param.storageEngineConcurrentWriteTransactions) parameters.

Take caution when modifying [`storageEngineConcurrentReadTransactions`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/parameters/#mongodb-parameter-param.storageEngineConcurrentReadTransactions) and [`storageEngineConcurrentWriteTransactions`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/parameters/#mongodb-parameter-param.storageEngineConcurrentWriteTransactions), as changing these settings can lead to performance issues or errors. We recommend you consult with MongoDB Support before changing these parameters.

### Document Structure Antipatterns

The query plan does not contain any metrics to reveal document structure [antipatterns](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/data-modeling/design-antipatterns/#std-label-schema-design-antipatterns), but you can look for antipatterns when debugging slow queries. Be careful of the following most common bad query practices that hurt performance:

- **Unbound arrays:** Arrays in a document that can grow without a size limit cause performance problems, because each time you update the array, MongoDB must rewrite the array into the document. For more information, see [Avoid Unbounded Arrays](https://www.mongodb.com/docs/atlas/schema-suggestions/avoid-unbounded-arrays/#std-label-unbounded-arrays-anti-pattern).

- **Embedded documents without bounds:** MongoDB supports inserting documents within documents, with up to 128 levels of nesting. Each MongoDB document, including embedded documents, has a size limit of 16MB. An excessive number of embedded documents can result in performance problems.

  To mitigate excessive embedded documents, move embedded documents to separate collections and reference them from the original document. For more information, see [Bloated Documents](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/data-modeling/design-antipatterns/bloated-documents/#std-label-bloated-documents-antipattern).

## Ensure a Top Speed Database

MongoDB has thousands of metrics that track all aspects of database performance, including reading, writing, and querying the database, as well as making sure background maintenance tasks like backups don't hinder performance. The following metrics help indicate problems with your database so you can ensure its optimal performance.

### Replication Lag

Replication lag occurs when a secondary member of a replica set falls behind the primary. To understand the cause of your replication lag, you can examine the [oplog](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-oplog)-related metrics. However, the following problems are the most common causes of replication lag:

- A networking issue between the primary and secondary, making nodes unreachable

- A secondary node applying data slower than the primary node

- Insufficient write capacity, in which case you should add more shards

- Slow operations on the primary node, blocking replication

### Locking Performance Problems

MongoDB's internal locking system is used to support simultaneous queries while avoiding write conflicts and inconsistent reads. Performance problems that are the result of locking occur when the remaining number of available read or write tickets reaches zero, meaning any new read or write requests will be queued until a new read or write ticket is available.

Locking performance problems can indicate suboptimal indexes and poor schema design patterns, which can both lead to locks being held longer than necessary.

### Open Cursors

If the number of open cursors is rising without a corresponding growth of traffic, this might be the result of poorly indexed queries, or long-running queries due to large result sets.

## Overloaded Clusters

When performance tuning, it is important to recognize when your total traffic, or the throughput of transactions through the system, is rising beyond the planned capacity of your cluster. By keeping track of growth in throughput, you can expand your cluster's capacity efficiently.

The following metrics can help you track your cluster's throughput. To find these metrics, run the [`serverStatus`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-dbcommand-dbcmd.serverStatus) command and examine the fields specified below.

### Read and Write Operations

The Read and Write Operations metrics indicate how much work the cluster does. You can find read operations through the [`opcounters.query`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-serverstatus-serverstatus.opcounters.query) field and write operations through [`opcounters.insert`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-serverstatus-serverstatus.opcounters.insert), [`opcounters.update`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-serverstatus-serverstatus.opcounters.update), and [`opcounters.delete`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-serverstatus-serverstatus.opcounters.delete), which count the total number of insert, update, and delete operations, respectively.

The ratio of reads to writes depends on the nature of the workloads running on the cluster.

- Monitoring read and write operations over time allows normal ranges and thresholds to be established.

- As trends in read and write operations show growth in throughput, you can gradually increase capacity.

### Document Metrics and Query Executor

Document Metrics and Query Executor indicate if the cluster is too busy. Similarly to the Read and Write operations metric, there is no right or wrong number for these metrics, but having a good idea of what's normal helps you discern whether poor performance is coming from large workload size or attributable to other reasons.

To retrieve Document Metrics, access the `metrics.keysExamined` and `metrics.totalExecMicros` fields. To retrieve Query Executor metrics, examine the `metrics.fromPlanCache` field. You can find all of these fields using the [`$queryStats`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/operator/aggregation/queryStats/#mongodb-pipeline-pipe.-queryStats) aggregation stage.

- MongoDB updates document metrics anytime you find a document or insert a document. The more documents that you find, insert, update or delete, the busier your cluster is.

  - Poor performance in a cluster that has plenty of capacity usually indicates query problems.

- The query executor tells you how many queries are being processed by using two data points:

  - **Scanned:** The average rate per second over the selected sample period of index items scanned during queries and query-plan evaluation.

  - **Scanned objects:** The average rate per second over the selected sample period of documents scanned during queries and query-plan evaluation.

## Hardware and Network Metrics

Hardware and Network metrics can indicate that throughput is rising and will exceed the capacity of computing infrastructure. These metrics are gathered from the operating system and networking infrastructure. To make these metrics useful for diagnostic purposes, you must have a sense of what is normal.

- If you are running MongoDB on-premises, you may be able to view hardware and network metrics using [Ops Manager](https://www.mongodb.com/docs/ops-manager/current/application/), depending on your operating system.

- While there are many metrics to track, some important metrics to have a baseline range for are:

  - Disk latency

  - Disk IOPS

  - Number of Connections

## Cluster and Key Resources

A MongoDB cluster uses a variety of resources that the underlying computing and networking infrastructure provides.

### Number of Client Connections

The Current Number of Client Connections metric, located in the [`connections.current`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-serverstatus-serverstatus.connections.current) field in the [`serverStatus`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-dbcommand-dbcmd.serverStatus) document, can indicate total load on a system. Keeping track of normal ranges at various times of the day or week can help you quickly identify spikes in traffic.

A related metric, percentage of connections used, can indicate when MongoDB is getting close to running out of available connections.

### Storage Metrics

Storage metrics track how MongoDB uses persistent storage. In the WiredTiger storage engine, each collection and each index are individual files. When you update a document in a collection, MongoDB re-writes the entire document.

- If memory space metrics such as [`dbStats.dataSize`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/dbStats/#mongodb-data-dbStats.dataSize), [`dbStats.indexSize`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/dbStats/#mongodb-data-dbStats.indexSize), [`dbStats.storageSize`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/dbStats/#mongodb-data-dbStats.storageSize), or the number of documents in the database show a significant unexpected change while the database traffic stays within ordinary ranges, it can indicate problems such as data deletion or corruption, unexpected data growth, or index changes.

- A sudden drop in [`dbStats.dataSize`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/dbStats/#mongodb-data-dbStats.dataSize) may indicate a large amount of data deletion. If this drop is unexpected, you should quickly investigate.

### Memory Metrics

Memory metrics show how MongoDB uses the virtual memory of the computing infrastructure that is hosting the cluster. You can find memory metrics in the [`mem`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-serverstatus-serverstatus.mem) document in the results of [`serverStatus`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#mongodb-dbcommand-dbcmd.serverStatus).

- An increasing number of page faults or a growing amount of data changed but not yet written to disk can indicate problems related to the amount of memory available to the cluster.

- Cache metrics can help determine if the working set is outgrowing the available cache.

## Critical Errors

MongoDB creates [asserts](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/serverStatus/#std-label-server-status-asserts) mostly through errors that MongoDB captures as part of its logging process.

Monitoring the number of asserts created at various levels of severity can provide a first level indication of unexpected problems. Asserts can be message asserts, the most serious kind, or warning assets, regular asserts, and user asserts.


