---
name: navisworks-autodesk-navisworks-api-data
description: This skill encodes the navisworks 2026.0 surface of the Autodesk.Navisworks.Api.Data namespace — 15 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DatabaseActionType, DatabaseChange, DatabaseChangeCollection, DatabaseChangeTypes, DatabaseChangedAction, DatabaseChangedEventArgs, DatabaseException, ModelTransformEventArgs, and 7 more types.
---

# Autodesk.Navisworks.Api.Data

Auto-generated from vendor docs for navisworks 2026.0. 15 types in this namespace.

## DatabaseActionType (enum)

Enumeration of database edit.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.DatabaseActionType)

### Values
- `Do` = `0` — A normal database edit
- `Redo` = `1` — A redo database edit
- `Undo` = `2` — An undo database edit

## DatabaseChange (class)

The element of DatabaseChangeCollection.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.DatabaseChange)

### Properties
- `ChangedType` (Autodesk.Navisworks.Api.Data.DatabaseChangeTypes, get) — The type of change DatabaseChangeTypes.
- `RowId` (long, get) — The row id of current database change, use the RowId to find out all columns.
- `TableName` (string, get) — The table name of current database change.

## DatabaseChangeCollection (class)

Collection of DatabaseChange that only supports Enumerable style interfaces

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.DatabaseChangeCollection)

### Methods
#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Data.DatabaseChange> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Data.DatabaseChange>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.DatabaseChangeCollection.GetEnumerator)

#### `System.Collections.IEnumerator InternalGetEnumerator()`

InternalGetEnumerator method of DatabaseChangeCollection.

**Returns:** `System.Collections.IEnumerator` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.DatabaseChangeCollection.InternalGetEnumerator)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.DatabaseChangeCollection.ToString)

#### `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Data.DatabaseChange> Where(Autodesk.Navisworks.Api.Data.DatabaseChangeTypes changeType)`

Filters the changes based on changeType

**Parameters:**
- `changeType` (Autodesk.Navisworks.Api.Data.DatabaseChangeTypes) — Which DatabaseChangeTypes will be cared

**Returns:** `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Data.DatabaseChange>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.DatabaseChangeCollection.Where%28Autodesk.Navisworks.Api.Data.DatabaseChangeTypes%29)

#### `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Data.DatabaseChange> Where(string tableNames)`

Filters the changes based on tablesName

**Parameters:**
- `tableNames` (string) — Which tables would be cared. support multi tables and use white space between table names, Like "ABC def".

**Returns:** `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Data.DatabaseChange>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.DatabaseChangeCollection.Where%28System.String%29)

#### `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Data.DatabaseChange> Where(string tableNames, Autodesk.Navisworks.Api.Data.DatabaseChangeTypes changeType)`

Filters the changes based on tableNames and changeType

**Parameters:**
- `tableNames` (string) — Which tables would be cared. support multi tables and use white space between table names, Like "ABC def".
- `changeType` (Autodesk.Navisworks.Api.Data.DatabaseChangeTypes) — Which DatabaseChangeTypes will be cared

