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
/// <para>
/// <c>BridgeVersion</c> is the bridge RUNNING inside that session, which is not
/// necessarily the one installed on disk: SketchUp loads plugins once at startup,
/// so a session keeps whatever it loaded until it restarts. Null means the session
/// is running a bridge older than 0.35.0, which did not report its version.
/// </para>
/// </summary>
internal sealed record SketchupInstance(
    int Pid,
    int Port,
    string Version,
    string? ModelPath,
    DateTime StartedAt,
    string? BridgeVersion = null);

/// <summary>
/// The bridge call failed BEFORE the whole request had been handed to the socket, so the
/// bridge cannot have evaluated anything and the model is provably untouched. Verbs that
/// mutate the model rely on this distinction: for them "we don't know what happened" and
/// "nothing happened" are different receipts.
/// </summary>
internal sealed class BridgeRequestNotDeliveredException : Exception
{
    public BridgeRequestNotDeliveredException(string message, Exception? inner = null)
        : base(message, inner) { }
}

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
        try
        {
            var connectTask = tcp.ConnectAsync(IPAddress.Loopback, port);
            if (!connectTask.Wait(Math.Min(timeoutMs, 5_000)))
                throw new TimeoutException($"connect to 127.0.0.1:{port} timed out");
        }
        catch (Exception e)
        {
            throw new BridgeRequestNotDeliveredException($"could not connect to 127.0.0.1:{port}: {Unwrap(e).Message}", e);
        }

        tcp.ReceiveTimeout = timeoutMs;
        // The caller's timeout is the only bound that means anything: a multi-megabyte
        // bake payload is handed over as fast as the bridge's pump drains it, which a
        // fixed 10s ceiling could cut off mid-send on a busy model.
        tcp.SendTimeout    = timeoutMs;

        using var stream = tcp.GetStream();
        var body = Encoding.UTF8.GetBytes(request.ToJsonString());
        try
        {
            WriteLengthPrefixed(stream, body);
        }
        catch (Exception e)
        {
            // A partial frame is never evaluated — bridge 0.35.0 drops a connection that
            // ends before a complete frame arrives — so a failed write leaves the model
            // provably untouched, and the caller must not be told the outcome is unknown.
            throw new BridgeRequestNotDeliveredException($"could not send the request to 127.0.0.1:{port}: {Unwrap(e).Message}", e);
        }

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
    /// <paramref name="timeoutMs"/> bounds header and body TOGETHER: one deadline for
    /// the whole reply, not a fresh one per read.
    /// </summary>
    internal static byte[] ReadLengthPrefixed(Stream s, int timeoutMs)
    {
        var deadline = DateTime.UtcNow.AddMilliseconds(timeoutMs);
        var lenBuf = ReadExactly(s, 4, deadline);
        // Big-endian → int32.
        int len = (lenBuf[0] << 24) | (lenBuf[1] << 16) | (lenBuf[2] << 8) | lenBuf[3];
        if (len < 0 || len > 64 * 1024 * 1024)
            throw new InvalidDataException($"absurd message length: {len}");
        return ReadExactly(s, len, deadline);
    }

    /// <summary>
    /// Blocking read of exactly <paramref name="count"/> bytes, giving up at
    /// <paramref name="deadlineUtc"/>. Each individual read is capped at the time
    /// left, so a silent peer can't stall past the deadline.
    /// </summary>
    internal static byte[] ReadExactly(Stream s, int count, DateTime deadlineUtc)
    {
        var buf = new byte[count];
        int got = 0;
        while (got < count)
        {
            var remaining = (deadlineUtc - DateTime.UtcNow).TotalMilliseconds;
            if (remaining <= 0)
                throw new TimeoutException($"read {count} bytes timed out (got {got})");
            if (s.CanTimeout) s.ReadTimeout = (int)Math.Ceiling(remaining);
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
        // bridge_version arrived in bridge 0.35.0; older sessions simply omit it.
        var bridgeVersion = node["bridge_version"]?.GetValue<string>();
        if (string.IsNullOrEmpty(bridgeVersion)) bridgeVersion = null;
        // started_at is informational; we don't fail if it's malformed.
        DateTime started = DateTime.MinValue;
        var sa = node["started_at"]?.GetValue<string>();
        if (!string.IsNullOrEmpty(sa))
            DateTime.TryParse(sa, null, System.Globalization.DateTimeStyles.AssumeUniversal | System.Globalization.DateTimeStyles.AdjustToUniversal, out started);
        return new SketchupInstance(pid.Value, port.Value, version!, modelPath, started, bridgeVersion);
    }

    /// <summary>
    /// What the user has to DO when the bridge running in that session is not the fixed
    /// one — otherwise the empty string.
    /// <para>
    /// This is the blind spot behind aware-aeco/aware#330: installing a fixed bridge
    /// changes nothing until the session restarts, and a stale session is
    /// indistinguishable from a healthy one to <c>list-instances</c>, which only reads
    /// the discovery file. So say it on the paths where the user is already looking at
    /// a failure.
    /// </para>
    /// <para>
    /// Three versions matter and conflating them gives useless advice: what the session
    /// is RUNNING, what is INSTALLED in the Plugins folder (what a restart would load),
    /// and what this sidecar has PACKAGED (what <c>--install-bridge</c> would put there).
    /// Upgrading the CLI does not re-install the bridge, so "just restart" is wrong
    /// advice whenever the installed loader is itself behind.
    /// </para>
    /// </summary>
    internal static string StaleBridgeNote(
        SketchupInstance inst,
        string? packagedVersion = null,
        string? installedVersion = null)
    {
        var packaged  = packagedVersion  ?? BridgeInstaller.PackagedVersion();
        var installed = installedVersion ?? BridgeInstaller.InstalledVersion();

        // A restart loads the INSTALLED loader; fall back to the packaged version only
        // when we can't read the Plugins folder at all.
        var wouldLoad = string.IsNullOrEmpty(installed) ? packaged : installed;
        if (string.IsNullOrEmpty(wouldLoad)) return "";

        var needsInstall = !string.IsNullOrEmpty(packaged)
                        && !string.IsNullOrEmpty(installed)
                        && packaged != installed;
        var needsRestart = inst.BridgeVersion != wouldLoad;
        if (!needsInstall && !needsRestart) return "";

        var running = inst.BridgeVersion ?? "older than 0.35.0 (it does not report a version)";
        var head = $" — note: SketchUp pid {inst.Pid} is running bridge {running}";
        if (needsInstall)
            return head + $", the installed bridge is {installed} and this sidecar ships {packaged}:"
                 + " run `aware-sketchup --install-bridge`, then restart SketchUp — it only loads"
                 + " plugins at startup";
        return head + $" while {wouldLoad} is installed; SketchUp only loads plugins at startup,"
             + " so restart SketchUp to pick the installed bridge up";
    }

    /// <summary>Peels the AggregateException that Task.Wait wraps everything in.</summary>
    static Exception Unwrap(Exception e)
        => e is AggregateException agg && agg.InnerExceptions.Count == 1 ? agg.InnerExceptions[0] : e;

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
