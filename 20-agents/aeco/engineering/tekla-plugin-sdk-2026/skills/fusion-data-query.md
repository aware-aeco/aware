---
name: tekla-plugin-sdk-fusion-data-query
description: This skill encodes the tekla-plugin-sdk 2026.0 surface of the Fusion.Data.Query namespace — 7 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Column, Parameter, Parameters, QueryBuilder, QueryExtensions, Result`1, Results`1.
---

# Fusion.Data.Query

Auto-generated from vendor docs for tekla-plugin-sdk 2026.0. 7 types in this namespace.

## Column (class)

Column class in Fusion.Data.Query.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Data.Query.Column)

### Constructors
- `Column(string name)` — Constructs a new Column.

### Properties
- `Name` (string, get) — Name property of Column.

## Parameter (class)

Parameter class in Fusion.Data.Query.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Data.Query.Parameter)

### Constructors
- `Parameter(string name, System.Action<Fusion.Data.Sqlite.Statement, int> bind)` — Constructs a new Parameter.

### Properties
- `Bind` (System.Action<Fusion.Data.Sqlite.Statement, int>, get) — Bind property of Parameter.

## Parameters (class)

Parameters class in Fusion.Data.Query.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Data.Query.Parameters)

### Constructors
- `Parameters()` — Constructs a new Parameters.

### Methods
#### `void Add(string name, System.DateTime value)`

Add method of Parameters.

**Parameters:**
- `name` (string)
- `value` (System.DateTime)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Parameters.Add%28System.String%2CSystem.DateTime%29)

#### `void Add(string name, System.TimeSpan value)`

Add method of Parameters.

**Parameters:**
- `name` (string)
- `value` (System.TimeSpan)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Parameters.Add%28System.String%2CSystem.TimeSpan%29)

#### `void Add(string name, byte[] value)`

Add method of Parameters.

**Parameters:**
- `name` (string)
- `value` (byte[])

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Parameters.Add%28System.String%2CSystem.Byte%5B%5D%29)

#### `void Add(string name, double value)`

Add method of Parameters.

**Parameters:**
- `name` (string)
- `value` (double)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Parameters.Add%28System.String%2CSystem.Double%29)

#### `void Add(string name, int value)`

Add method of Parameters.

**Parameters:**
- `name` (string)
- `value` (int)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Parameters.Add%28System.String%2CSystem.Int32%29)

#### `void Add(string name, long value)`

Add method of Parameters.

**Parameters:**
- `name` (string)
- `value` (long)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Parameters.Add%28System.String%2CSystem.Int64%29)

#### `void Add(string name, string value)`

Add method of Parameters.

**Parameters:**
- `name` (string)
- `value` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Parameters.Add%28System.String%2CSystem.String%29)

#### `void Bind(Fusion.Data.Sqlite.Statement statement, int baseIndex = 0)`

Bind method of Parameters.

**Parameters:**
- `statement` (Fusion.Data.Sqlite.Statement)
- `baseIndex` (int)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Parameters.Bind%28Fusion.Data.Sqlite.Statement%2CSystem.Int32%29)

#### `System.Collections.Generic.IEnumerator<Fusion.Data.Query.Parameter> GetEnumerator()`

GetEnumerator method of Parameters.