**Returns:** `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Data.DatabaseChange>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.DatabaseChangeCollection.Where%28System.String%2CAutodesk.Navisworks.Api.Data.DatabaseChangeTypes%29)

## DatabaseChangeTypes (enum)

Change type

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.DatabaseChangeTypes)

### Values
- `None` = `0` — No change
- `Insert` = `1` — Insert row
- `Delete` = `2` — Delete row
- `Update` = `4` — Update row
- `All` = `7` — All change types

## DatabaseChangedAction (enum)

Determines which action should do after transaction

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.DatabaseChangedAction)

### Values
- `Edited` = `0` — Undoable update database, insert|update|delete row
- `Reset` = `1` — Non undoable database operate, like create table

## DatabaseChangedEventArgs (class)

Event arguments when a database changed event.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.DatabaseChangedEventArgs)

### Properties
- `Action` (Autodesk.Navisworks.Api.Data.DatabaseChangedAction, get) — Gets the action that caused the event
- `ActionType` (Autodesk.Navisworks.Api.Data.DatabaseActionType, get) — Gets action type of database editing.
- `Changes` (Autodesk.Navisworks.Api.Data.DatabaseChangeCollection, get) — Gets change collection that were edited, if Action is not Edited, Changes will be null.

## DatabaseException (class)

The exception that is thrown when database failed

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.DatabaseException)

### Constructors
- `DatabaseException(int errorCode)` — Creates a DatabaseException
- `DatabaseException(int errorCode, string message)` — Creates a DatabaseException
- `DatabaseException(int errorCode, string message, System.Exception innerException)` — Creates a DatabaseException

### Properties
- `ErrorCode` (int, get) — Get database failed error code.

## ModelTransformEventArgs (class)

Event arguments for the model units and transform change event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.ModelTransformEventArgs)

### Properties
- `AllowChange` (bool, set) — Sets the allow change value. Unit and transform updates are not applied to the model when allow change is set to false.
- `GetSourceIsUI` (bool, get) — Gets the whether the event source is the UI.
- `LinearUnits` (Autodesk.Navisworks.Api.Units, get) — Gets the model linear units to be applied.
- `Model` (Autodesk.Navisworks.Api.Model, get) — Gets the model.
- `Transform` (Autodesk.Navisworks.Api.Transform3D, get) — Gets the transform to be applied.
- `TransformReverses` (bool, get) — Gets the whether the transform will be reversed when applied.

## NavisWorksDataReader (class)

Implementation of DbDataReader.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.NavisWorksDataReader)

### Methods
#### `void Close()`

Closes the datareader.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.Close)

#### `bool GetBoolean(int ordinal)`

Retrieves the column as a boolean value

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `bool` — bool

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetBoolean%28System.Int32%29)

#### `byte GetByte(int ordinal)`

Retrieves the column as a single byte value

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `byte` — byte

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetByte%28System.Int32%29)

#### `long GetBytes(int ordinal, long dataOffset, byte[] buffer, int bufferOffset, int length)`

Retrieves a column as an array of bytes (blob). To determine the number of bytes in the column, pass a null value for the buffer.The total length will be returned.

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve
- `dataOffset` (long) — The zero-based index of where to begin reading the data
- `buffer` (byte[]) — The buffer to write the bytes into
- `bufferOffset` (int) — The zero-based index of where to begin writing into the array
- `length` (int) — The number of bytes to retrieve

**Returns:** `long` — The actual number of bytes written into the array, if buffer is null, will return total length

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetBytes%28System.Int32%2CSystem.Int64%2CSystem.Byte%5B%5D%2CSystem.Int32%2CSystem.Int32%29)

#### `char GetChar(int ordinal)`

Returns the column as a single character

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `char` — char

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetChar%28System.Int32%29)

#### `long GetChars(int ordinal, long dataOffset, char[] buffer, int bufferOffset, int length)`

Retrieves a column as an array of chars (blob) To determine the number of characters in the column, pass a null value for the buffer. The total length will be returned.

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve
- `dataOffset` (long) — The zero-based index of where to begin reading the data
- `buffer` (char[]) — The buffer to write the characters into
- `bufferOffset` (int) — The zero-based index of where to begin writing into the array
- `length` (int) — The number of bytes to retrieve

**Returns:** `long` — The actual number of bytes written into the array, if buffer is null, will return total length

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetChars%28System.Int32%2CSystem.Int64%2CSystem.Char%5B%5D%2CSystem.Int32%2CSystem.Int32%29)

#### `string GetDataTypeName(int ordinal)`

Retrieves the name of the back-end datatype of the column

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `string` — string

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetDataTypeName%28System.Int32%29)

#### `System.DateTime GetDateTime(int ordinal)`

Retrieve the column as a date/time value

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `System.DateTime` — DateTime

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetDateTime%28System.Int32%29)

#### `decimal GetDecimal(int ordinal)`

Retrieve the column as a decimal value

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `decimal` — decimal

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetDecimal%28System.Int32%29)

#### `double GetDouble(int ordinal)`

Returns the column as a double

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `double` — double

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetDouble%28System.Int32%29)

#### `System.Collections.IEnumerator GetEnumerator()`

GetEnumerator method of NavisWorksDataReader.

**Returns:** `System.Collections.IEnumerator` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetEnumerator)

#### `System.Type GetFieldType(int ordinal)`

Returns the .NET type of a given column

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `System.Type` — Type

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetFieldType%28System.Int32%29)

#### `float GetFloat(int ordinal)`

Returns a column as a float value

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `float` — float

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetFloat%28System.Int32%29)

#### `System.Guid GetGuid(int ordinal)`

Returns the column as a Guid

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `System.Guid` — Guid

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetGuid%28System.Int32%29)

#### `short GetInt16(int ordinal)`

Returns the column as a short

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `short` — Int16

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetInt16%28System.Int32%29)

#### `int GetInt32(int ordinal)`

Retrieves the column as an int

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `int` — Int32

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetInt32%28System.Int32%29)

#### `long GetInt64(int ordinal)`

Retrieves the column as a long

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `long` — Int64

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetInt64%28System.Int32%29)

#### `string GetName(int ordinal)`

Retrieves the name of the column

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `string` — string

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetName%28System.Int32%29)

#### `int GetOrdinal(string name)`

Retrieves the index of a column, given its name

**Parameters:**
- `name` (string) — The name of the column to retrieve

**Returns:** `int` — The int index of the column

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetOrdinal%28System.String%29)

#### `System.Data.DataTable GetSchemaTable()`

Schema information in SQLite is difficult to map into .NET conventions, so a lot of work must be done to gather the necessary information so it can be represented in an ADO.NET manner.

**Returns:** `System.Data.DataTable` — Returns a DataTable containing the schema information for the active SELECT statement being processed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetSchemaTable)

#### `string GetString(int ordinal)`

Retrieves the column as a string

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `string` — string

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetString%28System.Int32%29)

#### `object GetValue(int ordinal)`

Retrieves the column as an object corresponding to the underlying datatype of the column

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `object` — object

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetValue%28System.Int32%29)

#### `int GetValues(object[] values)`

Retrieves the values of multiple columns, up to the size of the supplied array

**Parameters:**
- `values` (object[]) — The array to fill with values from the columns in the current resultset

**Returns:** `int` — The number of columns retrieved

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.GetValues%28System.Object%5B%5D%29)

#### `bool IsDBNull(int ordinal)`

Returns True if the specified column is null

**Parameters:**
- `ordinal` (int) — The index of the column to retrieve

**Returns:** `bool` — True or False

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.IsDBNull%28System.Int32%29)

#### `bool NextResult()`

Moves to the next resultset in multiple row-returning SQL command.

**Returns:** `bool` — True if the command was successful and a new resultset is available, False otherwise.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.NextResult)

#### `bool Read()`

Reads the next row from the resultset

**Returns:** `bool` — True if a new row was successfully loaded and is ready for processing

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisWorksDataReader.Read)

### Properties
- `Depth` (int, get) — Not implemented. Returns 0
- `FieldCount` (int, get) — Returns the number of columns in the current resultset.
- `HasRows` (bool, get) — Returns True if the resultset has rows that can be fetched
- `IsClosed` (bool, get) — Returns True if the data reader is closed
- `Item` (object, get) — Indexer to retrieve data from a column given its i
- `Item` (object, get) — Indexer to retrieve data from a column given its name
- `RecordsAffected` (int, get) — Retrieve the count of records affected by an update/insert/delete command. Only valid once the data reader is closed!

## NavisworksCommand (class)

Implementation of DbCommand.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.NavisworksCommand)

### Constructors
- `NavisworksCommand()` — Default constructor
- `NavisworksCommand(Autodesk.Navisworks.Api.Data.NavisworksConnection connection)` — Initializes the command and associates it with the specified connection.
- `NavisworksCommand(string commandText)` — Initializes the command with the given command text
- `NavisworksCommand(string commandText, Autodesk.Navisworks.Api.Data.NavisworksConnection connection)` — Initializes the command with the given SQL command text and attach the command to the specified connection.

### Methods
#### `void Cancel()`

Not implemented

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksCommand.Cancel)

#### `Autodesk.Navisworks.Api.Data.NavisworksParameter CreateParameter()`

Create a new parameter.

**Returns:** `Autodesk.Navisworks.Api.Data.NavisworksParameter` — Return NavisworksParameter

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksCommand.CreateParameter)

#### `int ExecuteNonQuery()`

Execute the command and return the number of rows inserted/updated affected by it.

**Returns:** `int` — The number of rows inserted/updated affected by this executed command

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksCommand.ExecuteNonQuery)

#### `Autodesk.Navisworks.Api.Data.NavisWorksDataReader ExecuteReader()`

Overrides the default behavior of DbDataReader to return a specialized NavisWorksDataReader class

**Returns:** `Autodesk.Navisworks.Api.Data.NavisWorksDataReader` — Return NavisWorksDataReader

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksCommand.ExecuteReader)

#### `Autodesk.Navisworks.Api.Data.NavisWorksDataReader ExecuteReader(System.Data.CommandBehavior behavior)`

Overrides the default behavior to return a NavisWorksDataReader specialization class

**Parameters:**
- `behavior` (System.Data.CommandBehavior) — The flags to be associated with the reader

**Returns:** `Autodesk.Navisworks.Api.Data.NavisWorksDataReader` — Return NavisWorksDataReader

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksCommand.ExecuteReader%28System.Data.CommandBehavior%29)

#### `object ExecuteScalar()`

Execute the command and return the first column of the first row of the resultset (if present), or null if no resultset was returned.

**Returns:** `object` — The first column of the first row of the first resultset from the query

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksCommand.ExecuteScalar)

#### `void Prepare()`

Does nothing. Commands are prepared as they are executed the first time, and kept in prepared state afterwards.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksCommand.Prepare)

### Properties
- `CommandText` (string, get/set) — The SQL command text associated with the command
- `CommandTimeout` (int, get/set) — Not implement
- `CommandType` (System.Data.CommandType, get/set) — The type of the command. NavisworksCommand only supports CommandType.Text
- `Connection` (Autodesk.Navisworks.Api.Data.NavisworksConnection, get/set) — The connection associated with this command
- `DesignTimeVisible` (bool, get/set) — Not implement
- `Parameters` (Autodesk.Navisworks.Api.Data.NavisworksParameterCollection, get) — Not implement, will implement Returns the NavisworksParameterCollection for this command
- `UpdatedRowSource` (System.Data.UpdateRowSource, get/set) — Not implement

## NavisworksConnection (class)

Implementation of DbConnection.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.NavisworksConnection)

### Methods
#### `System.Data.Common.DbTransaction BeginDbTransaction(System.Data.IsolationLevel A_0)`

Not implement

**Parameters:**
- `A_0` (System.Data.IsolationLevel)

**Returns:** `System.Data.Common.DbTransaction` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksConnection.BeginDbTransaction%28System.Data.IsolationLevel%29)

#### `void ChangeDatabase(string databaseName)`

Not implement

**Parameters:**
- `databaseName` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksConnection.ChangeDatabase%28System.String%29)

#### `void Close()`

Does nothing, always keep open.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksConnection.Close)

#### `Autodesk.Navisworks.Api.Data.NavisworksCommand CreateCommand()`

Create a new NavisworksCommand and associate it with this connection.

**Returns:** `Autodesk.Navisworks.Api.Data.NavisworksCommand` — Returns an instantiated NavisworksCommand object already assigned to this connection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksConnection.CreateCommand)

#### `System.Data.DataTable GetSchema(string collectionName, string[] restrictionValues)`

Returns schema information for the data source of this connection using the specified string for the schema name and the specified string array for the restriction values.

**Parameters:**
- `collectionName` (string) — Specifies the name of the schema to return. Now only support "INDEXES" and "INDEXCOLUMNS"
- `restrictionValues` (string[]) — Specifies a set of restriction values for the requested schema.

**Returns:** `System.Data.DataTable` — A System.Data.DataTable that contains schema information.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksConnection.GetSchema%28System.String%2CSystem.String%5B%5D%29)

#### `Autodesk.Navisworks.Api.Interop.LcUSQLiteConnection InternalGetConnection()`

InternalGetConnection method of NavisworksConnection.

**Returns:** `Autodesk.Navisworks.Api.Interop.LcUSQLiteConnection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksConnection.InternalGetConnection)

