// Connects to AwareRevit.dll's per-process named pipe and round-trips a
// length-prefixed JSON request. Connect-retries with 500ms backoff so we
// tolerate a short race between sidecar launch and add-in OnStartup.

using System.IO.Pipes;
using AwareRevit.Shared;

namespace AwareRevit.Sidecar;

internal sealed class PipeClient
{
    public string PipeName { get; }

    public PipeClient(string pipeName)
    {
        PipeName = pipeName;
    }

    /// <summary>Connect + write request + read response. Retries connect every
    /// 500ms until <paramref name="timeout"/> elapses. Throws TimeoutException
    /// if the pipe never appears, EndOfStreamException if the add-in closes the
    /// pipe mid-frame.</summary>
    public async Task<string> SendRequestAsync(string requestJson, TimeSpan timeout)
    {
        using var pipe = new NamedPipeClientStream(".", PipeName, PipeDirection.InOut,
            PipeOptions.Asynchronous);

        var deadline = DateTime.UtcNow + timeout;
        while (true)
        {
            try
            {
                await pipe.ConnectAsync(250);
                break;
            }
            catch (TimeoutException)
            {
                if (DateTime.UtcNow >= deadline)
                    throw new TimeoutException(
                        $"pipe '{PipeName}' did not appear within {timeout.TotalSeconds:0.0}s " +
                        $"(is AwareRevit.dll loaded? check Revit > Add-Ins ribbon)");
                await Task.Delay(250);
            }
        }

        await PipeFrame.WriteAsync(pipe, requestJson);
        var responseJson = await PipeFrame.ReadAsync(pipe)
            ?? throw new EndOfStreamException("pipe closed before response was sent");
        return responseJson;
    }
}
