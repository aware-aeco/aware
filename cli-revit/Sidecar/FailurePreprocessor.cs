// ── AWARE Revit failure handling ─────────────────────────────────────────────
// Resolves Revit's end-of-transaction failures programmatically, so an
// unattended write never waits on a human.
//
// WHY THIS EXISTS (#337). `SetForcedModalHandling(false)` does NOT suppress
// Revit's failure dialog — Autodesk documents it as "false to allow a
// non-blocking dialog". A non-blocking dialog is exactly the state in which
// `Transaction.Commit()` returns `TransactionStatus.Pending`, which Autodesk
// documents as "failure handling has not been finalized yet and Revit awaits a
// user's actions". So suppressing the modal (#328) did not remove the dialog; it
// made `Pending` the EXPECTED outcome of any warning-worthy write.
//
// `Pending` is a trap for a bridge:
//   * "Until committing is fully finalized, no changes to the document can be
//     made (including starting of new transactions)" — so the whole document is
//     blocked, not just this request.
//   * `RollBack()` throws when "the transaction's document is currently in
//     failure mode".
//   * Disposing an unfinished transaction makes "the default destructor roll it
//     back automatically, thus all changes ... will be discarded" — so a
//     `using` scope silently throws the user's edit away on the way out.
//
// A preprocessor removes the dialog itself. Revit calls it BEFORE showing
// anything; resolving the failures here means `ProceedWithCommit` re-runs the
// commit with nothing left to report ("If some failures were resolved here, they
// will be removed and not delivered to the user"), and `ProceedWithRollBack`
// with clear-after-rollback set rolls back silently. Neither path waits on a
// user, so `Pending` stops being reachable — and the failures Revit objected to
// become data the receipt can carry instead of a dialog nobody sees.
//
// This file is compiled INTO the in-Revit add-in (so `exec` uses it) AND
// embedded verbatim as source into the bake-scene script (so `bake-scene` uses
// the same text). One text, two compilations — the two write paths cannot drift.
//
// CONSTRAINTS, because the same text must compile as a C# *script*:
//   * no `namespace` (scripts forbid them) and no `using` directives (a script
//     requires every using to precede all other code; the script header owns
//     them). Revit types are therefore fully qualified; BCL collections rely on
//     the script header's usings and the add-in's ImplicitUsings.
#nullable disable

/// <summary>What to do with a transaction whose end-of-transaction checks failed.</summary>
internal enum AwareFailureDecision
{
    /// <summary>Warnings were cleared and nothing blocking remains — retry the commit.</summary>
    CommitAfterResolving,

    /// <summary>Something unresolvable remains (or the caller is already rolling
    /// back) — roll back, silently.</summary>
    RollBackSilently,
}

/// <summary>The pure half of the failure policy: no Revit types, so it is unit-testable.</summary>
internal static class AwareFailurePolicy
{
    /// <summary>Revit re-runs end-of-transaction checks after every
    /// ProceedWithCommit, and warns that a handler "should be careful not to try
    /// to repeatedly commit if it is unable to deal with all the errors". This is
    /// the bound that keeps a self-regenerating failure from looping forever.</summary>
    internal const int MaxCommitAttempts = 3;

    /// <param name="hasBlockingFailure">A failure of severity Error or worse — one Revit
    /// will not let a commit past without a resolution AWARE cannot supply unattended.</param>
    /// <param name="beingCommitted">False once Revit is already rolling back; ProceedWithCommit
    /// is not permitted then and would be treated as a rollback anyway.</param>
    /// <param name="attempt">1 for the first call, incremented on each re-entry.</param>
    internal static AwareFailureDecision Decide(bool hasBlockingFailure, bool beingCommitted, int attempt)
    {
        if (!beingCommitted) return AwareFailureDecision.RollBackSilently;
        if (hasBlockingFailure) return AwareFailureDecision.RollBackSilently;
        if (attempt >= MaxCommitAttempts) return AwareFailureDecision.RollBackSilently;
        return AwareFailureDecision.CommitAfterResolving;
    }
}

/// <summary>Collects what Revit objected to, clears what can be cleared, and decides
/// the transaction's fate without ever handing it to a dialog.</summary>
internal sealed class AwareFailurePreprocessor : Autodesk.Revit.DB.IFailuresPreprocessor
{
    readonly List<string> _warnings = new List<string>();
    readonly List<string> _errors = new List<string>();
    int _attempts;

    /// <summary>Warning text Revit posted, deduped, in first-seen order. Reported in the
    /// receipt so the caller learns what the model objected to — the thing the dialog
    /// used to say to nobody.</summary>
    internal IReadOnlyList<string> Warnings => _warnings;

    /// <summary>Failures of severity Error or worse. Non-empty means the write was rolled back.</summary>
    internal IReadOnlyList<string> Errors => _errors;

    /// <summary>True when a blocking failure forced the rollback, so the caller can say
    /// WHY rather than just that the commit did not happen.</summary>
    internal bool RolledBackForErrors => _errors.Count > 0;

    public Autodesk.Revit.DB.FailureProcessingResult PreprocessFailures(
        Autodesk.Revit.DB.FailuresAccessor failuresAccessor)
    {
        _attempts++;

        // Read every message BEFORE deleting anything: DeleteAllWarnings invalidates
        // the accessors, and these strings are the receipt's only account of what
        // Revit found.
        var blocking = false;
        foreach (var failure in failuresAccessor.GetFailureMessages())
        {
            string text;
            try { text = failure.GetDescriptionText(); }
            catch { text = "(Revit supplied no description for this failure)"; }
            if (string.IsNullOrWhiteSpace(text)) text = "(Revit supplied no description for this failure)";

            if (failure.GetSeverity() == Autodesk.Revit.DB.FailureSeverity.Warning)
            {
                if (!_warnings.Contains(text)) _warnings.Add(text);
            }
            else
            {
                if (!_errors.Contains(text)) _errors.Add(text);
                blocking = true;
            }
        }

        // Warnings are ignorable by definition ("do not prevent transactions from
        // being committed"), and deleting them is what stops Revit showing them.
        failuresAccessor.DeleteAllWarnings();

        var decision = AwareFailurePolicy.Decide(
            blocking, failuresAccessor.IsTransactionBeingCommitted(), _attempts);

        if (decision == AwareFailureDecision.RollBackSilently)
        {
            // Without clear-after-rollback the errors survive the rollback and Revit
            // still delivers them to the user — the dialog this whole file exists to
            // avoid.
            var options = failuresAccessor.GetFailureHandlingOptions();
            options.SetClearAfterRollback(true);
            failuresAccessor.SetFailureHandlingOptions(options);
            return Autodesk.Revit.DB.FailureProcessingResult.ProceedWithRollBack;
        }

        return Autodesk.Revit.DB.FailureProcessingResult.ProceedWithCommit;
    }
}