#### `void Open()`

Does nothing, connection is automatically opened by DocumentDatabase.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksConnection.Open)

### Properties
- `ConnectionString` (string, get/set) — Not implement
- `DataSource` (string, get) — Not implement
- `Database` (string, get) — Not implement
- `ServerVersion` (string, get) — Not implement
- `State` (System.Data.ConnectionState, get) — Returns the state of the connection

## NavisworksDataAdapter (class)

Implementation of DbDataAdapter.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.NavisworksDataAdapter)

### Constructors
- `NavisworksDataAdapter()` — Default constructor
- `NavisworksDataAdapter(Autodesk.Navisworks.Api.Data.NavisworksCommand selectCommand)` — Constructs a data adapter using the specified select command.
- `NavisworksDataAdapter(string commandText, Autodesk.Navisworks.Api.Data.NavisworksConnection connection)` — Constructs a data adapter with the supplied select command text and associated with the specified connection.

### Methods
#### `int Update(System.Data.DataRow[] data_row)`

Not implement

**Parameters:**
- `data_row` (System.Data.DataRow[])

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksDataAdapter.Update%28System.Data.DataRow%5B%5D%29)

#### `int Update(System.Data.DataSet ds)`

Not implement

**Parameters:**
- `ds` (System.Data.DataSet)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksDataAdapter.Update%28System.Data.DataSet%29)

