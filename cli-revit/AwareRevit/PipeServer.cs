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

    readonly ExecuteEventHandler _handler;
    readonly ExternalEvent _event;
    readonly string _pipeName;
    readonly CancellationTokenSource _cts = new();
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

        var resp = await _handler.Enqueue(req, _event);
        var respJson = JsonSerializer.Serialize(resp, JsonOpts);
        await PipeFrame.WriteAsync(server, respJson, ct);
    }
}
