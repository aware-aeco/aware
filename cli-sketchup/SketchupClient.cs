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
    string? BridgeVersion = null,
    string? BridgeLoaderPath = null);

/// <summary>
/// The bridge call failed before a single byte of the request left the sidecar — no
/// connection — so the bridge cannot have evaluated anything and the model is provably
/// untouched. Verbs that mutate the model rely on this distinction: for them "we don't
/// know what happened" and "nothing happened" are different receipts. Deliberately NOT
/// used for write failures: a write can throw after the peer already has the whole frame.
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
        // ONE deadline for the whole call. Sending a multi-megabyte scene and then reading
        // the reply must share the caller's budget — giving each its own would let a
        // "two minute" bake take four.
        var deadline = DateTime.UtcNow.AddMilliseconds(timeoutMs);
        int Remaining() => (int)Math.Ceiling((deadline - DateTime.UtcNow).TotalMilliseconds);

        using var tcp = new TcpClient();
        try
        {
            var connectTask = tcp.ConnectAsync(IPAddress.Loopback, port);
            if (!connectTask.Wait(Math.Min(timeoutMs, 5_000)))
                throw new TimeoutException($"connect to 127.0.0.1:{port} timed out");
        }
        catch (Exception e)
        {
            // The ONLY provably-nothing-happened failure: no byte of the request was ever
            // handed to a socket. Once the write starts, a failure is ambiguous — the peer
            // may have received the whole frame and still reset the connection — so those
            // stay ordinary exceptions and keep the "outcome unknown" treatment.
            throw new BridgeRequestNotDeliveredException($"could not connect to 127.0.0.1:{port}: {Unwrap(e).Message}", e);
        }

        // A fixed ceiling here could cut off a large send on a busy model; the caller's
        // remaining budget is the honest bound.
        tcp.SendTimeout    = Math.Max(1, Remaining());
        tcp.ReceiveTimeout = Math.Max(1, Remaining());

        using var stream = tcp.GetStream();
        var body = Encoding.UTF8.GetBytes(request.ToJsonString());
        WriteLengthPrefixed(stream, body);

        var left = Remaining();
        if (left <= 0)
            throw new TimeoutException($"the request to 127.0.0.1:{port} used the whole {timeoutMs} ms budget before a reply could be read");
        var responseBytes = ReadLengthPrefixed(stream, left);
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
        // bridge_version and bridge_loader arrived in bridge 0.35.0; older sessions omit them.
        var bridgeVersion = node["bridge_version"]?.GetValue<string>();
        if (string.IsNullOrEmpty(bridgeVersion)) bridgeVersion = null;
        var bridgeLoader = node["bridge_loader"]?.GetValue<string>();
        if (string.IsNullOrEmpty(bridgeLoader)) bridgeLoader = null;
        // started_at is informational; we don't fail if it's malformed.
        DateTime started = DateTime.MinValue;
        var sa = node["started_at"]?.GetValue<string>();
        if (!string.IsNullOrEmpty(sa))
            DateTime.TryParse(sa, null, System.Globalization.DateTimeStyles.AssumeUniversal | System.Globalization.DateTimeStyles.AdjustToUniversal, out started);
        return new SketchupInstance(pid.Value, port.Value, version!, modelPath, started, bridgeVersion, bridgeLoader);
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
        var packaged = packagedVersion ?? BridgeInstaller.PackagedVersion();
        // Prefer the loader THIS session actually loaded (bridge 0.35.0+ advertises its
        // path, so a custom --plugins-dir is exact). Only fall back to the Plugins folder
        // of the session's own year — never the default year, which would compare a 2025
        // session against an unrelated 2026 install.
        var installed = installedVersion
                        ?? BridgeInstaller.ReadInstalledVersion(inst.BridgeLoaderPath ?? "")
                        ?? BridgeInstaller.InstalledVersion(BridgeInstaller.PluginYearForHostVersion(inst.Version));
        var running = inst.BridgeVersion ?? "older than 0.35.0 (it does not report a version)";
        var where = inst.BridgeLoaderPath is null ? "" : $" ({inst.BridgeLoaderPath})";
        var head = $" — note: SketchUp pid {inst.Pid} is running bridge {running}";

        // Installing only helps when the packaged bridge is NEWER than the installed one;
        // running --install-bridge over a newer installed bridge would downgrade it.
        var installIsBehind = BridgeInstaller.CompareBridgeVersions(packaged, installed) > 0;

        if (string.IsNullOrEmpty(installed))
        {
            // Can't see what a restart would load (unknown year, or a custom --plugins-dir).
            // Don't assert a fix; name both facts and let the user decide.
            if (string.IsNullOrEmpty(packaged) || inst.BridgeVersion == packaged) return "";
            return head + $" and this sidecar ships {packaged}, but the installed bridge could not be"
                 + " read; run `aware-sketchup --install-bridge` and restart SketchUp if this keeps"
                 + " failing";
        }

        if (installIsBehind)
            return head + $", the installed bridge is {installed}{where} and this sidecar ships"
                 + $" {packaged}: run `aware-sketchup --install-bridge`, then restart SketchUp — it"
                 + " only loads plugins at startup";

        if (inst.BridgeVersion != installed)
            return head + $" while {installed} is installed{where}; SketchUp only loads plugins at"
                 + " startup, so restart SketchUp to pick the installed bridge up";

        return "";
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