#### `int Update(System.Data.DataSet dt, string datatable_name)`

Not implement

**Parameters:**
- `dt` (System.Data.DataSet)
- `datatable_name` (string)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksDataAdapter.Update%28System.Data.DataSet%2CSystem.String%29)

#### `int Update(System.Data.DataTable dt)`

Not implement

**Parameters:**
- `dt` (System.Data.DataTable)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksDataAdapter.Update%28System.Data.DataTable%29)

### Properties
- `DeleteCommand` (Autodesk.Navisworks.Api.Data.NavisworksCommand, get/set) — Gets/sets the delete command for this DataAdapter
- `InsertCommand` (Autodesk.Navisworks.Api.Data.NavisworksCommand, get/set) — Gets/sets the insert command for this DataAdapter
- `SelectCommand` (Autodesk.Navisworks.Api.Data.NavisworksCommand, get/set) — Gets/sets the select command for this DataAdapter
- `UpdateCommand` (Autodesk.Navisworks.Api.Data.NavisworksCommand, get/set) — Gets/sets the update command for this DataAdapter

### Events
#### `RowUpdated` (`System.EventHandler<System.Data.Common.RowUpdatedEventArgs>`)

**Signature:** `event System.EventHandler<System.Data.Common.RowUpdatedEventArgs> RowUpdated`

