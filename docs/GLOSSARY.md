# Glossary

A virtual [collection](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-collection) that exposes MongoDB's [database commands](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-database-command). To use database commands, see [Issue Commands](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/use-database-commands/#std-label-issue-commands).

A field required in every MongoDB [document](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-document). The [_id](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/document/#std-label-document-id-field) field must have a unique value. You can think of the `_id` field as the document's [primary key](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary-key). If you create a new document without an `_id` field, MongoDB automatically creates the field and assigns a unique BSON [ObjectId](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-ObjectId) to the field.

System CPU utilization relative to the full amount of CPU available to cloud instances that share CPU.

- When a cloud provider throttles CPU utilization for a cloud instance, the instance's absolute system CPU utilization is equal to the [baseline CPU utilization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-baseline-CPU-utilization) assigned to this instance.

- When a cloud provider adds CPU above the baseline CPU, such as through a bursting mechanism, the sum of normalized kernel CPU utilization and user CPU utilization on an instance can exceed the instance's baseline CPU. In this case, the sum of the normalized kernel CPU utilization and user CPU utilization is still less than the full amount of CPU shared by cloud instances. See also [relative system CPU utilization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-relative-system-CPU-utilization), [baseline CPU utilization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-baseline-CPU-utilization), and [burstable instances](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-burstable-instances).

An [expression](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-expression) in an [aggregation pipeline](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-aggregation-pipeline) that maintains state between documents in the aggregation [pipeline](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-pipeline). For a list of accumulator operations, see [`$group`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/operator/aggregation/group/#mongodb-pipeline-pipe.-group).

An operation the user can perform on a resource. Actions and [resources](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-resource) combine to create [privileges](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-privilege). See [action](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/privilege-actions/#std-label-security-user-actions).

A privileged database. Users must have access to the `admin` database to run certain administrative commands. For a list of administrative commands, see [Administration Commands](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/#std-label-admin-commands).

In security, an attacker who gains and maintains long-term access to the network, disk and/or memory  and remains undetected for an extended period.

An operation that reduces and summarizes large sets of data. For more information, see [Aggregation Operations](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/aggregation/#std-label-aggregation).

Consists of one or more stages that process documents. Aggregation pipelines offer rich syntax to express complex queries. For a list of stages, see [Aggregation Stages](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/mql/aggregation-stages/#std-label-aggregation-pipeline-operator-reference).

Notification sent by Atlas when your database operations or server usage reach thresholds that affect cluster performance. To learn what conditions you can set to trigger alerts, see [Review Alert Conditions](https://www.mongodb.com/docs/atlas/reference/alert-conditions/#std-label-alert-conditions).

[Resolve Alerts](https://www.mongodb.com/docs/atlas/alert-resolutions/#std-label-respond-to-alerts)

Specialized read-only node that can isolate queries which you do not want to affect your operational workload. Analytics nodes are useful for handling analytic data such as reporting queries executed by BI tools. You can host analytics nodes in dedicated geographic regions to optimize read performance and reduce latency.

Communication protocol facilitating interaction between the client and MongoDB Atlas. You can use the Atlas Administration API to automate many of the tasks performed in the Atlas UI.

- [Atlas Programmatic Access](https://www.mongodb.com/docs/atlas/api/#std-label-atlas-api)

- [Atlas Administration API Specification](https://mongodb.com/docs/atlas/reference/api-resources-spec)

Computational technique used to quickly find points in a dataset that are close to a given query point. Vector Search uses ANN search to find vector embeddings in the data that are closest to the vector embeddings in the query without scanning every vector.

[MongoDB Search Overview](https://www.mongodb.com/docs/atlas/atlas-search/#std-label-fts-top-ref)

A [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) member that exists just to vote in [elections](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-election). Arbiters do not replicate data. An arbiter participates in elections for a [primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary) but cannot become a primary. For more details, see [Replica Set Arbiter](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-arbiter/#std-label-replica-set-arbiter-configuration).

[MongoDB Atlas](https://www.mongodb.com/atlas/database) is a cloud-hosted database-as-a-service.

Account used to access the MongoDB Atlas application. You can grant MongoDB Atlas users access to MongoDB Atlas organizations, projects, or both, with certain permissions defined by [user roles](https://www.mongodb.com/docs/atlas/reference/user-roles/#std-label-user-roles). A MongoDB Atlas user is different than a [database user](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-database-user). MongoDB Atlas users do not provide access to any MongoDB databases.

[Configure Access to the Atlas UI](https://www.mongodb.com/docs/atlas/organizations-projects/#std-label-atlas_users)

Set of permissions granted to an [Atlas user](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-Atlas-user). You can grant permissions at the [organization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-organization) or [project](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-project) level.

[Atlas User Roles](https://www.mongodb.com/docs/atlas/reference/user-roles/#std-label-user-roles)

An atomic operation is a write operation that either completes entirely or doesn't complete at all. For [distributed transactions](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/transactions/#std-label-transactions), which involve writes to multiple documents, all writes to each document must succeed for the transaction to succeed. Atomic operations cannot partially complete. See [Atomicity and Transactions](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/write-operations-atomicity/#std-label-transactions-write-atomicity).

Verification of the user identity. See [Authentication on Self-Managed Deployments](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/authentication/).

Provisioning of access to databases and operations. See [Role-Based Access Control in Self-Managed Deployments](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/authorization/).

Configurable option to have your cluster automatically increase or decrease its cluster tier, storage capacity, or both in response to cluster usage.

[Configure Auto-Scaling](https://www.mongodb.com/docs/atlas/cluster-autoscaling/#std-label-cluster-autoscaling)

When using [In-Use Encryption](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-In-Use-Encryption), automatically performing encryption and decryption based on your preconfigured encryption schema. The Automatic Encryption Shared Library translates MongoDB Query Language into the correct call, meaning you don't need to rewrite your application for specific encrypt and decrypt calls.

A data structure commonly used by database management systems to store indexes. MongoDB uses B-tree indexes.

Copy of your data that encapsulates the state of your cluster at a given time. Backups provide a safety measure in the case of data loss events.

MongoDB Atlas provides fully-managed [Cloud Backups](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-cloud-backups).

A [tailable cursor](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-tailable-cursor) that points to a list of backup files. Backup cursors are for internal use only.

An internal MongoDB process that runs in the context of a [sharded cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster) and manages the migration of [chunks](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-chunk). Administrators must disable the balancer for all maintenance operations on a sharded cluster. See [Sharded Cluster Balancer](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-balancer-administration/#std-label-sharding-balancing).

Fraction of the full amount of CPU available to cloud instances that share CPU. A cloud provider assigns a certain amount of baseline CPU to each cloud instance, based on the instance's cluster tier. Typically, baseline CPU utilization falls between 20% and 50% of [absolute system CPU utilization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-absolute-system-CPU-utilization). See also [relative system CPU utilization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-relative-system-CPU-utilization) and [burstable instances](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-burstable-instances).

A byte order in which the most significant byte (big end) of a multibyte data value is stored at the lowest memory address.

A sort that must be performed in memory before the output is returned. In-memory sorts may impact performance for large data sets. Use an [indexed sort](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-indexed-sort) to avoid an in-memory sort.

A plan used by the [query optimizer](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/query-plans/#std-label-read-operations-query-optimization) that excludes documents with specific field value ranges. For example, if a range of date field values is outside of a specified date range, the documents in that range are excluded from the query plan. See [Collection Scan](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/explain-results/#std-label-explain-output-collection-scan).

A serialization format used to store [documents](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-document) and make remote procedure calls in MongoDB. "BSON" is a combination of the words "binary" and "JSON". Think of BSON as a binary representation of JSON (JavaScript Object Notation) documents. See [BSON Types](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/bson-types/#std-label-bson-types) and [MongoDB Extended JSON (v2)](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/mongodb-extended-json/).

The set of types supported by the [BSON](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-BSON) serialization format. For a list of BSON types, see [BSON Types](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/bson-types/#std-label-bson-types).

Cloud instance types that share a common physical CPU that, for some cloud providers, use a "CPU credit" model. When you use burstable instances, portions of shared CPU may either become available to each of the virtual instances or may become unavailable, under different demands on the instance resources. To learn more, see [AWS burstable instances](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/burstable-performance-instances.html), [Azure disk bursting](https://learn.microsoft.com/en-us/azure/virtual-machines/disk-bursting), and [GCP CPU Bursting](https://cloud.google.com/compute/docs/general-purpose-machines#cpu-bursting). See also [baseline CPU utilization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-baseline-CPU-utilization), [absolute system CPU utilization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-absolute-system-CPU-utilization), and [relative system CPU utilization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-relative-system-CPU-utilization).

Given three properties of computing systems, consistency, availability, and partition tolerance, a distributed computing system can provide any two of these features, but never all three.

A fixed-sized [collection](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-collection) that automatically overwrites its oldest entries when the collection reaches its maximum size. The MongoDB [oplog](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-oplog) that is used in [replication](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replication) is a capped collection. See [Capped Collections](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/capped-collections/).

The measure of the number of elements within a set of values. For example, the set `A = { 2, 4, 6 }` contains 3 elements, and has a cardinality of 3. See [Shard Key Cardinality](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-choose-a-shard-key/#std-label-shard-key-cardinality).

The result of combining two data sets where the combined set contains every possible combination of values.

Complete Fairness Queueing (cfq) is an I/O operation scheduler that allocates bandwidth for incoming request processes.

A calculated value used to ensure data integrity. The [md5](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-md5) algorithm is sometimes used as a checksum.

A contiguous range of [shard key](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-shard-key) values within a [shard](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-shard). Chunk ranges are inclusive of the lower boundary and exclusive of the upper boundary. MongoDB splits chunks when data needs to be balanced inside the cluster. While the default chunk size is 128 megabytes, it is possible for chunks to grow larger than the default size without triggering an automatic split. Chunk splitting occurs as part of the balancing process to distribute data evenly across the cluster. For more details, see [Data Partitioning with Chunks](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-data-partitioning/#std-label-sharding-data-partitioning), [Sharded Cluster Balancer](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-balancer-administration/#std-label-sharding-balancing), and [Manage Sharded Cluster Balancer](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/manage-sharded-cluster-balancer/#std-label-sharded-cluster-balancer).

The application layer that uses a database for data persistence and storage. [Drivers](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-driver) provide the interface level between the application layer and the database server.

A client can also be a single thread or process.

A consistent client connection to a specified data source.

Localized cluster [backup](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-backup) storage using the native [snapshot](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-snapshot) functionality of the cluster's cloud service provider.

MongoDB Atlas supports Cloud Backups for clusters served on:

- [Amazon Web Services (AWS)](https://www.mongodb.com/docs/atlas/reference/amazon-aws/#std-label-amazon-aws)

- [Google Cloud Platform (GCP)](https://www.mongodb.com/docs/atlas/reference/google-gcp/#std-label-google-gcp)

- [Microsoft Azure](https://www.mongodb.com/docs/atlas/reference/microsoft-azure/#std-label-microsoft-azure)

[Back Up Your Cluster](https://www.mongodb.com/docs/atlas/backup/cloud-backup/overview/#std-label-backup-cloud-provider)

Set of nodes comprising a MongoDB [deployment](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-deployment). Clusters can be [replica sets](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) or [sharded deployments](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster).

[sharded cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster)

*Configurable for M40+ clusters hosted on AWS.*

Storage class of your cluster. Your selected class affects cluster storage performance and cluster costs. You can choose one of the following classes:

- Low CPU

- General

- Local NVMe SSD

- [Customize Cluster Storage](https://www.mongodb.com/docs/atlas/customize-storage/#std-label-create-cluster-storage)

- [NVMe storage](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-NVMe-storage)

Dictates the memory, storage, vCPUs, and IOPS (Input/Output Operations per Second) specification for each data-bearing server in the cluster. Cluster storage size and overall performance increase as the cluster tier increases.

- [Select Cluster Tier](https://www.mongodb.com/docs/atlas/manage-clusters/#std-label-create-cluster-instance)

- [Atlas Cluster Sizing and Tier Selection](https://www.mongodb.com/docs/atlas/sizing-tier-selection/#std-label-sizing)

Synchronizes data between [sharded clusters](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster). Also known as C2C sync.

A [collection](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-collection) that stores documents ordered by a [clustered index](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/db.createCollection/#std-label-db.createCollection.clusteredIndex) key. See [Clustered Collections](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/clustered-collections/#std-label-clustered-collections).

Abbreviation of Customer Master Key, see [Customer Master Key](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-Customer-Master-Key).

A grouping of MongoDB [documents](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-document). A collection is the equivalent of an [RDBMS](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-RDBMS) table. A collection is in a single [database](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-database). Collections do not enforce a schema. Documents in a collection can have different fields. Typically, documents in a collection have a similar or related purpose. See [Namespaces](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/limits/#std-label-faq-dev-namespace).

Collection scans are a query execution strategy where MongoDB must inspect every document in a collection to see if it matches the query criteria. These queries are very inefficient and don't use indexes. See [Query Optimization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/query-optimization/) for details about query execution strategies.

Saves data changes made after the start of the [`startSession`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/startSession/#mongodb-dbcommand-dbcmd.startSession) command. Operations within a [transaction](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-transaction) are not permanent until they are committed with the [`commitTransaction`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/commitTransaction/#mongodb-dbcommand-dbcmd.commitTransaction) command.

During an [index build](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/index-creation/#std-label-index-operations-replicated-build) the [commit quorum](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/createIndexes/#std-label-createIndexes-cmd-commitQuorum) specifies how many secondaries must be ready to commit their local index build before the primary node performs the commit.

An [index](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-index) consisting of two or more keys. See [Compound Indexes](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/indexes/index-types/index-compound/#std-label-index-type-compound).

Concurrency control ensures that database operations can be executed concurrently without compromising correctness. Pessimistic concurrency control, such as that used in systems with [locks](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-lock), blocks any potentially conflicting operations even if they may not conflict. Optimistic concurrency control, the approach used by [WiredTiger](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/wiredtiger/#std-label-storage-wiredtiger), delays checking until after a conflict may have occurred, ending and retrying one of the operations in any [write conflict](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-write-conflict).

An internal database with metadata for a [sharded cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster). Typically, you don't modify the `config` database. For more information about the `config` database, see [Config Database](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/config-database/).

A [`mongod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/#mongodb-binary-bin.mongod) instance that stores all the metadata associated with a [sharded cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster). See [Config Servers](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharded-cluster-config-servers/#std-label-sharding-config-server).

A `mongod` instance that stores all the metadata associated with a [sharded cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster) and can also store application data. See [Config Shards](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharded-cluster-config-servers/#std-label-sharded-cluster-config-server-config-shards).

A cache of database connections maintained by the driver. The cached connections are re-used when connections to the database are required, instead of opening new connections.

A scenario where a driver attempts to open more connections to a deployment than that deployment can handle. When requests for new connections fail, the driver requests to establish even more connections in response to the deployment slowing down or failing to open new connections. These continuous requests can overload the deployment and lead to outages.

A collected set of software and its dependent libraries that are packaged together to make transferring between computing environments easier. Containers run as compartmentalized processes on your operating system, and can be given their own resource constraints. Common container technologies are Docker and Kubernetes.

Multiple operations attempting to modify the same resource, such as a document field, cause conflicts that delay operations. The contention factor is a setting used with Queryable Encryption to internally partition encrypted field/value pairs and optimize operations. See [contention](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/queryable-encryption/fundamentals/encrypt-and-query/#std-label-qe-contention).

Metric that uses the angle between two vectors to determine the similarity between those vectors. Cosine similarity is sensitive to vector orientation. You can use cosine similarity function when indexing your vector embeddings for Vector Search. If the vectors are normalized to unit length, use [dotProduct similarity](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-dotProduct-similarity) function instead.

The percentage by which the CPU usage exceeds the guaranteed baseline CPU credit accumulation rate. CPU steal is relevant for cloud providers that rely on the credit model in their bursting strategy. CPU credits are units of CPU utilization that you accumulate. The credits accumulate at a constant rate to provide a guaranteed level of performance. You can use these credits for additional CPU performance. When the credit balance is exhausted, MongoDB only provides the guaranteed baseline of CPU performance and displays the amount of excess as steal percent See also [relative system CPU utilization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-relative-system-CPU-utilization), [baseline CPU utilization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-baseline-CPU-utilization), and [burstable instances](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-burstable-instances).

An acronym for the fundamental operations of a database: Create, Read, Update, and Delete. See [MongoDB CRUD Operations](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/crud/#std-label-crud).

A text data format with comma-separated values. CSV files can be used to exchange data between relational databases because CSV files have tabular data. You can import CSV files using [`mongoimport`](https://www.mongodb.com/docs/database-tools/mongoimport/#mongodb-binary-bin.mongoimport).

A pointer to the result set of a [query](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-query). Clients can iterate through a cursor to retrieve results. By default, cursors not opened within a session automatically timeout after 10 minutes of inactivity. Cursors opened in a session close with the end or timeout of the session. See [Cursors](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/cursors/#std-label-cursors).

Custom set of MongoDB [privilege actions](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-action) and MongoDB [roles](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-role) that you can save and assign to a [database user](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-database-user). Create custom roles when MongoDB Atlas's [built-in roles](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/built-in-roles/#std-label-atlas-user-privileges) don't describe your desired set of privileges.

[Configure Custom Database Roles](https://www.mongodb.com/docs/atlas/security-add-mongodb-roles/#std-label-mongodb-roles)

A key that encrypts your [Data Encryption Key](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-Data-Encryption-Key). The customer master key must be hosted in a remote key provider.

A background, non-interactive process.

The file system location where [`mongod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/#mongodb-binary-bin.mongod) stores data files. [`dbPath`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/configuration-options/#mongodb-setting-storage.dbPath) specifies the data directory.

A key you use to encrypt the fields in your MongoDB documents. The **encrypted** Data Encryption Key is stored in your Key Vault collection. The Data Encryption Key is encrypted by the [Customer Master Key](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-Customer-Master-Key).

Tool within MongoDB Atlas to view and interact with cluster data. You can also use the Data Explorer to manage indexes and run aggregation pipelines to process your data.

[Interact with Your Data](https://www.mongodb.com/docs/atlas/atlas-ui/#std-label-atlas-ui)

MongoDB's solution for querying data stored in low-cost S3 buckets, MongoDB Atlas clusters, and HTTP (HyperText Transport Protocol) endpoints using the MongoDB Query Language. Analytics applications can use Atlas Data Federation to make use of archived data for their data processing needs.

[Atlas Data Federation](https://www.mongodb.com/docs/atlas/data-federation/#std-label-atlas-data-federation)

Store document data and indexes. The [`dbPath`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/configuration-options/#mongodb-setting-storage.dbPath) option specifies the file system location for the data files.

Workflow for organizing and transforming data by using RAG (Retrieval-Augmented Generation) and storing it in a vector database such as MongoDB Atlas.

A distributed system architecture that splits data into ranges. [Sharding](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharding) uses partitioning. See [Data Partitioning with Chunks](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-data-partitioning/#std-label-sharding-data-partitioning).

A property that allows clients to address members in a system based on their locations. [Replica sets](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) implement data-center awareness using [tagging](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-tag). See [Data Center Awareness](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/data-center-awareness/).

A container for [collections](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-collection). Each database has a set of files in the file system. One MongoDB server typically has multiple databases.

A MongoDB operation, other than an insert, update, remove, or query. For a list of database commands, see [Database Commands](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/). To use database commands, see [Issue Commands](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/use-database-commands/#std-label-issue-commands).

Database exfiltration refers to an authorized party taking data from a secured system and either sharing it with an unauthorized party or storing it on an unsecured system. This may be malicious or accidental.

A tool that, when enabled, keeps a record on all long-running operations in a database's `system.profile` collection. The profiler is most often used to diagnose slow queries. See [Database Profiler](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/manage-the-database-profiler/#std-label-database-profiling).

Credentials used to authenticate a client to access a MongoDB cluster. You can assign [privileges](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/built-in-roles/#std-label-atlas-user-privileges) to a database user to determine that user's access level to a cluster. Database users are different from [Atlas users](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-Atlas-user). Database users have access to MongoDB deployments, not the MongoDB Atlas application.

[Configure Database Users](https://www.mongodb.com/docs/atlas/security-add-mongodb-users/#std-label-mongodb-users)

The location of MongoDB's data file storage. See [`dbPath`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/configuration-options/#mongodb-setting-storage.dbPath).

DDL includes commands that create and modify collections and indexes.

A dead letter queue is a collection within an MongoDB Atlas database that stores documents that throw errors during ingestion.

Cluster category containing clusters of tier `M10` and greater.

<table>
<tr>
<th id="Tier">
Tier

</th>
<th id="Recommended%20environments">
Recommended environments

</th>
</tr>
<tr>
<td headers="Tier">
`M10` and `M20`

</td>
<td headers="Recommended%20environments">
- Development

- Low-traffic production

</td>
</tr>
<tr>
<td headers="Tier">
`M30` and greater

</td>
<td headers="Recommended%20environments">
Production

</td>
</tr>
</table>A `mongod` instance that only stores all the metadata associated with a [sharded cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster).

Data Encryption Key. For more details, see [Data Encryption Key](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-Data-Encryption-Key).

A [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) member that cannot become primary and applies operations at a specified delay. The delay is useful for protecting data from human error (unintentionally deleted databases) or updates that have unforeseen effects on the production database. See [Delayed Replica Set Members](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-delayed-member/#std-label-replica-set-delayed-members).

Numeric representation of data where most or all of the dimensions contain non-zero values. Vector Search uses dense vectors, which are packed with more data, to capture more complex relationships.

A group of MongoDB servers containing your data. MongoDB Atlas-managed clusters are clusters ([replica sets](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) or [sharded clusters](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster)).

Number of components or elements that make up the features or attributes of data in multi-dimensional space. Vector Search supports up to `4096` dimensions at index-time and query-time.

A record in a MongoDB [collection](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-collection) and the basic unit of data in MongoDB. Documents are analogous to [JSON](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-JSON) objects but exist in the database in a more type-rich format known as [BSON](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-BSON). See [Documents](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/document/).

MongoDB uses the dot notation to access the elements of an array and to access the fields of an embedded document. See [Dot Notation](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/document/#std-label-document-dot-notation).

Measures similarity between two vectors in multi-dimensional space and returns a scalar value, which is positive when the vectors point in roughly the same direction, negative when the vectors point in opposite direction, and zero when the vectors have no similarity. Vector Search supports using `dotproduct` similarity function when searching for nearest neighbors. We recommend this similarity function instead of cosine similarity if the vectors are normalized to unit length.

The process of removing or "shedding" [chunks](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-chunk) from one [shard](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-shard) to another. Administrators must drain shards before removing them from the cluster. See [Remove Shards from a Sharded Cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/remove-shards-from-cluster/).

A client library for interacting with MongoDB in a particular computer language. See [driver](https://www.mongodb.com/docs/drivers/).

A write operation is durable when it persists after a shutdown (or crash) and restart of one or more server processes. For a single [`mongod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/#mongodb-binary-bin.mongod) server, a write operation is considered durable when it has been written to the server's [journal](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-journal) file. For a [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/replication/), a write operation is considered durable after the write operation achieves durability on a majority of voting nodes and written to a majority of voting nodes' journals.

Node which is eligible to become the [primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary) member of your [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set). MongoDB Atlas prioritizes nodes in the [highest priority region](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-highest-priority-region) for primary eligibility during elections. To ensure reliable elections, the total number of electable nodes across an entire region must be 3, 5, or 7.

The process where members of a [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) select a [primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary) on startup and in the event of a failure. See [Replica Set Elections](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-elections/#std-label-replica-set-elections).

Representation of data such as text, images, audio, video, and so on as an array of numbers, which can be interpreted as coordinates in multi-dimensional space. MongoDB Atlas supports storing embeddings in an MongoDB Atlas cluster and Vector Search supports indexing and querying vector embeddings of up to `4096` dimensions.

Random string of bits generated specifically to encrypt and decrypt data.

MongoDB Atlas [`Project Owners`](https://www.mongodb.com/docs/atlas/reference/user-roles/#mongodb-authrole-Project-Owner) can configure an additional layer of encryption on their data in addition to the default [encryption at rest](https://www.mongodb.com/docs/atlas/security-kms-encryption/#std-label-security-kms-encryption) that MongoDB Atlas provides. Project owners can use their MongoDB Atlas-compatible customer key management provider with the MongoDB [encrypted storage engine](https://www.mongodb.com/docs/manual/core/security-encryption-at-rest/).

MongoDB Atlas supports the following customer key management providers when configuring Encryption at Rest:

- [Amazon Web Services Key Management Service](https://www.mongodb.com/docs/atlas/security-aws-kms/#std-label-security-aws-kms)

- [Azure Key Vault](https://www.mongodb.com/docs/atlas/security-azure-kms/#std-label-security-azure-kms)

- [Google Cloud Platform Key Management Service](https://www.mongodb.com/docs/atlas/security-gcp-kms/#std-label-security-gcp-kms)

[Encryption at Rest using Customer Key Management](https://www.mongodb.com/docs/atlas/security-kms-encryption/#std-label-security-kms-encryption)

In [Queryable Encryption](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/queryable-encryption/#std-label-qe-manual-feature-qe), the [encryptedFields document](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/queryable-encryption/fundamentals/encrypt-and-query/#std-label-qe-encryption-schema) that defines which fields are queryable and which query types are permitted on those fields.

In computing, endianness refers to the order in which bytes are arranged. This ordering can refer to transmission over a communication medium or more commonly how the bytes are ordered in computer memory, based on their significance and position. For details, see [big-endian](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-big-endian) and [little-endian](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-little-endian).

An encryption procedure where data is encrypted using a [Data Encryption Key](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-Data-Encryption-Key) and the data encryption key is encrypted by another key called the [Customer Master Key](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-Customer-Master-Key). The encrypted keys are stored as [BSON](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-BSON) documents in a MongoDB collection called the KeyVault.

Formula to calculate the similarity by using the distance between two vectors in multi-dimensional space. Euclidean distance is sensitive to the magnitude of the vectors. Vector Search supports using `euclidean` similarity function for indexing vectors and when searching for nearest neighbors.

A property of a distributed system that allows changes to the system to propagate gradually. In a database system, this means that readable members aren't required to have the latest updates.

When using [In-Use Encryption](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-In-Use-Encryption), explicitly specifying the encryption or decryption operation, keyID, and query type (for Queryable Encryption) or algorithm (for Client-Side Field Level Encryption) when working with encrypted data. Compare to [automatic encryption](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-automatic-encryption).

A component of a query that resolves to a value. Expressions are stateless, meaning they return a value without mutating any of the values used to build the expression.

In the MongoDB Query Language, you can build expressions from the following components:

<table>
<tr>
<th id="Component">
Component

</th>
<th id="Example">
Example

</th>
</tr>
<tr>
<td headers="Component">
Constants

</td>
<td headers="Example">
`3`

</td>
</tr>
<tr>
<td headers="Component">
Operators

</td>
<td headers="Example">
[`$add`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/operator/aggregation/add/#mongodb-expression-exp.-add)

</td>
</tr>
<tr>
<td headers="Component">
Field path expressions

</td>
<td headers="Example">
`"$<path.to.field>"`

</td>
</tr>
</table>For example, `{ $add: [ 3, "$inventory.total" ] }` is an expression that consists of the `$add` operator and two operands:

- The constant `3`

- The [field path expression](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/aggregation-pipeline/#std-label-agg-quick-ref-field-paths)
  `"$inventory.total"`

The expression returns the result of adding 3 to the value at path `inventory.total` of the input document.

The process that allows a [secondary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-secondary) member of a [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) to become [primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary) in the event of a failure. See [Automatic Failover](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/replication/#std-label-replication-auto-failover).

A name-value pair in a [document](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-document). A document has zero or more fields. Fields are analogous to columns in relational databases. See [Document Structure](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/document/#std-label-document-structure).

Path to a field in a document. To specify a field path, use a string that prefixes the field name with a dollar sign (`$`).

A system level network filter that restricts access based on IP addresses and other parameters. Firewalls are part of a secure network. See [Firewalls](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/security-hardening/#std-label-security-firewalls).

Cluster category containing `Free` ([free tier](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-free-tier)) tier clusters. Free and Flex clusters are generally used for development and small production workloads.

[Atlas M0 (Free Cluster) Limits](https://www.mongodb.com/docs/atlas/reference/free-shared-limitations/#std-label-atlas-free-tier)

Free-to-use cluster tier that provides a small-scale development environment to host your data. Free clusters never expire, and provide access to a [subset](https://www.mongodb.com/docs/atlas/reference/free-shared-limitations/#std-label-atlas-free-tier) of Atlas features and functionality. Free clusters might also be referred to by their instance size, `M0`.

- [Get Started with Atlas](https://www.mongodb.com/docs/atlas/getting-started/#std-label-atlas-getting-started)

- [Atlas M0 (Free Cluster) Limits](https://www.mongodb.com/docs/atlas/reference/free-shared-limitations/#std-label-atlas-free-tier)

A system call that flushes all dirty, in-memory pages to storage. As applications write data, MongoDB records the data in the storage layer.

To provide [durable](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-durable) data, [WiredTiger](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/wiredtiger/#std-label-storage-wiredtiger) uses [checkpoints](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/wiredtiger/#std-label-storage-wiredtiger-checkpoints). For more details, see [Journaling and the WiredTiger Storage Engine](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/journaling/#std-label-journaling-wiredTiger).

A geohash value is a binary representation of the location on a coordinate grid. See [Geohash Values](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/indexes/index-types/geospatial/2d/internals/#std-label-geospatial-indexes-geohash).

A [geospatial](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-geospatial) data interchange format based on JavaScript Object Notation ([JSON](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-JSON)). GeoJSON is used in [geospatial queries](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/geospatial-queries/#std-label-geospatial-queries). For supported GeoJSON objects, see [Geospatial Data](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/geospatial-queries/#std-label-geo-overview-location-data). For the GeoJSON format specification, see [https://tools.ietf.org/html/rfc7946#section-3.1](https://tools.ietf.org/html/rfc7946#section-3.1).

Relating to geographical location. See [Geospatial Queries](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/geospatial-queries/).

Clusters with defined geographic zones to support location-aware read and write operations for globally distributed application instances and clients. You can enable global sharding on clusters of tier `M30` and greater.

- [Manage Global Clusters](https://www.mongodb.com/docs/atlas/global-clusters/#std-label-global-clusters)

- [Create a Global Cluster](https://www.mongodb.com/docs/atlas/tutorial/create-global-cluster/#std-label-create-new-global-write-cluster)

Geographic zone representing a subset of your global cluster distribution. Each [global cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-global-cluster) supports up to 9 distinct global write zones. Each zone consists of one [highest priority region](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-highest-priority-region) and one or more [electable](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-electable-node), [read-only](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-read-only-node), or [analytics](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-analytics-node) regions.

The available geographic regions depend on the selected cloud service provider.

A convention for storing large files in a MongoDB database. All of the official MongoDB drivers support the GridFS convention, as does the [`mongofiles`](https://www.mongodb.com/docs/database-tools/mongofiles/#mongodb-binary-bin.mongofiles) program. See [GridFS](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/gridfs/).

See [project](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-project).

See [project ID](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-project-ID).

A type of [shard key](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-shard-key) that uses a hash of the value in the shard key field to distribute documents among members of the [sharded cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster). See [Hashed Indexes](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/indexes/index-types/index-hashed/#std-label-index-type-hashed).

A health manager runs health checks on a [health manager facet](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-health-manager-facet) at a specified [intensity level](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/administration/health-managers/#std-label-health-managers-intensity-levels). The health manager checks are run at specified time intervals. A health manager can be configured to move a failing [mongos](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongos/#std-label-mongos) out of a cluster automatically.

A set of features that a [health manager](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-health-manager) can be configured to run health checks for. For example, you can configure a health manager to monitor and manage DNS or LDAP cluster health issues automatically. See [Health Manager Facets](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/administration/health-managers/#std-label-health-managers-facets) for details.

A [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) member that cannot become [primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary) and are invisible to client applications. See [Hidden Replica Set Members](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-hidden-member/#std-label-replica-set-hidden-members).

Algorithm for performing efficient nearest neighbor search in multi-dimensional space. Vector Search performs ANN (Approximate Nearest Neighbor) search with [Hierarchical Navigable Small Worlds](https://arxiv.org/abs/1603.09320).

High availability indicates a system designed for durability, redundancy, and automatic failover. Applications supported by the system can operate without downtime for a long time period. MongoDB [replica sets](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/replication/#std-label-replication) support high availability when deployed according to the [best practices](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/administration/production-checklist-operations/#std-label-production-checklist-replication).

For guidance on replica set deployment architecture, see [Replica Set Deployment Architectures](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-architectures/#std-label-replica-set-architecture).

Region in a [multi-region cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-multi-region-cluster) which MongoDB Atlas prioritizes for [primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary) eligibility during elections.

[Configure High Availability and Workload Isolation](https://www.mongodb.com/docs/atlas/cluster-config/multi-cloud-distribution/#std-label-create-cluster-multi-region)

Method of combining different search methods, such as a full-text and a semantic search, to take advantage of their respective strengths. The results are combined by using a technique such as Reciprocal Rank Fusion (RRF).

An operation produces the same result with the same input when run multiple times.

Estimated performance improvement of an index that Performance Advisor suggests.

[Review Index Ranking](https://www.mongodb.com/docs/atlas/performance-advisor/index-ranking/#std-label-pa-index-ranking)

A sort that must be performed in memory before the output is returned. In-memory sorts may impact performance for large data sets. Use an [indexed sort](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-indexed-sort) to avoid an in-memory sort.

See [Sort and Index Use](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/cursor.sort/#std-label-sort-index-use) for more information on indexed sort operations.

Encryption that secures data when transmitted, stored, and processed, and enables supported queries on that encrypted data. MongoDB provides two approaches to In-Use Encryption: [Queryable Encryption](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/queryable-encryption/#std-label-qe-manual-feature-qe) and [Client-Side Field Level Encryption](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/csfle/#std-label-manual-csfle-feature).

A data structure that optimizes queries. See [Indexes](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/indexes/).

The range of index values that MongoDB searches when using an index to run a query. To learn more, see [Multikey Index Bounds](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/indexes/index-types/index-multikey/multikey-index-bounds/#std-label-multikey-index-bounds).

The combination of parameters that uniquely identify the index.

A sort where an index provides the sorted result. Sort operations that use an index often have better performance than an [in-memory sort](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-in-memory-sort). See [Use Indexed to Sort Query Results](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/sort-results-with-indexes/#std-label-sorting-with-indexes) for more information.

A shell script used by a Linux platform's [init system](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-init-system) to start, restart, or stop a [daemon](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-daemon) process. If you installed MongoDB using a package manager, an init script is provided for your system as part of the installation. See the respective [Installation Guide](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/installation/#std-label-tutorial-installation) for your operating system.

The init system is the first process started on a Linux platform after the kernel starts, and manages all other processes on the system. The init system uses an [init script](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-init-script) to start, restart, or stop a [daemon](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-daemon) process, such as [`mongod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/#mongodb-binary-bin.mongod) or [`mongos`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongos/#mongodb-binary-bin.mongos). Recent Linux versions typically use the **systemd** init system and the `systemctl` command. Older Linux versions typically use the **System V** init system and the `service` command. See the Installation Guide for your operating system.

The [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) operation that replicates data from an existing replica set member to a new replica set member. See [Initial Sync](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-sync/#std-label-replica-set-initial-sync).

A [lock](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-lock) on a resource that indicates the lock holder will read from (intent shared) or write to (intent exclusive) the resource using [concurrency control](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-concurrency-control) at a finer granularity than that of the resource with the intent lock. Intent locks allow concurrent readers and writers of a resource. See [What type of locking does MongoDB use?](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/faq/concurrency/#std-label-faq-concurrency-locking).

AWS (Amazon Web Services) VPC (Virtual Private Cloud) endpoint with a private IP address that sends traffic to the MongoDB Atlas private endpoint service over AWS (Amazon Web Services) PrivateLink.

[Learn About Private Endpoints in Atlas](https://www.mongodb.com/docs/atlas/security-private-endpoint/#std-label-private-endpoint-concepts)

A point in an operation when it can safely end. MongoDB only ends an operation at designated interrupt points. See [Terminate Running Operations](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/terminate-running-operations/).

List of IP (Internet Protocol) addresses and CIDR (Classless Inter-Domain Routing) blocks with access to clusters within an MongoDB Atlas [project](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-project). For client connections over the public Internet, MongoDB Atlas allows connections to a cluster only from entries in the corresponding project's IP access list. The access list may have up to 200 entries.

MongoDB Atlas also allows client connections over nonpublic networking, such [network peering connections](https://www.mongodb.com/docs/atlas/security-vpc-peering/#std-label-vpc-peering) or private endpoints. These types of connections work irrespective of the IP access list. To learn more, see [Set Up a Network Peering Connection](https://www.mongodb.com/docs/atlas/security-vpc-peering/#std-label-vpc-peering) and [Learn About Private Endpoints in Atlas](https://www.mongodb.com/docs/atlas/security-private-endpoint/#std-label-private-endpoint-concepts).

A revision to the IP (Internet Protocol) standard with a large address space to support Internet hosts.

The international date format used by [`mongosh`](https://www.mongodb.com/docs/mongodb-shell/#mongodb-binary-bin.mongosh) to display dates. The format is `YYYY-MM-DD HH:MM.SS.millis`.

A scripting language. [mongosh](https://www.mongodb.com/docs/mongodb-shell/), the legacy [`mongo`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/mongo/#mongodb-binary-bin.mongo) shell, and certain server functions use a JavaScript interpreter. See [Server-side JavaScript](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/server-side-javascript/) for more information.

A sequential, binary transaction log used to bring the database into a valid state in the event of a hard shutdown. Journaling writes data first to the journal and then to the core data files. Journal files are pre-allocated and exist as files in the [data directory](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-data-directory). See [Journaling](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/journaling/).

JavaScript Object Notation. A plain text format for expressing structured data with support in many programming languages. For more information, see [http://www.json.org](http://www.json.org). Certain MongoDB tools render an approximation of MongoDB [BSON](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-BSON) documents in JSON format. See [MongoDB Extended JSON (v2)](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/mongodb-extended-json/).

A [JSON](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-JSON) document is a collection of fields and values in a structured format. For sample JSON documents, see [http://json.org/example.html](http://json.org/example.html).

A string prefixed with a `/` character that specifies a particular field value in a [JSON document](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-JSON-document).

[JSON](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-JSON) with padding. Refers to a method of injecting JSON into applications. **Presents potential security concerns**.

A [chunk](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-chunk) that grows beyond the [specified chunk size](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-data-partitioning/#std-label-sharding-chunk-size) and cannot split into smaller chunks. For more details, see [Indivisible/Jumbo Chunks](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-data-partitioning/#std-label-jumbo-chunks).

Given a set of points *P* with a defined similarity function *S*, for a query point *q*, finds the set of *k* points in *P* with the best values of *S*(*p*, *q*). Vector Search ENN (Exact Nearest Neighbor) search returns the exact top *k* points and ANN (Approximate Nearest Neighbor) search returns *k* points that are similar to *q*, but not necessarily the *k* most similar to *q*.

The random string of bits used by an encryption algorithm to encrypt and decrypt data.

A MongoDB collection that stores the encrypted [Data Encryption Keys](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-Data-Encryption-Key) as [BSON](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-BSON) documents.

Cross-platform protocol used to authenticate users and authorize them to access data on a cluster. You can use MongoDB Atlas to manage user authentication and authorization from all MongoDB clients using your own LDAP (Lightweight Directory Access Protocol) server over TLS (Transport Layer Security). A single LDAPS (Secure Lightweight Directory Access Protocol) configuration applies to all clusters in an MongoDB Atlas project.

An authorization policy that grants a user only the access that is essential to that user's work.

The format used for [geospatial](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-geospatial) data before MongoDB version 2.4. This format stores geospatial data as points on a planar coordinate system (for example, `[ x, y ]`). See [Geospatial Queries](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/geospatial-queries/).

A LineString is an array of two or more positions. A closed LineString with four or more positions is called a LinearRing, as described in the GeoJSON LineString specification: [https://tools.ietf.org/html/rfc7946#section-3.1.4](https://tools.ietf.org/html/rfc7946#section-3.1.4). To use a LineString in MongoDB, see [GeoJSON Objects](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/geojson/#std-label-geospatial-indexes-store-geojson).

String that contains the information necessary to connect from Cloud Manager or Ops Manager to MongoDB Atlas during a live migration from a Cloud Manager or Ops Manager deployment to a cluster in MongoDB Atlas.

A byte order in which the least significant byte (little end) of a multibyte data value is stored at the lowest memory address.

Process to seamlessly move an existing source replica set or sharded cluster to MongoDB Atlas. During the live migration process, MongoDB Atlas keeps the target cluster in sync with the remote source until you cut your applications over to the MongoDB Atlas cluster.

[Migrate or Import Data](https://www.mongodb.com/docs/atlas/import/#std-label-import-strategies)

MongoDB uses locks to ensure that [concurrency](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/faq/concurrency/#std-label-faq-concurrency) does not affect correctness. MongoDB uses [read locks](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-read-lock), [write locks](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-write-lock) and [intent locks](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-intent-lock). For more information, see [What type of locking does MongoDB use?](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/faq/concurrency/#std-label-faq-concurrency-locking).

Contain server events, such as incoming connections, commands run, and issues encountered. For more details, see [Log Messages](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/log-messages/#std-label-log-messages-ref).

Logical volume manager. LVM is a program that abstracts disk images from physical devices and provides a number of raw disk manipulation and snapshot capabilities useful for system management. For information on LVM and MongoDB, see [Back Up and Restore Using LVM on Linux](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/backup-with-filesystem-snapshots/#std-label-lvm-backup-and-restore).

Day and time of the week when MongoDB Atlas should start weekly maintenance on your cluster. You can set your maintenance window in your [Project Settings](https://www.mongodb.com/docs/atlas/tutorial/manage-project-settings/#std-label-project-settings).

Urgent Maintenance Activities Urgent maintenance activities such as security patches cannot wait for your chosen window. MongoDB Atlas will start those maintenance activities when needed.

Ongoing Maintenance Operations Once maintenance is scheduled for your cluster, you cannot change your maintenance window until the current maintenance efforts have completed.

Maintenance Requires Replica Set Elections MongoDB Atlas performs maintenance the same way as the maintenance procedure described in the [MongoDB Manual](https://www.mongodb.com/docs/manual/tutorial/perform-maintence-on-replica-set-members/). This procedure requires at least one [replica set election](https://www.mongodb.com/docs/manual/core/replica-set-elections/) during the maintenance window per replica set.

Maintenance Starts As Close to the Hour As Possible Maintenance always begins as close to the scheduled hour as possible, but in-progress cluster updates or unexpected system issues could delay the start time.

An aggregation process that has a "map" phase that selects the data and a "reduce" phase that transforms the data. In MongoDB, you can run arbitrary aggregations over data using map-reduce. For the map-reduce implementation, see [Map-Reduce](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/map-reduce/). For all approaches to aggregation, see [Aggregation Operations](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/aggregation/#std-label-aggregation).

A structure in programming languages that associate keys with values. Keys may contain embedded pairs of keys and values (for example, dictionaries, hashes, maps, and associative arrays). The properties of these structures depend on the language specification and implementation. Typically, the order of keys in mapping types is arbitrary and not guaranteed.

A hashing algorithm that calculates a [checksum](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-checksum) for the supplied data. The algorithm returns a unique value to identify the data. MongoDB uses md5 to identify chunks of data for [GridFS](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-GridFS). See [filemd5 (database command)](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/filemd5/).

Average of a set of numbers.

In a dataset, the median is the percentile value where 50% of the data falls at or below that value.

An individual [mongod](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-mongod) process. A [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) has multiple members. A member is also known as a [node](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-node).

In [Queryable Encryption](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/queryable-encryption/#std-label-qe-manual-feature-qe), the internal collections MongoDB uses to enable querying on encrypted fields. See [Metadata Collections](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/queryable-encryption/fundamentals/manage-collections/#std-label-qe-metadata-collections).

Multipurpose Internet Mail Extensions. A standard set of type and encoding definitions used to declare the encoding and type of data in multiple data storage, transmission, and email contexts. The [`mongofiles`](https://www.mongodb.com/docs/database-tools/mongofiles/#mongodb-binary-bin.mongofiles) tool provides an option to specify a MIME type to describe a file inserted into [GridFS](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-GridFS) storage.

Number that occurs most frequently in a set of numbers.

The legacy MongoDB shell. The [`mongo`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/mongo/#mongodb-binary-bin.mongo) process starts the legacy shell as a [daemon](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-daemon) connected to either a [`mongod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/#mongodb-binary-bin.mongod) or [`mongos`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongos/#mongodb-binary-bin.mongos) instance. The shell has a JavaScript interface.

Starting in MongoDB v5.0, [`mongo`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/mongo/#mongodb-binary-bin.mongo) is deprecated and [mongosh](https://www.mongodb.com/docs/mongodb-shell/) replaces [`mongo`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/mongo/#mongodb-binary-bin.mongo) as the client shell. See [mongosh](https://www.mongodb.com/docs/mongodb-shell/).

The MongoDB database server. The [`mongod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/#mongodb-binary-bin.mongod) process starts the MongoDB server as a [daemon](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-daemon). The MongoDB server manages data requests and background operations. See [`mongod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/).

Visualization tool for your MongoDB Atlas data. You can launch MongoDB Charts from your MongoDB Atlas cluster and view your data with the Charts application to begin visualizing your data.

[MongoDB Charts](https://www.mongodb.com/docs/charts/)

Fine-grained text indexing enabling advanced text search on your data without any additional required management. MongoDB Search provides options for several kinds of [text analyzers](https://www.mongodb.com/docs/atlas/atlas-search/analyzers/#std-label-analyzers-ref), score-based results ranking, and a rich [query language](https://www.mongodb.com/docs/atlas/atlas-search/query-ref/#std-label-query-syntax-ref).

[MongoDB Search Overview](https://www.mongodb.com/docs/atlas/atlas-search/#std-label-fts-top-ref)

The MongoDB sharded cluster query router. The [`mongos`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongos/#mongodb-binary-bin.mongos) process starts the MongoDB router as a [daemon](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-daemon). The MongoDB router acts as an interface between an application and a MongoDB [sharded cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster) and handles all routing and load balancing across the cluster. See [`mongos` Instances](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongos/).

MongoDB Shell. [mongosh](https://www.mongodb.com/docs/mongodb-shell/) provides a shell interface to either a [`mongod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/#mongodb-binary-bin.mongod) or a [`mongos`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongos/#mongodb-binary-bin.mongos) instance.

Starting in MongoDB v5.0, [mongosh](https://www.mongodb.com/docs/mongodb-shell/) replaces [`mongo`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/mongo/#mongodb-binary-bin.mongo) as the preferred shell.

MongoDB Atlas [cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-cluster) spanning multiple geographic regions. Multi-region clusters can increase availability and improve performance by routing application queries to the most appropriate geographic regions.

Multi-region clusters must contain [electable nodes](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-electable-node).

Multi-region clusters may contain [read-only nodes](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-read-only-node) and [analytics nodes](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-analytics-node).

A namespace is a combination of the database name and the name of the collection or index: `<database-name>.<collection-or-index-name>`. All documents belong to a namespace. See [Namespaces](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/limits/#std-label-faq-dev-namespace).

MongoDB Atlas tool that monitors collection-level [query latency](https://www.mongodb.com/docs/manual/reference/operator/aggregation/collStats/#latencystats-document). You can view query latency metrics and statistics for certain hosts and operation types. Manage pinned namespaces and choose up to five namespaces to show in the corresponding query latency charts.

[Monitor Collection-Level Query Latency with Namespace Insights](https://www.mongodb.com/docs/atlas/namespace-insights/#std-label-namespace-insights)

The order `recordIds` are created and stored in the [WiredTiger](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/wiredtiger/#std-label-storage-wiredTiger) index. The default sort order for a [collection scan](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-collection-scan) run on a single instance is natural order.

In replica sets, natural order is not guaranteed to be consistent and can differ between members.

In sharded collections, natural order is not defined. However, using [`$natural`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/cursor.hint/#mongodb-operator-metaOp.-natural) still forces each shard to perform a collection scan.

For details, see [Return in Natural Order](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/cursor.sort/#std-label-return-natural-order).

A network failure that separates a distributed system into partitions such that nodes in one partition cannot communicate with the nodes in the other partition.

Sometimes, partitions are partial or asymmetric. An example partial partition is the a division of the nodes of a network into three sets, where members of the first set cannot communicate with members of the second set, and the reverse, but all nodes can communicate with members of the third set.

In an asymmetric partition, communication may be possible only when it originates with certain nodes. For example, nodes on one side of the partition can communicate with the other side only if they originate the communications channel.

Process by which two Internet networks connect and exchange traffic. You can directly peer your VPC (Virtual Private Cloud) with the MongoDB Atlas
VPC (Virtual Private Cloud) created for your MongoDB clusters. Using network peering, your application servers can directly connect to MongoDB Atlas while remaining isolated from public networks.

[Set Up a Network Peering Connection](https://www.mongodb.com/docs/atlas/security-vpc-peering/#std-label-vpc-peering)

An individual [mongod](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-mongod) process. A [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) has multiple nodes. A node is also known as a [member](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-member).

No Operation (noop), is an I/O operation scheduler that allocates I/O bandwidth for incoming processes based on a first in, first out queue.

The [Unicode Normalized Form](https://unicode.org/reports/tr15/) of a string applies Unicode code points in a standardized manner. Two strings may appear identical to a user, but have differences such as the order of combining marks. Normalizing the strings ensures they have the same binary representation.

NVMe (Non-Volatile Memory Express) is a protocol for accessing high-speed storage media.

*Available for M40+ clusters hosted on AWS*

For applications hosted on AWS (Amazon Web Services) which require low-latency and high-throughput IO, you can use the NVMe [cluster class](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-cluster-class). The NVMe cluster class leverages a unique data protocol to greatly improve data access speeds.

NVMe clusters use a [hidden secondary node](https://www.mongodb.com/docs/manual/core/replica-set-hidden-member/) consisting of a provisioned volume with high throughput and IOPS (Input/Output Operations per Second) to facilitate backup.

[Customize Cluster Storage](https://www.mongodb.com/docs/atlas/customize-storage/#std-label-create-cluster-storage)

See [ObjectId](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-ObjectId).

A 12-byte [BSON](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-BSON) type that is unique within a [collection](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-collection). The ObjectId is generated using the timestamp, computer ID, process ID, and a local process incremental counter. MongoDB uses ObjectId values as the default values for [_id](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-_id) fields.

See [oplog](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-oplog).

Information about the execution of processes rather than their content, such as the number and time of insert, update, and delete operations.

A rejected [query shape](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/query-shapes/#std-label-query-shapes). For more details, see [Block Slow Queries with Operation Rejection Filters](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/operation-rejection-filters/#std-label-operation-rejection-filters).

See [optime](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-optime).

Any [electable node](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-electable-node) or a [read-only node](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-read-only-node) in your MongoDB Atlas cluster.

A keyword beginning with `$` used to express MQL (MongoDB Query Language) components such as query predicates, expressions, and aggregation stages. For example, `$gt` is the MQL "greater than" operator. For available operators, see [MongoDB Query Language Reference](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/mql/#std-label-mql-reference).

A [capped collection](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-capped-collection) that stores an ordered history of logical writes to a MongoDB database. The oplog is the basic mechanism enabling [replication](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replication) in MongoDB. See [Replica Set Oplog](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-oplog/).

A temporary collection created during [resharding](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-reshard-a-collection/#std-label-sharding-resharding) operations that stores [oplog](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-oplog) entries from a donor shard.

Oplog buffer collections ensure that recipient shards can access oplog entries when they get deleted from the donor shard. Oplog buffer collections are removed when resharding is complete.

A temporary gap in the oplog because the oplog writes aren't in sequence. Replica set [primaries](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-primary/#std-label-replica-set-primary) apply oplog entries in parallel as a batch operation. As a result, temporary gaps in the oplog can occur from entries that aren't yet written from a batch.

[oplog](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-oplog) entries are time-stamped. The oplog window is the time difference between the newest and the oldest timestamps in the `oplog`. If a secondary node loses connection with the primary, it can only use [replication](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/replication/#std-label-replication) to sync up again if the connection is restored within the oplog window.

A reference to a position in the replication [oplog](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-oplog). The optime value is a document that contains:

- `ts`, the [Timestamp](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/bson-types/#std-label-document-bson-type-timestamp) of the operation.

- `t`, the [`term`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/replSetGetStatus/#mongodb-data-replSetGetStatus.term) in which the operation was originally generated on the primary.

A query plan that returns results in the order consistent with the [`sort()`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/cursor.sort/#mongodb-method-cursor.sort) order. See [Query Plans](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/query-plans/#std-label-read-operations-query-optimization).

Logical grouping of MongoDB Atlas [projects](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-project). You can leverage an organization to manage billing, users, and security settings for the projects it contains.

- Billing happens at the organization level while preserving visibility into usage in each project.

- You can view all projects within an organization.

- You can use teams to bulk assign organization users to projects within the organization.

[Organizations](https://www.mongodb.com/docs/atlas/organizations-projects/#std-label-organizations)

Unique 24-digit hexadecimal string used to identify your MongoDB Atlas [organization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-organization). The [Return All Organizations](https://www.mongodb.com/docs/api/doc/atlas-admin-api-v2/operation/operation-listorgs) endpoint returns the ID of all organizations that the authenticated user executing the API (Application Programming Interface) call can access.

A cursor that is not correctly closed or iterated over in your application code. Orphaned cursors can cause performance issues in your MongoDB deployment.

In a sharded cluster, orphaned documents are those documents on a shard that also exist in chunks on other shards. This is caused by a failed migration or an incomplete migration cleanup because of an atypical shutdown.

Orphaned documents are cleaned up automatically after a chunk migration completes. You no longer need to run `cleanupOrphaned` to delete orphaned documents.

A member of a [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) that cannot become primary because its [`members[n].priority`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/replica-configuration/#mongodb-rsconf-rsconf.members-n-.priority) is `0`. See [Priority 0 Replica Set Members](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-priority-0-member/).

A type of cache that locally stores memory for a specific CPU core. Per-CPU caches are used by the new version of TCMalloc, which is introduced in MongoDB 8.0.

A type of cache that locally stores memory for each application thread. Per-thread caches are used by the legacy version of TCMalloc, which is used in MongoDB 7.0 and earlier.

In a dataset, a percentile is a value where that percentage of the data is at or below the specified value. For details, see [Calculation Considerations](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/operator/aggregation/percentile/#std-label-percentile-calculation-considerations).

MongoDB Atlas tool that monitors slow queries executed on your cluster and suggests indexes to improve query performance. Each index that the Performance Advisor suggests include an [impact](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-impact) score indicating the potential performance improvement that index would bring.

[Monitor and Improve Slow Queries with the Performance Advisor](https://www.mongodb.com/docs/atlas/performance-advisor/#std-label-performance-advisor)

A process identifier. UNIX-like systems assign a unique-integer PID to each running process. You can use a PID to inspect a running process and send signals to it. See [`/proc` File System](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/ulimit/#std-label-proc-file-system).

A communication channel in UNIX-like systems allowing independent processes to send and receive data. In the UNIX shell, piped operations allow users to direct the output of one command into the input of another.

A series of operations in an [aggregation](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-aggregation). See [Aggregation Pipeline](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/aggregation-pipeline/#std-label-aggregation-pipeline).

A combination of query predicate, sort, [projection](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/project-fields-from-query-results/#std-label-projection), and [collation](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/collation/#std-label-collation). The plan cache query shape allows MongoDB to identify equivalent queries and analyze their performance.

For the query predicate, only the predicate structure and field names are used. The values in the query predicate aren't used. For example, a query predicate `{ type: 'food' }` is equivalent to `{ type: 'drink' }`.

To identify slow queries with the same plan cache query shape, each plan cache query shape has a hexadecimal `planCacheShapeHash` value. For more information, see [planCacheShapeHash and planCacheKey](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/query-plans/#std-label-query-hash-plan-cache-key).

Starting in MongoDB 8.0, the existing `queryHash` field is duplicated in a new field named `planCacheShapeHash`. If you're using an earlier MongoDB version, you'll only see the `queryHash` field. Future MongoDB versions will remove the deprecated `queryHash` field, and you'll need to use the `planCacheShapeHash` field instead.

A single coordinate pair as described in the GeoJSON Point specification: [https://tools.ietf.org/html/rfc7946#section-3.1.2](https://tools.ietf.org/html/rfc7946#section-3.1.2). To use a Point in MongoDB, see [GeoJSON Objects](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/geojson/#std-label-geospatial-indexes-store-geojson).

An array of [LinearRing](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-LineString) coordinate arrays, as described in the GeoJSON Polygon specification: [https://tools.ietf.org/html/rfc7946#section-3.1.6](https://tools.ietf.org/html/rfc7946#section-3.1.6). For Polygons with multiple rings, the first must be the exterior ring and any others must be interior rings or holes.

MongoDB does not permit the exterior ring to self-intersect. Interior rings must be fully contained within the outer loop and cannot intersect or overlap with each other. See [GeoJSON Objects](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/geojson/#std-label-geospatial-indexes-store-geojson).

A document after it was inserted, replaced, or updated. See [Change Streams with Document Pre- and Post-Images](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/db.collection.watch/#std-label-db.collection.watch-change-streams-pre-and-post-images-example).

A setting for each collection that allocates space for each [document](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-document) to maximize storage reuse and reduce fragmentation. `powerOf2Sizes` is the default for [TTL Collections](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/expire-data/#std-label-ttl-collections). To change collection settings, see [`collMod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/collMod/#mongodb-dbcommand-dbcmd.collMod).

A document before it was replaced, updated, or deleted. See [Change Streams with Document Pre- and Post-Images](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/db.collection.watch/#std-label-db.collection.watch-change-streams-pre-and-post-images-example).

An operation performed before inserting data that divides the range of possible shard key values into chunks to facilitate easy insertion and high write throughput. In some cases pre-splitting expedites the initial distribution of documents in [sharded cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster) by manually dividing the collection rather than waiting for the MongoDB [balancer](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-balancer) to do so. See [Create Ranges in a Sharded Cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/create-chunks-in-sharded-cluster/).

Reduces memory and disk consumption by storing any identical index key prefixes only once, per page of memory. See: [Compression](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/wiredtiger/#std-label-storage-wiredtiger-compression) for more about WiredTiger's compression behavior.

In a [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set), the primary is the member that receives all write operations. See [Primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-members/#std-label-replica-set-primary-member).

A record's unique immutable identifier. In [RDBMS](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-RDBMS) software, the primary key is typically an integer stored in each row's `id` field. In MongoDB, the [_id](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-_id) field stores a document's primary key, which is typically a BSON [ObjectId](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-ObjectId).

Each database in a sharded cluster has a primary shard. It is the default shard for all unsharded collections in the database. See [Primary Shard](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharded-cluster-shards/#std-label-primary-shard).

A configurable value that helps determine which members in a [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) are most likely to become [primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary). See [`members[n].priority`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/replica-configuration/#mongodb-rsconf-rsconf.members-n-.priority).

A combination of specified [resource](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-resource) and [actions](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-action) permitted on the resource. See [privilege](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/authorization/#std-label-privileges).

Logical grouping of [clusters](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-deployment). You can have multiple clusters within a single project and multiple projects within a single [organization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-organization).

Project is synonymous with [group](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-group).

Unique 24-digit hexadecimal string used to identify your MongoDB Atlas [project](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-project). The [Get All Projects](https://www.mongodb.com/docs/api/doc/atlas-admin-api-v2/operation/operation-listgroups) API (Application Programming Interface) endpoint returns the ID of all projects that the authenticated user executing the API call can access.

Project ID is synonymous with group ID.

A document supplied to a [query](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-query) that specifies the fields MongoDB returns in the result set. For more information about projections, see [Project Fields to Return from Query](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/project-fields-from-query-results/#std-label-projection).

Method of compressing the value of individual dimensions in a vector into a smaller range to reduce resource consumption and improve speed. Vector Search supports indexing and querying quantized vectors.

A read request. MongoDB uses a [JSON](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-JSON) form of query language that includes [query operators](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-operator) with names that begin with a `$` character. In [`mongosh`](https://www.mongodb.com/docs/mongodb-shell/#mongodb-binary-bin.mongosh), you can run queries using the [`db.collection.find()`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/db.collection.find/#mongodb-method-db.collection.find) and [`db.collection.findOne()`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/db.collection.findOne/#mongodb-method-db.collection.findOne) methods. See [Query Documents](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/query-documents/#std-label-read-operations-queries).

A combination of the [query optimizer](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-query-optimizer) and query execution engine that processes an operation.

A keyword beginning with `$` in a query. For example, [`$gt`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/operator/query/gt/#mongodb-query-op.-gt) is the "greater than" operator. For a list of query operators, see [query operators](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/mql/query-predicates/#std-label-query-selectors).

A process that generates query plans. For each query, the optimizer generates a plan that matches the query to the index that returns the results as efficiently as possible. The optimizer reuses the query plan each time the query runs. If a collection changes significantly, the optimizer creates a new query plan. See [Query Plans](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/query-plans/#std-label-read-operations-query-optimization).

Most efficient execution plan chosen by the query planner. For more details, see [Query Plans](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/query-plans/#std-label-query-plans-query-optimization).

An expression that returns a boolean indicating whether a document matches the specified query. For example, `{ name: { $eq: "Alice" } }`, which returns documents that have a field `"name"` whose value is the string `"Alice"`.

Query predicates can contain child expressions and operators for more complex matching. To see available query operators, see [Query Predicates](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/mql/query-predicates/#std-label-query-projection-operators-top).

MongoDB Atlas tool that diagnoses and monitors performance issues in your cluster. The Query Profiler can expose long-running queries and their performance statistics. You can filter the data returned by the Query Profiler to hone in on specific namespaces and operation types.

A **query shape** is a set of specifications that group similar queries. For details, see [Query Shapes](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/query-shapes/#std-label-query-shapes).

A contiguous range of [shard key](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-shard-key) values within a chunk. Data ranges include the lower boundary and exclude the upper boundary. MongoDB migrates data when a shard contains [too much data of a collection](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-balancer-administration/#std-label-sharding-migration-thresholds) relative to other shards. See [Data Partitioning with Chunks](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-data-partitioning/#std-label-sharding-data-partitioning) and [Sharded Cluster Balancer](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-balancer-administration/#std-label-sharding-balancing).

Relational Database Management System. A database management system based on the relational model, typically using [SQL](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-SQL) as the query language.

Specifies a level of isolation for read operations. For example, you can use read concern to only read data that has propagated to a majority of nodes in a [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set). See [Read Concern](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/read-concern/).

A shared [lock](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-lock) on a resource such as a collection or database that, while held, allows concurrent readers but no writers. See [What type of locking does MongoDB use?](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/faq/concurrency/#std-label-faq-concurrency-locking).

A setting that determines how clients direct read operations. Read preference affects all replica sets, including shard replica sets. By default, MongoDB directs reads to [primaries](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary). However, you may also direct reads to secondaries for [eventually consistent](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-eventual-consistency) reads. See [Read Preference](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/read-preference/#std-label-read-preference).

Replica set in a dedicated geographic region that supplements your [electable node](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-electable-node) regions. You can use read-only nodes to localize data where it is most frequently read to improve performance.

MongoDB Atlas monitoring service that displays current network traffic, database operations on your clusters, and hardware statistics about your host machines. Use the RTPP (Real-Time Performance Panel) to visually evaluate query execution times, monitor network activity, and discover potential replication lag on secondary members of replica sets.

[Monitor Real-Time Performance](https://www.mongodb.com/docs/atlas/real-time-performance-panel/#std-label-real-time-metrics-status-tab)

Measures the fraction of true nearest neighbors that were returned by an ANN (Approximate Nearest Neighbor) search. This measure reflects how close the algorithm approximates the results of ENN (Exact Nearest Neighbor) search. The notation *Recall@k* refers to the measurement of how many of the true nearest neighbors were present in the top *k* results returned by Vector Search.

A [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) member status indicating that a member is not ready to begin activities of a secondary or primary. Recovering members are unavailable for reads.

The CPU utilization relative to the amount of baseline CPU assigned to a cloud instance. You can calculate relative system CPU utilization by dividing the [absolute system CPU utilization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-absolute-system-CPU-utilization) by the amount of baseline CPU assigned to a cloud instance.

MongoDB caps relative system CPU utilization at 100%. When a cloud provider throttles CPU utilization for a cloud instance, or bursts CPU utilization for an instance above the baseline amount of CPU available to that instance, the relative system CPU value is 100%.

See also [absolute system CPU utilization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-absolute-system-CPU-utilization), and [burstable instances](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-burstable-instances).

Group of MongoDB servers that maintain the same data set. Replica sets provide redundancy, high availability, and are the basis for all production deployments.

A feature allowing multiple database servers to share the same data. Replication ensures data redundancy and enables load balancing. See [Replication](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/replication/).

The time period between the last operation in the [primary's](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary) [oplog](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-oplog) and the last operation applied to a particular [secondary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-secondary). You typically want replication lag as short as possible. See [Replication Lag](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/troubleshoot-replica-sets/#std-label-replica-set-replication-lag).

The subset of an application's memory currently stored in physical RAM. Resident memory is a subset of [virtual memory](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-virtual-memory), which includes memory mapped to physical RAM and to storage.

A database, collection, set of collections, or cluster. A [privilege](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-privilege) permits [actions](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-action) on a specified resource. See [resource](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/resource-document/#std-label-resource-document).

A set of privileges that permit [actions](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-action) on specified [resources](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-resource). Roles assigned to a user determine the user's access to resources and operations. See [Security](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/security/).

A process that reverts write operations to ensure the consistency of all replica set members. See [Rollbacks During Replica Set Failover](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-rollbacks/#std-label-replica-set-rollback).

Process that restarts all nodes in the cluster in sequence. To maintain cluster availability, MongoDB Atlas restarts one node at a time starting with a [secondary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-secondary) node. MongoDB Atlas always maintains a primary node until the rolling restart completes.

Scalar quantization involves selecting the minimum and maximum values across all indexed vectors within a segment for each dimension, and producing equally sized bins between them. The mappings for each of these dimensions to the bins yields the new quantized values. Vector Search supports automatic scalar quantization for your float32 vectors, and ingestion and indexing of your scalar quantized vectors from embedding providers.

A [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) member that replicates the contents of the master database. Secondary members may run read requests, but only the [primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary) members can run write operations. See [Secondaries](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-members/#std-label-replica-set-secondary-members).

A database [index](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-index) that improves query performance by minimizing the amount of work that the query engine must perform to run a query. See [Indexes](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/indexes/).

See [secondary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-secondary). Also known as a secondary node.

A seed list is used by drivers and clients (like [`mongosh`](https://www.mongodb.com/docs/mongodb-shell/#mongodb-binary-bin.mongosh)) for initial discovery of the replica set configuration. Seed lists can be provided as a list of `host:port` pairs (see [Standard Connection String Format](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/connection-string-formats/#std-label-connections-standard-connection-string-format) or through DNS entries.) For more information, see [SRV Connection Format](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/connection-string-formats/#std-label-connections-dns-seedlist).

A MongoDB instance that is set up and maintained by an individual or organization, and not an external management or third-party services (such as MongoDB Atlas).

Search for values that have a similar meaning to query. Semantic search captures the natural relationship between words or phrases even when there is no lexical overlap. Semantic search and vector search are often used interchangeably. Vector Search supports semantic search on vector data stored in MongoDB Atlas clusters.

The arbitrary name given to a replica set. All members of a replica set must have the same name specified with the [`replSetName`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/configuration-options/#mongodb-setting-replication.replSetName) setting or the [`--replSet`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/#std-option-mongod.--replSet) option.

A single [`mongod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/#mongodb-binary-bin.mongod) instance or [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) that stores part of a [sharded cluster's](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster) total data set. Typically, in a production deployment, ensure all shards are part of replica sets. See [Shards](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharded-cluster-shards/).

The field MongoDB uses to distribute documents among members of a [sharded cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster). See [Shard Keys](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-shard-key/#std-label-shard-key).

The set of nodes comprising a [sharded](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharding) MongoDB deployment. A sharded cluster consists of config servers, shards, and one or more [`mongos`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongos/#mongodb-binary-bin.mongos) routing processes. See [Sharded Cluster Components](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharded-cluster-components/).

A database architecture that partitions data by key ranges and distributes the data among two or more database instances. Sharding enables horizontal scaling. See [Sharding](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/sharding/).

A method in `mongosh` that has a concise syntax for a [database command](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/#std-label-database-commands). Shell helpers improve the interactive experience. See [`mongosh` Methods](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/).

Measures the similarity between two vectors. Vector Search supports `euclidean`, `cosine`, and `dotProduct` similarity functions.

A [replication](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replication) topology where only a single database instance accepts writes. Single-master replication ensures consistency and is the replication topology used by MongoDB. See [Replica Set Primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-primary/).

A compression/decompression library to balance efficient computation requirements with reasonable compression rates. Snappy is the default compression library for MongoDB's use of [WiredTiger](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/wiredtiger/#std-label-storage-wiredtiger). See [Snappy](https://google.github.io/snappy/) and the [WiredTiger compression documentation](http://source.wiredtiger.com/mongodb-5.0/compression.html) for more information.

A [snapshot](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-snapshot) is a copy of the data in a [`mongod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/#mongodb-binary-bin.mongod) instance at a specific point in time. You can retrieve snapshot metadata for the whole cluster or replica set, or for a single config server in a cluster.

The CPU utilization metric that reflects a portion of CPU that a cloud instance currently uses to process software interrupt requests. On some cloud providers, this metric is useful for tracking CPU utilization on [burstable instances](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-burstable-instances).

The value compared against when sorting fields. To learn how MongoDB determines the sort key for non-numeric fields, see [Comparison/Sort Order](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/bson-type-comparison-order/#std-label-bson-types-comparison-order).

The division between [chunks](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-chunk) in a [sharded cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster). See [Data Partitioning with Chunks](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-data-partitioning/).

Structured Query Language (SQL) is used for interaction with relational databases.

Solid State Disk. High-performance storage that uses solid state electronics for persistence instead of rotating platters and movable read/write heads used by mechanical hard drives.

A stale read refers to when a transaction reads old (stale) data that has been modified by another transaction but not yet committed to the database.

An instance of [`mongod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/#mongodb-binary-bin.mongod) that runs as a single server and not as part of a [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set). To convert a standalone instance to a replica set, see [Convert a Standalone Self-Managed mongod to a Replica Set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/convert-standalone-to-replica-set/#std-label-server-replica-set-deploy-convert).

A standalone instance is **not** a replica set with only one member.

A temporary collection created on the recipient shard for each donor shard during [resharding](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/sharding-reshard-a-collection/#std-label-sharding-resharding) operations.

Stash collections temporarily hold documents that cannot be immediately inserted due to operation conflicts. For example, if a document's shard key has been updated, it now belongs to a different shard, and the order of operations applied to this document can be ambiguous. The recipient stores these documents in a stash collection until it can apply operations in the correct order.

The [primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary) member of the replica set removes itself as primary and becomes a [secondary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-secondary) member.

- If a replica set loses contact with the primary, the secondaries elect a new primary.  When the old primary learns of the election, it steps down and rejoins the replica set as a secondary.

- If the user runs the [`replSetStepDown`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/command/replSetStepDown/#mongodb-dbcommand-dbcmd.replSetStepDown) command, the primary steps down, forcing the replica set to elect a new primary.

The part of a database that is responsible for managing how data is stored and accessed, both in memory and on disk. Different storage engines perform better for specific workloads. See [Storage Engines for Self-Managed Deployments](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/storage-engines/) for specific details on the built-in storage engines in MongoDB.

See [natural order](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-natural-order).

A property of a distributed system requiring that all members contain the latest changes to the system. In a database system, this means that any system that can provide data must contain the latest writes.

Subject Alternative Name (SAN) is an extension of the X.509 certificate which allows an array of values such as IP addresses and domain names that specify the resources a single security certificate may secure.

The [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) operation where members replicate data from the [primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary). Sync first occurs when MongoDB creates or restores a member, which is called [initial sync](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-initial-sync). Sync then occurs continually to keep the member updated with changes to the replica set's data. See [Replica Set Data Synchronization](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/replica-set-sync/).

On UNIX-like systems, a logging process that provides a uniform standard for servers and processes to submit logging information. MongoDB provides an option to send output to the host's syslog system. See [`syslogFacility`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/configuration-options/#mongodb-setting-systemLog.syslogFacility).

A label applied to a replica set member and used by clients to issue data-center-aware operations. For more information on using tags with replica sets, see [Read Preference Tag Set Lists](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/read-preference-tags/#std-label-replica-set-read-preference-tag-sets).

Sharded cluster [zones](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-zone) replace [tags](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-tag).

A document containing zero or more [tags](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-tag).

For a [capped collection](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-capped-collection), a tailable cursor is a cursor that remains open after the client exhausts the results in the initial cursor. As clients insert new documents into the capped collection, the tailable cursor continues to retrieve documents.

Group of [Atlas users](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-Atlas-user) in the same organization. You can use teams to grant access to the same group of Atlas users across multiple [projects](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-project). All users in the team share the same project access.

For the members of a replica set, a monotonically increasing number that corresponds to an election attempt.

A [collection](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-collection) that efficiently stores sequences of measurements over a period of time. See [Time Series](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/timeseries-collections/).

The state of a deployment of MongoDB instances. Includes:

- Type of deployment (standalone, replica set, or sharded cluster).

- Availability of servers.

- Role of each server ([primary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-primary), [secondary](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-secondary), [config server](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-config-server), or [`mongos`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongos/#mongodb-binary-bin.mongos)).

Group of read or write operations. For details, see [Transactions](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/transactions/#std-label-transactions).

A component of MongoDB that manages [transactions](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-transaction) in a [replica set](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set) or a [sharded cluster](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-sharded-cluster). It coordinates the execution and completion of multi-document transactions across nodes and allows a complex operation to be treated as an [atomic operation](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-atomic-operation).

A text-based data format consisting of tab-separated values. This format is commonly used to exchange data between relational databases because the format is suited to tabular data. You can import TSV files using [`mongoimport`](https://www.mongodb.com/docs/database-tools/mongoimport/#mongodb-binary-bin.mongoimport).

Time-to-live (TTL) is an expiration time or period for a given piece of information to remain in a cache or other temporary storage before the system deletes it or ages it out. MongoDB has a TTL collection feature. See [Expire Data from Collections by Setting TTL](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/tutorial/expire-data/).

An array that consistently grows larger over time. If a document field value is an unbounded array, the array may negatively impact performance. In general, design your schema to avoid unbounded arrays.

An index that enforces uniqueness for a particular field in a single collection. See [Unique Indexes](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/index-unique/#std-label-index-type-unique).

January 1st, 1970 at 00:00:00 UTC. Commonly used in expressing time, where the number of seconds or milliseconds since this point is counted.

A query plan that returns results in an order inconsistent with the [`sort()`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/cursor.sort/#mongodb-method-cursor.sort) order. See [Query Plans](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/query-plans/#std-label-read-operations-query-optimization).

The process of changing a cluster from one version of MongoDB to a newer version.

An option for update operations. For example: [`db.collection.updateOne()`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/db.collection.updateOne/#mongodb-method-db.collection.updateOne), [`db.collection.findAndModify()`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/db.collection.findAndModify/#mongodb-method-db.collection.findAndModify). If upsert is `true`, the update operation either:

- updates the document(s) matched by the query.

- or if no documents match, inserts a new document. The new document has the field values specified in the update operation.

For more information about upserts, see [Insert a New Document if No Match Exists (`Upsert`)](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/method/db.collection.update/#std-label-upsert-parameter).

System that stores vector embeddings and associated metadata, and enables nearest neighbor search on the stored vector embeddings. You can use MongoDB Atlas as your vector database and Vector Search to perform vector search on the stored vector embeddings. You can use vector database to implement RAG (Retrieval-Augmented Generation).

Data structure that efficiently processes nearest neighbor search queries. Vector Search supports creating indexes of type `vector` to index fields for running [`$vectorSearch`](https://www.mongodb.com/docs/atlas/atlas-vector-search/vector-search-stage/#mongodb-pipeline-pipe.-vectorSearch) queries.

Feature that allows you to perform semantic search on vector embeddings by comparing query vectors with indexed vectors to find the closest match.

Method of performing *k* nearest neighbor search over a set of vectors stored in a vector index. Vector Search supports ANN (Approximate Nearest Neighbor) and ENN (Exact Nearest Neighbor) search for *k* nearest neighbors.

An application's working memory, typically residing on both disk and in physical RAM.

The time that elapses between the start and completion of a computer program or calculation.

The default reference system and geodetic datum that MongoDB uses to calculate geometry over an Earth-like sphere for geospatial queries on [GeoJSON](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-GeoJSON) objects. See the "EPSG:4326: WGS 84" specification: [http://spatialreference.org/ref/epsg/4326/](http://spatialreference.org/ref/epsg/4326/).

Returns values from a span of documents from a collection. See [window operators](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/operator/aggregation/setWindowFields/#std-label-setWindowFields-window-operators).

The data that MongoDB uses most often.

Specifies whether a write operation has succeeded. Write concern allows your application to detect insertion errors or unavailable [`mongod`](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/program/mongod/#mongodb-binary-bin.mongod) instances. For [replica sets](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-replica-set), you can configure write concern to confirm replication to a specified number of members. See [Write Concern](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/write-concern/).

A situation where two concurrent operations, at least one of which is a write, try to use a resource that violates the constraints for a storage engine that uses optimistic [concurrency control](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-concurrency-control). MongoDB automatically ends and retries one of the conflicting write operations.

An exclusive [lock](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-lock) on a resource such as a collection or database. When a process writes to a resource, it takes an exclusive write lock to prevent other processes from writing to or reading from that resource. For more information on locks, see [FAQ: Concurrency](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/faq/concurrency/).

A data compression library that provides higher compression rates at the cost of more CPU, compared to MongoDB's use of [snappy](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-snappy). You can configure [WiredTiger](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/wiredtiger/#std-label-storage-wiredtiger) to use zlib as its compression library. See [http://www.zlib.net](http://www.zlib.net) and the [WiredTiger compression documentation](http://source.wiredtiger.com/mongodb-5.0/compression.html) for more information.

A grouping of documents based on ranges of [shard key](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-shard-key) values for a given sharded collection. Each shard in the sharded cluster can be in one or more zones. In a balanced cluster, MongoDB directs reads and writes for a zone only to those shards inside that zone. See the [Zones](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/core/zone-sharding/#std-label-zone-sharding) manual page for more information.

A data compression library that provides higher compression rates and lower CPU usage when compared to [zlib](https://mongodbcom-cdn.staging.corp.mongodb.com/docs/reference/glossary/#std-term-zlib).


