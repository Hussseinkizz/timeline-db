# Timeline DB

An open source event store, versioned and distributed NoSQL local first database.

## **What is Timeline?**

**Timeline** is a next-generation NoSQL database that takes great inspiration from the best features of Redis, Event Store, Firestore, MongoDB, and Git.

**Timeline** is a distributed, versioned, and local-first database that is designed to be used in a wide range of applications, from simple key-value stores to complex event-sourced systems, collaborative applications, financial systems that need to be audited, transparency and replay of events, and more with great simplicity and developer experience.

**Timeline** is json first and schemaless with optional schema validation and enforcement, giving both flexibility of nosql databases and the structure of sql databases where needed, json is the primary interface of querying which makes it easy to use and understand.

## **How Developers Could Interact with Timeline**

### **1. JSON Query Example**

```json
{
  "find": "users",
  "filter": { "age": { "$gt": 18 } },
  "sort": { "name": 1 },
  "limit": 10
}

```

### **2. Real-Time Subscriptions**

```jsx
const updates = timeline.subscribe('users', { age: { $gt: 18 } });
updates.on('data', (user) => console.log(user));

```

### **3. Version Control**

```jsx
await timeline.branch('experiment-1');
await timeline.write('users', { id: 1, name: 'Alice' });
await timeline.commit('experiment-1');
await timeline.merge('experiment-1', 'main');

```

## Main Features

- **Distributed**: Timeline is designed to be distributed from the ground up with a strong consistency model, it can be used in a single node or a cluster of nodes.
- **Versioned**: Every write operation in Timeline is immutable and a new version of the data, this allows for easy auditing, replaying of events, branching, merging, rollbacks and time travel.
- **Local-first**: Timeline is designed to be used in offline-first applications, it can be used in the browser, mobile, and desktop applications and can sync across devices automatically and on demand.
- **Event Sourcing**: Timeline is designed to be used in event-sourced systems, it can be used to store events, snapshots, and projections.
- **Schemaless**: Timeline is schemaless by default, but it supports optional schema validation and enforcement via foreign keys or otherwise.
- **Json First**: Timeline is designed to be used with json as the primary interface of querying and storing data with some optimizations to aid performance but giving flexibility in use and integration with other tools and languages.
- **Built-in real-time updates** via sockets and streaming to empower collaborative and real time systems and dashboards.
- **AI-ready vector embeddings** for seamless integration with machine learning workflows, embeddings are supported.
- **Local-first applications** support for offline functionality with automatic sync.
- **Efficient storage model** that stores original data and then after only store incremental changes to optimize space with configurable snapshot lifetime and caching for hot data.
- **Configurable data retention and caching** to balance performance and resource usage.
- **Advanced time-range selection and time series based sharding** for scalable storage and retrieval.

## **Why Timeline Would Be Great Deal?**

### **1. Simplified Data Modeling**

- Native JSON support ensures easy integration with existing applications.
- Optional schema enforcement allows developers to choose between flexibility and structure.
- Foreign key relationships provide referential integrity for more complex data models.
- SQL goodies like views, joins, triggers, and stored procedures are supported for advanced use cases.

### **2. Immutable Data for Transparency**

- All writes are stored as immutable events, enabling full audit trails.
- Supports replaying events to rebuild state or debug issues.

### **3. Real-Time Capabilities**

- Built-in WebSocket or gRPC streaming for pushing updates to clients instantly.
- Subscriptions for granular event tracking, making it ideal for live dashboards.

### **4. Version Control**

- Each data change creates a new version, similar to Git commits.
- Developers can branch and merge data, enabling safe experimentation and conflict resolution.

### **5. Scalability and Performance**

- Horizontal scalability with multi-node replication and sharding.
- Eventual consistency ensures high availability in distributed systems.
- JSONB storage ensures fast querying and indexing of structured data.

### **6. AI-Ready Features**

- Store and query vector embeddings for AI and machine learning applications.
- Real-time updates and event sourcing provide a strong foundation for AI-driven systems.

### **7. Security and Accountability**

- Each event is tied to a user or system action, ensuring traceability.
- Support for user authentication and authorization at the event level.
- Built-in encryption and audit trails for secure data management.

### **8. Local-First Applications**

- Offline-first support allows applications to function without network connectivity.
- Automatic conflict resolution and synchronization when reconnected.

### **9. Efficient Storage Model**

- Original data is stored alongside incremental changes.
- Snapshots periodically consolidate changes to optimize storage and improve query performance.

### **10. Configurability and Extensibility**

- Configurable data retention policies for caching and sharding hot data by time range.
- Hooks and extensions allow developers to customize functionality, adding new features seamlessly.

## **Example Use Cases**

### **1. Collaborative Editing**

- **Scenario**: A real-time document editing application.
- **How Timeline Helps**:
    - Tracks changes as immutable events.
    - Enables branching for individual user edits.
    - Merges changes with conflict resolution.

### **2. Live Dashboards**

- **Scenario**: Financial or IoT monitoring systems.
- **How Timeline Helps**:
    - Pushes real-time updates to dashboards via subscriptions.
    - Efficiently processes and streams events from connected devices.

### **3. Audit Trails and Compliance**

- **Scenario**: E-commerce platforms or banking systems.
- **How Timeline Helps**:
    - Immutable events ensure full auditability.
    - Replay functionality helps investigate and debug transactions.

### **4. Versioned Data Analysis**

- **Scenario**: Data science or machine learning workflows.
- **How Timeline Helps**:
    - Allows branching for experimentations.
    - Provides rollback to previous states for reproducibility.

### **5. AI-Driven Applications**

- **Scenario**: A recommendation engine for e-commerce.
- **How Timeline Helps**:
    - Stores and queries vector embeddings alongside user data.
    - Enables real-time updates to improve recommendation accuracy.

### **6. Local-First Applications**

- **Scenario**: A task management app for remote teams.
- **How Timeline Helps**:
    - Enables offline functionality with automatic sync.
    - Resolves conflicts when users reconnect.

### **7. Time-Based Data Retrieval**

- **Scenario**: Historical data analysis for weather or stock markets.
- **How Timeline Helps**:
    - Retrieves events or snapshots for specific time spans.
    - Supports sharding by time range for faster queries.

## What's Next?

Timeline is still relatively new and in design and development phase, but we are actively working on building a prototype and a proof of concept to demonstrate the capabilities and features, we are also looking for contributors and feedback to help shape the future of Timeline. Any contributions, feedback, and ideas are welcome.

Just reach out to me at [hssnkizz@gmail.com](hssnkizz@gmali.com) thanks, looking forward to hearing from you.

## **Conclusion**

Timeline is can be the game changer for modern applications and systems. Am also still considering the best tech stack to implement it, between rust, go, typescript, or python,and whether to build db engine from scratch or use existing like libsql for a quick start, am open to suggestions and contributions.

Thanks!
