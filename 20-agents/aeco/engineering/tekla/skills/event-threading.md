---
name: tekla-event-threading
description: This skill should be used when subscribing to Tekla events, registering event handlers, or designing reactive flows that respond to Tekla model changes. Encodes the threading-model gotcha — `ModelUnloadingSync` runs on the main Tekla thread while most other events run async on a worker thread. Apply when wiring `ModelObjectChanged`, `ConnectionInserted`, `DrawingChanged`, `SelectionChanged`, or any event handler that may touch UI, cross-process resources, or shared state, and when handling burst event streams.
---

# Tekla event threading

**`ModelUnloadingSync` runs on the main Tekla thread. Most other events run async on a worker thread. Marshal back to the UI dispatcher when downstream nodes need UI access.**

The threading model is not consistent across Tekla event handlers. The Open API surface gives no hint. Getting this wrong produces either:

- Deadlocks (sync UI work from a worker thread that's blocking on the main thread), or
- Silent failures (touching UI from a non-UI thread).

## The two thread classes

| Event | Thread | Notes |
|---|---|---|
| `ModelUnloadingSync` | **Main Tekla thread** | Fires synchronously as the model unloads. UI is responsive here. |
| `ModelObjectChanged` | Worker thread (async) | Fires after the change is committed. Buffer + dispatch. |
| `ConnectionInserted` | Worker thread (async) | Same. |
| `DrawingChanged` | Worker thread (async) | Same. |
| `SelectionChanged` | UI thread | Mostly safe but can fire in bursts. |

When in doubt, read the per-event note on `developer.tekla.com` — the threading is documented per event, and it has been wrong in places. Do not trust IntelliSense alone.

## Marshaling pattern

```csharp
events.ModelObjectChanged += (changes) =>
{
    Dispatcher.Invoke(() =>
    {
        foreach (var ch in changes)
            if (ch.Object is Assembly a)
                EmitDownstream(new { mark = a.GetMark(), type = a.AssemblyType });
    });
};
```

Use `Dispatcher.Invoke` to marshal to the UI thread. Use `BeginInvoke` (fire-and-forget) when no return value is needed and blocking the worker is undesirable.

## What goes wrong without marshaling

- WPF/WinForms throws `InvalidOperationException` when touching UI controls from a non-UI thread.
- COM-marshaled UI objects throw `RPC_E_WRONG_THREAD`.
- Some Tekla operations silently fail when called from the wrong thread (no exception, no log — just nothing happens).

## Burst handling

Worker-thread events can fire in rapid bursts (e.g. a bulk paste of 500 assemblies). Two options:

- Debounce 100–250ms before emitting downstream.
- Batch into a single emit with an array payload.

The agent's `watch` command currently emits one event per change (no debounce). For bursty models, debounce or batch downstream — apply it in the consuming node rather than relying on the source.

## Out-of-process subscribers: two hard requirements (delegate shape + message pump)

A subscriber in a **separate process** (e.g. the `aware-tekla` bridge's `watch`) does receive Tekla events, but two things must be right or events silently never fire. Both were learned the hard way against a live model (verified on Tekla 2025 + 2026); see [`aware-aeco/aware#219`](https://github.com/aware-aeco/aware/issues/219) and its follow-up.

**1. The handler must be a delegate shape Tekla actually invokes.**

- ✅ **Open static** (`Delegate.CreateDelegate(type, staticMethodInfo)`, `Target == null`) — works. `OnChanged(object)` binds to `void(List<ChangeData>)` by reference contravariance.
- ✅ **Instance method** (`events.X += obj.Handler`, or `CreateDelegate(type, instance, instanceMethodInfo)`, `Target == an object`) — works. This is what the Open API samples and FloLess's `TeklaEventManager` use.
- ❌ **`Reflection.Emit.DynamicMethod`** — accepted by `+=`, but Tekla **never invokes it**. A version-agnostic bridge with no compile-time Tekla types is tempted to emit one; don't.
- ❌ **Closed *static*** (`CreateDelegate(type, firstArg, staticMethodInfo)`, where `firstArg` is e.g. a string — `Target` is a string) — also **not invoked**. To carry per-event state without a closure, use an **instance** emitter object (`Target` = the object), not a string-closed static.

**2. UI-thread events need a Windows message pump on an STA thread.**

The two event classes deliver differently:
- **Worker-thread events** (`ModelObjectChanged`, `ConnectionInserted`, …) arrive on Tekla's own async thread — delivered to a plain MTA process with no pump.
- **UI-thread events** (`SelectionChange`, `AnnotationSelectionChange`, `ViewClosed`, `ViewCameraChanged`, `HiddenObjectsChanged`, `TemporaryStatesChanged`, …) are posted to the **message queue** and fire **only while that thread pumps**. The Open API's own `TeklaEvents` sample is a WinForms (STA + `Application.Run`) app for exactly this reason.

So a watcher that only blocks on a wait handle (MTA, no pump) gets `ModelObjectChanged` but **never** `SelectionChange`. Run the registration + handlers on a **dedicated STA thread with a message pump** (`MsgWaitForMultipleObjects` + `PeekMessage`/`TranslateMessage`/`DispatchMessage`, waiting on your stop handle alongside the queue), and both classes deliver.

Symptom signatures: a `DynamicMethod`/closed-static handler → *zero* events of any kind; a real handler but no pump → `ModelObjectChanged` fires but UI events (`SelectionChange`, …) never do.

## Cross-process variant

`TCM_GETITEMW` and other Win32 messages sent cross-process from Tekla event handlers need `VirtualAllocEx` + `WriteProcessMemory` — they cannot share a heap with the target process. Settle ≥ 300ms after `SwitchTab` for reliable reads.

## Source

Verified per release against `developer.tekla.com`. Specific timings (300ms settle) from production observation.
