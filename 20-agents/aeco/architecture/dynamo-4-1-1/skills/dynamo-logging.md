---
name: dynamo-dynamo-logging
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Logging namespace — 11 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: LogLevel, WarningLevel, LogEventHandler, LogEventArgs, DynamoLogger, ILogger, ILogMessage, ILogSource, and 3 more types.
---

# Dynamo.Logging

Auto-generated from vendor docs for dynamo 4.1.1. 11 types in this namespace.

## DynamoLogger (class)

This class contains methods and properties used for logging in Dynamo,

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

### Constructors
- `void DynamoLogger(Dynamo.Configuration.DebugSettings debugSettings, string logDirectory, bool isTestMode, bool isCLIMode, bool isServiceMode)` — Initializes a new instance of Dynamo.Logging.DynamoLogger class with specified debug settings and directory where to write logs

### Methods
#### `void ClearStartupNotifications()`

Clear any notifications after displaying them.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void Dispose()`

Disposes the logger and finishes logging.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void Log(System.Exception e)`

Log an exception

**Parameters:**
- `e` (System.Exception) — Exception to log

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void Log(string message)`

Log a message

**Parameters:**
- `message` (string) — Message to log

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void Log(string message, Dynamo.Logging.LogLevel level)`

Logs the specified message.

**Parameters:**
- `message` (string) — The message.
- `level` (Dynamo.Logging.LogLevel) — The level.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void Log(string message, Dynamo.Logging.LogLevel level, bool reportModification)`

Log the message to the the correct path

**Parameters:**
- `message` (string) — 
- `level` (Dynamo.Logging.LogLevel) — 
- `reportModification` (bool) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void Log(string tag, string data)`

Log some data with an associated tag

**Parameters:**
- `tag` (string) — Tag of the message to log
- `data` (string) — Message to log

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void LogError(string error)`

Logs the error.

**Parameters:**
- `error` (string) — The error.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void LogError(string tag, string error)`

Logs the error.

**Parameters:**
- `tag` (string) — The tag.
- `error` (string) — The error.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void LogInfo(string tag, string info)`

Log an information message

**Parameters:**
- `tag` (string) — Tag of the message to log
- `info` (string) — Message to log

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void LogNotification(string sender, string title, string shortMessage, string detailedMessage)`

Logs a tagged notification to the console, also fires an event that a notification was logged

**Parameters:**
- `sender` (string) — 
- `title` (string) — 
- `shortMessage` (string) — 
- `detailedMessage` (string) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void LogWarning(string message, Dynamo.Logging.WarningLevel level)`

Logs the warning.

**Parameters:**
- `message` (string) — The message.
- `level` (Dynamo.Logging.WarningLevel) — The level.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void StartLoggingToConsoleAndFile(string logDirectory)`

Begin logging.

**Parameters:**
- `logDirectory` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

### Properties
- `LogPath` (string, get) — Returns full path to log file
- `LogText` (string, get) — Contains all message which have been logged
- `StartupNotifications` (System.Collections.Generic.IEnumerable<Dynamo.Logging.NotificationMessage>, get) — Notifications logged during startup such as library load failures that need to be displayed to user.
- `Warning` (string, get/set) — Returns warning message text
- `WarningLevel` (Dynamo.Logging.WarningLevel, get/set) — Returns warning level for log messages.

### Events
#### `NotificationLogged` (`System.Action<Dynamo.Logging.NotificationMessage>`)

**Signature:** `public event System.Action<Dynamo.Logging.NotificationMessage> NotificationLogged`

event that is raised when a notification is logged

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

## ILogMessage (interface)

A message that can be logged with an ILogger.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/ILogger.cs)

### Methods
#### `void Log(Dynamo.Logging.ILogger logger)`

Makes logger log message.

**Parameters:**
- `logger` (Dynamo.Logging.ILogger) — Logger.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/ILogger.cs)

## ILogSource (interface)

An object that emits log messages.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/ILogger.cs)

### Events
#### `MessageLogged` (`System.Action<Dynamo.Logging.ILogMessage>`)

**Signature:** `public event System.Action<Dynamo.Logging.ILogMessage> MessageLogged`

Emits LogMessages.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/ILogger.cs)

## ILogger (interface)

Consumes messages to be used for logging.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/ILogger.cs)

### Methods
#### `void Log(System.Exception e)`

Logs exception.

**Parameters:**
- `e` (System.Exception) — Exception.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/ILogger.cs)

#### `void Log(string message)`

Logs the specified message.

**Parameters:**
- `message` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/ILogger.cs)

#### `void Log(string tag, string message)`

Logs data with an associated tag.

**Parameters:**
- `tag` (string) — Tag.
- `message` (string) — Message to be logged.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/ILogger.cs)

#### `void LogError(string error)`

Logs error.

**Parameters:**
- `error` (string) — Error message.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/ILogger.cs)

#### `void LogWarning(string warning, Dynamo.Logging.WarningLevel level)`

Logs warning.

**Parameters:**
- `warning` (string) — Warning message.
- `level` (Dynamo.Logging.WarningLevel) — Warning level.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/ILogger.cs)

