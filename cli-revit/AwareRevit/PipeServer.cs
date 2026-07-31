// NamedPipeServerStream listener inside Revit's process. Owns a worker thread
// that ConnectAsync-loops, reads one length-prefixed JSON request per
// connection, marshals it through the IExternalEventHandler, writes the
// response back, closes the pipe, opens a new one. Single connection at a
// time (the sidecar serializes its requests).

using System.IO.Pipes;
using System.Text.Json;
using Autodesk.Revit.UI;
using AwareRevit.Shared;

namespace AwareRevit.AddIn;

internal sealed class PipeServer
{
    static readonly JsonSerializerOptions JsonOpts = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
        DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull,
    };

    /// <summary>How long to wait for Revit's API thread to run a request before
    /// giving that ONE caller an answer and re-arming the listener. Generous by
    /// design — a legitimate exec can run for minutes, and this is a wedge
    /// detector, not an execution budget. Override with
    /// AWARE_REVIT_HANDLER_TIMEOUT_MS in the environment Revit was launched
    /// with (the add-in lives in Revit's process, so it inherits Revit's
    /// environment, not the CLI's).</summary>
    internal const int DefaultHandlerTimeoutMs = 10 * 60 * 1000;

    internal static int ResolveHandlerTimeoutMs(string? raw)
        => int.TryParse(raw, out var ms) && ms > 0 ? ms : DefaultHandlerTimeoutMs;

    readonly ExecuteEventHandler _handler;
    readonly ExternalEvent _event;
    readonly string _pipeName;
    readonly CancellationTokenSource _cts = new();
    readonly int _handlerTimeoutMs = ResolveHandlerTimeoutMs(
        Environment.GetEnvironmentVariable("AWARE_REVIT_HANDLER_TIMEOUT_MS"));
    Thread? _listener;

    public PipeServer(ExecuteEventHandler handler, ExternalEvent ev)
    {
        _handler = handler;
        _event = ev;
        var pid = System.Diagnostics.Process.GetCurrentProcess().Id;
        _pipeName = $"aware-revit-{pid}";
    }

    public string PipeName => _pipeName;

    public void Start()
    {
        // Run the async listen-loop on a dedicated background thread. We
        // wrap the await of ListenLoopAsync with `GetAwaiter().GetResult()` so
        // that any unhandled exception in the loop terminates THIS thread, not
        // Revit's main thread. The Thread is IsBackground=true so it dies with
        // Revit. async void is avoided because unhandled async-void exceptions
        // can take down the host process.
        _listener = new Thread(() =>
        {
            try { ListenLoopAsync(_cts.Token).GetAwaiter().GetResult(); }
            catch (OperationCanceledException) { /* normal shutdown */ }
            catch (Exception ex)
            {
                try { System.Diagnostics.Debug.WriteLine($"AwareRevit pipe loop crashed: {ex}"); }
                catch { }
            }
        })
        {
            IsBackground = true,
            Name = "AwareRevit.PipeServer",
        };
        _listener.Start();
    }

    public void Stop()
    {
        _cts.Cancel();
        // The listening NamedPipeServerStream.WaitForConnectionAsync respects
        // the token; the loop exits naturally.
    }

    async Task ListenLoopAsync(CancellationToken ct)
    {
        while (!ct.IsCancellationRequested)
        {
            try
            {
                var server = new NamedPipeServerStream(
                    _pipeName,
                    PipeDirection.InOut,
                    maxNumberOfServerInstances: 1,
                    PipeTransmissionMode.Byte,
                    PipeOptions.Asynchronous);

                try
                {
                    await server.WaitForConnectionAsync(ct);
                    await HandleOne(server, ct);
                }
                finally
                {
                    await server.DisposeAsync();
                }
            }
            catch (OperationCanceledException) { /* stopping */ }
            catch (Exception ex)
            {
                // Don't kill the loop on per-connection errors; just log.
                try { System.Diagnostics.Debug.WriteLine($"AwareRevit pipe error: {ex}"); }
                catch { }
                // Short backoff so a broken state doesn't busy-loop.
                try { await Task.Delay(500, ct); } catch { }
            }
        }
    }

    async Task HandleOne(NamedPipeServerStream server, CancellationToken ct)
    {
        var reqJson = await PipeFrame.ReadAsync(server, ct);
        if (reqJson is null) return;

        ExecRequest? req;
        try
        {
            req = JsonSerializer.Deserialize<ExecRequest>(reqJson, JsonOpts);
        }
        catch (Exception ex)
        {
            var bad = new ExecResponse
            {
                Id = "",
                Ok = false,
                Error = $"request JSON parse failed: {ex.Message}",
                StdoutLog = "",
            };
            await PipeFrame.WriteAsync(server, JsonSerializer.Serialize(bad, JsonOpts), ct);
            return;
        }
        if (req is null)
        {
            var bad = new ExecResponse
            {
                Id = "",
                Ok = false,
                Error = "request JSON deserialised to null",
                StdoutLog = "",
            };
            await PipeFrame.WriteAsync(server, JsonSerializer.Serialize(bad, JsonOpts), ct);
            return;
        }

        // Revit runs Execute() on its API thread. When something blocks that thread
        // — most often a modal failure dialog raised by a user script's own
        // unguarded transaction — this task never completes. Awaiting it forever
        // takes the WHOLE bridge down, not just this request: HandleOne never
        // returns, so the listen loop never constructs the next
        // NamedPipeServerStream (maxNumberOfServerInstances is 1), every later verb
        // fails to connect, and the sidecar reports that as `addin_loaded:false` —
        // "not installed", for an add-in that is loaded and merely waiting on a
        // dialog. Bounding the wait degrades ONE request and re-arms the listener,
        // so the next caller reaches the bridge and gets told what is actually
        // wrong (#328).
        var work = _handler.Enqueue(req, _event);
        // Linked source so the timer is cancelled the moment the work wins,
        // rather than being left to fire minutes later.
        using var timeoutCts = CancellationTokenSource.CreateLinkedTokenSource(ct);
        var completed = await Task.WhenAny(work, Task.Delay(_handlerTimeoutMs, timeoutCts.Token));
        timeoutCts.Cancel();

        ExecResponse resp;
        if (completed == work)
        {
            resp = await work;
        }
        else
        {
            // The request is still queued and Revit may yet run it, so do not
            // claim it failed — say what is true and name the recovery.
            resp = new ExecResponse
            {
                Id = req.Id,
                Ok = false,
                Error =
                    "the add-in is loaded but Revit's API thread has not run this request within " +
                    $"{_handlerTimeoutMs / 1000}s — check Revit for an open dialog waiting on a click. " +
                    "The request is still queued and may still be applied once Revit is released.",
                Stack = "",
                StdoutLog = "",
                HostPid = System.Diagnostics.Process.GetCurrentProcess().Id,
            };
        }

        var respJson = JsonSerializer.Serialize(resp, JsonOpts);
        await PipeFrame.WriteAsync(server, respJson, ct);
    }
}
