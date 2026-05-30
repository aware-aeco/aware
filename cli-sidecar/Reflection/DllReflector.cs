using System.Xml.Linq;
using AwareReader;

namespace AwareSidecar.Reflection;

/// <summary>
/// The `aware build --from-dlls / --from-nuget` reflector. Resolves the DLL glob(s) to a
/// set of files, merges any sibling XML docs for member summaries, reflects the whole set
/// via <see cref="MetadataReflector.ReflectSet"/> (PE metadata only — no IL execution), and
/// synthesizes one coherent agent across the set via <see cref="AgentSynthesizer"/>.
/// </summary>
/// <remarks>
/// As of #180 this is a thin orchestrator: the type/member reflection lives in the shared
/// AwareReader library (so the source-based aware-roslyn reader produces the same IR), and
/// the agent projection lives in AgentSynthesizer (so compiled + source inputs and the
/// Tekla recipe all share one code path). The large-stack worker thread is retained as
/// defence-in-depth for very large product directories (hundreds of DLLs).
/// </remarks>
public static class DllReflector
{
    public static GeneratedAgent Reflect(string[] globs, string? agentIdOverride)
    {
        var dllPaths = ResolveGlobs(globs);
        if (dllPaths.Count == 0)
        {
            throw new InvalidOperationException($"no dlls matched globs: {string.Join(", ", globs)}");
        }

        // Run on a dedicated thread with a large stack as defence-in-depth. MetadataReader is
        // non-recursive, but processing thousands of type/method tokens from hundreds of DLLs
        // still benefits from extra stack headroom.
        GeneratedAgent result = default!;
        Exception? threadEx = null;

        var workerThread = new System.Threading.Thread(
            () =>
            {
                try { result = ReflectCore(dllPaths, agentIdOverride); }
                catch (Exception ex) { threadEx = ex; }
            },
            64 * 1024 * 1024); // 64 MB

        workerThread.Start();
        workerThread.Join();

        if (threadEx != null)
            System.Runtime.ExceptionServices.ExceptionDispatchInfo.Capture(threadEx).Throw();

        return result;
    }

    private static GeneratedAgent ReflectCore(List<string> dllPaths, string? agentIdOverride)
    {
        // Merge sibling XML docs (Type + method summaries) across the whole set, keyed by
        // canonical xml-doc id — the same key shape MetadataReflector emits for DocId.
        var docSummaries = new Dictionary<string, string>(StringComparer.Ordinal);
        foreach (var dll in dllPaths)
        {
            var xmlPath = Path.ChangeExtension(dll, ".xml");
            if (!File.Exists(xmlPath)) continue;
            foreach (var kv in LoadXmlDoc(xmlPath))
                docSummaries.TryAdd(kv.Key, kv.Value);
        }

        var set = MetadataReflector.ReflectSet(dllPaths);
        return AgentSynthesizer.Synthesize(set, new SynthesisOptions
        {
            AgentIdOverride = agentIdOverride,
            DocSummaries = docSummaries,
        });
    }

    private static List<string> ResolveGlobs(string[] globs)
    {
        var out_ = new List<string>();
        foreach (var g in globs)
        {
            // Case 1: literal file path.
            if (File.Exists(g))
            {
                out_.Add(Path.GetFullPath(g));
                continue;
            }
            // Case 2: bare directory path — auto-expand to *.dll so callers do not need to
            // append \*.dll manually (e.g. --from-dlls "C:\Tekla\bin").
            if (Directory.Exists(g))
            {
                foreach (var hit in Directory.EnumerateFiles(g, "*.dll"))
                    out_.Add(Path.GetFullPath(hit));
                continue;
            }
            // Case 3: glob pattern — split directory + pattern and enumerate.
            var dir = Path.GetDirectoryName(g);
            var pattern = Path.GetFileName(g);
            if (string.IsNullOrEmpty(dir)) dir = ".";
            if (!Directory.Exists(dir)) continue;
            foreach (var hit in Directory.EnumerateFiles(dir, pattern))
                out_.Add(Path.GetFullPath(hit));
        }
        return out_.Distinct(StringComparer.OrdinalIgnoreCase).ToList();
    }

    /// <summary>
    /// Loads a sibling XML doc file into a dictionary keyed by canonical member id
    /// (T:Foo, M:Foo.Bar(args), …) → collapsed summary text. Malformed docs are skipped.
    /// </summary>
    private static Dictionary<string, string> LoadXmlDoc(string path)
    {
        var dict = new Dictionary<string, string>(StringComparer.Ordinal);
        try
        {
            var doc = XDocument.Load(path);
            foreach (var member in doc.Descendants("member"))
            {
                var name = (string?)member.Attribute("name");
                if (string.IsNullOrEmpty(name)) continue;
                var summary = member.Element("summary")?.Value.Trim().Replace('\n', ' ').Replace('\r', ' ');
                if (!string.IsNullOrEmpty(summary)) dict[name] = summary;
            }
        }
        catch { /* skip malformed XML docs */ }
        return dict;
    }
}
