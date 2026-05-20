using System.IO.Pipes;
using System.Text.Json;
using System.Text.Json.Nodes;
using AwareRevit.Shared;
using AwareRevit.Sidecar.Verbs;
using Xunit;

namespace AwareRevit.Tests;

public class EndToEndTests
{
    static readonly JsonSerializerOptions JsonOpts = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
        DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull,
    };

    /// <summary>Spin up an in-process pipe server that mimics AwareRevit.dll:
    /// reads one request, returns a canned ok-receipt. Drive Exec.RunAsync
    /// against it with a known pipe name and verify the envelope.</summary>
    [Fact]
    public async Task ExecRunAsync_AgainstStubAddin_EmitsOkEnvelope()
    {
        var pipeName = $"aware-revit-test-{Guid.NewGuid():N}";
        var serverTask = Task.Run(async () =>
        {
            using var server = new NamedPipeServerStream(pipeName, PipeDirection.InOut, 1,
                PipeTransmissionMode.Byte, PipeOptions.Asynchronous);
            await server.WaitForConnectionAsync();
            var reqJson = await PipeFrame.ReadAsync(server);
            Assert.NotNull(reqJson);
            var req = JsonSerializer.Deserialize<ExecRequest>(reqJson!, JsonOpts);
            Assert.NotNull(req);
            Assert.Equal("return 42;", req!.Code);

            var resp = new ExecResponse
            {
                Id = req.Id,
                Ok = true,
                Result = 42,
                StdoutLog = "stub-noise",
                HostVersion = "2026",
                HostPid = 99999,
            };
            await PipeFrame.WriteAsync(server, JsonSerializer.Serialize(resp, JsonOpts));
        });

        var input = JsonNode.Parse("""{"code":"return 42;"}""");

        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = await Exec.RunAsync(input, pipeNameOverride: pipeName); }
        finally { Console.SetOut(originalOut); }

        Assert.Equal(0, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.NotNull(receipt);
        Assert.True(receipt!["ok"]!.GetValue<bool>());
        Assert.Equal("revit", receipt["host"]!.GetValue<string>());
        Assert.Equal("exec", receipt["verb"]!.GetValue<string>());
        Assert.Equal("2026", receipt["host_version"]!.GetValue<string>());
        Assert.Equal(99999, receipt["host_pid"]!.GetValue<int>());
        Assert.Equal(42, receipt["result"]!.GetValue<int>());
        Assert.Equal("stub-noise", receipt["stdout_log"]!.GetValue<string>());

        await serverTask;
    }

    [Fact]
    public async Task ExecRunAsync_AddinReturnsError_EmitsFailEnvelope()
    {
        var pipeName = $"aware-revit-test-{Guid.NewGuid():N}";
        var serverTask = Task.Run(async () =>
        {
            using var server = new NamedPipeServerStream(pipeName, PipeDirection.InOut, 1,
                PipeTransmissionMode.Byte, PipeOptions.Asynchronous);
            await server.WaitForConnectionAsync();
            var reqJson = await PipeFrame.ReadAsync(server);
            var req = JsonSerializer.Deserialize<ExecRequest>(reqJson!, JsonOpts);

            var resp = new ExecResponse
            {
                Id = req!.Id,
                Ok = false,
                Error = "CS0103: 'doc' missing",
                Stack = "stub-stack",
                StdoutLog = "",
                HostVersion = "2026",
                HostPid = 99999,
            };
            await PipeFrame.WriteAsync(server, JsonSerializer.Serialize(resp, JsonOpts));
        });

        var input = JsonNode.Parse("""{"code":"return doc;"}""");

        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = await Exec.RunAsync(input, pipeNameOverride: pipeName); }
        finally { Console.SetOut(originalOut); }

        Assert.Equal(2, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.NotNull(receipt);
        Assert.False(receipt!["ok"]!.GetValue<bool>());
        Assert.Contains("CS0103", receipt["error"]!.GetValue<string>());
        Assert.Equal("stub-stack", receipt["stack"]!.GetValue<string>());

        await serverTask;
    }
}
