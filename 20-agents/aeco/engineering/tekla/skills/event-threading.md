# Tekla Skill · Event threading

**`ModelUnloadingSync` runs on the main Tekla thread. Most other events run async on a worker thread. Marshal back to the UI dispatcher when downstream nodes need UI access.**

The threading model is not consistent across Tekla event handlers. The Open API surface gives you no hint. Get this wrong and you get either:
- Deadlocks (calling sync UI work from a worker thread that's blocking on the main thread), or
- Silent failures (trying to touch UI from a non-UI thread)

## The two thread classes

| Event | Thread | Notes |
|---|---|---|
| `ModelUnloadingSync` | **Main Tekla thread** | Fires synchronously as the model unloads. UI is responsive here. |
| `ModelObjectChanged` | Worker thread (async) | Fires after the change is committed. Buffer + dispatch. |
| `ConnectionInserted` | Worker thread (async) | Same. |
| `DrawingChanged` | Worker thread (async) | Same. |
| `SelectionChanged` | UI thread | Mostly safe but can fire in bursts. |

When in doubt, **read the per-event note on `developer.tekla.com`** — the threading is documented per event, and it has been wrong in places. Don't trust IntelliSense alone.

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

`Dispatcher.Invoke` marshals to the UI thread. Use `BeginInvoke` (fire-and-forget) when you don't need a return value and want to avoid blocking the worker.

## What goes wrong without marshaling

- WPF/WinForms throws `InvalidOperationException` when touching UI controls from a non-UI thread
- COM-marshaled UI objects throw `RPC_E_WRONG_THREAD`
- Some Tekla operations silently fail when called from the wrong thread (no exception, no log — just nothing happens)

## Burst handling

Worker-thread events can fire in rapid bursts (e.g. a bulk paste of 500 assemblies). Either:
- Debounce 100–250ms before emitting downstream
- Batch into a single emit with an array payload

The agent's `watch` command does debounce by default. If you bypass `watch` and subscribe directly, debounce yourself.

## Cross-process variant

`TCM_GETITEMW` and other Win32 messages sent cross-process from the Tekla event handlers need `VirtualAllocEx` + `WriteProcessMemory` — they cannot share a heap with the target process. See `feedback_tekla_walker_lessons.md` in the FloLess memory for the proven settle timings (300ms minimum after `SwitchTab`).
