---
name: tekla-events-and-clashes
description: This skill should be used when subscribing to Tekla model events or running clash detection from generated code — selection changes, model object changes (insertions / modifications / deletions), model save, application exit, or clash check workflows. Encodes the `Events` instance lifecycle (Register / UnRegister), the `ChangeData.ChangeType` enum, the `ClashCheckHandler` pattern, and the IDisposable wrapper for event monitors. Apply when composing AWARE apps that react to model state changes, when detecting clashes between model objects, or when monitoring user selections inside Tekla.
---

# Tekla — events and clash detection

## Critical patterns

- Create the `Events` instance and subscribe to delegates **before** calling `Register()`.
- Always call `UnRegister()` when done to avoid memory leaks. Wrap in `IDisposable` when ownership is unclear.
- Event handlers run on Tekla's thread — avoid heavy processing in callbacks. Marshal back to a worker thread for anything expensive. See [`event-threading`](./event-threading.md) for the per-event thread map.
- Use `ClashCheckHandler.RunClashCheck()` to trigger clash analysis.
- ClashCheck events (`ClashDetected`, `ClashCheckDone`) fire during and after analysis, in that order.

## Event types

| Event | Fires when |
|-------|------------|
| `SelectionChange` | User changes selection in a model view |
| `ModelObjectChanged` | Any model object is created, modified, or deleted |
| `ModelSave` | Model is saved (before or after) |
| `Interrupted` | Tekla interrupts the current operation |
| `TeklaStructuresExit` | Tekla is closing |
| `ClashDetected` | A clash is found during a clash check |
| `ClashCheckDone` | Clash check completes |

## Register and unregister events

```csharp
using Tekla.Structures.Model;

var events = new Events();

// Subscribe to events
events.SelectionChange += OnSelectionChanged;
events.ModelObjectChanged += OnModelObjectChanged;
events.ModelSave += OnModelSaved;
events.TeklaStructuresExit += OnTeklaExit;
events.Interrupted += OnInterrupted;

// Start listening
events.Register();

// ... application runs ...

// Stop listening (IMPORTANT — always unregister)
events.UnRegister();
```

## Selection change event

```csharp
private static void OnSelectionChanged()
{
    var model = new Model();
    var selector = new Tekla.Structures.Model.UI.ModelObjectSelector();
    var selected = selector.GetSelectedObjects();

    while (selected.MoveNext())
    {
        if (selected.Current is Part part)
        {
            part.Select();
            // Process selected part
        }
    }
}
```

## Model object changed event

```csharp
private static void OnModelObjectChanged(List<ChangeData> changes)
{
    foreach (var change in changes)
    {
        var obj = change.Object;

        switch (change.Type)
        {
            case ChangeData.ChangeType.OBJECT_INSERTED:
                // New object created
                break;
            case ChangeData.ChangeType.OBJECT_MODIFIED:
                // Existing object modified
                break;
            case ChangeData.ChangeType.OBJECT_DELETED:
                // Object deleted (obj may be invalid)
                break;
        }
    }
}
```

> When emitting downstream from `ModelObjectChanged`, key by `Mark`, never `Name`. See [`drawing-identity`](./drawing-identity.md).

## Model save event

```csharp
private static void OnModelSaved()
{
    // Fires after model save completes.
    // Suitable for post-save automation (export, logging, etc.).
}
```

## Clash detection

```csharp
using Tekla.Structures.Model;

var events = new Events();
var clashResults = new List<(ModelObject, ModelObject)>();

events.ClashDetected += (ClashCheckData data) =>
{
    // Fires for each clash found.
    clashResults.Add((data.Object1, data.Object2));
};

events.ClashCheckDone += () =>
{
    // All clashes detected — process results.
    foreach (var (obj1, obj2) in clashResults)
    {
        // Report or fix clashes.
    }
};

events.Register();

// Trigger clash check
var clashChecker = new ClashCheckHandler();
clashChecker.RunClashCheck();

// Unregister when done
events.UnRegister();
```

## Complete event-monitor pattern (IDisposable)

```csharp
using Tekla.Structures;
using Tekla.Structures.Model;

public class TeklaEventMonitor : IDisposable
{
    private readonly Events events;
    private bool disposed;

    public TeklaEventMonitor()
    {
        events = new Events();
        events.SelectionChange += OnSelectionChanged;
        events.ModelObjectChanged += OnModelObjectChanged;
        events.Register();
    }

    private void OnSelectionChanged()
    {
        // Handle selection changes
    }

    private void OnModelObjectChanged(List<ChangeData> changes)
    {
        // Handle model changes
    }

    public void Dispose()
    {
        if (disposed is false)
        {
            events.UnRegister();
            disposed = true;
        }
    }
}
```

`IDisposable` is the right wrapper when an agent command owns the subscription's lifetime. The Tekla agent's `watch` command uses this pattern internally.

## `ChangeData.ChangeType` enum

| Value | Description |
|-------|-------------|
| `OBJECT_INSERTED` | New object was created |
| `OBJECT_MODIFIED` | Existing object was modified |
| `OBJECT_DELETED` | Object was deleted (the `obj` reference may already be invalid — guard reads) |

## See also

- [`event-threading`](./event-threading.md) — per-event thread map, marshaling rules, burst handling
- [`drawing-identity`](./drawing-identity.md) — what to use as the identity in emitted payloads
- [`commands/watch.md`](../commands/watch.md) — the agent's watch command (built on this skill's patterns)
