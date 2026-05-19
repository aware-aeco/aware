---
name: rhino-rhino-runtime-inprocess
description: This skill encodes the rhino 8.0 surface of the Rhino.Runtime.InProcess namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: RhinoCore, WindowStyle.
---

# Rhino.Runtime.InProcess

Auto-generated from vendor docs for rhino 8.0. 2 types in this namespace.

## RhinoCore (class)

Represents an instance of RhinoCore.DLL.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_InProcess_RhinoCore.htm)

### Constructors
- `public RhinoCore()` — Initializes a new instance of the RhinoCore class.
- `public RhinoCore(string[] args)` — Initializes a new instance of the RhinoCore class.
- `public RhinoCore(string[] args, WindowStyle windowStyle)` — Initializes a new instance of the RhinoCore class.
- `public RhinoCore(string[] args, WindowStyle windowStyle, IntPtr hostWnd)` — Initializes a new instance of the RhinoCore class.

### Methods
#### `public void Dispose()`

IDisposable implementation

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_InProcess_RhinoCore_Dispose.htm)

#### `public bool DoEvents()`

If a Rhino owned window is active processes all Windows messages currently in the message queue.

**Returns:** `Boolean` — Returns true if a Rhino owned window is still active or Idle tasks are pending.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_InProcess_RhinoCore_DoEvents.htm)

#### `public bool DoIdle()`

Processes all Rhino Idle tasks.

**Returns:** `Boolean` — True if Idle tasks are still pending.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_InProcess_RhinoCore_DoIdle.htm)

#### `protected override void Finalize()`

IDisposable implementation

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_InProcess_RhinoCore_Finalize.htm)

#### `public void InvokeInHostContext(Action action)`

Invokes action in Host context

**Parameters:**
- `action` (System.Action) — [Missing <param name="action"/> documentation for "M:Rhino.Runtime.InProcess.RhinoCore.InvokeInHostContext(System.Action)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_InProcess_RhinoCore_InvokeInHostContext.htm)

#### `public T InvokeInHostContext<T>(Func<T> func)`

Invokes function in Host context

**Parameters:**
- `func` (System.Func<T>) — [Missing <param name="func"/> documentation for "M:Rhino.Runtime.InProcess.RhinoCore.InvokeInHostContext``1(System.Func{``0})"]

**Returns:** `T` — Returns argument function return value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_InProcess_RhinoCore_InvokeInHostContext__1.htm)

#### `public void RaiseIdle()`

Raises the Idle event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_InProcess_RhinoCore_RaiseIdle.htm)

#### `public int Run()`

Runs RhinoApp Message loop

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Runtime.InProcess.RhinoCore.Run"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_InProcess_RhinoCore_Run.htm)

## WindowStyle (enum)

Specifies how a new window should appear when Rhino starts.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_InProcess_WindowStyle.htm)

### Values
- `NoWindow` = `-1` — The no window style.
- `Normal` = `0` — The normal, visible window style.
- `Hidden` = `1` — The hidden window style.
- `Minimized` = `2` — The minimized window style.
- `Maximized` = `3` — The maximized window style.

