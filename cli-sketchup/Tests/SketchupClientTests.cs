using System.IO;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Text.Json.Nodes;
using AwareSketchup;
using Xunit;

namespace AwareSketchup.Tests;

public class SketchupClientTests : IDisposable
{
    readonly string _discoveryDir;

    public SketchupClientTests()
    {
        _discoveryDir = Path.Combine(Path.GetTempPath(), $"aware-sketchup-test-{Guid.NewGuid():N}");
        Directory.CreateDirectory(_discoveryDir);
    }

    public void Dispose()
    {
        try { Directory.Delete(_discoveryDir, recursive: true); } catch { /* best effort */ }
    }

    [Fact]
    public void ListInstances_EmptyDir_ReturnsEmpty()
    {
        var c = new SketchupClient(_discoveryDir);
        Assert.Empty(c.ListInstances());
    }

    [Fact]
    public void ListInstances_NonExistentDir_ReturnsEmpty()
    {
        var c = new SketchupClient(Path.Combine(_discoveryDir, "nope"));
        Assert.Empty(c.ListInstances());
    }

    [Fact]
    public void ListInstances_LiveAndDeadPids_FiltersOutDead()
    {
        // PID 1 is the system "Idle" process on Windows — always alive but
        // not ours. PID -1 / 99999 are almost certainly dead.
        File.WriteAllText(Path.Combine(_discoveryDir, "1.json"),
            "{\"pid\":1,\"port\":8765,\"version\":\"26.0\",\"started_at\":\"2026-01-01T00:00:00Z\"}");
        File.WriteAllText(Path.Combine(_discoveryDir, "99999.json"),
            "{\"pid\":99999,\"port\":8766,\"version\":\"26.0\",\"started_at\":\"2026-01-01T00:00:00Z\"}");

        // Stub `pidAlive` so the test doesn't depend on real OS state.
        var c = new SketchupClient(_discoveryDir, pidAlive: pid => pid == 1);
        var list = c.ListInstances();

        Assert.Single(list);
        Assert.Equal(1, list[0].Pid);
        Assert.Equal(8765, list[0].Port);
        Assert.Equal("26.0", list[0].Version);

        // Dead PID's file should have been cleaned up.
        Assert.False(File.Exists(Path.Combine(_discoveryDir, "99999.json")));
        Assert.True(File.Exists(Path.Combine(_discoveryDir, "1.json")));
    }

    [Fact]
    public void ListInstances_CorruptFile_IsRemoved()
    {
        File.WriteAllText(Path.Combine(_discoveryDir, "1.json"), "not valid json");
        var c = new SketchupClient(_discoveryDir, pidAlive: _ => true);
        Assert.Empty(c.ListInstances());
        Assert.False(File.Exists(Path.Combine(_discoveryDir, "1.json")));
    }

    [Fact]
    public void Resolve_ByPid_MatchesExact()
    {
        File.WriteAllText(Path.Combine(_discoveryDir, "100.json"),
            "{\"pid\":100,\"port\":8765,\"version\":\"26.0\",\"started_at\":\"2026-01-01T00:00:00Z\"}");
        File.WriteAllText(Path.Combine(_discoveryDir, "200.json"),
            "{\"pid\":200,\"port\":8766,\"version\":\"25.0\",\"started_at\":\"2026-01-01T00:00:00Z\"}");

        var c = new SketchupClient(_discoveryDir, pidAlive: _ => true);
        var inst = c.Resolve(sketchupId: "200", version: null);
        Assert.NotNull(inst);
        Assert.Equal(200, inst!.Pid);
        Assert.Equal("25.0", inst.Version);
    }

    [Fact]
    public void Resolve_ByVersion_StartsWith()
    {
        File.WriteAllText(Path.Combine(_discoveryDir, "100.json"),
            "{\"pid\":100,\"port\":8765,\"version\":\"26.0.123\",\"started_at\":\"2026-01-01T00:00:00Z\"}");
        var c = new SketchupClient(_discoveryDir, pidAlive: _ => true);
        var inst = c.Resolve(sketchupId: null, version: "26");
        Assert.NotNull(inst);
        Assert.Equal(100, inst!.Pid);
    }

    [Fact]
    public void Resolve_NoFiltersOneInstance_AutoPicks()
    {
        File.WriteAllText(Path.Combine(_discoveryDir, "100.json"),
            "{\"pid\":100,\"port\":8765,\"version\":\"26.0\",\"started_at\":\"2026-01-01T00:00:00Z\"}");
        var c = new SketchupClient(_discoveryDir, pidAlive: _ => true);
        Assert.NotNull(c.Resolve(sketchupId: null, version: null));
    }

