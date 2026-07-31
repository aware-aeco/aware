// Custody of a transaction Revit has not finished with (#337).
//
// WHY THIS LIVES IN THE ADD-IN, not beside the failure preprocessor: the bake
// script is a Roslyn submission compiled fresh for every request. Anything
// static declared in that script text is a NEW type per invocation, so a later
// bake could never see — let alone release — what an earlier one rooted. With
// finalization suppressed that is a native transaction stranded until Revit
// exits. The add-in assembly is the only thing on this path that outlives a
// single request, so custody belongs here and both write paths call in.
//
// Public, and deliberately typed in terms of `object`, so the bake script can
// reach it from its own assembly without every caller needing a compile-time
// Revit reference.

using Autodesk.Revit.DB;

namespace AwareRevit.AddIn;

/// <summary>Holds transactions whose commit came back Pending, until Revit resolves them.</summary>
/// <remarks>
/// Not disposing such a transaction is NOT enough on its own: once the caller's local
/// goes out of scope the object is unrooted, and the GC may run Revit's finalizer —
/// documented to roll back an unfinished transaction and discard the edit. So the
/// finalizer is suppressed AND a durable root held, until the status says Revit is done.
/// </remarks>
public static class AwarePendingRevitCommits
{
    static readonly List<object> Rooted = new();

    /// <summary>Hand a Pending transaction to Revit: no rollback, no dispose, no
    /// finalizer, until Revit has finished with it.</summary>
    public static void LeaveWithRevit(object? transaction)
    {
        // Sweep first. Pending resolves asynchronously and Revit refuses further writes
        // until it does, so by the time another one arrives the previous is almost always
        // finished and can be released. Without this the list would only ever grow.
        ReleaseFinished();

        if (transaction is null) return;
        GC.SuppressFinalize(transaction);
        lock (Rooted)
        {
            if (!Rooted.Contains(transaction)) Rooted.Add(transaction);
        }
    }

    /// <summary>Dispose and drop every rooted transaction Revit has since finished with.
    /// Returns how many were released.</summary>
    public static int ReleaseFinished()
    {
        lock (Rooted)
        {
            var released = 0;
            for (var i = Rooted.Count - 1; i >= 0; i--)
            {
                if (!HasEnded(Rooted[i])) continue;

                // Disposing a FINISHED transaction is the ordinary path — it releases the
                // native object without rolling anything back. Only safe because the
                // status was tested first.
                if (Rooted[i] is IDisposable disposable)
                {
                    try { disposable.Dispose(); } catch { /* already gone; drop it anyway */ }
                }
                Rooted.RemoveAt(i);
                released++;
            }
            return released;
        }
    }

    /// <summary>How many transactions Revit still owns. Diagnostic only.</summary>
    public static int Count
    {
        get { lock (Rooted) { return Rooted.Count; } }
    }

    static bool HasEnded(object? candidate)
    {
        // Anything we can no longer even query is treated as ended: holding a reference
        // that answers nothing serves nobody.
        if (candidate is not Transaction transaction) return true;
        try { return transaction.HasEnded(); }
        catch { return true; }
    }
}
