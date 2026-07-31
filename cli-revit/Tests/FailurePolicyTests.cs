// Tests for the failure policy that keeps a Revit write from ever waiting on a
// human (#337).
//
// `AwareFailurePreprocessor` itself cannot be driven here — `PreprocessFailures`
// takes a `FailuresAccessor`, which only Revit can hand out. The decision it makes
// is factored into `AwareFailurePolicy` precisely so the part that matters is
// testable without Revit. The plumbing around it is covered by
// BakeSceneScriptTests' compile-against-the-real-RevitAPI check.

using Xunit;

namespace AwareRevit.Tests;

public class FailurePolicyTests
{
    [Fact]
    public void AWarningOnlyCommitIsAllowedToFinish()
    {
        // Nothing blocking and Revit is committing: the warnings have been deleted, so
        // default processing has nothing left to show and the commit finishes. This is
        // the path that stops a dialog from ever opening, which is what stops Commit()
        // returning Pending.
        Assert.Equal(
            AwareFailureDecision.ContinueAfterResolving,
            AwareFailurePolicy.Decide(hasBlockingFailure: false, beingCommitted: true));
    }

    [Fact]
    public void ABlockingFailureRollsBackRatherThanProceeding()
    {
        // Revit will not let a commit past an Error, and AWARE has no resolution to
        // offer unattended. The honest move is a silent rollback the receipt can explain.
        Assert.Equal(
            AwareFailureDecision.RollBackSilently,
            AwareFailurePolicy.Decide(hasBlockingFailure: true, beingCommitted: true));
    }

    [Fact]
    public void ARollbackAlreadyUnderwayIsNeverTurnedIntoACommit()
    {
        // Autodesk: a commit result "cannot be used if the transaction is already being
        // rolled back, and will be treated as ProceedWithRollBack in this case." Decide
        // it explicitly rather than relying on Revit to reinterpret it.
        Assert.Equal(
            AwareFailureDecision.RollBackSilently,
            AwareFailurePolicy.Decide(hasBlockingFailure: false, beingCommitted: false));
        Assert.Equal(
            AwareFailureDecision.RollBackSilently,
            AwareFailurePolicy.Decide(hasBlockingFailure: true, beingCommitted: false));
    }

    [Fact]
    public void ARepostedWarningIsNeverEscalatedIntoARollback()
    {
        // The decision must not depend on how many times the handler has run. A warning
        // the model re-posts on every pass — a permanently off-axis brace is exactly that
        // — would otherwise exhaust a retry bound and roll back a transaction whose only
        // sin was a warning, which Revit itself defines as ignorable. Continue takes one
        // pass, so there is no loop and nothing to escalate.
        for (var i = 0; i < 25; i++)
        {
            Assert.Equal(
                AwareFailureDecision.ContinueAfterResolving,
                AwareFailurePolicy.Decide(hasBlockingFailure: false, beingCommitted: true));
        }
    }

    [Fact]
    public void APendingTransactionIsRootedAndItsFinalizerSuppressed()
    {
        // Not disposing is not enough: an unrooted Transaction can still have Revit's
        // finalizer run, and that rolls back the edit this path exists to protect.
        var before = AwarePendingCommits.Count;
        var sentinel = new object();

        AwarePendingCommits.LeaveWithRevit(sentinel);
        Assert.Equal(before + 1, AwarePendingCommits.Count);

        // Idempotent — handing the same transaction over twice must not double-root it.
        AwarePendingCommits.LeaveWithRevit(sentinel);
        Assert.Equal(before + 1, AwarePendingCommits.Count);

        // Null is a no-op rather than a crash on an already-bad path.
        AwarePendingCommits.LeaveWithRevit(null);
        Assert.Equal(before + 1, AwarePendingCommits.Count);
    }
}
