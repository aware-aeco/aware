// SketchupClient — talks to the in-SketchUp Ruby bridge over TCP using a
// 4-byte big-endian length prefix + UTF-8 JSON body framing.
//
// Discovery: each running SketchUp PID writes %TEMP%\aware-sketchup\<pid>.json
// when its bridge boots. This client reads that folder, validates the PID is
// alive, returns the list. Stale files (PID exited without cleanup) are
// best-effort deleted.

using System.Diagnostics;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace AwareSketchup;

/// <summary>
/// One running SketchUp instance, as advertised by its discovery file.
/// </summary>
internal sealed record SketchupInstance(
    int Pid,
    int Port,
    string Version,
    string? ModelPath,
    DateTime StartedAt);

internal sealed class SketchupClient
{
    /// <summary>
    /// Directory that bridge instances write their discovery JSON into.
    /// Configurable for tests; defaults to %TEMP%\aware-sketchup\.
    /// </summary>
    public string DiscoveryDir { get; }

    readonly Func<int, bool> _pidAlive;

    public SketchupClient(string? discoveryDirOverride = null, Func<int, bool>? pidAlive = null)
    {
        DiscoveryDir = discoveryDirOverride
            ?? Environment.GetEnvironmentVariable("AWARE_SKETCHUP_DISCOVERY_DIR")
            ?? Path.Combine(Path.GetTempPath(), "aware-sketchup");
        _pidAlive = pidAlive ?? IsProcessAlive;
    }

    /// <summary>
    /// Returns every live SketchUp instance with a registered bridge.
    /// Stale discovery files are silently cleaned up.
    /// </summary>
    public List<SketchupInstance> ListInstances()
    {
        var result = new List<SketchupInstance>();
        if (!Directory.Exists(DiscoveryDir)) return result;

        foreach (var path in Directory.EnumerateFiles(DiscoveryDir, "*.json"))
        {
            SketchupInstance? inst;
            try
            {
                var json = File.ReadAllText(path);
                inst = ParseDiscoveryFile(json);
            }
            catch
            {
                // Corrupt file → drop it.
                TryDelete(path);
                continue;
            }

            if (inst is null)
            {
                TryDelete(path);
                continue;
            }

            if (!_pidAlive(inst.Pid))
            {
                // Bridge died without cleaning up. Drop the file.
                TryDelete(path);
                continue;
            }

            result.Add(inst);
        }

        return result;
    }

    /// <summary>
    /// Picks an instance matching the supplied filters.
    /// - If `sketchupId` (the PID-as-string used in the public AWARE API) is given,
    ///   the match is exact-PID.
    /// - Else if `version` is given, the first instance whose version startswith
    ///   that prefix wins.
    /// - Else if there is exactly one running instance, it wins.
    /// - Else returns null (caller decides whether ambiguity is an error).
    /// </summary>
    public SketchupInstance? Resolve(string? sketchupId, string? version)
    {
        var all = ListInstances();
        if (all.Count == 0) return null;

        if (!string.IsNullOrEmpty(sketchupId))
        {
            if (!int.TryParse(sketchupId, out var pid)) return null;
            return all.FirstOrDefault(i => i.Pid == pid);
        }

        if (!string.IsNullOrEmpty(version))
        {
            return all.FirstOrDefault(i => i.Version.StartsWith(version!, StringComparison.Ordinal));
        }

        return all.Count == 1 ? all[0] : null;
    }

    /// <summary>
    /// Connects to the bridge on the given port, ships the request JSON, returns
    /// the response JSON. Times out at <paramref name="timeoutMs"/>.
    /// </summary>
    public JsonNode SendRequest(int port, JsonObject request, int timeoutMs = 30_000)
    {
        using var tcp = new TcpClient();
        var connectTask = tcp.ConnectAsync(IPAddress.Loopback, port);
        if (!connectTask.Wait(Math.Min(timeoutMs, 5_000)))
            throw new TimeoutException($"connect to 127.0.0.1:{port} timed out");

        tcp.ReceiveTimeout = timeoutMs;
        tcp.SendTimeout    = Math.Min(timeoutMs, 10_000);

        using var stream = tcp.GetStream();
        var body = Encoding.UTF8.GetBytes(request.ToJsonString());
        WriteLengthPrefixed(stream, body);

        var responseBytes = ReadLengthPrefixed(stream, timeoutMs);
        var responseJson  = Encoding.UTF8.GetString(responseBytes);
        return JsonNode.Parse(responseJson)
               ?? throw new InvalidDataException("bridge returned non-JSON response");
    }

