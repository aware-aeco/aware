// Tests for the in-Revit pipe server's wedge detection (#328).
//
// The server itself cannot be driven without Revit — Enqueue needs an
// ExternalEvent, which only Revit can construct — so what is pinned here is the
// part that is pure logic: how long the server is willing to wait for Revit's
// API thread before it answers the caller and re-arms its listener.

using AwareRevit.AddIn;
using Xunit;

namespace AwareRevit.Tests;

public class PipeServerTests
{
    [Fact]
    public void AnAbsentOrUnusableOverrideFallsBackToTheDefaultWait()
    {
        // A wedge detector that silently resolved to 0 would abandon every
        // request immediately, so anything unparseable or non-positive must land
        // on the default rather than being taken literally.
        Assert.Equal(PipeServer.DefaultHandlerTimeoutMs, PipeServer.ResolveHandlerTimeoutMs(null));
        Assert.Equal(PipeServer.DefaultHandlerTimeoutMs, PipeServer.ResolveHandlerTimeoutMs(""));
        Assert.Equal(PipeServer.DefaultHandlerTimeoutMs, PipeServer.ResolveHandlerTimeoutMs("   "));
        Assert.Equal(PipeServer.DefaultHandlerTimeoutMs, PipeServer.ResolveHandlerTimeoutMs("not-a-number"));
        Assert.Equal(PipeServer.DefaultHandlerTimeoutMs, PipeServer.ResolveHandlerTimeoutMs("0"));
        Assert.Equal(PipeServer.DefaultHandlerTimeoutMs, PipeServer.ResolveHandlerTimeoutMs("-1"));
        Assert.Equal(PipeServer.DefaultHandlerTimeoutMs, PipeServer.ResolveHandlerTimeoutMs("12.5"));
    }

    [Fact]
    public void APositiveOverrideIsHonoured()
    {
        Assert.Equal(1, PipeServer.ResolveHandlerTimeoutMs("1"));
        Assert.Equal(30_000, PipeServer.ResolveHandlerTimeoutMs("30000"));
    }

    [Fact]
    public void TheDefaultWaitIsGenerousEnoughNotToCancelRealWork()
    {
        // This bound exists to detect a blocked API thread, not to budget
        // execution: a legitimate exec or bake can run for minutes, and cutting
        // one off would report a wedge that isn't there. Ten minutes matches the
        // Rhino sink's mutating-bake policy.
        Assert.Equal(10 * 60 * 1000, PipeServer.DefaultHandlerTimeoutMs);
        Assert.True(PipeServer.DefaultHandlerTimeoutMs >= 5 * 60 * 1000);
    }
}