    [Fact]
    public void Resolve_AmbiguousNoFilter_ReturnsNull()
    {
        File.WriteAllText(Path.Combine(_discoveryDir, "100.json"),
            "{\"pid\":100,\"port\":8765,\"version\":\"26.0\",\"started_at\":\"2026-01-01T00:00:00Z\"}");
        File.WriteAllText(Path.Combine(_discoveryDir, "200.json"),
            "{\"pid\":200,\"port\":8766,\"version\":\"25.0\",\"started_at\":\"2026-01-01T00:00:00Z\"}");
        var c = new SketchupClient(_discoveryDir, pidAlive: _ => true);
        Assert.Null(c.Resolve(sketchupId: null, version: null));
    }

    [Fact]
    public void ParseDiscoveryFile_HandlesMissingOptionals()
    {
        var inst = SketchupClient.ParseDiscoveryFile(
            "{\"pid\":1,\"port\":8765,\"version\":\"26.0\"}");
        Assert.NotNull(inst);
        Assert.Null(inst!.ModelPath);
    }

    [Fact]
    public void ParseDiscoveryFile_RejectsMissingRequired()
    {
        // No port.
        Assert.Null(SketchupClient.ParseDiscoveryFile("{\"pid\":1,\"version\":\"26.0\"}"));
        // No version.
        Assert.Null(SketchupClient.ParseDiscoveryFile("{\"pid\":1,\"port\":8765}"));
        // Empty version is rejected as "missing".
        Assert.Null(SketchupClient.ParseDiscoveryFile("{\"pid\":1,\"port\":8765,\"version\":\"\"}"));
    }

    [Fact]
    public void LengthPrefix_RoundTrip()
    {
        using var ms = new MemoryStream();
        var payload = Encoding.UTF8.GetBytes("hello world");
        SketchupClient.WriteLengthPrefixed(ms, payload);
        ms.Position = 0;
        var got = SketchupClient.ReadLengthPrefixed(ms, timeoutMs: 1000);
        Assert.Equal(payload, got);
    }

    [Fact]
    public void LengthPrefix_BigEndianOrder()
    {
        // 256 bytes — should encode as 0x00 0x00 0x01 0x00, not 0x00 0x01 0x00 0x00.
        using var ms = new MemoryStream();
        var payload = new byte[256];
        SketchupClient.WriteLengthPrefixed(ms, payload);
        var bytes = ms.ToArray();
        Assert.Equal(0x00, bytes[0]);
        Assert.Equal(0x00, bytes[1]);
        Assert.Equal(0x01, bytes[2]);
        Assert.Equal(0x00, bytes[3]);
    }

    [Fact]
    public void SendRequest_AgainstStubServer_RoundTrips()
    {
        // Spin up a fake bridge: accept one connection, read a length-prefixed
        // request, echo back a canned response.
        var listener = new TcpListener(IPAddress.Loopback, 0);
        listener.Start();
        var port = ((IPEndPoint)listener.LocalEndpoint).Port;

        var serverTask = Task.Run(() =>
        {
            using var client = listener.AcceptTcpClient();
            using var stream = client.GetStream();
            var got = SketchupClient.ReadLengthPrefixed(stream, timeoutMs: 5000);
            // Verify the bridge would have seen our request body.
            var requestJson = JsonNode.Parse(Encoding.UTF8.GetString(got)) as JsonObject;
            Assert.NotNull(requestJson);
            Assert.Equal("exec", requestJson!["type"]!.GetValue<string>());
            Assert.Equal("return 42", requestJson["code"]!.GetValue<string>());

            // Send the response.
            var canned = "{\"ok\":true,\"result\":42,\"stdout_log\":\"\"}";
            SketchupClient.WriteLengthPrefixed(stream, Encoding.UTF8.GetBytes(canned));
            listener.Stop();
        });

        var c = new SketchupClient(_discoveryDir, pidAlive: _ => true);
        var response = c.SendRequest(port, new JsonObject
        {
            ["type"] = "exec",
            ["code"] = "return 42",
            ["args"] = new JsonObject(),
        }, timeoutMs: 10_000);

        serverTask.Wait(TimeSpan.FromSeconds(10));
        Assert.True((response as JsonObject)!["ok"]!.GetValue<bool>());
        Assert.Equal(42, (response as JsonObject)!["result"]!.GetValue<int>());
    }
}