**Returns:** `System.Collections.Generic.IEnumerator<Fusion.Data.Query.Parameter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Parameters.GetEnumerator)

### Properties
- `Count` (int, get) — Count property of Parameters.

## QueryBuilder (class)

QueryBuilder class in Fusion.Data.Query.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Data.Query.QueryBuilder)

### Constructors
- `QueryBuilder()` — Constructs a new QueryBuilder.

### Methods
#### `Fusion.Data.Query.QueryBuilder And(string expression)`

And method of QueryBuilder.

**Parameters:**
- `expression` (string)

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.And%28System.String%29)

#### `Fusion.Data.Query.QueryBuilder Delete()`

Delete method of QueryBuilder.

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.Delete)

#### `Fusion.Data.Query.QueryBuilder From(string table)`

From method of QueryBuilder.

**Parameters:**
- `table` (string)

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.From%28System.String%29)

#### `Fusion.Data.Query.QueryBuilder GroupBy(string[] columns)`

GroupBy method of QueryBuilder.

**Parameters:**
- `columns` (string[])

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.GroupBy%28System.String%5B%5D%29)

#### `Fusion.Data.Query.QueryBuilder Having(string expression)`

Having method of QueryBuilder.

**Parameters:**
- `expression` (string)

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.Having%28System.String%29)

#### `Fusion.Data.Query.QueryBuilder Insert()`

Insert method of QueryBuilder.

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.Insert)

#### `Fusion.Data.Query.QueryBuilder Into(string table)`

Into method of QueryBuilder.

**Parameters:**
- `table` (string)

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.Into%28System.String%29)

#### `Fusion.Data.Query.QueryBuilder Limit(int count, int offset = 0)`

Limit method of QueryBuilder.

**Parameters:**
- `count` (int)
- `offset` (int)

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.Limit%28System.Int32%2CSystem.Int32%29)

#### `Fusion.Data.Query.QueryBuilder OrIgnore()`

OrIgnore method of QueryBuilder.

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.OrIgnore)

#### `Fusion.Data.Query.QueryBuilder OrReplace()`

OrReplace method of QueryBuilder.

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.OrReplace)

#### `Fusion.Data.Query.QueryBuilder OrderBy(string[] columns)`

OrderBy method of QueryBuilder.

**Parameters:**
- `columns` (string[])

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.OrderBy%28System.String%5B%5D%29)

#### `Fusion.Data.Query.QueryBuilder Select(string expression)`

Select method of QueryBuilder.

**Parameters:**
- `expression` (string)

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.Select%28System.String%29)

#### `Fusion.Data.Query.QueryBuilder Select<T>(System.Collections.Generic.IEnumerable<T> columns)`

Select method of QueryBuilder.

**Parameters:**
- `columns` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.Select%60%601%28System.Collections.Generic.IEnumerable%7B%60%600%7D%29)

#### `Fusion.Data.Query.QueryBuilder Set<T>(System.Collections.Generic.IEnumerable<T> columns)`

Set method of QueryBuilder.

**Parameters:**
- `columns` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.Set%60%601%28System.Collections.Generic.IEnumerable%7B%60%600%7D%29)

#### `string ToString()`

ToString method of QueryBuilder.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.ToString)

#### `Fusion.Data.Query.QueryBuilder Update(string table)`

Update method of QueryBuilder.

**Parameters:**
- `table` (string)

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.Update%28System.String%29)

#### `Fusion.Data.Query.QueryBuilder Values<T>(System.Collections.Generic.IEnumerable<T> columns)`

Values method of QueryBuilder.

**Parameters:**
- `columns` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.Values%60%601%28System.Collections.Generic.IEnumerable%7B%60%600%7D%29)

#### `Fusion.Data.Query.QueryBuilder Where(string expression)`

Where method of QueryBuilder.

**Parameters:**
- `expression` (string)

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.Where%28System.String%29)

#### `Fusion.Data.Query.QueryBuilder Where<T>(System.Collections.Generic.IEnumerable<T> columns)`

Where method of QueryBuilder.

**Parameters:**
- `columns` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `Fusion.Data.Query.QueryBuilder` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryBuilder.Where%60%601%28System.Collections.Generic.IEnumerable%7B%60%600%7D%29)

## QueryExtensions (static-class)

QueryExtensions static class in Fusion.Data.Query.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Data.Query.QueryExtensions)

### Methods
#### `static void Delete(Fusion.Data.Sqlite database, string table, Fusion.Data.Query.Parameters where)`

Delete method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `where` (Fusion.Data.Query.Parameters)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Delete%28Fusion.Data.Sqlite%2CSystem.String%2CFusion.Data.Query.Parameters%29)

#### `static void Insert(Fusion.Data.Sqlite database, string table, Fusion.Data.Query.Parameters columns)`

Insert method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `columns` (Fusion.Data.Query.Parameters)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Insert%28Fusion.Data.Sqlite%2CSystem.String%2CFusion.Data.Query.Parameters%29)

#### `static void InsertOrIgnore(Fusion.Data.Sqlite database, string table, Fusion.Data.Query.Parameters columns)`

InsertOrIgnore method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `columns` (Fusion.Data.Query.Parameters)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.InsertOrIgnore%28Fusion.Data.Sqlite%2CSystem.String%2CFusion.Data.Query.Parameters%29)

#### `static void InsertOrReplace(Fusion.Data.Sqlite database, string table, Fusion.Data.Query.Parameters columns)`

InsertOrReplace method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `columns` (Fusion.Data.Query.Parameters)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.InsertOrReplace%28Fusion.Data.Sqlite%2CSystem.String%2CFusion.Data.Query.Parameters%29)

#### `static System.Collections.Generic.IEnumerable<T> Select<T>(Fusion.Data.Sqlite database, string table, Fusion.Data.Query.Results<T> results, Fusion.Data.Query.Parameters where = null, string orderBy = null, int limit = 0, int offset = 0)`

Select method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `results` (Fusion.Data.Query.Results<T>)
- `where` (Fusion.Data.Query.Parameters)
- `orderBy` (string)
- `limit` (int)
- `offset` (int)

**Returns:** `System.Collections.Generic.IEnumerable<T>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Select%60%601%28Fusion.Data.Sqlite%2CSystem.String%2CFusion.Data.Query.Results%7B%60%600%7D%2CFusion.Data.Query.Parameters%2CSystem.String%2CSystem.Int32%2CSystem.Int32%29)

