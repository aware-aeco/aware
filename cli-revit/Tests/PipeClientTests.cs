using System.IO.Pipes;
using AwareRevit.Shared;
using AwareRevit.Sidecar;
using Xunit;

namespace AwareRevit.Tests;

public class PipeClientTests
{
    /// <summary>Spin up an in-process pipe server on a random name and verify the
    /// client can round-trip a request through it.</summary>
    [Fact]
    public async Task SendRequest_RoundTrips()
    {
        var pipeName = $"aware-revit-test-{Guid.NewGuid():N}";
        var serverTask = Task.Run(async () =>
        {
            using var server = new NamedPipeServerStream(pipeName, PipeDirection.InOut, 1,
                PipeTransmissionMode.Byte, PipeOptions.Asynchronous);
            await server.WaitForConnectionAsync();
            var reqJson = await PipeFrame.ReadAsync(server);
            Assert.NotNull(reqJson);
            Assert.Contains("\"abc\"", reqJson);
            // Echo back a canned response.
            await PipeFrame.WriteAsync(server,
                """{"id":"abc","ok":true,"result":42,"host_version":"2026","host_pid":1111}""");
        });

        var client = new PipeClient(pipeName);
        var respJson = await client.SendRequestAsync(
            """{"id":"abc","code":"return 1;"}""",
            timeout: TimeSpan.FromSeconds(5));

        Assert.Contains("\"ok\":true", respJson);
        Assert.Contains("\"host_version\":\"2026\"", respJson);
        await serverTask;
    }

    [Fact]
    public async Task SendRequest_FailsClearly_WhenPipeMissing()
    {
        var client = new PipeClient($"aware-revit-noexist-{Guid.NewGuid():N}");
        await Assert.ThrowsAsync<TimeoutException>(async () =>
            await client.SendRequestAsync("""{"id":"x"}""", timeout: TimeSpan.FromSeconds(1)));
    }
}