Row updated event handler

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Data.NavisworksDataAdapter.RowUpdated)

#### `RowUpdating` (`System.EventHandler<System.Data.Common.RowUpdatingEventArgs>`)

**Signature:** `event System.EventHandler<System.Data.Common.RowUpdatingEventArgs> RowUpdating`

Row updating event handler

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Data.NavisworksDataAdapter.RowUpdating)

## NavisworksParameter (class)

Implementation of DbParameter.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.NavisworksParameter)

### Constructors
- `NavisworksParameter()` — Default constructor
- `NavisworksParameter(string parameterName, object value)` — Constructs a named parameter given the specified parameter name and initial value

### Methods
#### `void ResetDbType()`

ResetDbType method of NavisworksParameter.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameter.ResetDbType)

### Properties
- `DbType` (System.Data.DbType, get/set) — Returns the datatype of the parameter
- `Direction` (System.Data.ParameterDirection, get/set) — Supports only input parameters
- `IsNullable` (bool, get/set) — Whether or not the parameter can contain a null value
- `ParameterName` (string, get/set) — Returns the parameter name
- `Size` (int, get/set) — Returns the size of the parameter
- `SourceColumn` (string, get/set) — Gets/sets the source column
- `SourceColumnNullMapping` (bool, get/set) — Used by DbCommandBuilder to determine the mapping for nullable fields
- `SourceVersion` (System.Data.DataRowVersion, get/set) — Gets and sets the row version
- `Value` (object, get/set) — Gets and sets the parameter value. If no datatype was specified, the datatype will assume the type from the value given.

## NavisworksParameterCollection (class)

Implementation of DbParameterCollection.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.NavisworksParameterCollection)

### Methods
#### `int Add(Autodesk.Navisworks.Api.Data.NavisworksParameter parameter)`

Adds a parameter to the collection

**Parameters:**
- `parameter` (Autodesk.Navisworks.Api.Data.NavisworksParameter) — The parameter to add

**Returns:** `int` — A zero-based index of where the parameter is located in the array

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.Add%28Autodesk.Navisworks.Api.Data.NavisworksParameter%29)

#### `int Add(object value)`

Adds a parameter to the collection

**Parameters:**
- `value` (object) — The parameter to add

**Returns:** `int` — A zero-based index of where the parameter is located in the array

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.Add%28System.Object%29)

