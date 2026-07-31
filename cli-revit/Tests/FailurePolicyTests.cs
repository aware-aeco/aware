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
    public void OnlyResolvableWarningsOnACommitAreWorthRetrying()
    {
        // Nothing blocking, Revit is committing, first attempt: clear the warnings and
        // let the commit re-run. This is the path that stops a dialog from ever opening,
        // which is what stops Commit() returning Pending.
        Assert.Equal(
            AwareFailureDecision.CommitAfterResolving,
            AwareFailurePolicy.Decide(hasBlockingFailure: false, beingCommitted: true, attempt: 1));
    }

    [Fact]
    public void ABlockingFailureRollsBackRatherThanRetrying()
    {
        // Revit will not let a commit past an Error, and AWARE has no resolution to
        // offer unattended. Retrying would just re-post it; the honest move is a silent
        // rollback the receipt can explain.
        Assert.Equal(
            AwareFailureDecision.RollBackSilently,
            AwareFailurePolicy.Decide(hasBlockingFailure: true, beingCommitted: true, attempt: 1));
    }

    [Fact]
    public void ARollbackAlreadyUnderwayIsNeverTurnedIntoACommit()
    {
        // Autodesk: ProceedWithCommit "cannot be used if the transaction is already
        // being rolled back, and will be treated as ProceedWithRollBack in this case."
        // Decide it explicitly rather than relying on Revit to reinterpret it.
        Assert.Equal(
            AwareFailureDecision.RollBackSilently,
            AwareFailurePolicy.Decide(hasBlockingFailure: false, beingCommitted: false, attempt: 1));
        Assert.Equal(
            AwareFailureDecision.RollBackSilently,
            AwareFailurePolicy.Decide(hasBlockingFailure: true, beingCommitted: false, attempt: 1));
    }

    [Fact]
    public void ASelfRegeneratingFailureCannotLoopForever()
    {
        // Every ProceedWithCommit re-runs end-of-transaction checks, and Autodesk warns a
        // handler "should be careful not to try to repeatedly commit if it is unable to
        // deal with all the errors". A failure that re-posts itself must terminate in a
        // rollback, not spin.
        for (var attempt = 1; attempt < AwareFailurePolicy.MaxCommitAttempts; attempt++)
        {
            Assert.Equal(
                AwareFailureDecision.CommitAfterResolving,
                AwareFailurePolicy.Decide(false, true, attempt));
        }

        Assert.Equal(
            AwareFailureDecision.RollBackSilently,
            AwareFailurePolicy.Decide(false, true, AwareFailurePolicy.MaxCommitAttempts));
        Assert.Equal(
            AwareFailureDecision.RollBackSilently,
            AwareFailurePolicy.Decide(false, true, AwareFailurePolicy.MaxCommitAttempts + 1));
    }

    [Fact]
    public void TheRetryBoundIsSmallButLeavesRoomForOnePass()
    {
        // One pass is the normal case (delete the warnings, commit succeeds). A bound of
        // 1 would mean never retrying at all, which defeats the mechanism.
        Assert.True(AwareFailurePolicy.MaxCommitAttempts >= 2);
        Assert.True(AwareFailurePolicy.MaxCommitAttempts <= 5);
    }
}
