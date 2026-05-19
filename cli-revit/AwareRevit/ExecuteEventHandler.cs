// IExternalEventHandler — Revit's ONLY safe cross-thread mechanism. The pipe
// server runs on a background thread; when a request arrives it enqueues an
// ExecRequest and calls ExternalEvent.Raise(). Revit calls Execute() on the
// main thread the next time it goes idle. Execute() pops the request, runs
// the ScriptEngine against the live UIApplication, and stashes the response
// for the pipe server to pick up.

using System.Collections.Concurrent;
using Autodesk.Revit.UI;
using AwareRevit.Shared;

namespace AwareRevit.AddIn;

internal sealed class ExecuteEventHandler : IExternalEventHandler
{
    readonly ConcurrentQueue<PendingRequest> _queue = new();

    public string GetName() => "AwareRevit.ExecuteEventHandler";

    /// <summary>Called by the pipe server thread. Returns a Task that completes
    /// when Execute() finishes the work. Holds at most one outstanding request
    /// per UI idle cycle (the queue is drained by Execute()).</summary>
    public Task<ExecResponse> Enqueue(ExecRequest req, ExternalEvent ev)
    {
        var tcs = new TaskCompletionSource<ExecResponse>();
        _queue.Enqueue(new PendingRequest(req, tcs));
        ev.Raise();
        return tcs.Task;
    }

    public void Execute(UIApplication app)
    {
        while (_queue.TryDequeue(out var pending))
        {
            try
            {
                var resp = ScriptEngine.RunOnMainThread(app, pending.Req);
                pending.Tcs.TrySetResult(resp);
            }
            catch (Exception ex)
            {
                pending.Tcs.TrySetResult(new ExecResponse
                {
                    Id = pending.Req.Id,
                    Ok = false,
                    Error = $"add-in execute crashed: {ex.GetType().Name}: {ex.Message}",
                    Stack = ex.StackTrace ?? "",
                    StdoutLog = "",
                    HostPid = System.Diagnostics.Process.GetCurrentProcess().Id,
                    HostVersion = app.Application.VersionNumber,
                });
            }
        }
    }

    sealed record PendingRequest(
        ExecRequest Req,
        TaskCompletionSource<ExecResponse> Tcs);
}