    /// <summary>
    /// Writes a 4-byte big-endian length prefix + the payload.
    /// </summary>
    internal static void WriteLengthPrefixed(Stream s, byte[] payload)
    {
        var lenBuf = new byte[4];
        // Big-endian (network byte order) — what Ruby's `unpack1('N')` reads.
        lenBuf[0] = (byte)((payload.Length >> 24) & 0xFF);
        lenBuf[1] = (byte)((payload.Length >> 16) & 0xFF);
        lenBuf[2] = (byte)((payload.Length >> 8)  & 0xFF);
        lenBuf[3] = (byte)(payload.Length         & 0xFF);
        s.Write(lenBuf, 0, 4);
        s.Write(payload, 0, payload.Length);
        s.Flush();
    }

    /// <summary>
    /// Reads a 4-byte big-endian length prefix + that many bytes.
    /// </summary>
    internal static byte[] ReadLengthPrefixed(Stream s, int timeoutMs)
    {
        var lenBuf = ReadExactly(s, 4, timeoutMs);
        // Big-endian → int32.
        int len = (lenBuf[0] << 24) | (lenBuf[1] << 16) | (lenBuf[2] << 8) | lenBuf[3];
        if (len < 0 || len > 64 * 1024 * 1024)
            throw new InvalidDataException($"absurd message length: {len}");
        return ReadExactly(s, len, timeoutMs);
    }

    /// <summary>
    /// Blocking read of exactly <paramref name="count"/> bytes from the stream.
    /// </summary>
    internal static byte[] ReadExactly(Stream s, int count, int timeoutMs)
    {
        var buf = new byte[count];
        int got = 0;
        var deadline = DateTime.UtcNow.AddMilliseconds(timeoutMs);
        while (got < count)
        {
            if (DateTime.UtcNow > deadline)
                throw new TimeoutException($"read {count} bytes timed out (got {got})");
            int n = s.Read(buf, got, count - got);
            if (n == 0)
                throw new EndOfStreamException($"stream closed after {got}/{count} bytes");
            got += n;
        }
        return buf;
    }

    /// <summary>
    /// Parses a discovery-file JSON body. Returns null if any field is missing.
    /// </summary>
    internal static SketchupInstance? ParseDiscoveryFile(string json)
    {
        var node = JsonNode.Parse(json) as JsonObject;
        if (node is null) return null;
        var pid     = node["pid"]?.GetValue<int?>();
        var port    = node["port"]?.GetValue<int?>();
        var version = node["version"]?.GetValue<string>();
        if (pid is null || port is null || string.IsNullOrEmpty(version)) return null;
        string? modelPath = node["model_path"]?.GetValue<string>();
        // started_at is informational; we don't fail if it's malformed.
        DateTime started = DateTime.MinValue;
        var sa = node["started_at"]?.GetValue<string>();
        if (!string.IsNullOrEmpty(sa))
            DateTime.TryParse(sa, null, System.Globalization.DateTimeStyles.AssumeUniversal | System.Globalization.DateTimeStyles.AdjustToUniversal, out started);
        return new SketchupInstance(pid.Value, port.Value, version!, modelPath, started);
    }

    static bool IsProcessAlive(int pid)
    {
        try
        {
            using var p = Process.GetProcessById(pid);
            return !p.HasExited;
        }
        catch
        {
            return false;
        }
    }

    static void TryDelete(string path)
    {
        try { File.Delete(path); } catch { /* best effort */ }
    }
}