#### `static System.Collections.Generic.IEnumerable<TResult> Select<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, TResult>(Fusion.Data.Sqlite database, string table, System.Func<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, TResult> select, Fusion.Data.Query.Parameters where = null, string orderBy = null, int limit = 0, int offset = 0)`

Select method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `select` (System.Func<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, TResult>)
- `where` (Fusion.Data.Query.Parameters)
- `orderBy` (string)
- `limit` (int)
- `offset` (int)

**Returns:** `System.Collections.Generic.IEnumerable<TResult>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Select%60%6011%28Fusion.Data.Sqlite%2CSystem.String%2CSystem.Func%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%2C%60%605%2C%60%606%2C%60%607%2C%60%608%2C%60%609%2C%60%6010%7D%2CFusion.Data.Query.Parameters%2CSystem.String%2CSystem.Int32%2CSystem.Int32%29)

#### `static System.Collections.Generic.IEnumerable<TResult> Select<T1, T2, T3, T4, T5, T6, T7, T8, T9, TResult>(Fusion.Data.Sqlite database, string table, System.Func<T1, T2, T3, T4, T5, T6, T7, T8, T9, TResult> select, Fusion.Data.Query.Parameters where = null, string orderBy = null, int limit = 0, int offset = 0)`

Select method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `select` (System.Func<T1, T2, T3, T4, T5, T6, T7, T8, T9, TResult>)
- `where` (Fusion.Data.Query.Parameters)
- `orderBy` (string)
- `limit` (int)
- `offset` (int)

**Returns:** `System.Collections.Generic.IEnumerable<TResult>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Select%60%6010%28Fusion.Data.Sqlite%2CSystem.String%2CSystem.Func%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%2C%60%605%2C%60%606%2C%60%607%2C%60%608%2C%60%609%7D%2CFusion.Data.Query.Parameters%2CSystem.String%2CSystem.Int32%2CSystem.Int32%29)

#### `static System.Collections.Generic.IEnumerable<TResult> Select<T1, T2, T3, T4, T5, T6, T7, T8, TResult>(Fusion.Data.Sqlite database, string table, System.Func<T1, T2, T3, T4, T5, T6, T7, T8, TResult> select, Fusion.Data.Query.Parameters where = null, string orderBy = null, int limit = 0, int offset = 0)`

Select method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `select` (System.Func<T1, T2, T3, T4, T5, T6, T7, T8, TResult>)
- `where` (Fusion.Data.Query.Parameters)
- `orderBy` (string)
- `limit` (int)
- `offset` (int)

**Returns:** `System.Collections.Generic.IEnumerable<TResult>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Select%60%609%28Fusion.Data.Sqlite%2CSystem.String%2CSystem.Func%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%2C%60%605%2C%60%606%2C%60%607%2C%60%608%7D%2CFusion.Data.Query.Parameters%2CSystem.String%2CSystem.Int32%2CSystem.Int32%29)

#### `static System.Collections.Generic.IEnumerable<TResult> Select<T1, T2, T3, T4, T5, T6, T7, TResult>(Fusion.Data.Sqlite database, string table, System.Func<T1, T2, T3, T4, T5, T6, T7, TResult> select, Fusion.Data.Query.Parameters where = null, string orderBy = null, int limit = 0, int offset = 0)`

Select method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `select` (System.Func<T1, T2, T3, T4, T5, T6, T7, TResult>)
- `where` (Fusion.Data.Query.Parameters)
- `orderBy` (string)
- `limit` (int)
- `offset` (int)

**Returns:** `System.Collections.Generic.IEnumerable<TResult>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Select%60%608%28Fusion.Data.Sqlite%2CSystem.String%2CSystem.Func%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%2C%60%605%2C%60%606%2C%60%607%7D%2CFusion.Data.Query.Parameters%2CSystem.String%2CSystem.Int32%2CSystem.Int32%29)