#### `void AddRange(System.Array values)`

Adds an array of parameters to the collection

**Parameters:**
- `values` (System.Array) — The array of parameters to add

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.AddRange%28System.Array%29)

#### `Autodesk.Navisworks.Api.Data.NavisworksParameter AddWithValue(string parameterName, object value)`

Adds a named/unnamed parameter and its value to the parameter collection.

**Parameters:**
- `parameterName` (string) — Name of the parameter, or null to indicate an unnamed parameter
- `value` (object) — The initial value of the parameter

**Returns:** `Autodesk.Navisworks.Api.Data.NavisworksParameter` — Returns the NavisworksParameter object created during the call.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.AddWithValue%28System.String%2CSystem.Object%29)

#### `void Clear()`

Clears the array and resets the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.Clear)

#### `bool Contains(object value)`

Determines if the parameter exists in the collection

**Parameters:**
- `value` (object) — The NavisworksParameter to check

**Returns:** `bool` — True if the parameter is in the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.Contains%28System.Object%29)

#### `bool Contains(string value)`

Determines if the named parameter exists in the collection

**Parameters:**
- `value` (string) — The name of the parameter to check

**Returns:** `bool` — True if the parameter is in the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.Contains%28System.String%29)

#### `void CopyTo(System.Array values, int index)`

Not implemented

**Parameters:**
- `values` (System.Array) — 
- `index` (int) — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.CopyTo%28System.Array%2CSystem.Int32%29)

#### `void Dispose()`

Dispose method of NavisworksParameterCollection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.Dispose)

#### `System.Collections.IEnumerator GetEnumerator()`

GetEnumerator method of NavisworksParameterCollection.

**Returns:** `System.Collections.IEnumerator` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.GetEnumerator)

#### `int IndexOf(object value)`

Returns the index of a parameter

**Parameters:**
- `value` (object) — The parameter to find

**Returns:** `int` — -1 if not found, otherwise a zero-based index of the parameter

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.IndexOf%28System.Object%29)

#### `int IndexOf(string parameterName)`

Returns the index of a parameter given its name

**Parameters:**
- `parameterName` (string) — The name of the parameter to find

**Returns:** `int` — -1 if not found, otherwise a zero-based index of the parameter

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.IndexOf%28System.String%29)

#### `void Insert(int index, object value)`

Inserts a parameter into the array at the specified location

**Parameters:**
- `index` (int) — The zero-based index to insert the parameter at
- `value` (object) — The parameter to insert

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.Insert%28System.Int32%2CSystem.Object%29)

#### `void Remove(object value)`

Removes specified parameter from the collection

**Parameters:**
- `value` (object) — The parameter to remove

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.Remove%28System.Object%29)

#### `void RemoveAt(int index)`

Removes a parameter from the collection given its index

**Parameters:**
- `index` (int) — The zero-based parameter index to remove

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.RemoveAt%28System.Int32%29)

#### `void RemoveAt(string parameterName)`

Removes a parameter from the collection given its name

**Parameters:**
- `parameterName` (string) — The name of the parameter to remove

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksParameterCollection.RemoveAt%28System.String%29)

### Properties
- `Count` (int, get) — Returns a count of parameters in the collection
- `IsFixedSize` (bool, get) — Returns false
- `IsReadOnly` (bool, get) — Returns false
- `IsSynchronized` (bool, get) — Returns true
- `Item` (Autodesk.Navisworks.Api.Data.NavisworksParameter, get/set) — Overloaded to specialize the return value of the default indexer
- `Item` (Autodesk.Navisworks.Api.Data.NavisworksParameter, get/set) — Overloaded to specialize the return value of the default indexer
- `SyncRoot` (object, get) — Returns null

## NavisworksTransaction (class)

Implementation of DbTransaction.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Data.NavisworksTransaction)

### Methods
#### `void Commit()`

Commits the current transaction.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksTransaction.Commit)

#### `void Rollback()`

Rolls back the active transaction.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Data.NavisworksTransaction.Rollback)

### Properties
- `Connection` (Autodesk.Navisworks.Api.Data.NavisworksConnection, get) — Returns the underlying connection to which this transaction applies.
- `IsolationLevel` (System.Data.IsolationLevel, get) — Gets the isolation level of the transaction. NavisworksTransaction only supports Serializable transactions.

