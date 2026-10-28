---
name: revit-transactions
description: This skill should be used when composing snippets that mutate Revit model state — adding/removing elements, modifying parameters, placing families, changing geometry, or any write operation against the active Document. Encodes the `Transaction` / `TransactionGroup` / `SubTransaction` hierarchy, the mandatory `Start` / `Commit` lifecycle (no implicit transactions in Revit), why writes outside a transaction throw `InvalidOperationException`, the v0.11 AWARE safety-contract integration, and common rollback gotchas.
---

# Revit transactions

Revit's data model is **transactional by force**. The API REFUSES to mutate any document state outside an open `Transaction`. This is non-negotiable; there's no implicit-transaction mode.

## The three transaction types

| Type | Scope | Undo behavior |
|---|---|---|
| `Transaction` | Single atomic change | One entry on the user's undo stack |
| `TransactionGroup` | Multiple Transactions coalesced | One entry on undo stack (covers all child transactions when assimilated) |
| `SubTransaction` | Inside a parent Transaction | Internal-only; no undo entry. Use for partial rollback within an op. |

## The canonical lifecycle

```csharp
using Autodesk.Revit.DB;

var doc = uiapp.ActiveUIDocument.Document;

using (var t = new Transaction(doc, "AWARE: my-write-op"))
{
    t.Start();
    try
    {
        // ... write operations ...
        // doc.Create.NewWall(...)
        // someParam.Set(value)
        t.Commit();
    }
    catch
    {
        t.RollBack();
        throw;
    }
}
```

The transaction name (`"AWARE: my-write-op"`) appears in the user's Edit > Undo menu. Use a stable verb-prefixed string so users can correlate the undo entry with the AWARE app that ran.

## TransactionGroup — for multi-step ops

When a single AWARE verb performs multiple write steps that should be ONE user undo, wrap them in a `TransactionGroup`:

```csharp
using (var tg = new TransactionGroup(doc, "AWARE: revision.bump"))
{
    tg.Start();
    try
    {
        using (var t1 = new Transaction(doc, "Step 1: bump revision sequence"))
        {
            t1.Start();
            // ...
            t1.Commit();
        }
        using (var t2 = new Transaction(doc, "Step 2: stamp affected sheets"))
        {
            t2.Start();
            // ...
            t2.Commit();
        }
        tg.Assimilate();   // collapse children into ONE undo entry
    }
    catch
    {
        tg.RollBack();
        throw;
    }
}
```

`Assimilate()` vs `Commit()`: `Assimilate` collapses all child Transactions into one undo entry. `Commit` preserves them as separate entries. AWARE workflow verbs should use `Assimilate` — the user sees ONE entry per verb.

## SubTransaction — for partial rollback

```csharp
using (var t = new Transaction(doc, "AWARE: complex-op"))
{
    t.Start();

    using (var sub = new SubTransaction(doc))
    {
        sub.Start();
        // ... tentative writes that may fail ...
        if (some_condition) sub.RollBack();
        else                 sub.Commit();
    }

    // ... continue with more writes that DON'T roll back ...

    t.Commit();
}
```

SubTransactions don't appear in the user's undo stack. They're useful when an algorithm wants to "try a write and back out if it doesn't work" without losing the surrounding transaction.

## What throws when you write without a transaction

Almost everything that mutates the document:

- `doc.Create.NewWall(...)` → `InvalidOperationException: Attempt to create a new wall outside of a transaction`
- `param.Set(value)` → `InvalidOperationException`
- `doc.Delete(elemId)` → `InvalidOperationException`
- `doc.Regenerate()` → throws if no transaction is open

**Reads NEVER need a transaction.** You can `OfClass(typeof(Wall))` and iterate freely outside a transaction.

## Transaction status

A Transaction can be in one of these states:

| Status | Meaning |
|---|---|
| `Uninitialized` | Just constructed; `Start()` not called |
| `Started` | `Start()` called; writes allowed |
| `Committed` | `Commit()` succeeded |
| `RolledBack` | `RollBack()` or constructor failure |
| `Error` | Internal Revit error during commit |

Check via `t.GetStatus()`. If you `Start()` an already-Started transaction (e.g. via accidental nesting outside a SubTransaction), Revit throws.

## Common gotchas

- **Nested `Transaction`s throw.** Use `SubTransaction` for nesting INSIDE a Transaction; use `TransactionGroup` to coalesce SIBLING Transactions.
- **`Regenerate()` is sometimes required mid-Transaction.** When subsequent writes depend on geometry recomputed from prior writes, call `doc.Regenerate()` to flush.
- **`Element.GetTypeId()` is sometimes invalid mid-Transaction.** If your write modified the type, re-read the element via `doc.GetElement(elemId)`.
- **Failed `Commit()` returns `TransactionStatus.Error` — it doesn't throw.** Always check the status:
  ```csharp
  var status = t.Commit();
  if (status != TransactionStatus.Committed)
      return new { ok = false, error = $"Commit failed: {status}" };
  ```
- **`RollBack()` after a failed `Commit()` is a no-op** — Revit already rolled back internally.

## Failure handling — `FailuresPreprocessor`

When Revit detects a constraint violation (overlapping walls, intersecting beams, etc.), it raises a `Failure` event. By default, the failure becomes a UI dialog asking the user how to proceed — which BLOCKS an AWARE exec script forever waiting for human input.

For non-interactive scripts:

```csharp
using (var t = new Transaction(doc, "AWARE: foo"))
{
    var options = t.GetFailureHandlingOptions();
    options.SetFailuresPreprocessor(new SuppressWarningsPreprocessor());
    options.SetClearAfterRollback(true);
    t.SetFailureHandlingOptions(options);
    t.Start();
    // ...
    t.Commit();
}

class SuppressWarningsPreprocessor : IFailuresPreprocessor
{
    public FailureProcessingResult PreprocessFailures(FailuresAccessor a)
    {
        a.DeleteAllWarnings();
        return FailureProcessingResult.Continue;
    }
}
```

AWARE's v0.33 add-in installs a default preprocessor for this — but if you need stricter or looser handling, install your own.

## Relation to AWARE v0.11 safety contract

The v0.11 app-spec mandates `safety.transaction-group: <name>` on write-mode nodes. The AWARE runtime uses the name as the **TransactionGroup's** name when shipping the snippet. The user-supplied `code` should NOT wrap its writes in its OWN `TransactionGroup` — AWARE owns the outer scope. User code wraps in individual `Transaction`s (and these get assimilated into the AWARE-owned group on commit).

## Standard pattern in `aware-revit exec` snippets

```csharp
var doc = uiapp.ActiveUIDocument.Document;

using (var t = new Transaction(doc, "AWARE: <verb-name>"))
{
    var opts = t.GetFailureHandlingOptions();
    opts.SetFailuresPreprocessor(new SuppressWarningsPreprocessor());
    t.SetFailureHandlingOptions(opts);

    t.Start();
    try
    {
        // ... user writes ...
        var status = t.Commit();
        if (status != TransactionStatus.Committed)
            return new { ok = false, error = $"Commit returned {status}" };
        return new { ok = true, /* result data */ };
    }
    catch
    {
        if (t.HasStarted() && !t.HasEnded()) t.RollBack();
        throw;
    }
}
```

## See also

- [revit-element-collector](./revit-element-collector.md) — read-side query
- [revit-parameters](./revit-parameters.md) — what most writes touch
- [v0.11 safety contract spec](../../../../10-core/app-spec.md#safety-contract-write-mode-nodes)