## LogEventArgs (class)

Represents Event arguments that are passed to log event handler.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

### Constructors
- `void LogEventArgs(System.Exception e, Dynamo.Logging.LogLevel level)` — Creates LogEventArgs based on exception.
- `void LogEventArgs(string message, Dynamo.Logging.LogLevel level)` — Creates LogEventArgs based on log message.

### Properties
- `Level` (Dynamo.Logging.LogLevel, get/set) — The log level at which to log the message.
- `Message` (string, get/set) — The message to be logged.

## LogEventHandler (delegate)

This is a delegate used by Log events.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

### Constructors
- `void LogEventHandler(object object, IntPtr method)` — LogEventHandler.LogEventHandler

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Logging.LogEventArgs args, System.AsyncCallback callback, object object)`

LogEventHandler.BeginInvoke

**Parameters:**
- `args` (Dynamo.Logging.LogEventArgs)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void EndInvoke(System.IAsyncResult result)`

LogEventHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

#### `void Invoke(Dynamo.Logging.LogEventArgs args)`

LogEventHandler.Invoke

**Parameters:**
- `args` (Dynamo.Logging.LogEventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

## LogLevel (enum)

Specifies the level for log messages. A log message could be a console or file or warning.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

### Values
- `Console` = `0`
- `ConsoleOnly` = `3`
- `File` = `1`
- `Warning` = `2`

## LogMessage (static-class)

Factory methods for creating log messages.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogMessage.cs)

### Methods
#### `Dynamo.Logging.ILogMessage Error(System.Exception exception)`

Creates a LogMessage representing an error.

**Parameters:**
- `exception` (System.Exception) — 

**Returns:** `Dynamo.Logging.ILogMessage` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogMessage.cs)

#### `Dynamo.Logging.ILogMessage Error(string message)`

Creates a LogMessage representing an error.

**Parameters:**
- `message` (string) — 

**Returns:** `Dynamo.Logging.ILogMessage` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogMessage.cs)

#### `Dynamo.Logging.ILogMessage Info(string message)`

Creates a basic LogMessage.

**Parameters:**
- `message` (string) — 

**Returns:** `Dynamo.Logging.ILogMessage` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogMessage.cs)

#### `void Log(Dynamo.Logging.ILogger logger, Dynamo.Logging.ILogMessage message)`

Logs a LogMessage.

**Parameters:**
- `logger` (Dynamo.Logging.ILogger) — 
- `message` (Dynamo.Logging.ILogMessage) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogMessage.cs)

#### `Dynamo.Logging.ILogMessage Warning(string message, Dynamo.Logging.WarningLevel severity)`

Creates a LogMessage representing a warning.

**Parameters:**
- `message` (string) — 
- `severity` (Dynamo.Logging.WarningLevel) — 

**Returns:** `Dynamo.Logging.ILogMessage` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogMessage.cs)

## LogSourceBase (class)

An object that can log messages.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogSource.cs)

### Constructors
- `void LogSourceBase()` — LogSourceBase.LogSourceBase

### Methods
#### `Dynamo.Logging.ILogger AsLogger()`

Creates an ILogger out of this LogSourceBase; logging to the ILogger will send messages out of the LogMessage event.

**Returns:** `Dynamo.Logging.ILogger` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogSource.cs)

#### `void Log(Dynamo.Logging.ILogMessage obj)`

Logs ILogMessage objects.

**Parameters:**
- `obj` (Dynamo.Logging.ILogMessage) — Message object

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogSource.cs)

#### `void Log(System.Exception ex)`

Transform exception into ILogMessage object and logs it.

**Parameters:**
- `ex` (System.Exception) — Exception

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogSource.cs)

#### `void Log(string msg)`

Transform string into ILogMessage object and logs it.

**Parameters:**
- `msg` (string) — String

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogSource.cs)

#### `void Log(string msg, Dynamo.Logging.WarningLevel severity)`

Logs string message with some warning level.

**Parameters:**
- `msg` (string) — String
- `severity` (Dynamo.Logging.WarningLevel) — Warning level

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogSource.cs)

### Events
#### `MessageLogged` (`System.Action<Dynamo.Logging.ILogMessage>`)

**Signature:** `public event System.Action<Dynamo.Logging.ILogMessage> MessageLogged`

Emits LogMessages.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/LogSource.cs)

## NotificationMessage (class)

Class representing a notification that a host Application intends to send to a Dynamo user.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/NotificationMessage.cs)

### Constructors
- `void NotificationMessage(string sender, string shortMessage, string detailedMessage, string title)` — Constructor for a NotificationMessage

### Properties
- `DetailedMessage` (string, get) — a more detailed message that displayed
- `Sender` (string, get) — The sender of this notification.
- `ShortMessage` (string, get) — A short message that is displayed to inform the user briefly for the reason of the notification
- `Title` (string, get) — The title of the notification message.

## WarningLevel (enum)

Specifies the warning level for log messages.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Logging/DynamoLogger.cs)

### Values
- `Error` = `2`
- `Mild` = `0`
- `Moderate` = `1`

