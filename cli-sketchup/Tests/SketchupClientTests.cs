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
        // A pre-0.35.0 bridge advertises no version — that must parse, not fail.
        Assert.Null(inst.BridgeVersion);
    }

    [Fact]
    public void ParseDiscoveryFile_ReadsBridgeVersion()
    {
        var inst = SketchupClient.ParseDiscoveryFile(
            "{\"pid\":1,\"port\":8765,\"version\":\"26.0\",\"bridge_version\":\"0.35.0\"}");
        Assert.NotNull(inst);
        Assert.Equal("0.35.0", inst!.BridgeVersion);

        // An empty string is "not reported", not a version.
        var blank = SketchupClient.ParseDiscoveryFile(
            "{\"pid\":1,\"port\":8765,\"version\":\"26.0\",\"bridge_version\":\"\"}");
        Assert.NotNull(blank);
        Assert.Null(blank!.BridgeVersion);
    }

    [Fact]
    public void StaleBridgeNote_SilentWhenEverythingIsInStep()
    {
        var inst = new SketchupInstance(42, 8765, "26.1", null, DateTime.MinValue, "0.35.0");
        Assert.Equal("", SketchupClient.StaleBridgeNote(inst, packagedVersion: "0.35.0", installedVersion: "0.35.0"));
        // Nothing readable to compare against → say nothing rather than guess.
        Assert.Equal("", SketchupClient.StaleBridgeNote(inst, packagedVersion: "", installedVersion: ""));
    }

    [Fact]
    public void StaleBridgeNote_TellsUserToRestartWhenOnlyTheSessionIsStale()
    {
        var older = new SketchupInstance(42, 8765, "26.1", null, DateTime.MinValue, "0.34.0");
        var note = SketchupClient.StaleBridgeNote(older, packagedVersion: "0.35.0", installedVersion: "0.35.0");
        Assert.Contains("0.34.0", note);
        Assert.Contains("restart SketchUp", note);
        Assert.DoesNotContain("--install-bridge", note);

        // A session from before bridge_version existed is stale in the same way.
        var unversioned = new SketchupInstance(42, 8765, "26.1", null, DateTime.MinValue, null);
        var legacyNote = SketchupClient.StaleBridgeNote(unversioned, packagedVersion: "0.35.0", installedVersion: "0.35.0");
        Assert.Contains("older than 0.35.0", legacyNote);
        Assert.Contains("restart SketchUp", legacyNote);
    }

    [Fact]
    public void StaleBridgeNote_SaysInstallFirstWhenThePluginsFolderIsAlsoBehind()
    {
        // Upgrading the CLI does not re-run --install-bridge, so restarting alone would
        // reload the SAME stale bridge. Saying "just restart" there is wrong advice.
        var running = new SketchupInstance(42, 8765, "26.1", null, DateTime.MinValue, "0.34.0");
        var note = SketchupClient.StaleBridgeNote(running, packagedVersion: "0.35.0", installedVersion: "0.34.0");
        Assert.Contains("--install-bridge", note);
        Assert.Contains("restart SketchUp", note);
    }

    [Fact]
    public void StaleBridgeNote_NeverAdvisesInstallingOverANewerBridge()
    {
        // Sidecar rolled back while a newer bridge stayed installed: --install-bridge
        // would DOWNGRADE it, so the note must not ask for it.
        var running = new SketchupInstance(42, 8765, "26.1", null, DateTime.MinValue, "0.35.0");
        var note = SketchupClient.StaleBridgeNote(running, packagedVersion: "0.35.0", installedVersion: "0.36.0");
        Assert.DoesNotContain("--install-bridge", note);
        Assert.Contains("restart SketchUp", note);   // 0.36.0 is installed, the session is on 0.35.0

        // …and when the session is already on that newer bridge, there is nothing to say.
        var current = new SketchupInstance(42, 8765, "26.1", null, DateTime.MinValue, "0.36.0");
        Assert.Equal("", SketchupClient.StaleBridgeNote(current, packagedVersion: "0.35.0", installedVersion: "0.36.0"));
    }

    [Fact]
    public void StaleBridgeNote_DoesNotAssertAFixWhenTheInstalledBridgeIsUnreadable()
    {
        // Unknown SketchUp year or a custom --plugins-dir: we cannot see what a restart
        // would load, so the note names the facts instead of prescribing a cure.
        var running = new SketchupInstance(42, 8765, "26.1", null, DateTime.MinValue, "0.34.0");
        var note = SketchupClient.StaleBridgeNote(running, packagedVersion: "0.35.0", installedVersion: "");
        Assert.Contains("could not be read", note);

        // Session already runs what this sidecar ships → nothing worth saying.
        var same = new SketchupInstance(42, 8765, "26.1", null, DateTime.MinValue, "0.35.0");
        Assert.Equal("", SketchupClient.StaleBridgeNote(same, packagedVersion: "0.35.0", installedVersion: ""));
    }

    [Theory]
    [InlineData("26.1.189", 2026)]
    [InlineData("25.0.455", 2025)]
    [InlineData("", null)]
    [InlineData("nonsense", null)]
    [InlineData("3.0", null)]          // implausible year — refuse rather than guess
    public void PluginYearForHostVersion_MapsSketchupMajorToItsYear(string hostVersion, int? expected)
    {
        Assert.Equal(expected, BridgeInstaller.PluginYearForHostVersion(hostVersion));
    }

    [Fact]
    public void CompareBridgeVersions_OrdersNumericallyAndRefusesGarbage()
    {
        Assert.Equal(1,  BridgeInstaller.CompareBridgeVersions("0.35.0", "0.34.0"));
        Assert.Equal(-1, BridgeInstaller.CompareBridgeVersions("0.35.0", "0.36.0"));
        Assert.Equal(0,  BridgeInstaller.CompareBridgeVersions("0.35.0", "0.35"));
        Assert.Equal(1,  BridgeInstaller.CompareBridgeVersions("0.35.1", "0.35"));
        // 10 > 9 numerically, which a string compare would get backwards.
        Assert.Equal(1,  BridgeInstaller.CompareBridgeVersions("0.10.0", "0.9.0"));
        Assert.Null(BridgeInstaller.CompareBridgeVersions("0.35.0", null));
        Assert.Null(BridgeInstaller.CompareBridgeVersions("v0.35.0", "0.34.0"));
    }

    [Fact]
    public void SendRequest_UnreachablePort_ReportsTheRequestWasNeverDelivered()
    {
        // Bind and immediately release a port so nothing is listening on it.
        var probe = new TcpListener(IPAddress.Loopback, 0);
        probe.Start();
        var deadPort = ((IPEndPoint)probe.LocalEndpoint).Port;
        probe.Stop();

        var c = new SketchupClient(_discoveryDir, pidAlive: _ => true);
        var ex = Assert.Throws<BridgeRequestNotDeliveredException>(
            () => c.SendRequest(deadPort, new JsonObject { ["type"] = "ping" }, timeoutMs: 2_000));
        Assert.Contains($"{deadPort}", ex.Message);
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