#### `static System.Collections.Generic.IEnumerable<TResult> Select<T1, T2, T3, T4, T5, T6, TResult>(Fusion.Data.Sqlite database, string table, System.Func<T1, T2, T3, T4, T5, T6, TResult> select, Fusion.Data.Query.Parameters where = null, string orderBy = null, int limit = 0, int offset = 0)`

Select method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `select` (System.Func<T1, T2, T3, T4, T5, T6, TResult>)
- `where` (Fusion.Data.Query.Parameters)
- `orderBy` (string)
- `limit` (int)
- `offset` (int)

**Returns:** `System.Collections.Generic.IEnumerable<TResult>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Select%60%607%28Fusion.Data.Sqlite%2CSystem.String%2CSystem.Func%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%2C%60%605%2C%60%606%7D%2CFusion.Data.Query.Parameters%2CSystem.String%2CSystem.Int32%2CSystem.Int32%29)

#### `static System.Collections.Generic.IEnumerable<TResult> Select<T1, T2, T3, T4, T5, TResult>(Fusion.Data.Sqlite database, string table, System.Func<T1, T2, T3, T4, T5, TResult> select, Fusion.Data.Query.Parameters where = null, string orderBy = null, int limit = 0, int offset = 0)`

Select method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `select` (System.Func<T1, T2, T3, T4, T5, TResult>)
- `where` (Fusion.Data.Query.Parameters)
- `orderBy` (string)
- `limit` (int)
- `offset` (int)

**Returns:** `System.Collections.Generic.IEnumerable<TResult>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Select%60%606%28Fusion.Data.Sqlite%2CSystem.String%2CSystem.Func%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%2C%60%605%7D%2CFusion.Data.Query.Parameters%2CSystem.String%2CSystem.Int32%2CSystem.Int32%29)

#### `static System.Collections.Generic.IEnumerable<TResult> Select<T1, T2, T3, T4, TResult>(Fusion.Data.Sqlite database, string table, System.Func<T1, T2, T3, T4, TResult> select, Fusion.Data.Query.Parameters where = null, string orderBy = null, int limit = 0, int offset = 0)`

Select method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `select` (System.Func<T1, T2, T3, T4, TResult>)
- `where` (Fusion.Data.Query.Parameters)
- `orderBy` (string)
- `limit` (int)
- `offset` (int)

**Returns:** `System.Collections.Generic.IEnumerable<TResult>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Select%60%605%28Fusion.Data.Sqlite%2CSystem.String%2CSystem.Func%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%7D%2CFusion.Data.Query.Parameters%2CSystem.String%2CSystem.Int32%2CSystem.Int32%29)

#### `static System.Collections.Generic.IEnumerable<TResult> Select<T1, T2, T3, TResult>(Fusion.Data.Sqlite database, string table, System.Func<T1, T2, T3, TResult> select, Fusion.Data.Query.Parameters where = null, string orderBy = null, int limit = 0, int offset = 0)`

Select method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `select` (System.Func<T1, T2, T3, TResult>)
- `where` (Fusion.Data.Query.Parameters)
- `orderBy` (string)
- `limit` (int)
- `offset` (int)

**Returns:** `System.Collections.Generic.IEnumerable<TResult>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Select%60%604%28Fusion.Data.Sqlite%2CSystem.String%2CSystem.Func%7B%60%600%2C%60%601%2C%60%602%2C%60%603%7D%2CFusion.Data.Query.Parameters%2CSystem.String%2CSystem.Int32%2CSystem.Int32%29)

#### `static System.Collections.Generic.IEnumerable<TResult> Select<T1, T2, TResult>(Fusion.Data.Sqlite database, string table, System.Func<T1, T2, TResult> select, Fusion.Data.Query.Parameters where = null, string orderBy = null, int limit = 0, int offset = 0)`

Select method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `select` (System.Func<T1, T2, TResult>)
- `where` (Fusion.Data.Query.Parameters)
- `orderBy` (string)
- `limit` (int)
- `offset` (int)

