---
name: dynamo-applications-dynamo-migration
description: API reference for namespace Dynamo.Migration from DynamoCore.dll
---

# Dynamo.Migration

- **MigrationManager**
  - This class contains methods and properties used for migration of Dynamo workspaces.
- **NodeMigrationAttribute**
  - Marks methods on a NodeModel to be used for version migration.
- **NodeMigrationData**
  - This class contains the resulting nodes as a result of node migration.             Note that this class may contain other information (e.g. connectors) in             the future in the event a migration process results in other elements.
- **PortId**
  - This structure uniquely identifies a given port in the graph.
- **WorkspaceMigrationAttribute**
  - Specifies versions which workspace should be migrated between.
- **WorkspaceMigrations**
  - Contains methods to migrate a workspace from one version to another
