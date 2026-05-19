---
name: dynamo-protocore-debugservices
description: This skill encodes the dynamo 4.1.1 surface of the ProtoCore.DebugServices namespace — 5 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BeginDocument, EndDocument, PrintMessage, EventSink, ConsoleEventSink.
---

# ProtoCore.DebugServices

Auto-generated from vendor docs for dynamo 4.1.1. 5 types in this namespace.

## BeginDocument (delegate)

Type BeginDocument

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

### Constructors
- `void BeginDocument(object object, IntPtr method)` — BeginDocument.BeginDocument

### Methods
#### `System.IAsyncResult BeginInvoke(string script, System.AsyncCallback callback, object object)`

BeginDocument.BeginInvoke

**Parameters:**
- `script` (string)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

#### `void EndInvoke(System.IAsyncResult result)`

BeginDocument.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

#### `void Invoke(string script)`

BeginDocument.Invoke

**Parameters:**
- `script` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

## ConsoleEventSink (class)

Type ConsoleEventSink

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

### Constructors
- `void ConsoleEventSink()` — ConsoleEventSink.ConsoleEventSink

## EndDocument (delegate)

Type EndDocument

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

### Constructors
- `void EndDocument(object object, IntPtr method)` — EndDocument.EndDocument

### Methods
#### `System.IAsyncResult BeginInvoke(string script, System.AsyncCallback callback, object object)`

EndDocument.BeginInvoke

**Parameters:**
- `script` (string)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

#### `void EndInvoke(System.IAsyncResult result)`

EndDocument.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

#### `void Invoke(string script)`

EndDocument.Invoke

**Parameters:**
- `script` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

## EventSink (class)

Type EventSink

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

### Constructors
- `void EventSink()` — EventSink.EventSink

## PrintMessage (delegate)

Type PrintMessage

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

### Constructors
- `void PrintMessage(object object, IntPtr method)` — PrintMessage.PrintMessage

### Methods
#### `System.IAsyncResult BeginInvoke(string message, System.AsyncCallback callback, object object)`

PrintMessage.BeginInvoke

**Parameters:**
- `message` (string)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

#### `void EndInvoke(System.IAsyncResult result)`

PrintMessage.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

#### `void Invoke(string message)`

PrintMessage.Invoke

**Parameters:**
- `message` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