**Returns:** `System.Collections.Generic.IEnumerable<TResult>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Select%60%603%28Fusion.Data.Sqlite%2CSystem.String%2CSystem.Func%7B%60%600%2C%60%601%2C%60%602%7D%2CFusion.Data.Query.Parameters%2CSystem.String%2CSystem.Int32%2CSystem.Int32%29)

#### `static System.Collections.Generic.IEnumerable<TResult> Select<T1, TResult>(Fusion.Data.Sqlite database, string table, System.Func<T1, TResult> select, Fusion.Data.Query.Parameters where = null, string orderBy = null, int limit = 0, int offset = 0)`

Select method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `select` (System.Func<T1, TResult>)
- `where` (Fusion.Data.Query.Parameters)
- `orderBy` (string)
- `limit` (int)
- `offset` (int)

**Returns:** `System.Collections.Generic.IEnumerable<TResult>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Select%60%602%28Fusion.Data.Sqlite%2CSystem.String%2CSystem.Func%7B%60%600%2C%60%601%7D%2CFusion.Data.Query.Parameters%2CSystem.String%2CSystem.Int32%2CSystem.Int32%29)

#### `static void Update(Fusion.Data.Sqlite database, string table, Fusion.Data.Query.Parameters columns, Fusion.Data.Query.Parameters where)`

Update method of QueryExtensions.

**Parameters:**
- `database` (Fusion.Data.Sqlite)
- `table` (string)
- `columns` (Fusion.Data.Query.Parameters)
- `where` (Fusion.Data.Query.Parameters)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.QueryExtensions.Update%28Fusion.Data.Sqlite%2CSystem.String%2CFusion.Data.Query.Parameters%2CFusion.Data.Query.Parameters%29)

## Result`1 (class)

Result`1 class in Fusion.Data.Query.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Data.Query.Result`1)

### Constructors
- `Result`1(string name, System.Action<Fusion.Data.Sqlite.Statement, int, T> bind)` — Constructs a new Result`1.

### Properties
- `Bind` (System.Action<Fusion.Data.Sqlite.Statement, int, T>, get) — Bind property of Result`1.

## Results`1 (class)

Results`1 class in Fusion.Data.Query.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Data.Query.Results`1)

### Constructors
- `Results`1()` — Constructs a new Results`1.

### Methods
#### `void Add(string name, System.Action<T, System.DateTime> value)`

Add method of Results`1.

**Parameters:**
- `name` (string)
- `value` (System.Action<T, System.DateTime>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Results%601.Add%28System.String%2CSystem.Action%7B%600%2CSystem.DateTime%7D%29)

#### `void Add(string name, System.Action<T, System.TimeSpan> value)`

Add method of Results`1.

**Parameters:**
- `name` (string)
- `value` (System.Action<T, System.TimeSpan>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Results%601.Add%28System.String%2CSystem.Action%7B%600%2CSystem.TimeSpan%7D%29)

#### `void Add(string name, System.Action<T, byte[]> value)`

Add method of Results`1.

**Parameters:**
- `name` (string)
- `value` (System.Action<T, byte[]>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Results%601.Add%28System.String%2CSystem.Action%7B%600%2CSystem.Byte%5B%5D%7D%29)

#### `void Add(string name, System.Action<T, double> value)`

Add method of Results`1.

**Parameters:**
- `name` (string)
- `value` (System.Action<T, double>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Results%601.Add%28System.String%2CSystem.Action%7B%600%2CSystem.Double%7D%29)

#### `void Add(string name, System.Action<T, int> value)`

Add method of Results`1.

**Parameters:**
- `name` (string)
- `value` (System.Action<T, int>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Results%601.Add%28System.String%2CSystem.Action%7B%600%2CSystem.Int32%7D%29)

#### `void Add(string name, System.Action<T, long> value)`

Add method of Results`1.

**Parameters:**
- `name` (string)
- `value` (System.Action<T, long>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Results%601.Add%28System.String%2CSystem.Action%7B%600%2CSystem.Int64%7D%29)

#### `void Add(string name, System.Action<T, string> value)`

Add method of Results`1.

**Parameters:**
- `name` (string)
- `value` (System.Action<T, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Results%601.Add%28System.String%2CSystem.Action%7B%600%2CSystem.String%7D%29)

#### `void Bind(Fusion.Data.Sqlite.Statement statement, T result, int baseIndex = 0)`

Bind method of Results`1.

**Parameters:**
- `statement` (Fusion.Data.Sqlite.Statement)
- `result` (T)
- `baseIndex` (int)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Results%601.Bind%28Fusion.Data.Sqlite.Statement%2C%600%2CSystem.Int32%29)

#### `System.Collections.Generic.IEnumerator<Fusion.Data.Query.Result<T>> GetEnumerator()`

GetEnumerator method of Results`1.

**Returns:** `System.Collections.Generic.IEnumerator<Fusion.Data.Query.Result<T>>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Data.Query.Results%601.GetEnumerator)

### Properties
- `Count` (int, get) — Count property of Results`1.

