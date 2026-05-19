using System.IO;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Text.Json.Nodes;
using AwareSketchup;
using AwareSketchup.Verbs;
using Xunit;

namespace AwareSketchup.Tests;

public class ExecVerbTests : IDisposable
{
    readonly string _discoveryDir;

    public ExecVerbTests()
    {
        _discoveryDir = Path.Combine(Path.GetTempPath(), $"aware-sketchup-exec-test-{Guid.NewGuid():N}");
        Directory.CreateDirectory(_discoveryDir);
    }

    public void Dispose()
    {
        try { Directory.Delete(_discoveryDir, recursive: true); } catch { }
    }

    [Fact]
    public void Exec_NoInstance_FailsWithDescriptiveError()
    {
        var client = new SketchupClient(_discoveryDir, pidAlive: _ => false);
        var input = JsonNode.Parse("{\"verb\":\"exec\",\"code\":\"return 1\"}");

        using var sw = new StringWriter();
        var original = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = Exec.Run(input, client); }
        finally { Console.SetOut(original); }

        Assert.Equal(1, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.NotNull(receipt);
        Assert.False(receipt!["ok"]!.GetValue<bool>());
        Assert.Contains("no SketchUp instance", receipt["error"]!.GetValue<string>());
    }

    [Fact]
    public void Exec_MissingCode_Fails()
    {
        var client = new SketchupClient(_discoveryDir);
        var input = JsonNode.Parse("{\"verb\":\"exec\"}");

        using var sw = new StringWriter();
        var original = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = Exec.Run(input, client); }
        finally { Console.SetOut(original); }

        Assert.Equal(2, exit);
        Assert.Contains("missing required field: code", sw.ToString());
    }

    [Fact]
    public void Exec_AgainstStubBridge_ReturnsOkReceipt()
    {
        // Stand up a TCP listener pretending to be the SketchUp bridge.
        var listener = new TcpListener(IPAddress.Loopback, 0);
        listener.Start();
        var port = ((IPEndPoint)listener.LocalEndpoint).Port;
        var pid  = System.Diagnostics.Process.GetCurrentProcess().Id;

        // Write a discovery file pointing at our stub.
        File.WriteAllText(Path.Combine(_discoveryDir, $"{pid}.json"),
            $"{{\"pid\":{pid},\"port\":{port},\"version\":\"26.0\",\"started_at\":\"2026-01-01T00:00:00Z\"}}");

        var serverTask = Task.Run(() =>
        {
            using var conn = listener.AcceptTcpClient();
            using var stream = conn.GetStream();
            var got = SketchupClient.ReadLengthPrefixed(stream, timeoutMs: 5000);
            // Sanity-check the request shape.
            var req = JsonNode.Parse(Encoding.UTF8.GetString(got)) as JsonObject;
            Assert.NotNull(req);
            Assert.Equal("exec", req!["type"]!.GetValue<string>());
            Assert.Equal("'hello'", req["code"]!.GetValue<string>());

            // Respond with a canned ok-receipt.
            var canned = "{\"ok\":true,\"result\":\"hello\",\"stdout_log\":\"some debug\"}";
            SketchupClient.WriteLengthPrefixed(stream, Encoding.UTF8.GetBytes(canned));
            listener.Stop();
        });

        var client = new SketchupClient(_discoveryDir, pidAlive: p => p == pid);
        var input = JsonNode.Parse($"{{\"verb\":\"exec\",\"sketchup_id\":\"{pid}\",\"code\":\"'hello'\"}}");

        using var sw = new StringWriter();
        var original = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = Exec.Run(input, client, timeoutMs: 10_000); }
        finally { Console.SetOut(original); }

        serverTask.Wait(TimeSpan.FromSeconds(10));
        Assert.Equal(0, exit);

        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.NotNull(receipt);
        Assert.True(receipt!["ok"]!.GetValue<bool>());
        Assert.Equal("sketchup", receipt["host"]!.GetValue<string>());
        Assert.Equal(pid, receipt["host_pid"]!.GetValue<int>());
        Assert.Equal("26.0", receipt["host_version"]!.GetValue<string>());
        Assert.Equal("hello", receipt["result"]!.GetValue<string>());
        Assert.Equal("some debug", receipt["stdout_log"]!.GetValue<string>());
    }

    [Fact]
    public void Exec_BridgeReportsUserError_ReturnsFailReceiptButExit0()
    {
        var listener = new TcpListener(IPAddress.Loopback, 0);
        listener.Start();
        var port = ((IPEndPoint)listener.LocalEndpoint).Port;
        var pid  = System.Diagnostics.Process.GetCurrentProcess().Id;

        File.WriteAllText(Path.Combine(_discoveryDir, $"{pid}.json"),
            $"{{\"pid\":{pid},\"port\":{port},\"version\":\"26.0\",\"started_at\":\"2026-01-01T00:00:00Z\"}}");

        var serverTask = Task.Run(() =>
        {
            using var conn = listener.AcceptTcpClient();
            using var stream = conn.GetStream();
            SketchupClient.ReadLengthPrefixed(stream, timeoutMs: 5000);
            var canned = "{\"ok\":false,\"error\":\"NameError: undefined local var foo\",\"stack\":\"(aware-exec):1\",\"stdout_log\":\"\"}";
            SketchupClient.WriteLengthPrefixed(stream, Encoding.UTF8.GetBytes(canned));
            listener.Stop();
        });

        var client = new SketchupClient(_discoveryDir, pidAlive: p => p == pid);
        var input = JsonNode.Parse($"{{\"verb\":\"exec\",\"sketchup_id\":\"{pid}\",\"code\":\"foo\"}}");

        using var sw = new StringWriter();
        var original = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = Exec.Run(input, client, timeoutMs: 5_000); }
        finally { Console.SetOut(original); }

        serverTask.Wait(TimeSpan.FromSeconds(10));
        // The exec mechanism worked — the user's code is what failed. Receipt
        // is {ok:false}, but the verb returns 0 (only dispatch failures get non-zero).
        Assert.Equal(0, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.False(receipt!["ok"]!.GetValue<bool>());
        Assert.Contains("NameError", receipt["error"]!.GetValue<string>());
    }
}
