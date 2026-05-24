---
name: tekla-plugin-sdk-fusion-data
description: This skill encodes the tekla-plugin-sdk 2025.0 surface of the Fusion.Data namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Sqlite.
---

# Fusion.Data

Auto-generated from vendor docs for tekla-plugin-sdk 2025.0. 1 types in this namespace.

## Sqlite (class)

Sqlite class in Fusion.Data.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Data.Sqlite)

### Constructors
- `Sqlite(Fusion.Data.Sqlite connection)` — Constructs a new Sqlite.
- `Sqlite(string path)` — Constructs a new Sqlite.
- `Sqlite(string path, System.TimeSpan timeout)` — Constructs a new Sqlite.

### Methods
#### `Fusion.Data.Sqlite.Transaction BeginTransaction()`

BeginTransaction method of Sqlite.

**Returns:** `Fusion.Data.Sqlite.Transaction` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Sqlite.BeginTransaction)

#### `void Dispose()`

Dispose method of Sqlite.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Sqlite.Dispose)

#### `void Execute(string sql)`

Execute method of Sqlite.

**Parameters:**
- `sql` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Sqlite.Execute%28System.String%29)

#### `Fusion.Data.Sqlite.Statement Prepare(string sql)`

Prepare method of Sqlite.

**Parameters:**
- `sql` (string)

**Returns:** `Fusion.Data.Sqlite.Statement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Sqlite.Prepare%28System.String%29)

